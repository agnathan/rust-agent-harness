# Chapter 14: When Things Fail

An empty reply that costs $0.30 is the turn spending money on nothing. The first empty drops the retry budget from three to one, because $0.30 is past the $0.25 line, and a second empty from the same model, the same provider, and zero output tokens gives that model up. The next model answers `hello`. I expected chapter 9's stops to name an empty reply. They still treated every failure as one outcome, so an empty reply, a cut-off reply, a repeated paragraph, and a timeout all looked like the same kind of stop. Naming them is what lets a continuously improving harness spend the rest of the turn on a model that still has something to say, instead of burning the budget on a blank one.

The other half is the approval floor. YOLO can wave a lot of commands through, and it still can't delete the binary that is running or `git reset --hard` the checkout this process was built from. An agent that is learning inside a tree, and that can rewrite that tree, destroys the session it was supposed to learn from. Both denials are hardline, and both run before the fast path, so the cheap allow never gets a vote.

The name on a stopped turn is `Kind`. An empty reply retries, a repetition stops the loop, a cut-off reply can continue when it isn't itself a loop, and a timeout drops the permit it was holding. `run_models` walks to the next model, which is the seed of chapter 18's fallback. `approval::check` calls `command_deletes_runtime` and `self_repo_git_mutation`, and deleting a parent of the running binary counts as a hit.

Chapter 16 copies this harness, and chapter 15, the server, stays a separate program. Chapter 52's stress set still includes the stall and the recovery, which is why those failures are named kinds here.

I kept `Kind` and `Reply` as enums rather than strings, because a missed arm fails the build and a new spelling can't fall through. `OrderedF64` stores the cost as an integer of millionths, since a floating-point cost can't derive `Eq` while `NaN` isn't equal to itself. The step is `steps/ch14-when-things-fail`, package `ch14-when-things-fail`, library `harness_core`: chapter 13 plus the `failure` module.

`cargo run -p ch14-when-things-fail` prints `hello` and one fallback, from two fake models and no key. `my-harness/tests/ch14.rs` is the floor: YOLO still denies deleting the running binary and `git reset --hard` of this crate's manifest directory.

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

## What this chapter decides

`Kind` is `Empty`, `Truncated`, `Repeated`, `Timeout`, `RateLimit`, `Transport`, or `Crash`.

`retry_budget(None)` is 3; `retry_budget(Some(0.25))` is still 3; `retry_budget(Some(0.26))` is 1; the threshold is exceeded, not met.

`deterministic_empty` needs two attempts; both must have usage, both must have zero output, and the model, provider, and finish reason must match. Missing usage does not count; the fake turn then keeps going until the budget is spent. Three unclear empties and then the text `ok` stay on the first model.

Two zero-output empties skip the rest of that model, including a later text reply, and the next model answers. The demo is that case with a cost of $0.30, so the reduced budget and the deterministic rule agree; the second model says `hello`. `fallbacks` is 1.

`is_repetition_dominated` returns false under 400 characters; `"abcdefghij"` repeated to 800 characters returns true. `length_verdict("length", that)` is `StopRepeated`, and `run_models` stores the interrupted-loop sentence; a short length reply is `Continue`; a `stop` finish is `Finished`.

`call_with_timeout` holds one permit from a semaphore of 8 and drops it when a 30 millisecond timer beats a 5 second sleep. `retry_in_bound` peaks at one permit; `FakeMcp::new(1)` reconnects once and returns `pong`.

`floor_runtime_and_repo` runs before `fast_safe` and before YOLO; the runtime path is `std::env::current_exe()`; the source root is `env!("CARGO_MANIFEST_DIR")`. A hit is `Deny` with `hardline: true`; `echo hello` stays `Allow`.

## 14.1 `Kind`

`failure.rs` has two jobs, and the failure policy is the one to read first. A kind, an empty-reply budget, a repetition scan, a timeout that drops its permit, and a fake crashed session are the rules chapter 9 deferred, and this walk keeps those rules as they are. The second job sits at the bottom of the same file, where `command_deletes_runtime` and `self_repo_git_mutation` are what the approval floor calls. The detectors come after the retry, because the retry is the decision the empty reply forces.

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

