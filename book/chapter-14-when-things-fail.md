# Chapter 14: When Things Fail

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 9's loop can stop, stall, and wrap up. It still treats a failure as one outcome. An empty reply, a truncated reply, a repeated paragraph, a timeout, and a model that should have been the fallback all look different and need different next steps. Two dangerous commands also still sit outside the approval floor: deleting the binary that is running, and rewriting this crate's own git tree. This chapter classifies the failure and closes those two floor checks. The crate is `steps/ch14-when-things-fail`, a copy of chapter 13 plus a `failure` module. The package is `ch14-when-things-fail`. The library is `harness_core`.

`Kind` is the classification. `retry_budget` and `EmptyAttempt` implement the empty-response rule chapter 9 deferred: three retries, or one if a single attempt costs more than a quarter dollar. `deterministic_empty` refuses to retry a reply that will be empty again for the same reason. `is_repetition_dominated` is the repetition guard: a run of the same text covering enough of the output stops the loop. `length_verdict` is the continuation decision when a reply was cut off. `run_models` walks a route, which is the seed of chapter 18's fallback. `call_with_timeout` cancels the in-flight call and releases its permit, so a stuck call does not pin one of the eight slots. `retry_in_bound` keeps the retries inside that same bound. `FakeMcp` is the stand-in for a server that crashes, times out, or needs a reconnect.

`approval::check` calls `failure::command_deletes_runtime` and `failure::self_repo_git_mutation` before the fast path and before YOLO. Both denials are hardline. The runtime path is `std::env::current_exe()`. The source root is `CARGO_MANIFEST_DIR` of this step. The detectors cover `rm` and its Windows cousins, `find … -delete`, and `uv python uninstall` when the spec is the runtime. Overlap works in both directions, so deleting a parent of the binary is a hit. Alias lookup and heredocs are not parsed, matching the limit Hermes itself documents. A global `git -C` and the wrappers `sudo`, `command`, `time`, `nohup`, and `env` are stripped before the git check. YOLO still denies both.

Chapter 16 copies this harness, not chapter 15. Chapter 15 is the server. The failure types stay on the harness line because they classify the model's behavior and the shell the harness is about to run. A self-improving agent that retries an empty reply forever, or that can `git reset --hard` the tree it is learning inside, will destroy the session it was supposed to learn from. Chapter 52's stress set still includes the stall and the recovery. This chapter is where those failures become named kinds instead of one more stop.

Rust, taught at the match: `Kind` and `Reply` are enums, and `OrderedF64` exists because floating-point cost does not implement the ordering you want in a `BTree` or a sort. `Result` is the floor's denial. Matching on the error is the whole design of the retry.

`cargo run -p ch14-when-things-fail` runs the demo. The exercise is `my-harness/tests/ch14.rs`: YOLO still denies deleting the running binary and `git reset --hard` of this crate's manifest directory.

**In this chapter**

- 14.1 `Kind`
- 14.2 `EmptyAttempt`
- 14.3 `retry_budget`
- 14.4 `deterministic_empty`
- 14.5 `is_repetition_dominated`
- 14.6 `length_verdict`
- 14.7 `Reply` and `OrderedF64`
- 14.8 `run_models`
- 14.9 `run_one`
- 14.10 `call_with_timeout`
- 14.11 `retry_in_bound`
- 14.12 `FakeMcp`
- 14.13 The top of `check`
- 14.14 `command_deletes_runtime`
- 14.15 `path_hits`
- 14.16 `uv_uninstalls_runtime`
- 14.17 `strip_wrappers`
- 14.18 `self_repo_git_mutation`
- 14.19 `locate_git`
- 14.20 `word_hits`

| Section | Step | What it adds |
|---|---|---|
| 14 | `steps/ch14-when-things-fail` | `Kind`, empty retries, repetition, a timeout that drops its permit |

> **How Hermes does it**
>
> I read `agent/empty_response_guard.py`, `agent/repetition_guard.py`, `agent/turn_truncation.py`, and `agent/deadline.py` (`kill_process_tree` is chapter 13; the timeout class is this chapter) on Hermes `main` at `cf71b1aa`.
>
> An empty completion retries up to 3 times. If one attempt costs more than $0.25, the budget drops to 1. Two consecutive empties with usage present and zero output tokens, from the same model, provider, and finish reason, skip the rest and go to the fallback. `is_repetition_dominated` is true when the text is at least 400 characters and a 60-character window repeats enough to cover at least half of it, with a floor of 5 repeats. `REPETITION_LOOP_INTERRUPTED` is the text stored instead of the loop. `turn_truncation.py` continues a `finish_reason == "length"` reply, and the repetition guard refuses to continue a loop.
>
> **Adopted.** The budgets 3 and 1, the $0.25 threshold (strictly greater), the two-empty deterministic rule, the 400 / 60 / 5 / half constants, and the interrupted-loop sentence. A length reply that is not a loop is kept and marked truncated. A crashed fake session reconnects and the next call returns `pong`.
>
> **Adapted.** The repetition check is the window count. I did not port the line fast path or the anchor scan. Fallback is the next `Model` in a slice, not Hermes's provider chain. The timeout uses `tokio::time::timeout` and drops an owned semaphore permit. Retries take that permit one at a time.
>
> **Deferred.** Preflight compression before the model call stays chapter 34. Think-block stripping is chapter 19. Rate-limit backoff on the server is chapter 15.

[personal note: an empty reply you would have retried, and what the cost did to the budget]

## What this chapter decides

`Kind` is `Empty`, `Truncated`, `Repeated`, `Timeout`, `RateLimit`, `Transport`, or `Crash`.

`retry_budget(None)` is 3. `retry_budget(Some(0.25))` is still 3. `retry_budget(Some(0.26))` is 1. The threshold is exceeded, not met.

`deterministic_empty` needs two attempts. Both must have usage, both must have zero output, and the model, provider, and finish reason must match. Missing usage does not count. The fake turn then keeps going until the budget is spent. Three unclear empties and then the text `ok` stay on the first model.

Two zero-output empties skip the rest of that model, including a later text reply, and the next model answers. The demo is that case with a cost of $0.30, so the reduced budget and the deterministic rule agree. The second model says `hello`. `fallbacks` is 1.

`is_repetition_dominated` returns false under 400 characters. `"abcdefghij"` repeated to 800 characters returns true. `length_verdict("length", that)` is `StopRepeated`, and `run_models` stores the interrupted-loop sentence. A short length reply is `Continue`. A `stop` finish is `Finished`.

`call_with_timeout` holds one permit from a semaphore of 8 and drops it when a 30 millisecond timer beats a 5 second sleep. `retry_in_bound` peaks at one permit. `FakeMcp::new(1)` reconnects once and returns `pong`.

`floor_runtime_and_repo` runs before `fast_safe` and before YOLO. The runtime path is `std::env::current_exe()`. The source root is `env!("CARGO_MANIFEST_DIR")`. A hit is `Deny` with `hardline: true`. `echo hello` stays `Allow`.

## 14.1 `Kind`

`failure.rs` has two jobs. The first is the failure policy: a kind, an empty-reply budget, a repetition scan, a timeout that drops its permit, and a fake crashed session. Chapter 9 deferred the empty retries and the repetition guard. They land here, and this walk does not change their rules. The second job is at the bottom of the same file. `command_deletes_runtime` and `self_repo_git_mutation` are what the approval floor calls. I read the detectors first.

The module opens with the constants the tests match against.

```rust
pub const DEFAULT_EMPTY_RETRY_BUDGET: u32 = 3;
pub const REDUCED_EMPTY_RETRY_BUDGET: u32 = 1;
pub const COST_THRESHOLD_USD: f64 = 0.25;
pub const MIN_FRAGMENT_LENGTH: usize = 400;
pub const REPEAT_WINDOW: usize = 60;
pub const MIN_REPEAT_COUNT: usize = 5;
pub const DOMINANCE_RATIO: f64 = 0.5;
pub const PERMIT_BOUND: usize = 8;

pub const REPETITION_NOTE: &str =
    "[the reply degenerated into a repetition loop and was interrupted]";
```

`Kind` is the name a turn stores when it stops for a reason other than ordinary text.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Empty,
    Truncated,
    Repeated,
    Timeout,
    RateLimit,
    Transport,
    Crash,
}
```

`Copy` is why `run_one` can write `kind: Some(*kind)`. The star copies the kind out of the borrowed reply. `RateLimit` and `Transport` are names in the enum. The fake models in this chapter's tests do not construct them.

## 14.2 `EmptyAttempt`

`EmptyAttempt` is one empty reply reduced to the fields the deterministic rule compares.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyAttempt {
    pub model: String,
    pub provider: String,
    pub finish_reason: String,
    pub usage_present: bool,
    pub zero_output: bool,
}

impl EmptyAttempt {
    fn signature(&self) -> (String, String, String) {
        (
            self.model.clone(),
            self.provider.clone(),
            self.finish_reason.clone(),
        )
    }
}
```

`signature` is private. It clones three strings into a tuple so `==` can compare them. The bools stay outside the tuple, so two attempts can share a signature and still disagree on `zero_output`.

## 14.3 `retry_budget`

`retry_budget` looks at one optional cost.

```rust
pub fn retry_budget(attempt_cost_usd: Option<f64>) -> u32 {
    match attempt_cost_usd {
        Some(cost) if cost > COST_THRESHOLD_USD => REDUCED_EMPTY_RETRY_BUDGET,
        _ => DEFAULT_EMPTY_RETRY_BUDGET,
    }
}
```

> **Rust sidebar: `Option` and a match guard**
> - `Option<f64>` is either `Some(cost)` or `None`. There is no third case.
> - `Some(cost) if cost > COST_THRESHOLD_USD` is a pattern plus a condition. Both have to hold.
> - The `_` arm is everything else: `None`, and a `Some` whose cost is not greater than `0.25`.
> - `0.25` itself takes the `_` arm. The test expects `retry_budget(Some(0.25))` to be 3. `Some(0.26)` is 1.

## 14.4 `deterministic_empty`

`deterministic_empty` looks only at the last pair.

```rust
pub fn deterministic_empty(attempts: &[EmptyAttempt]) -> bool {
    let Some(pair) = attempts.windows(2).last() else {
        return false;
    };
    let (prev, last) = (&pair[0], &pair[1]);
    prev.usage_present
        && last.usage_present
        && prev.zero_output
        && last.zero_output
        && prev.signature() == last.signature()
}
```

> **Rust sidebar: `windows` and `let`/`else`**
> - `windows(2)` on a slice yields overlapping pairs: indexes `(0, 1)`, then `(1, 2)`, and so on.
> - `.last()` is the final pair. A log of three attempts compares the second and the third. It does not search the whole log for any matching pair.
> - One attempt has no pair. `last()` is `None`. `let Some(pair) = ... else { return false }` is the early return for that case.
> - `pair` is a `&[EmptyAttempt]` of length 2. `pair[0]` and `pair[1]` are references, so the `let` binds `&pair[0]` and `&pair[1]`.

Both attempts need usage. Both need zero output tokens. The model, the provider, and the finish reason have to match. A missing usage flag makes the `&&` chain false. The test builds one attempt, checks that a single attempt is false, checks that two clones are true, then flips `zero_output` on the second and checks false.

## 14.5 `is_repetition_dominated`

The repetition scan is the other detector that was already in the file.

```rust
pub fn is_repetition_dominated(text: &str) -> bool {
    let n = text.len();
    if n < MIN_FRAGMENT_LENGTH {
        return false;
    }
    let needed =
        MIN_REPEAT_COUNT.max((n as f64 * DOMINANCE_RATIO / REPEAT_WINDOW as f64).ceil() as usize);
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut start = 0;
    while start + REPEAT_WINDOW <= n {
        if text.is_char_boundary(start) && text.is_char_boundary(start + REPEAT_WINDOW) {
            let window = &text[start..start + REPEAT_WINDOW];
            let count = counts.entry(window).or_insert(0);
            *count += 1;
            if *count >= needed {
                return true;
            }
        }
        start += 1;
    }
    false
}
```

`text.len()` is bytes. Under 400 the function returns false and does not build the map. `"ab"` repeated 10 times is 20 bytes. The test expects false. A one-line stutter stays under the floor.