`Copy` is why `run_one` can write `kind: Some(*kind)`, the star copying the kind out of the borrowed reply. `RateLimit` and `Transport` are names in the enum so a later caller can store them, and the fake models in this chapter's tests happen to build the other variants. A string would have let those two names appear without a match arm, which is the reason the classification is an enum.

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

`signature` is private; it clones three strings into a tuple so `==` can compare them. The bools stay outside the tuple, so two attempts can share a signature and still disagree on `zero_output`.

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
>
> `Option<f64>` is `Some(cost)` or `None`, and `Some(cost) if cost > COST_THRESHOLD_USD` is a pattern plus a condition that both have to hold, so the `_` arm is `None` and any cost that isn't greater than `0.25`. I treat the threshold as exceeded rather than met: `retry_budget(Some(0.25))` stays 3, and `Some(0.26)` is 1. `Option` itself is in [chapter 1](chapter-01-first-contact.md).

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
>
> `windows(2)` yields overlapping pairs, and `.last()` is the final one, so a log of three attempts compares the second and the third. One attempt has no pair, `last()` is `None`, and `let Some(pair) = ... else { return false }` returns early. `pair` is a `&[EmptyAttempt]` of length 2, so the `let` binds references to `pair[0]` and `pair[1]`.

Both attempts need usage, both need zero output tokens, and the model, the provider, and the finish reason have to match, so a missing usage flag makes the `&&` chain false. I only look at the last pair. An earlier empty that the model then recovered from shouldn't veto a later reply, which is why `windows(2).last()` is that pair and not a search of the whole log. One attempt is false, two clones are true, and flipping `zero_output` on the second is false again.

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

`text.len()` is bytes; under 400 the function returns false and does not build the map; `"ab"` repeated 10 times is 20 bytes. The test expects false; a one-line stutter stays under the floor.

`needed` is the larger of 5 and the ceiling of `n * 0.5 / 60`. For 800 bytes that ceiling is 7, so a 60-byte window has to show up at least 7 times. Just over 400 bytes, half the text divided by 60 is under 5, and the floor of 5 is what binds.

The map key is `&str`, borrowed from `text`; the window is not an owned `String`. `entry` then `or_insert(0)` returns `&mut usize`, and `*count += 1` writes through it; the first window that reaches `needed` returns true. The scan stops there.

`is_char_boundary` keeps the slice from panicking on a byte in the middle of a UTF-8 character; the loop steps one byte. ASCII starts are all boundaries; `"abcdefghij"` repeated 80 times is the 800-byte string `repeated_fragment` builds. The same 60-byte slice lines up every 10 bytes, the count crosses 7, and the function returns true. The bench calls `repetition_probe`, a one-line forward to this function, on a string built before the timer.

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

A finish reason other than `"length"` is `Finished`; the test passes `"stop"` and a short sentence. A `"length"` finish that fails the repetition check is `Continue`, and `kept` owns a copy of the fragment. A `"length"` finish that passes is `StopRepeated`. The loop text is not what we keep.

## 14.7 `Reply` and `OrderedF64`

The fake model has to store a cost the tests can match on; `f64` is not `Eq`.

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
>
> `PartialEq` for `f64` exists, and `Eq` doesn't, because `NaN` isn't equal to itself. `Reply` derives `Eq`, so a field of type `f64` would make that derive fail. `OrderedF64` stores `micros`, an `i64`: `usd` multiplies by one million and rounds, and `as_f64` divides back.

The comment above the struct says cents; the field is `micros`, and `usd` multiplies by `1_000_000.0`. A comment and a field that disagree can't both be the spec, and the tests match `usd`, so I follow the field.

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
>
> `work` is `FnOnce`, so calling `work()` consumes it and produces a future that `timeout` races against the duration. `Ok(text)` is the future finishing, and `Err` is the timer (`Elapsed`, bound as `_`). `drop(permit)` sits before the `match`, which means both arms have already released the slot when they return. When the timer wins, dropping the future cancels the sleep, and that cancel doesn't return the slot on its own. The `drop` does.

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
>
> `std::env::current_exe()` runs when `check` runs and returns this process's path, so a test binary lives under `target/`, `display()` is the lossy string handed to the detector, and `Err` skips the runtime check while the git check still runs. `env!("CARGO_MANIFEST_DIR")` is the crate directory at compile time, pasted in as a string literal, and `cd` can't change it. The two calls protect different paths, the binary that is running and the source tree it was built from, so a test that passed the manifest dir as the runtime path would be checking the wrong overlap.

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