`needed` is the larger of 5 and the ceiling of `n * 0.5 / 60`. For 800 bytes that ceiling is 7, so a 60-byte window has to show up at least 7 times. Just over 400 bytes, half the text divided by 60 is under 5, and the floor of 5 is what binds.

The map key is `&str`, borrowed from `text`. The window is not an owned `String`. `entry` then `or_insert(0)` returns `&mut usize`, and `*count += 1` writes through it. The first window that reaches `needed` returns true. The scan stops there.

`is_char_boundary` keeps the slice from panicking on a byte in the middle of a UTF-8 character. The loop steps one byte. ASCII starts are all boundaries. `"abcdefghij"` repeated 80 times is the 800-byte string `repeated_fragment` builds. The same 60-byte slice lines up every 10 bytes, the count crosses 7, and the function returns true. The bench calls `repetition_probe`, a one-line forward to this function, on a string built before the timer.

## 14.6 `length_verdict`

`length_verdict` is what a length finish does with that bool.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthVerdict {
    Continue { kept: String },
    StopRepeated,
    Finished,
}

pub fn length_verdict(finish_reason: &str, text: &str) -> LengthVerdict {
    if finish_reason != "length" {
        return LengthVerdict::Finished;
    }
    if is_repetition_dominated(text) {
        LengthVerdict::StopRepeated
    } else {
        LengthVerdict::Continue {
            kept: text.to_string(),
        }
    }
}
```

A finish reason other than `"length"` is `Finished`. The test passes `"stop"` and a short sentence. A `"length"` finish that fails the repetition check is `Continue`, and `kept` owns a copy of the fragment. A `"length"` finish that passes is `StopRepeated`. The loop text is not what we keep.

## 14.7 `Reply` and `OrderedF64`

The fake model has to store a cost the tests can match on. `f64` is not `Eq`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    Empty {
        cost: Option<OrderedF64>,
        usage_present: bool,
        zero_output: bool,
        finish_reason: String,
    },
    Text(String),
    Length(String),
    Fail(Kind),
}

/// `f64` cannot be `Eq`. The fake model stores cents so tests can match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderedF64 {
    pub micros: i64,
}

impl OrderedF64 {
    pub fn usd(dollars: f64) -> Self {
        Self {
            micros: (dollars * 1_000_000.0).round() as i64,
        }
    }

    pub fn as_f64(self) -> f64 {
        self.micros as f64 / 1_000_000.0
    }
}
```

> **Rust sidebar: `Eq` on a float**
> - `PartialEq` for `f64` exists. `Eq` does not, because `NaN` is not equal to itself.
> - `Reply` derives `Eq`. A field of type `f64` would make that derive fail.
> - `OrderedF64` stores `micros`, an `i64`. `usd` multiplies by one million and rounds. `as_f64` divides back.

The comment above the struct says cents. The field is `micros`, and `usd` multiplies by `1_000_000.0`. I follow the field.

## 14.8 `run_models`

`Model` and `Turn` are the slice element and the value `run_models` returns.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    pub name: String,
    pub provider: String,
    pub replies: Vec<Reply>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turn {
    pub text: String,
    pub kind: Option<Kind>,
    pub attempts: u32,
    pub fallbacks: u32,
}
```

`name` and `provider` are what `run_one` copies into each `EmptyAttempt`. `attempts` is how far into that model's reply list the turn got. `fallbacks` is how many models were skipped before the one that answered. `kind: None` means the turn ended on ordinary text.

`run_models` walks the slice. The first model that produces a turn wins.

```rust
pub fn run_models(models: &[Model]) -> Turn {
    let mut fallbacks = 0;
    for (index, model) in models.iter().enumerate() {
        if index > 0 {
            fallbacks += 1;
        }
        if let Some(turn) = run_one(model, fallbacks) {
            return turn;
        }
    }
    Turn {
        text: String::new(),
        kind: Some(Kind::Empty),
        attempts: 0,
        fallbacks,
    }
}
```

`index > 0` means the first model is not a fallback. Each later model increments `fallbacks` before `run_one`. `None` from `run_one` gives that model up. If every model returns `None`, the turn is empty text, `Kind::Empty`, and zero attempts.

## 14.9 `run_one`

`run_one` is the per-model loop. The empty arm is where the budget and the deterministic rule meet.

```rust
fn run_one(model: &Model, fallbacks: u32) -> Option<Turn> {
    let mut attempts_log = Vec::new();
    let mut empties = 0u32;
    let mut budget = DEFAULT_EMPTY_RETRY_BUDGET;
    for (index, reply) in model.replies.iter().enumerate() {
        let attempts = index as u32 + 1;
        match reply {
            Reply::Text(text) => {
                return Some(Turn {
                    text: text.clone(),
                    kind: None,
                    attempts,
                    fallbacks,
                });
            }
            Reply::Length(text) => {
                return Some(match length_verdict("length", text) {
                    LengthVerdict::StopRepeated => Turn {
                        text: REPETITION_NOTE.into(),
                        kind: Some(Kind::Repeated),
                        attempts,
                        fallbacks,
                    },
                    LengthVerdict::Continue { kept } => Turn {
                        text: kept,
                        kind: Some(Kind::Truncated),
                        attempts,
                        fallbacks,
                    },
                    LengthVerdict::Finished => Turn {
                        text: text.clone(),
                        kind: None,
                        attempts,
                        fallbacks,
                    },
                });
            }
            Reply::Fail(kind) => {
                return Some(Turn {
                    text: String::new(),
                    kind: Some(*kind),
                    attempts,
                    fallbacks,
                });
            }
            Reply::Empty {
                cost,
                usage_present,
                zero_output,
                finish_reason,
            } => {
                if empties == 0 {
                    budget = retry_budget(cost.map(OrderedF64::as_f64));
                }
                empties += 1;
                attempts_log.push(EmptyAttempt {
                    model: model.name.clone(),
                    provider: model.provider.clone(),
                    finish_reason: finish_reason.clone(),
                    usage_present: *usage_present,
                    zero_output: *zero_output,
                });
                if deterministic_empty(&attempts_log) || empties > budget {
                    return None;
                }
            }
        }
    }
    None
}
```

`Reply::Text` returns immediately with `kind: None`. `attempts` is the 1-based index in the script. `Reply::Length` calls `length_verdict("length", text)`. That finish reason means `Finished` does not come back from this call site. The arm is still in the `match` because the enum has three variants. `StopRepeated` stores `REPETITION_NOTE` and `Kind::Repeated`. `Continue` stores `kept` and `Kind::Truncated`. `Reply::Fail` copies the `Kind` and returns an empty string. It does not consult the empty budget.

The empty arm is the one to read slowly. `budget` starts at 3. `if empties == 0` replaces it, and that test runs before `empties += 1`. A later empty does not call `retry_budget` again. `cost.map(OrderedF64::as_f64)` turns `Option<OrderedF64>` into `Option<f64>`. `None` stays `None`, and `retry_budget(None)` is 3.

After the log push, the give-up test is `deterministic_empty(&attempts_log) || empties > budget`. The comparison is `>`. A budget of 3 lets the first, second, and third empty continue, and the fourth empty returns `None`.

That is why three unclear empties and then the text `ok` stay on the first model, with `fallbacks` still 0. The test `missing_usage_keeps_the_full_empty_budget` is that script.

Two matching zero-output empties take the other half of the `||`. On the second empty, `deterministic_empty` is true while `empties` is 2, and `2 > 3` is still false. `run_one` returns `None` anyway. The later `Reply::Text("late")` on that same model is never read. `run_models` moves to the next model, counts one fallback, and returns `hello`. The test `deterministic_empties_skip_to_the_fallback` uses a cost of `0.01`, so the budget stays 3. The deterministic rule is what skips, not the reduced budget.

The demo uses $0.30 so the two rules agree. `demo_turn` builds a primary model with two empties at `OrderedF64::usd(0.30)`, usage present, zero output, finish reason `"stop"`. The side model has one text reply, `hello`. The first empty sets the budget to 1, because `0.30 > 0.25`. The second empty trips both `deterministic_empty` and `empties > 1`. `run_one` returns `None`. The side model returns `hello` with `fallbacks == 1`.

A reply list that ends on an empty which did not trip the test falls out of the loop and returns `None`.

## 14.10 `call_with_timeout`

The permit helpers are the timeout half of the same file. `agent.rs` does not call them. The tests do. The floor does not.

```rust
pub struct ToolPermits {
    sem: Arc<Semaphore>,
}

impl ToolPermits {
    pub fn new(bound: usize) -> Self {
        Self {
            sem: Arc::new(Semaphore::new(bound)),
        }
    }

    pub fn available(&self) -> usize {
        self.sem.available_permits()
    }

    pub async fn acquire(&self) -> OwnedSemaphorePermit {
        self.sem
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore open")
    }
}
```

`Semaphore::new(bound)` starts with that many permits. `acquire` clones the `Arc` and calls `acquire_owned`, so the permit holds its own pointer. The `expect` is the closed-semaphore path. This code never closes it.

```rust
pub async fn call_with_timeout<F, Fut>(
    permits: &ToolPermits,
    timeout: Duration,
    work: F,
) -> Result<String, Kind>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = String>,
{
    let permit = permits.acquire().await;
    let result = tokio::time::timeout(timeout, work()).await;
    drop(permit);
    match result {
        Ok(text) => Ok(text),
        Err(_) => Err(Kind::Timeout),
    }
}
```

> **Rust sidebar: the permit is dropped on both arms**
> - `work` is `FnOnce`. Calling `work()` consumes it and produces a future. `timeout` races that future against the duration.
> - `Ok(text)` is the future finishing. `Err` is the timer. The error type is `Elapsed`. The match binds it as `_`.
> - `drop(permit)` sits before the `match`. Both arms have already released the slot when they return.
> - When the timer wins, `timeout` drops `work()`'s future. Dropping that future is what cancels the sleep. The permit drop is a separate line. The sleep's cancel does not, by itself, return the slot. The `drop` does.

The test uses `ToolPermits::new(PERMIT_BOUND)`. The work sleeps five seconds and would return `"late"`. The timeout is 30 milliseconds. The result is `Err(Kind::Timeout)`, `available()` is 8 again, and elapsed time is under two seconds. The sleep's future was dropped.

## 14.11 `retry_in_bound`

`retry_in_bound` takes a permit, records the peak, and drops it before the next try.

```rust
pub async fn retry_in_bound(permits: &ToolPermits, times: u32, peak: &AtomicUsize) -> u32 {
    let mut done = 0;
    for _ in 0..times {
        let permit = permits.acquire().await;
        let now = PERMIT_BOUND - permits.available();
        peak.fetch_max(now, Ordering::SeqCst);
        drop(permit);
        done += 1;
    }
    done
}
```

`now` is `PERMIT_BOUND` minus `available()`. The test builds the semaphore with that constant, 8, so one held permit makes `now` equal 1. `fetch_max` only raises `peak`. Three iterations still peak at 1, because the permit is dropped before the next acquire. The test expects `done == 3` and 8 permits free at the end.

## 14.12 `FakeMcp`

`FakeMcp` is the crashed session. It is a counter, not a socket.

```rust
#[derive(Debug)]
pub struct FakeMcp {
    crashes_left: u32,
    reconnects: u32,
}

impl FakeMcp {
    pub fn new(crashes_left: u32) -> Self {
        Self {
            crashes_left,
            reconnects: 0,
        }
    }

    pub fn reconnects(&self) -> u32 {
        self.reconnects
    }