Those arms are chapter 11's detectors, and they return before YOLO. Past YOLO, a command `dangerous` does not match is `Allow`. Unattended deny, smart review, and `Ask` run only when YOLO has not returned.

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

An empty command or an empty runtime path is false, because there is nothing to overlap. `normalized` rewrites `&&` and then `||` into newlines, and `split([';', '\n', '|'])` breaks the result on semicolon, newline, and a single pipe, so a chain becomes several segments while a lone `&` stays inside one. `fast_safe` would have rejected that `&` anyway if the floor had missed, and the git detector's split, in section 14.18, is the shorter one.

Each segment goes through `strip_wrappers`, an empty word list is skipped, and a segment that was only `sudo` becomes empty without panicking on `words[0]`. The name is the first real word after that, and `rsplit(['/', '\\'])` takes the last path component, so `/bin/rm` and `C:\Windows\rm` both yield `rm`. `to_ascii_lowercase` is why `RM` and `Remove-Item` match the lowercase table.

`uv` is checked before the deleting-name table, because `uv` isn't in `DELETING`. When the name is `uv` and `uv_uninstalls_runtime` returns true, the function returns true. Otherwise it falls through and the segment is skipped. `find` is a substring check plus an overlap: `-delete` or `-exec rm` is sought in the raw segment, flags are skipped, and `path_hits` decides, so `find /opt/uv/python/... -delete` hits when that path overlaps the runtime and `find /tmp -delete` doesn't. `contains` is a substring, which means a longer token that begins with `-exec rm` matches too. The deleting branch is the same overlap walk, flags skipped, and the first segment that hits returns true. Later segments aren't read.

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

`trim_matches` strips matching quotes from both ends, so `rm "/opt/uv/..."` and `rm /opt/uv/...` compare the same path and the quotes aren't part of it. Equality is the first arm, which is where `rm` of the exact `current_exe` string hits. `yolo_still_refuses_to_delete_the_running_binary` uses a stand-in runtime, `/opt/uv/python/cpython-3.12.7-x86_64/bin/python`, for the pure function, then calls `check` with `std::env::current_exe()` under `yolo = true` and expects `Deny` with `hardline: true`.

`rm -rf /` misses, and that miss is the point of the function. With the word `/`, `format!("{word}/")` is `//`, and a normal absolute runtime doesn't start with `//`. The backslash arm writes one backslash, so the needle is `/\\`, and `command_deletes_runtime("rm -rf /", runtime)` is false. A parent is `runtime_path.starts_with` of the word plus `/`, so `rm /opt` hits `/opt/uv/...`, while a child is the word starting with the runtime plus `/`. `rm -rf /tmp/scratch` is neither, and the expectation is false. The root delete is still denied by `hardline`.

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

`lower` owns a lowercased copy of every word, so `Python` and `UNINSTALL` still line up. `position` finds `python`, which is why `uv tool uninstall` returns false, and `uninstall` is sought after that index. `position` on the skipped iterator is relative, so `uninstall_at` adds `python_at + 1` back, and the specs are the original words after that index, quote-trimmed the same way `path_hits` trims.

`--all`, `-a`, and `all` in any ASCII case return true without reading the path. A spec that starts with `-`, or a spec shorter than two characters, is skipped. Anything left hits when `runtime_path` contains `cpython-{spec}`, or when the path ends with the spec. `uv python uninstall 3.12` hits the test path because that path contains `cpython-3.12`, and `3.11` doesn't. The test path ends in `python`, so the `3.12` hit is the `contains` arm, not `ends_with`.

## 14.17 `strip_wrappers`

`strip_wrappers` is shared. The git detector calls it too, so the walk is here.

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

`first()` yields `Option<&&str>`, and `copied()` copies the inner `&str`. `sudo`, `command`, `time`, and `nohup` are peeled, including a path whose base name is one of those, so `sudo command rm` peels both. `env` peels itself and then every following word that contains `=`, which means `env FOO=1 sudo rm` peels `env` and `FOO=1` and the outer loop then peels `sudo`. The first word that isn't a wrapper breaks, and that word is `words[0]` for `rm`, `uv`, `find`, and `git`. `sudo -u root rm` stops on `-u`, because `-u` isn't one of the five names, so the delete detector sees the command name `-u` and leaves the segment alone. I stopped the peel there on purpose: treating every flag as a wrapper would hide the `rm` that follows a real `sudo -u`.

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