    pub fn call(&mut self) -> Result<&'static str, Kind> {
        if self.crashes_left > 0 {
            self.crashes_left -= 1;
            self.reconnects += 1;
            return Err(Kind::Crash);
        }
        Ok("pong")
    }

    pub fn call_until_up(&mut self) -> Result<&'static str, Kind> {
        loop {
            match self.call() {
                Err(Kind::Crash) if self.reconnects < 3 => continue,
                other => return other,
            }
        }
    }
}
```

`call` returns `Err(Kind::Crash)` while `crashes_left` is positive, and each failure increments `reconnects`. `call_until_up` loops on `Crash` while `reconnects < 3`, and that guard runs after `call` has already incremented the counter. `FakeMcp::new(1)` crashes once. `reconnects` becomes 1, `1 < 3` continues, and the next `call` returns `Ok("pong")`. The test expects one reconnect. On the third crash `reconnects` is 3, `3 < 3` is false, and that `Err` is returned. There is no fourth call. The test uses one.

`repeated_fragment` is `"abcdefghij".repeat(80)`. `repetition_probe` calls `is_repetition_dominated`. `demo_turn` is the two-model script the binary prints. None of them look at a shell command. The rest of the file is the floor.

## 14.13 The top of `check`

`approval::check` is the pipeline from chapter 11, copied forward with the harness. This chapter inserts one call at the top. `run_loop` reaches that call through `deny_tool`, which uses `Policy::manual()` for a `terminal` command. `execute_code` goes to `sandbox::check_execute_code` and skips this floor.

```rust
pub fn check(policy: &Policy, session: &mut Session, command: &str) -> Verdict {
    if let Some(message) = floor_runtime_and_repo(command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message,
            hardline: true,
        };
    }
    if policy.deny.is_empty() && fast_safe(command) {
        session.denials = 0;
        return Verdict::Allow { risk: Risk::Safe };
    }
    if let Some(description) = hardline(command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message: hardline_message(&description),
            hardline: true,
        };
    }
    if pipes_password_to_sudo(command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message: format!(
                "BLOCKED: password piping to sudo -S. Do not pipe passwords to 'sudo -S'.{STOP_COMMAND}"
            ),
            hardline: false,
        };
    }
    if let Some(rule) = deny_match(policy, command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message: format!(
                "BLOCKED: user deny rule `{rule}` matched.{STOP_COMMAND}{}",
                breaker_addendum(session, policy)
            ),
            hardline: false,
        };
    }
    if policy.yolo || policy.mode == Mode::Off {
        session.denials = 0;
        return Verdict::Allow {
            risk: risk_of(command),
        };
    }
```

The floor is the first `if`. A `Some(message)` increments `denials` with `saturating_add` and returns `Deny`. `hardline` is `true` for both messages the floor can produce. The runtime-delete message and the git-rewrite message take the same variant. YOLO is the last `if` in this excerpt. A command that already returned never reaches it. Mode `Off` is the same branch. The floor does not read `policy.yolo`.

`floor_runtime_and_repo` is the whole guard.

```rust
fn floor_runtime_and_repo(command: &str) -> Option<String> {
    if let Ok(runtime) = std::env::current_exe() {
        let runtime = runtime.display().to_string();
        if crate::failure::command_deletes_runtime(command, &runtime) {
            return Some(format!(
                "BLOCKED (hardline): recursive/any delete of the running harness binary ({runtime}). This command is on the unconditional blocklist and cannot be executed via the agent — not even with --yolo, /yolo, approvals.mode=off, or cron approve mode."
            ));
        }
    }
    let root = env!("CARGO_MANIFEST_DIR");
    if crate::failure::self_repo_git_mutation(command, root) {
        return Some(
            "BLOCKED: this command would rewrite the live checkout backing this process. Run it on a copy, not on the source this binary was built from.".to_string(),
        );
    }
    None
}
```

> **Rust sidebar: `env!` versus `std::env::current_exe`**
> - `std::env::current_exe()` runs when `check` runs. It returns the path of this process. A test binary lives under `target/`. `Ok` is a `PathBuf`. `display()` is the lossy string we hand to the detector. `Err` skips the runtime check and still runs the git check.
> - `env!("CARGO_MANIFEST_DIR")` is the crate directory at compile time. Cargo sets it. The macro pastes a string literal into the binary. `cd` does not change it.
> - The two calls protect different paths. One is the binary that is running. The other is the source tree that binary was built from. A test that passed the manifest dir as the runtime path would be checking the wrong overlap.

The comment on the function points the runtime delete at Hermes's `_floor_block` and the git detector at `tools/self_repo_guard.py`. It also says Hermes turns that git block on only for Windows (`guard_active`), and this harness runs the detector on every platform. A rewrite of the checkout this binary was built from is the same hazard. The denial text says to run the command on a copy.

`fast_safe` is why the floor has to be first.

```rust
fn fast_safe(command: &str) -> bool {
    let bytes = command.as_bytes();
    if bytes.iter().any(|byte| {
        matches!(
            byte,
            b'"' | b'\'' | b'$' | b'`' | b'|' | b';' | b'>' | b'(' | b')' | b'&'
        )
    }) {
        return false;
    }
    const WORDS: &[&str] = &[
        "rm", "sudo", "mkfs", "dd", "curl", "kill", "shutdown", "reboot", "halt", "poweroff",
        "python", "python3", "perl", "ruby", "node", "bash", "sh", "eval", "nohup",
    ];
    !WORDS.iter().any(|word| contains_word_ci(command, word))
}
```

A quote, dollar, backtick, pipe, semicolon, redirect, parenthesis, or ampersand means the command is not fast-safe. So does one of the listed words as a whole word. `git` and `checkout` are absent from `WORDS`, and `git checkout main` has none of those bytes, so `fast_safe` returns true. With an empty deny list, `check` would `Allow` that command and clear `denials` before `hardline` or YOLO. `floor_runtime_and_repo` has already returned `Deny` for a checkout aimed at the manifest directory.

`echo hello` still passes. No special bytes, and `echo` is not in `WORDS`, so `fast_safe` is true. The test sets `policy.yolo = true` and expects `Allow`. The floor returned `None`. The same policy denies `rm` of `current_exe` and `git -C <manifest> reset --hard` with `hardline: true`.

The arms between the fast path and YOLO are the chapter 11 detectors. `hardline` denies a recursive `rm` of `/`, `/*`, `/.`, `/./`, or `//`, or of `/etc`, `/boot`, `/usr`, `~`, or `$home`, plus `mkfs`, `dd` onto `/dev/sd` or `/dev/nvme`, a fork bomb (`:(){` with `:|:&`), `kill -1`, and `shutdown`, `reboot`, `halt`, or `poweroff`. Quoted spans are blanked, and an `sh -c`, `bash -c`, or `eval` payload is checked too. `pipes_password_to_sudo` denies a masked command that contains `-S` and `sudo`. `deny_match` denies a user rule found in the masked, wrapper-stripped command. Those three return before YOLO. Past YOLO, a command that `dangerous` does not match is `Allow`. `dangerous` matches a recursive `rm`, a token under `/etc` or containing `/.ssh` or `/.env`, a `curl` upload, and `python`, `python3`, `perl`, `ruby`, or `node` with `-c` or `-e`. Unattended deny, smart review, and `Ask` run only when YOLO has not returned.

`rm -rf /` misses the runtime detector. Section 14.15 shows why `/` fails `path_hits`. `fast_safe` returns false on the word `rm`, and `hardline` denies the command. `hardline_survives_yolo_and_mode_off` expects `recursive delete of root filesystem` in the message.

## 14.14 `command_deletes_runtime`

The deleting commands and the git subcommands are tables at the bottom of `failure.rs`, above the functions.

```rust
const GIT_MUTATIONS: &[&str] = &[
    "checkout",
    "switch",
    "rebase",
    "merge",
    "pull",
    "restore",
    "clean",
    "cherry-pick",
    "revert",
    "bisect",
];
const RESET_MODES: &[&str] = &["--hard", "--merge", "--keep"];
const DELETING: &[&str] = &["rm", "rmdir", "rd", "del", "erase", "remove-item", "ri"];
```

`DELETING` is the program names. `ri` is the short form of `remove-item`. The git tables are the same block. Section 14.18 is where a subcommand is read against them. The doc comment on `command_deletes_runtime` says the caller names the runtime path, because this harness has no `sys.executable`.

```rust
pub fn command_deletes_runtime(command: &str, runtime_path: &str) -> bool {
    if command.is_empty() || runtime_path.is_empty() {
        return false;
    }
    let normalized = command.replace("&&", "\n").replace("||", "\n");
    for segment in normalized.split([';', '\n', '|']) {
        let words = strip_wrappers(segment);
        if words.is_empty() {
            continue;
        }
        let name = words[0]
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(words[0])
            .to_ascii_lowercase();
        if name == "uv" && uv_uninstalls_runtime(&words, runtime_path) {
            return true;
        }
        if name == "find"
            && (segment.contains("-delete") || segment.contains("-exec rm"))
            && words
                .iter()
                .skip(1)
                .any(|word| !word.starts_with('-') && path_hits(word, runtime_path))
        {
            return true;
        }
        if !DELETING.iter().any(|item| *item == name) {
            continue;
        }
        if words
            .iter()
            .skip(1)
            .any(|word| !word.starts_with('-') && path_hits(word, runtime_path))
        {
            return true;
        }
    }
    false
}
```

An empty command or an empty runtime path is false. There is nothing to overlap.

`normalized` is an owned `String`. `replace("&&", "\n")` runs first, then `replace("||", "\n")`. A chain joined by `&&` or `||` becomes several lines. `split([';', '\n', '|'])` then breaks on semicolon, newline, and a single pipe. A single `&` is not in that list. Backgrounding with `&` stays inside the segment, and `fast_safe` would have rejected `&` anyway if the floor had missed. The git detector's split is shorter, and section 14.18 reads it.


Each segment goes through `strip_wrappers`. An empty word list is skipped. A segment that was only `sudo` becomes empty and does not panic on `words[0]`.

`name` is the first real word, after the wrappers are gone. `rsplit(['/', '\\'])` takes the last path component, so `/bin/rm` and `C:\Windows\rm` both yield `rm`. `unwrap_or` is the empty-split case. `to_ascii_lowercase` means `RM` and `Remove-Item` match the lowercase table. The comparison is `*item == name`, byte for byte, after that lowercase.

The `uv` branch runs before the deleting-name check. `uv` is not in `DELETING`. If the name is `uv` and `uv_uninstalls_runtime` returns true, the function returns true. Otherwise it falls through, `uv` fails the `DELETING` test, and the segment is skipped.

The `find` branch is a substring check plus an overlap check. `segment.contains("-delete")` or `segment.contains("-exec rm")` looks at the raw segment, not at the word list. `-delete` as its own flag matches. The overlap then walks `words` from the second word, skips anything that starts with `-`, and asks `path_hits`. `find /opt/uv/python/... -delete` hits when that path overlaps the runtime. `find /tmp -delete` does not. `contains` is a substring, so the `-exec rm` test also matches a longer token that begins with those characters.

The deleting branch is the same overlap walk. Flags are skipped, so `rm -rf /opt/uv/...` ignores `-rf` and tests the path. Any one overlapping non-flag word is enough. The function returns true on the first segment that hits. Later segments are not read.

## 14.15 `path_hits`

`path_hits` is the overlap, in both directions.

```rust
fn path_hits(word: &str, runtime_path: &str) -> bool {
    let word = word.trim_matches(|ch| ch == '"' || ch == '\'');
    word == runtime_path
        || runtime_path.starts_with(&format!("{word}/"))
        || runtime_path.starts_with(&format!("{word}\\"))
        || word.starts_with(&format!("{runtime_path}/"))
        || word.starts_with(&format!("{runtime_path}\\"))
}
```

`trim_matches` strips matching quote characters from both ends. `rm "/opt/uv/..."` and `rm /opt/uv/...` compare the same path. The quotes are not part of the path.


Equality is the first arm. `rm` of the exact `current_exe` string hits here. The test `yolo_still_refuses_to_delete_the_running_binary` uses a stand-in runtime, `/opt/uv/python/cpython-3.12.7-x86_64/bin/python`, for the pure function, then calls `check` with `std::env::current_exe()` under `yolo = true` and expects `Deny` with `hardline: true`.

`rm -rf /` misses. With the word `/`, `format!("{word}/")` is `//`, and a normal absolute runtime does not start with `//`. The backslash arm's format string writes `\\`, one backslash, so the needle is `/\\`. `command_deletes_runtime("rm -rf /", runtime)` is false. A parent is `runtime_path.starts_with` of the word plus `/`, so `rm /opt` hits `/opt/uv/...`. A child is the word starting with the runtime plus `/`. `rm -rf /tmp/scratch` is neither, and the test expects false.

## 14.16 `uv_uninstalls_runtime`

`uv_uninstalls_runtime` is the version check.

```rust
fn uv_uninstalls_runtime(words: &[&str], runtime_path: &str) -> bool {
    let lower: Vec<String> = words.iter().map(|word| word.to_ascii_lowercase()).collect();
    let Some(python_at) = lower.iter().position(|word| word == "python") else {
        return false;
    };
    let Some(offset) = lower
        .iter()
        .skip(python_at + 1)
        .position(|word| word == "uninstall")
    else {
        return false;
    };
    let uninstall_at = python_at + 1 + offset;
    for spec in words.iter().skip(uninstall_at + 1) {
        let spec = spec.trim_matches(|ch| ch == '"' || ch == '\'');
        if spec == "--all" || spec == "-a" || spec.eq_ignore_ascii_case("all") {
            return true;
        }
        if spec.starts_with('-') || spec.len() < 2 {
            continue;
        }
        if runtime_path.contains(&format!("cpython-{spec}")) || runtime_path.ends_with(spec) {
            return true;
        }
    }
    false
}
```

`lower` owns a lowercased copy of every word, so `Python` and `UNINSTALL` still line up. `position` finds `python`. `uv tool uninstall` has no `python` and returns false. `uninstall` is sought after that index. `position` on the skipped iterator is relative, so `uninstall_at` adds `python_at + 1` back. Specs are the original words after that index. Quote trimming matches `path_hits`.

`--all`, `-a`, and `all` in any ASCII case return true without reading the path. `--all` uses `==`. The bare word `all` uses `eq_ignore_ascii_case`. A spec that starts with `-`, or a spec shorter than two characters, is skipped.

Anything left hits when `runtime_path` contains `cpython-{spec}`, or when the path ends with the spec. `uv python uninstall 3.12` hits the test path because that path contains `cpython-3.12`. `3.11` does not. The test path ends in `python`, so the `3.12` hit is the `contains` arm, not `ends_with`.

## 14.17 `strip_wrappers`

`strip_wrappers` is shared. The git detector calls it too. I read it once, here.

```rust
fn strip_wrappers(segment: &str) -> Vec<&str> {
    let mut words: Vec<&str> = segment.split_whitespace().collect();
    while let Some(first) = words.first().copied() {
        let base = first.rsplit(['/', '\\']).next().unwrap_or(first);
        if matches!(base, "sudo" | "command" | "time" | "nohup") {
            words.remove(0);
            continue;
        }
        if base == "env" {
            words.remove(0);
            while words.first().is_some_and(|word| word.contains('=')) {
                words.remove(0);
            }
            continue;
        }
        break;
    }
    words
}
```

`first()` yields `Option<&&str>`, and `copied()` copies the inner `&str`. `sudo`, `command`, `time`, and `nohup` are peeled, including a path whose base name is one of those. `sudo command rm` peels both. `env` peels itself and then every following word that contains `=`. `env FOO=1 sudo rm` peels `env` and `FOO=1`, then the outer loop peels `sudo`. The first word that is not a wrapper breaks. That word is `words[0]` for `rm`, `uv`, `find`, and `git`. `sudo -u root rm` stops on `-u`, because `-u` is not one of the five names. The delete detector then sees the command name `-u` and does not treat the segment as `rm`.

## 14.18 `self_repo_git_mutation`

The git detector returns a bool. The floor turns `true` into the checkout-rewrite denial.

```rust
pub fn self_repo_git_mutation(command: &str, source_root: &str) -> bool {
    if command.is_empty() || source_root.is_empty() {
        return false;
    }
    for segment in command.split([';', '\n']) {
        let words = strip_wrappers(segment);
        let Some((git_at, dash_c)) = locate_git(&words) else {
            continue;
        };
        let rest = &words[git_at + 1..];
        let targets =
            dash_c == Some(source_root) || words.iter().any(|word| word_hits(word, source_root));
        if !targets {
            continue;
        }
        let Some(sub) = rest.iter().copied().find(|word| {
            !word.starts_with('-') && !word_hits(word, source_root) && Some(*word) != dash_c
        }) else {
            continue;
        };
        if sub == "reset" {
            if rest.iter().any(|word| RESET_MODES.contains(word)) {
                return true;
            }
            continue;
        }
        if GIT_MUTATIONS.contains(&sub) {
            return true;
        }
    }
    false
}
```

Empty command, empty root, false. The split is `split([';', '\n'])`. Semicolon and newline only. This function does not replace `&&` or `||`, and it does not split on `|`. A pipeline kept in one segment stays one segment. The delete detector's split is the wider one: it rewrites `&&` and `||` to newlines, then splits on semicolon, newline, and `|`. `tests/command_guards.rs` does not shell out. It calls the functions with strings.

## 14.19 `locate_git`

`locate_git` finds the first `git` and every `-C`.

```rust
fn locate_git<'a>(words: &[&'a str]) -> Option<(usize, Option<&'a str>)> {
    let mut index = 0;
    let mut dash_c = None;
    let mut git_at = None;
    while index < words.len() {
        if words[index] == "-C" && index + 1 < words.len() {
            dash_c = Some(words[index + 1]);
            index += 2;
            continue;
        }
        let base = words[index]
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(words[index]);
        if base == "git" && git_at.is_none() {
            git_at = Some(index);
        }
        index += 1;
    }
    git_at.map(|at| (at, dash_c))
}
```

> **Rust sidebar: the lifetime on `locate_git`**
> - `'a` is the lifetime of the word slices. The input is `&[&'a str]`.
> - The returned `Option<&'a str>` is the path that followed `-C`. It points into the same command text. It does not allocate.

Each `-C` overwrites `dash_c`. Two `-C` flags in one segment leave the last path. `-C` at the end with no following word fails `index + 1 < words.len()` and is treated as an ordinary word. `/usr/bin/git` matches because the base name is `git`.

Back in `self_repo_git_mutation`, `rest` is the words after `git`. `targets` is true when `-C`'s path is exactly `source_root`, or when any word in the whole segment overlaps the root through `word_hits`. The `-C` comparison is `==`, not the prefix test. `git -C /tmp/other-repo checkout main` is false against the manifest dir. The test expects that. `git -C {root} checkout main` is true, because `dash_c` equals the root and `checkout` is in `GIT_MUTATIONS`.

## 14.20 `word_hits`

`word_hits` is one direction. `path_hits` looks both ways. This one asks whether the word is the root or a path inside it.

```rust
fn word_hits(word: &str, root: &str) -> bool {
    word == root
        || word.starts_with(&format!("{root}/"))
        || word.starts_with(&format!("{root}\\\\"))
}
```

Equality, or the word begins with the root plus a slash, or the word begins with the root plus two backslashes. A parent of the root does not hit. A child path under the manifest does. That is how a command can name the repo without `-C`.

The subcommand is the first word of `rest` that is not a flag, not a word that `word_hits` the root, and not the `-C` path. `iter().copied()` yields `&str`, and `find` lends the closure a reference to each item, so `*word` is the `&str` compared with `dash_c`. `find` stops at the first match. `git -C {root} checkout main` skips `-C` (it starts with `-`), skips the root path, and takes `checkout`. If every remaining word is a flag or a path, `find` returns `None` and the segment continues. There is no subcommand to classify.

`reset` is not in `GIT_MUTATIONS`. It hits only when some word in `rest` is `--hard`, `--merge`, or `--keep`. `sudo git -C {root} reset --hard` peels `sudo`, matches the root, sees `reset` and `--hard`, and returns true. The test then calls `check` under YOLO and expects `Deny`. `git reset` and `git reset --soft` continue. `--soft` is not in `RESET_MODES`.

The other listed subcommands hit by name: `checkout`, `switch`, `rebase`, `merge`, `pull`, `restore`, `clean`, `cherry-pick`, `revert`, `bisect`. `git -C {root} status` targets the root and returns false. Alias lookup and heredocs are outside this parser. The doc comment says so.

`tests/command_guards.rs` calls both functions and then `check` with `yolo = true`. The root is `env!("CARGO_MANIFEST_DIR")`. `echo hello` is still `Allow`.

## Tests

Eight tests. The budgets are 3 and 1. Two matching zero-output empties are deterministic, and a pair that disagrees on zero output is not. Those two empties skip a later `late` and the fallback says `hello`. Three empties without usage still reach `ok`. A short repeat is kept, a long one stops continuation, and the turn text is the interrupted-loop sentence. The timeout drops the permit. Three retries peak at one permit. A crashed session reconnects once.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch14-when-things-fail
```

```text
Starting 94 tests across 24 binaries
     Summary [   0.170s] 94 tests run: 94 passed, 0 skipped
```

```bash
cargo run -p ch14-when-things-fail
```

```text
result: hello
fallbacks: 1
```

The binary runs two fake models. The first emits two empty replies at $0.30. The second says `hello`. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `empty_budget` | 0.9046 ns | under 1 µs | `retry_budget(Some(0.30))` |
| `repetition_guard` | 2.77 µs | under 1 ms | the window scan on an 800-character loop |

`empty_budget` calls `retry_budget` with `Some(0.30)` passed through `black_box`. The timed closure is `b.iter`. There is no model and no session. The result is the reduced budget, 1.

`repetition_guard` builds `"abcdefghij"` repeated 80 times before the bench (800 characters). Each sample calls `repetition_probe` on that string, which calls `is_repetition_dominated`. Building the string is excluded. The scan is a `HashMap` of 60-character windows. It does not call a model.

The timeout is not a bench. The test above is the measurement of the cancel: a 30 millisecond timeout against a 5 second sleep, permits back to 8, elapsed under 2 seconds. A bench that slept would measure the timer, not the permit.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `14`.

### Milestone demo

The `cargo run` above. The result is `hello`. One fallback.

## Pitfalls

Two zero-output empties never reach a third reply on that model. If you wanted the model to "try once more" after a proven empty pair, this rule will not do it. The fallback is the next try.

`retry_budget` only looks at the first empty's cost. A later cheap empty does not raise the budget again.

A length reply under 400 characters is continued even if it is `"ok ok ok"`. The guard fails open on short text, which is what Hermes does. Do not point this function at a one-line stutter and expect a stop.

`call_with_timeout` drops the permit when the timeout wins. The work future is dropped too, which is what cancels the sleep. A tool that has already started a side effect does not get undone. The permit is the only thing this function returns.

`FakeMcp` stops reconnecting after three crashes. The test uses one.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layers L3 and L4.

**J4.** Empty replies retry 3 times, or 1 time when one attempt costs more than $0.25. Two consecutive zero-output empties with usage skip the rest and fall back. Missing usage keeps the full budget.

**J3.** At least 400 characters, a 60-character window, at least 5 repeats, covering at least half the text. The stored text is the interrupted-loop sentence, not the loop itself.

**Length continuation.** `finish_reason == "length"` keeps the fragment unless the repetition guard fired. That is the deferral from chapter 9.

**Timeout.** `Kind::Timeout` drops the tool permit. Retries in `retry_in_bound` never hold two permits. The bound stays 8.

**B6.** `command_deletes_runtime` refuses a delete of the running binary, a parent of that binary, `find … -delete`, and `uv python uninstall` of the matching version. It sits on the approval floor before the fast path and before YOLO.

**B5.** `self_repo_git_mutation` refuses a worktree-rewriting `git` aimed at this crate's manifest directory, including `reset --hard`. Alias lookup and heredocs are not parsed. Hermes enables this block on Windows only; this harness runs it on every platform.

**Left for later.** Server-side 429 and 503 behavior is chapter 15. Think-block stripping is chapter 19 (K5). Outbound sanitization beyond this chapter's fake replies is chapter 18 (K4). Secret redaction (E1, E2) is chapter 20. The gateway lifecycle block (B4) is chapter 39.

## Chapter summary

- **Empty.** Three retries, or one if the attempt costs more than $0.25. Two proven empties fall back.
- **Repeat.** An 800-character loop is not continued. The turn stores the interrupted sentence.
- **Timeout.** The permit is back. The 5 second sleep does not finish.
- **Session.** One crash, one reconnect, then `pong`.
- **Floor.** `rm` of `current_exe` and `git reset --hard` of `CARGO_MANIFEST_DIR` are denied before the fast path and before YOLO.
- **Time.** The budget check measured **0.9046 ns**. The repetition scan measured **2.77 µs**.

[personal note: a fallback you did not expect, and which empty rule sent the turn there]

## What you implement

`my-harness/tests/ch14.rs` compiles, then panics in `todo!("chapter 14")` until `chapter_14_yolo_allows` is false for `"rm-the-running-binary"` and `"git-reset-hard-of-this-checkout"`, and true for `"echo hello"`. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 15 is the server. A tool gets its own concurrency limit, a retry only when it is marked idempotent, and a breaker that stops calling a backend that is already failing. The web-search cap of 50, still missing after chapter 9, is there too.