An empty command or an empty root is false. The split is semicolon and newline only, so this function doesn't rewrite `&&` or `||` and doesn't split on `|`. A pipeline kept in one segment stays one segment. The delete detector's split is the wider one, and `tests/command_guards.rs` calls both functions with strings rather than shelling out.

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
>
> `'a` is the lifetime of the word slices, the input is `&[&'a str]`, and the returned `Option<&'a str>` is the path that followed `-C`. It points into the same command text and doesn't allocate.

Each `-C` overwrites `dash_c`, so two flags in one segment leave the last path. `-C` at the end, with no following word, fails `index + 1 < words.len()` and is treated as an ordinary word. `/usr/bin/git` matches because the base name is `git`.

Back in `self_repo_git_mutation`, `rest` is the words after `git`. `targets` is true when `-C`'s path is exactly `source_root`, or when any word in the segment overlaps the root through `word_hits`. The `-C` comparison is `==`, not the prefix test, so `git -C /tmp/other-repo checkout main` is false against the manifest dir and `git -C {root} checkout main` is true, because `dash_c` equals the root and `checkout` is in `GIT_MUTATIONS`.

## 14.20 `word_hits`

`word_hits` looks one way, where `path_hits` looks both. This one asks whether the word is the root or a path inside it.

```rust
fn word_hits(word: &str, root: &str) -> bool {
    word == root
        || word.starts_with(&format!("{root}/"))
        || word.starts_with(&format!("{root}\\\\"))
}
```

A hit is equality, or the word begins with the root plus a slash, or the word begins with the root plus two backslashes. A parent of the root misses, and a child path under the manifest hits, which is how a command can name the repo without `-C`.

The subcommand is the first word of `rest` that isn't a flag, isn't a word that `word_hits` the root, and isn't the `-C` path. `find` stops at the first match, so `git -C {root} checkout main` skips `-C`, skips the root path, and takes `checkout`. If every remaining word is a flag or a path, `find` returns `None` and the segment continues, because there is no subcommand to classify.

`reset` isn't in `GIT_MUTATIONS`. It hits only when some word in `rest` is `--hard`, `--merge`, or `--keep`, so `sudo git -C {root} reset --hard` peels `sudo`, matches the root, and returns true, and `check` under YOLO expects `Deny`. `git reset` and `git reset --soft` continue, because `--soft` isn't in `RESET_MODES`. The other listed subcommands hit by name: `checkout`, `switch`, `rebase`, `merge`, `pull`, `restore`, `clean`, `cherry-pick`, `revert`, and `bisect`. `git -C {root} status` targets the root and returns false. Alias lookup and heredocs are outside this parser, which is what the doc comment says. `tests/command_guards.rs` calls both functions and then `check` with `yolo = true`, the root is `env!("CARGO_MANIFEST_DIR")`, and `echo hello` is still `Allow`.

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

The result is `hello`. One fallback.

The means are `mean.point_estimate` from [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `14`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

Two zero-output empties never reach a third reply on that model. If you wanted the model to "try once more" after a proven empty pair, this rule will not do it. The fallback is the next try.

`retry_budget` only looks at the first empty's cost. A later cheap empty does not raise the budget again.

A length reply under 400 characters is continued even if it is `"ok ok ok"`. The guard fails open on short text, which is what Hermes does. Do not point this function at a one-line stutter and expect a stop.

`call_with_timeout` drops the permit when the timeout wins. The work future is dropped too, which is what cancels the sleep. A tool that has already started a side effect does not get undone. The permit is the only thing this function returns.

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

## What you implement

`my-harness/tests/ch14.rs` compiles, then panics in `todo!("chapter 14")` until `chapter_14_yolo_allows` is false for `"rm-the-running-binary"` and `"git-reset-hard-of-this-checkout"`, and true for `"echo hello"`. 

## What's next

Chapter 15 is the server, not the next harness. A tool gets its own concurrency limit, a retry only when it is marked idempotent, and a breaker that stops calling a backend that is already failing. The web-search cap of 50, still missing after chapter 9, is there too. The harness line resumes at chapter 16, which copies this chapter, and the server track continues at chapter 23.
