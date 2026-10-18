# Chapter 11: Approvals and Guardrails

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 10 will not refuse a tool. A turn can sit in `Dispatching` and the loop will run whatever name it repaired. This chapter is chapter 10's harness copied forward, plus `approval`. The check runs before the dispatcher takes a permit: a command is either allowed, denied, or handed to one human prompt. The prompt is the only place concurrent calls wait, and that wait does not hold a tool-concurrency permit.

The step crate is `steps/ch11-approvals`. The package name stays `ch11-approvals` so `cargo bench -p ch11-approvals --bench decision` still matches the dashboard. The library name is `harness_core`. The module is `approval`, not a glob re-export, because `run_script` already exists on the turn table.

| Section | Step | What it adds |
|---|---|---|
| 11 | `steps/ch11-approvals` | `Risk`, the approval pipeline, one prompt, a fake model |

> **How Hermes does it**
>
> I read these files on Hermes `main` at `cf71b1aa`: `tools/approval.py` (`check_all_command_guards`, `_floor_block`, `_STOP_COMMAND`, `_STOP_ACTION`, the denial-breaker addendum), `tools/approval_floors.py` (`_hardline_block_result`), `tools/approval_detection.py` (`HARDLINE_PATTERNS`, `detect_hardline_command`, `detect_dangerous_command`), and `tools/approval_context.py` (`_get_approval_timeout`, `_binary_approval_mode`).
>
> The floor runs before YOLO, `approvals.mode: "off"`, and cron approve-mode. `_floor_block` is hardline, then a runtime self-delete, then `sudo -S`, then the user's `approvals.deny` list. After the floor, YOLO or mode `off` approves, then the permanent allowlist, then an unattended context, then `detect_dangerous_command`, then smart mode, then the human. `detect_hardline_command` says the hardline table is never bypassable, even in YOLO. The timeout default is 300 seconds and a miss denies. Cron, `-q`, and unattended mode default to deny. A denial tells the model the user has not consented and must not retry or rephrase. Three consecutive denials append a circuit-breaker sentence.
>
> **Adopted.** That order. Hardline text that names `--yolo` and `approvals.mode=off`. The stop sentence from `_STOP_COMMAND`. The breaker threshold of 3. Timeout copy that includes "Silence is not consent." Unattended deny as the default. A fake model that stops the script on the first denial, so the next call is not issued.
>
> **Adapted.** The pattern table is a short set of the commands I read in `HARDLINE_PATTERNS` (root `rm -rf`, `mkfs`, `dd` onto a raw disk, a fork bomb, `kill -1`, shutdown). Hermes's `DANGEROUS_PATTERNS` list is much longer; this chapter treats recursive `rm`, a write under `/etc` or `.ssh` or `.env`, a `curl` upload, and an interpreter `-c`/`-e` as the dangerous class. Quote masking is a character scan, not Hermes's quote-and-heredoc rewriter. A safe command with an empty deny list returns before that scan allocates. The smart step is `fake_smart_review`, not an auxiliary model. The human wait is one `tokio::sync::Mutex`.
>
> **Deferred.** Deleting the running binary (`agent/runtime_self_protection.py`, B6) and the self-repo git guard (B5, `tools/self_repo_guard.py`) are chapter 14, on this same floor, before YOLO. The gateway lifecycle block (B4, `tools/terminal_tool_guards.py` `gateway_lifecycle_block`) is chapter 39, because the chat gateway does not exist yet. Persisting "always" to a file is A7 and is not this chapter. A plugin that escalates to the human gate is chapter 12 (A8). The whole-script `execute_code` gate is chapter 13 (A2); until then `deny_tool` returns `None` for `execute_code` and the script is not blocked here.

[personal note: a command you expected the allowlist to pass, and which floor still refused it]

## What this chapter decides

`Risk` is `Safe`, `Sensitive`, or `Destructive`. It is a label. The pipeline is the decision.

`check` walks one command:

| Step | When it fires | Result |
|---|---|---|
| Fast skip | Empty deny list, no shell metacharacters, none of the command words | `Allow` |
| Hardline | Root delete, `mkfs`, raw `dd`, fork bomb, `kill -1`, shutdown | `Deny`, and YOLO does not help |
| `sudo -S` | The command pipes a password to sudo | `Deny` |
| Deny list | A configured substring matches the masked command | `Deny`, before YOLO |
| YOLO or mode off | The floor did not fire | `Allow` |
| Allowlist | The normalized command is listed | `Allow` |
| Safe | Nothing dangerous left | `Allow` |
| Unattended | Cron, single-query, or unattended, and the mode is deny | `Deny` |
| Smart | `fake_smart_review` returns deny or approve | `Deny` or `Allow` |
| Ask | Manual mode, or smart escalates | `Ask` |

`run_script` is the fake model. It is a list of `ScriptCall` values. The first `Deny` ends the list. The demo is `rm -rf /` followed by `echo hello`. The second call never runs.

`hardline_message` names `--yolo` and `approvals.mode=off`. It does not append `STOP_COMMAND`. The other `Deny` arms do. `breaker_addendum` appends the circuit-breaker sentence when `session.denials` has reached the threshold, and only on the arms that call it. An approval resets the tally. Hardline denials count too.

The human path is `run_call`. It acquires a permit from a semaphore of 8, which is the same bound chapter 3 uses. If the verdict is `Ask`, it drops that permit, then locks the prompt. The test records how many permits were free at the start of the wait, and the peak number of callers inside the lock. The peak is 1. A silent prompt waits for `Policy.timeout` and denies. The default timeout is 300 seconds. The test uses 30 milliseconds so the suite does not sit there.

## 11.1 Labels, policy, and the verdict

`Risk` is a label. The pipeline decides. I still made it an enum so a later match has to name every level.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    Safe,
    Sensitive,
    Destructive,
}
```

> **Rust sidebar: `enum` and `Copy`**
> - `Risk` is one of three variants. A `match` with no `_` stops compiling if I add a fourth and forget an arm.
> - The derive includes `Copy`. Assignment duplicates the tag. `check` can return `Verdict::Allow { risk }` by copying `Risk::Safe` and still use the same label again.
> - `Mode`, `Unattended`, `Presence`, and `SmartVote` are `Copy` for the same reason. They own no heap data.
> - `Policy` is not `Copy`. It owns two `Vec<String>`s, the deny list and the allowlist. Copying it would duplicate those lists and let two sessions diverge. Callers pass `&Policy`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `approvals.mode: "off"`. Floors still run.
    Off,
    /// A second model, then a human if it will not approve.
    Smart,
    /// Always ask a human for a dangerous command.
    Manual,
}
```

`Off` is the comment's `approvals.mode: "off"`. The floor still runs, which is why `Off` is checked only after hardline, `sudo -S`, and the deny list. `Smart` asks `fake_smart_review` and falls through to a human when that function escalates. `Manual` is the default. A dangerous command in manual mode becomes `Ask`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unattended {
    Deny,
    Approve,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    Interactive,
    Cron,
    SingleQuery,
    Unattended,
}
```

`Unattended` here is the mode for a session that has no human, not the `Presence::Unattended` variant. `Deny` refuses a dangerous command. `Approve` allows it after the floor. `Presence` picks which of the three mode fields to read. `Interactive` reads none of them.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub mode: Mode,
    pub timeout: Duration,
    pub cron_mode: Unattended,
    pub single_query_mode: Unattended,
    pub unattended_mode: Unattended,
    pub denial_breaker_threshold: u32,
    pub yolo: bool,
    pub deny: Vec<String>,
    pub allowlist: Vec<String>,
    pub presence: Presence,
}
```

`Default` builds the policy `deny_tool` actually uses, because `deny_tool` calls `Policy::manual()` and `manual` is `Self::default()`.

```rust
impl Default for Policy {
    fn default() -> Self {
        Self {
            mode: Mode::Manual,
            timeout: DEFAULT_TIMEOUT,
            cron_mode: Unattended::Deny,
            single_query_mode: Unattended::Deny,
            unattended_mode: Unattended::Deny,
            denial_breaker_threshold: DENIAL_BREAKER_THRESHOLD,
            yolo: false,
            deny: Vec::new(),
            allowlist: Vec::new(),
            presence: Presence::Interactive,
        }
    }
}
```

`DEFAULT_TIMEOUT` is `Duration::from_secs(300)`. `DENIAL_BREAKER_THRESHOLD` is 3. `yolo` is false. Both lists are empty. `presence` is `Interactive`, so the unattended branch does not fire. Tests that want cron or YOLO mutate these public fields and call `check` directly. The loop path does not. It always gets this manual policy.

`Session` holds the denial tally. The derive list is `Debug, Clone, PartialEq, Eq`. There is no `Copy`. `check` takes `&mut Session` and writes `denials`. A copy would have its own counter, and a reset on one would leave the other counting.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    denials: u32,
}
```

`new` starts at 0. `denials()` returns the `u32` by copy. The field is private, so another crate reads it through that method and changes it only by calling `check` or `run_call`.

`SmartVote` is what the fake reviewer returns. `Verdict` is what `check` returns. They are different enums on purpose. A vote is an opinion. A verdict is the pipeline's answer, and it owns the message strings.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmartVote {
    Approve,
    Deny,
    Escalate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Allow { risk: Risk },
    Deny { message: String, hardline: bool },
    Ask { description: String, risk: Risk },
}
```

`Verdict` is not `Copy`, because `Deny` and `Ask` own a `String`. `label` matches every variant and returns a `'static` string. `is_allow` uses `matches!` so the `Allow` payload can be ignored.

```rust
impl Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Allow { .. } => "allow",
            Verdict::Deny { .. } => "deny",
            Verdict::Ask { .. } => "ask",
        }
    }

    pub fn is_allow(&self) -> bool {
        matches!(self, Verdict::Allow { .. })
    }
}
```

`..` means the fields are not bound. `label` borrows `&self` and does not take the `String`. The demo prints `verdicts[0].label()`, which is `"deny"` for `rm -rf /`.

`ScriptCall` is one command from the fake model. It owns a `String`. `run_script` borrows a slice of them.

## 11.2 `check`, in the order the branches return

`check` takes `&Policy`, `&mut Session`, and `&str`. The command is borrowed. The session is borrowed mutably because every return path either zeroes `denials` or adds one. The policy is shared.

> **Rust sidebar: `&mut`, `Result` is not this function**
> - `&mut Session` is the only way `check` can write the tally the caller keeps. The function does not return the session.
> - `check` returns `Verdict`, not `Result`. Every command has an answer. There is no `Err` for "I could not decide."
> - The early `return`s are why later arms do not run. A hardline hit never reaches the YOLO test.

```rust
pub fn check(policy: &Policy, session: &mut Session, command: &str) -> Verdict {
    if policy.deny.is_empty() && fast_safe(command) {
        session.denials = 0;
        return Verdict::Allow { risk: Risk::Safe };
    }
```

The fast path requires both an empty deny list and `fast_safe`. `echo hello` takes it. `session.denials` becomes 0, so an earlier denial does not survive a later safe command. `skip_decision` times this path. `normalize` is not called, so the normalized `String` is not allocated.

`fast_safe` scans bytes, then words.

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

A quote, `$`, a backtick, `|`, `;`, `>`, `(`, `)`, or `&` rejects the command. `<` is not in that pattern. A word in `WORDS` also rejects it. `contains_word_ci` is case-insensitive and requires a word boundary. A boundary byte is anything that is not ASCII alphanumeric, `_`, or `-`. `form` does not contain the word `rm`, because the letters around `rm` are word bytes. `echo hello` has none of the bytes and none of the words, so `fast_safe` is true.

Anything else walks the floor, then the policy. Hardline is first.

```rust
    if let Some(description) = hardline(command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message: hardline_message(&description),
            hardline: true,
        };
    }
```

`if let Some` is the `Option` from `hardline`. `None` falls through. `Some` increments with `saturating_add`, so the `u32` stops at `u32::MAX` instead of wrapping to 0. The message is `hardline_message`. That string names `--yolo`, `/yolo`, `approvals.mode=off`, and cron approve mode. It does not append `STOP_COMMAND`, and it does not call `breaker_addendum`. `hardline: true` is the flag YOLO cannot clear, because YOLO is a later `if` that this `return` skipped. `hardline_survives_yolo_and_mode_off` sets both `yolo` and `Mode::Off` and still gets `Deny { hardline: true }` for `sudo rm -rf /`.

`sudo -S` is the next floor. It is not marked hardline.

```rust
    if pipes_password_to_sudo(command) {
        session.denials = session.denials.saturating_add(1);
        return Verdict::Deny {
            message: format!(
                "BLOCKED: password piping to sudo -S. Do not pipe passwords to 'sudo -S'.{STOP_COMMAND}"
            ),
            hardline: false,
        };
    }
```

`pipes_password_to_sudo` masks quotes first, then requires both the substring `-S` and `sudo` in the lowercase masked text. `echo secret | sudo -S id` matches. The message includes `STOP_COMMAND` and does not include the breaker sentence. `sudo_s_is_a_floor_under_yolo` sets `yolo` and still gets `hardline: false`. The floor ran before the YOLO arm.

The user deny list is still on the floor.

```rust
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
```

`deny_match` normalizes the masked command and returns the first rule that is a substring. The message quotes that rule, appends `STOP_COMMAND`, then appends whatever `breaker_addendum` returns. `user_deny_matches_before_yolo` puts `push --force` on the list, turns YOLO on, and the verdict is `Deny` with `hardline: false` and the rule text. The fast path did not apply, because `policy.deny` was not empty.

Only after those three floors does YOLO or mode off allow the command.

```rust
    if policy.yolo || policy.mode == Mode::Off {
        session.denials = 0;
        return Verdict::Allow {
            risk: risk_of(command),
        };
    }
```

`risk_of` still labels the command. Hardline and `sudo -S` already returned, so a command in this arm is neither. A recursive `rm` that was not hardline, such as `rm -rf ./build`, comes back `Allow { risk: Sensitive }`. A command `dangerous` does not recognize comes back `Allow { risk: Safe }`. The tally is cleared.

The allowlist is an equality check on normalized text, not a substring.

```rust
    let normalized = normalize(&mask_quotes(command));
    if policy
        .allowlist
        .iter()
        .any(|entry| normalize(entry) == normalized)
    {
        session.denials = 0;
        return Verdict::Allow { risk: Risk::Safe };
    }
```

`allowlist_skips_the_human_after_the_floor` lists `rm -rf ./build` and the same command is `Allow`. The risk is `Safe` even though `risk_of` would have said `Sensitive`. The allowlist is a bypass of the dangerous class, and it is after the floor, so `rm -rf /` never gets here.

```rust
    let danger = dangerous(command);
    if danger.is_none() {
        session.denials = 0;
        return Verdict::Allow { risk: Risk::Safe };
    }
    let description = danger.expect("checked");
```

`is_none` is the safe command that failed `fast_safe` for a quote or a metacharacter and then matched nothing dangerous. `echo "rm -rf /"` lands here: the quotes blocked the fast path, `mask_quotes` hid the payload from hardline, and `dangerous` finds no recursive `rm` outside the quotes. `expect("checked")` runs only after `is_none` was false, so the `Option` is `Some`. The string is the description later messages interpolate.

Unattended mode is next, and only for a dangerous command.

```rust
    if let Some(mode) = unattended_mode(policy) {
        if mode == Unattended::Deny {
            session.denials = session.denials.saturating_add(1);
            return Verdict::Deny {
                message: format!(
                    "BLOCKED: dangerous command in an unattended session ({description}).{STOP_COMMAND}{}",
                    breaker_addendum(session, policy)
                ),
                hardline: false,
            };
        }
        session.denials = 0;
        return Verdict::Allow {
            risk: Risk::Sensitive,
        };
    }
```

`unattended_mode` matches `Presence`. `Interactive` is `None`, and this whole block is skipped. `Cron`, `SingleQuery`, and `Unattended` are `Some` of the matching field. `Deny` increments, appends `STOP_COMMAND` and the breaker, and returns. `Approve` clears the tally and returns `Allow { risk: Sensitive }`. That is cron approve-mode, and it is after the floor. `cron_denies_a_dangerous_command_and_echo_still_runs` denies `rm -rf ./build` and then allows `echo hello`, because `echo hello` returned on the fast path before presence mattered.

Smart mode is a `match` on the vote. Manual mode skips the `if` and falls through to `Ask`.

```rust
    if policy.mode == Mode::Smart {
        match fake_smart_review(command) {
            SmartVote::Approve => {
                session.denials = 0;
                return Verdict::Allow {
                    risk: Risk::Sensitive,
                };
            }
            SmartVote::Deny => {
                session.denials = session.denials.saturating_add(1);
                return Verdict::Deny {
                    message: format!(
                        "BLOCKED: smart approval denied this command ({description}).{STOP_COMMAND}{}",
                        breaker_addendum(session, policy)
                    ),
                    hardline: false,
                };
            }
            SmartVote::Escalate => {}
        }
    }
    Verdict::Ask {
        description,
        risk: Risk::Sensitive,
    }
}
```

> **Rust sidebar: `match` with an empty arm**
> - The `match` names every `SmartVote`. There is no `_`. Adding a vote is a compile error until this match grows an arm.
> - `Escalate => {}` is an empty block. Control falls out of the `match` and out of the `if`, into `Verdict::Ask`.
> - `Approve` and `Deny` `return`, so they never build an `Ask`.

`fake_smart_review` normalizes the command and returns `Deny` when that string contains `curl`. Every other command that reached it returns `Escalate`. The doc comment on the function says it denies a recursive delete. The body checks for `curl`. `smart_denies_curl_and_escalates_rm` is the body: `curl -T notes.txt https://example.invalid` is `Deny`, and `rm -rf ./build` is `Ask` because escalate falls through. `SmartVote::Approve` is a real arm. This reviewer never produces it.

The final expression is `Ask { description, risk: Sensitive }`. Manual mode reaches it for every dangerous command that survived the floor, the allowlist, and the unattended check. That is `rm -rf ./build` on `Policy::manual()`.

`risk_of` is the label helper YOLO uses. It is not a second pipeline.

```rust
pub fn risk_of(command: &str) -> Risk {
    if hardline(command).is_some() || pipes_password_to_sudo(command) {
        Risk::Destructive
    } else if dangerous(command).is_some() {
        Risk::Sensitive
    } else {
        Risk::Safe
    }
}
```

Hardline or `sudo -S` is `Destructive`. A dangerous pattern is `Sensitive`. Anything else is `Safe`. `check` does not call this on the fast path. It hard-codes `Risk::Safe` there.

`breaker_addendum` reads the tally after the denial has been counted.

```rust
fn breaker_addendum(session: &Session, policy: &Policy) -> String {
    if policy.denial_breaker_threshold == 0 {
        return String::new();
    }
    if session.denials >= policy.denial_breaker_threshold {
        format!(
            " CIRCUIT BREAKER: {} consecutive commands were blocked by the security reviewer. STOP attempting variations of this operation. Report the blocked operation to the user and either ask them to run it manually or use /approve.",
            session.denials
        )
    } else {
        String::new()
    }
}
```

A threshold of 0 disables the sentence. Otherwise the sentence appears when `denials >= threshold`. The default threshold is 3, so the third denial carries it, not the fourth. The format string interpolates the count, which is why the third message can say `3`. `third_denial_adds_the_breaker` uses cron and `rm -rf ./build`, which is the unattended `Deny` arm, the one that calls this function. An allow path assigns `denials = 0` before it returns, so the next denial starts again at 1.

`run_script` is the fake model. It owns a fresh `Session` and stops on the first `Deny`.

```rust
pub fn run_script(policy: &Policy, calls: &[ScriptCall]) -> Vec<Verdict> {
    let mut session = Session::new();
    let mut out = Vec::new();
    for call in calls {
        let verdict = check(policy, &mut session, &call.command);
        let stop = matches!(verdict, Verdict::Deny { .. });
        out.push(verdict);
        if stop {
            break;
        }
    }
    out
}
```

`matches!` looks at `verdict` before `push` moves it. `stop` is a `bool`, so the move into `out` is still legal. `Allow` and `Ask` do not break. The demo passes `rm -rf /` and then `echo hello` under `Policy::manual()`. The first call is hardline `Deny`. `stopped` prints `verdicts.len()`, which is 1. The second call is never checked. `a_denial_stops_the_script` asserts that length and the label `"deny"`.

`skip_decision` is the bench entry. It builds a new session and calls `check(&Policy::manual(), ...)`. The bench passes `black_box("echo hello")` inside `b.iter`, so the fast path is what the 249.6 ns mean measures.

## 11.3 Masking, wrappers, hardline, and dangerous

`normalize` strips wrappers before a comparison. It allocates the `String` the fast path avoided.

```rust
pub fn normalize(command: &str) -> String {
    let mut parts: Vec<&str> = command.split_whitespace().collect();
    while let Some(first) = parts.first().copied() {
        if matches!(first, "sudo" | "nohup" | "command" | "time") {
            parts.remove(0);
            continue;
        }
        if first == "env" {
            parts.remove(0);
            while parts.first().is_some_and(|token| token.contains('=')) {
                parts.remove(0);
            }
            continue;
        }
        break;
    }
    parts.join(" ")
}
```

`split_whitespace` borrows slices of `command`. The `while let` repeats while a first token exists. `first()` is `Option<&&str>`, and `copied()` yields `Option<&str>` because `&str` is `Copy`. `sudo`, `nohup`, `command`, and `time` are removed and the loop tries the new head. `env` removes itself and then every following token that contains `=`. `sudo nohup env FOO=1 rm -rf /` becomes `rm -rf /`. Any other head hits `break`. The join is the normalized command.

`mask_quotes` is a character scan. It is not a quote-and-heredoc rewriter.

```rust
pub fn mask_quotes(command: &str) -> String {
    let mut out = String::with_capacity(command.len());
    let mut chars = command.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '"' || ch == '\'' {
            let quote = ch;
            out.push(' ');
            for inner in chars.by_ref() {
                if inner == quote {
                    break;
                }
                out.push(' ');
            }
        } else {
            out.push(ch);
        }
    }
    out
}
```

`with_capacity` reserves `command.len()` bytes. A quote pushes a space, then spaces until the same quote or the end of the string. The inside of `echo "rm -rf /"` becomes spaces, so hardline does not see `rm`. An unmatched quote consumes the rest of the command. `chars.by_ref()` lets the inner loop and the outer loop share one iterator. There is no escape handling. A backslash before a quote is just a character.

`variants` builds the strings hardline and dangerous scan.

```rust
fn variants(command: &str) -> Vec<String> {
    let mut found = vec![normalize(&mask_quotes(command))];
    if let Some(payload) = shell_c_payload(command) {
        found.push(normalize(&payload));
    }
    found
}
```

The first string is the masked command, normalized. The second is the raw shell payload, normalized, and only when `shell_c_payload` returns `Some`. The payload is not masked. That is why `bash -c 'rm -rf /'` is still hardline after the quotes hid the same text in the first variant.

```rust
fn shell_c_payload(command: &str) -> Option<String> {
    let lowered = command.to_ascii_lowercase();
    let marker = ["sh -c ", "bash -c ", "eval "]
        .into_iter()
        .find(|marker| lowered.contains(marker))?;
    let index = lowered.find(marker)? + marker.len();
    let rest = command[index..].trim();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return Some(rest.to_string());
    }
    let inner = &rest[quote.len_utf8()..];
    Some(inner.split(quote).next().unwrap_or(inner).to_string())
}
```

> **Rust sidebar: `?` on `Option`**
> - `?` on `Option` returns `None` from the function. `find` yields `None` when no marker is present, and `shell_c_payload` then returns `None`. `variants` keeps a one-element vec.
> - `lowered.find(marker)?` is the same early return if the marker disappeared between the two searches. It cannot, because `find` just succeeded, and the `?` is still the shape of the function.
> - `rest.chars().next()?` returns `None` on an empty payload.

The marker list is tried in order: `sh -c `, then `bash -c `, then `eval `. `bash -c 'rm -rf /'` contains `sh -c ` as the last two letters of `bash` plus ` -c `. `find` points at that `s`. Adding `marker.len()` lands on the same index as the end of `bash -c `, so `rest` is the payload either way. A payload that does not open with a quote is returned whole. A quoted payload is split on the quote, and `next` takes the text before the closer. `split` always yields at least one piece, so `unwrap_or` does not change the result.

`hardline` scans each variant and returns the first description.

```rust
fn hardline_text(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    if recursive_delete_of_root(&lower) {
        return Some("recursive delete of root filesystem".to_string());
    }
    if [
        "/etc", "/etc/*", "/boot", "/boot/*", "/usr", "/usr/*", "~", "~/", "~/*", "$home",
    ]
    .into_iter()
    .any(|path| recursive_delete_of(&lower, path))
    {
        return Some("recursive delete of system directory".to_string());
    }
    if command_word(&lower, "mkfs") {
        return Some("format filesystem (mkfs)".to_string());
    }
    if lower.contains("dd")
        && (lower.contains("of=/dev/sd") || lower.contains("of=/dev/nvme"))
        && command_word(&lower, "dd")
    {
        return Some("dd to raw block device".to_string());
    }
    if lower.contains(":(){") && lower.contains(":|:&") {
        return Some("fork bomb".to_string());
    }
    if kill_all(&lower) {
        return Some("kill all processes".to_string());
    }
    for word in ["shutdown", "reboot", "halt", "poweroff"] {
        if command_word(&lower, word) {
            return Some("system shutdown/reboot".to_string());
        }
    }
    None
}
```

The order is the policy. A root delete wins over a system-directory delete. `recursive_delete_of_root` requires `is_rm_recursive` and a token that is `/`, `/*`, `/.`, `/./`, or `//`. `recursive_delete_of` is the same `rm` test against one path from that list: `/etc`, `/boot`, `/usr`, and the home spellings. `mkfs` has to be a whole token. `dd` has to be a whole token and the line has to contain `of=/dev/sd` or `of=/dev/nvme`. The fork bomb needs both `:(){` and `:|:&`. `kill_all` looks at adjacent tokens through `windows(2)` and matches `kill` then `-1`. Shutdown is any of those four words as a whole token. `None` means this variant is not hardline. `hardline` tries the shell payload next.

`is_rm_recursive` is the shared test.

```rust
fn is_rm_recursive(lower: &str) -> bool {
    let mut saw_rm = false;
    for token in lower.split_whitespace() {
        if token == "rm" {
            saw_rm = true;
            continue;
        }
        if saw_rm && token.starts_with('-') && token.contains('r') {
            return true;
        }
        if saw_rm && !token.starts_with('-') {
            saw_rm = false;
        }
    }
    false
}
```

`rm -rf /` sets `saw_rm`, then `-rf` starts with `-` and contains `r`, so the function returns true. A non-flag token clears `saw_rm`. `rm file -rf` clears the flag on `file` before it sees `-rf`, and returns false. `command_word` is `split_whitespace().any(|token| token == word)`.

`dangerous_text` is the class that asks, or that cron denies. It is not the hardline table.

```rust
fn dangerous_text(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    if is_rm_recursive(&lower) {
        return Some("recursive delete".to_string());
    }
    if lower.split_whitespace().any(|token| {
        token.starts_with("/etc") || token.contains("/.ssh") || token.contains("/.env")
    }) {
        return Some("write to a sensitive path".to_string());
    }
    if lower.contains("curl")
        && (lower.contains("--upload-file") || lower.contains(" -t ") || lower.contains("-t"))
    {
        return Some("curl upload".to_string());
    }
    for word in ["python", "python3", "perl", "ruby", "node"] {
        if command_word(&lower, word)
            && lower
                .split_whitespace()
                .any(|token| token == "-c" || token == "-e")
        {
            return Some("interpreter inline code".to_string());
        }
    }
    None
}
```

Any recursive `rm` is `"recursive delete"`, including `rm -rf ./build`, which is not a root delete and so was not hardline. A token that starts with `/etc`, or contains `/.ssh` or `/.env`, is a sensitive path. `curl` plus `--upload-file`, ` -t `, or `-t` is an upload. The check is `contains` on the lowered line, so `-T` becomes `-t` before the test. An interpreter word plus a token that is exactly `-c` or `-e` is inline code. `dangerous` runs `variants`, so `bash -c 'rm -rf ./build'` is dangerous because the raw payload is, even when the masked command is only `bash -c`.

## 11.4 One prompt, and the permit it does not hold

`Prompt` is the human wait. Concurrent callers share it through `Arc` in the test. The lock is a `tokio::sync::Mutex<()>`. The unit value means the mutex carries no data. Holding it is the mutual exclusion.

```rust
pub struct Prompt {
    lock: Mutex<()>,
    inside: AtomicUsize,
    peak: AtomicUsize,
    permits_seen: AtomicUsize,
    answer: Notify,
    answered: AtomicUsize,
    /// `0` wait, `1` allow, `2` deny.
    script: AtomicUsize,
}
```

`allow` stores 1, `deny` stores 2, `silent` stores 0. The atomics use `Ordering::SeqCst` on every load and store. `peak()` and `permits_seen_free()` are read from the test after `tokio::join!`, and the increments happen on the runtime. SeqCst is the ordering those operations use.

> **Rust sidebar: `tokio::sync::Mutex` across `.await`**
> - `std::sync::Mutex`'s guard is not `Send`. An `.await` may move the task to another thread, so holding that guard across the wait does not compile.
> - `tokio::sync::Mutex` is the lock whose guard may be held while the task is parked. `wait` holds it for the whole timeout.
> - The tool permit is a different type. `OwnedSemaphorePermit` returns its slot when dropped. `drop(permit)` before `wait` is what makes the human pause not count against the bound.
> - `Session` in `run_call` is also a tokio mutex. Its guard ends before `prompt.wait`, so the tally lock is not held across the human.

`wait` is the critical section.

```rust
    async fn wait(&self, timeout: Duration, permits_free: usize) -> bool {
        let _guard = self.lock.lock().await;
        let now = self.inside.fetch_add(1, Ordering::SeqCst) + 1;
        self.peak.fetch_max(now, Ordering::SeqCst);
        if permits_free > 0 {
            self.permits_seen.fetch_add(1, Ordering::SeqCst);
        }
        let script = self.script.load(Ordering::SeqCst);
        let allowed = if script == 0 {
            timeout_or_release(&self.answer, &self.answered, timeout).await
        } else {
            script == 1
        };
        self.inside.fetch_sub(1, Ordering::SeqCst);
        allowed
    }
```

`lock().await` is the queue. The second caller blocks there, before it can increment `inside`. `fetch_add` returns the previous value, so `+ 1` is the count including this caller, and `fetch_max` records the peak. The two-ask test sees `peak() == 1`. `permits_free > 0` increments `permits_seen`. `permits_seen_free` is that counter being greater than zero, which is how the test knows the semaphore had a free slot at the start of the wait.

Script 0 calls `timeout_or_release`. Any other script skips the timer: `script == 1` is true for allow and false for deny. `fetch_sub` runs on the way out, while the guard is still held. The guard drops at the end of the function, and the next waiter enters.

```rust
async fn timeout_or_release(answer: &Notify, answered: &AtomicUsize, timeout: Duration) -> bool {
    if answered.load(Ordering::SeqCst) == 1 {
        return true;
    }
    match tokio::time::timeout(timeout, answer.notified()).await {
        Ok(()) => answered.load(Ordering::SeqCst) == 1,
        Err(_) => false,
    }
}
```

`answered == 1` means `release` already ran. `release` stores 1 and then `notify_waiters`. The store is first so a waiter that wakes on `Ok(())` still observes the flag. `Err` is the timeout, and the function returns false. A notify that left `answered` at 0 also returns false. `Prompt::silent` never calls `release`, so the wait hits `Policy.timeout` and denies.

`ToolPermits` wraps `Arc<Semaphore>`. `new` starts it with `bound` permits. `available` reads `available_permits`. `TOOL_PERMIT_BOUND` is 8, the same number as `DISPATCH_BOUND`. They are two semaphores. `run_call` uses this one. The dispatcher inside the loop uses its own.

```rust
pub async fn run_call(
    policy: &Policy,
    session: &Mutex<Session>,
    prompt: &Prompt,
    permits: &ToolPermits,
    command: &str,
) -> Verdict {
    let permit = permits
        .sem
        .clone()
        .acquire_owned()
        .await
        .expect("semaphore open");
    let verdict = {
        let mut guard = session.lock().await;
        check(policy, &mut guard, command)
    };
    match verdict {
        Verdict::Ask { risk, .. } => {
            drop(permit);
            let free = permits.available();
            let allowed = prompt.wait(policy.timeout, free).await;
            if allowed {
                let _permit = permits
                    .sem
                    .clone()
                    .acquire_owned()
                    .await
                    .expect("semaphore open");
                Verdict::Allow { risk }
            } else {
                let mut guard = session.lock().await;
                guard.denials = guard.denials.saturating_add(1);
                let extra = breaker_addendum(&guard, policy);
                Verdict::Deny {
                    message: format!(
                        "BLOCKED: Command timed out without user response.{STOP_COMMAND} Silence is not consent.{extra}"
                    ),
                    hardline: false,
                }
            }
        }
        other => other,
    }
}
```

> **Rust sidebar: `async fn` and an owned permit**
> - `async fn` returns a future. `acquire_owned().await` parks until a permit exists. `expect` panics if the semaphore is closed. `ToolPermits::new` never closes it.
> - `acquire_owned` consumes an `Arc<Semaphore>` clone and returns `OwnedSemaphorePermit`. Dropping that permit returns the slot and drops the clone.
> - The session guard is a block. It ends before the `match`, so `prompt.wait` does not run while the tally is locked.
> - `other => other` moves an `Allow` or a `Deny` out. The original `permit` drops when the function returns. Only the `Ask` arm drops it early.

The `Ask` arm drops the permit, then reads `available`, then waits. The free count is what `wait` records. An allow script returns true without sleeping. `run_call` then acquires a new permit into `_permit` and returns `Allow { risk }`. That new permit drops at the end of the block. The function does not run the tool. A deny script, or a silent prompt that times out, locks the session again, increments `denials`, and returns `Deny` with `STOP_COMMAND` and `Silence is not consent.` `breaker_addendum` sees the tally after that increment.

`the_human_wait_is_one_prompt_and_drops_the_permit` joins two `run_call`s on `rm -rf ./build` and `rm -rf ./other`, with `Prompt::allow` and a bound of 8. Both verdicts are `Allow`. `peak` is 1. `permits_seen_free` is true. After both return, `available` is 8 again. The policy timeout in that test is 200 milliseconds. Script 1 never reads it.

`a_silent_prompt_denies_at_the_timeout` uses 30 milliseconds and a bound of 1. The elapsed time is under two seconds. The message contains `Silence is not consent` and `has NOT consented`, `hardline` is false, and the one permit is back. The default timeout stays 300 seconds. The test is the one that shortens it.

An `Allow` or a `Deny` from `check` takes `other => other` and holds the original permit until `run_call` returns. That return is the tool body this function does not run. A caller that then waits on its own must drop the permit too. The joined-ask test is the one that checks the drop, because both calls took the `Ask` arm.

## 11.5 `deny_tool` before `run_raced`

The loop does not call `run_call`. It calls `deny_tool`, and it calls it before the dispatcher takes a permit.

```rust
pub fn deny_tool(
    session: &mut Session,
    name: &str,
    arguments: &serde_json::Value,
) -> Option<String> {
    if name != "terminal" && name != "execute_code" {
        return None;
    }
    let command = if name == "execute_code" {
        arguments
            .get("code")
            .and_then(|value| value.as_str())
            .unwrap_or("")
    } else {
        arguments
            .get("command")
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    if name == "execute_code" {
        return None;
    }
    match check(&Policy::manual(), session, command) {
        Verdict::Allow { .. } => None,
        Verdict::Deny { message, .. } => Some(message),
        Verdict::Ask { description, .. } => Some(format!(
            "BLOCKED: approval required ({description}). No prompt is installed on this turn.{STOP_COMMAND}"
        )),
    }
}
```

> **Rust sidebar: `Option` as the run-or-refuse bit**
> - `None` means run the tool. `Some(message)` means do not, and the `String` is the tool result the model will read.
> - The first `if` returns `None` for every name except `terminal` and `execute_code`. `echo` and `delegate` take that arm. They are not gated.
> - `and_then` plus `unwrap_or("")` turns a missing key, or a non-string, into an empty command. `?` is not used. The function's return type is `Option<String>`, and a missing argument is not `None` in that sense.

`execute_code` reads `arguments["code"]` into `command` and then returns `None` before `check`. The string is dropped. This chapter does not block the script. The sandbox check arrives in chapter 13. The branch is here so the argument key is already `code`.

`terminal` reads `arguments["command"]`. `check` always sees `Policy::manual()`: manual mode, interactive, YOLO off, empty deny list, empty allowlist, 300 second timeout. The loop cannot pass a different policy in. Tests of YOLO and cron call `check` themselves.

`Allow` returns `None`. The tool runs. An empty command string is `fast_safe` when the deny list is empty, so a `terminal` call with no `command` key is allowed through.

`Deny` returns the message `check` built. For `rm -rf /` that is `hardline_message`. `Ask` becomes a denial too. The text says no prompt is installed and appends `STOP_COMMAND`. There is no `Prompt`, no `Semaphore`, and no `await`. A human wait cannot hold a tool permit on this path, because this path never acquires one and never waits. `rm -rf ./build` on the manual policy is `Ask` from `check` and a `Some` from `deny_tool`. The tool does not run.

In `drive_loop`, after `ModelToolCalls` has moved the phase to `Dispatching` and the assistant has been pushed, the runnable calls are split before `run_raced`.

```rust
        let mut denied: Vec<(String, String)> = Vec::new();
        let mut approval_session = crate::approval::Session::new();
        for call in runnable {
            if let Some(message) =
                crate::approval::deny_tool(&mut approval_session, &call.name, &call.arguments)
            {
                denied.push((call.id.clone(), message));
                continue;
            }
            if call.name == "delegate" {
                if usage.subagent_calls >= budget.max_subagent_iterations {
                    blocked.push(call);
                    continue;
                }
                usage.subagent_calls += 1;
            }
            allowed.push(call);
        }
        let blocked_ids: Vec<String> = blocked.iter().map(|call| call.id.clone()).collect();
        let calls: Vec<ToolCall> = allowed
            .iter()
            .map(|call| ToolCall {
                name: call.name.clone(),
                arguments: call.arguments.clone(),
            })
            .collect();
        let outputs = dispatcher
            .run_raced(registry, &calls, &cancel.token())
            .await;
```

`approval_session` is a new `Session` for this batch. Denials in one runnable list share the tally. The next model round builds another session. `if let Some(message)` is `deny_tool`'s `Option`. `Some` pushes `(id, message)` and `continue`s, so the call is not moved into `allowed` and is not in the `ToolCall` vec. `delegate`'s cap runs only for a call `deny_tool` did not take. `run_raced` sees `allowed` only. A denied `terminal` never becomes a `ToolCall`, so the dispatcher does not acquire a permit for it.

The result walk has to keep the zip aligned. Denied ids were not in `allowed`, so they must be handled before `ran.next()`.

```rust
            if let Some((_, message)) = denied.iter().find(|(id, _)| id == &planned.id) {
                messages.push(Message::tool(planned.id.clone(), message.clone()));
                continue;
            }
```

I skipped the error arm and the blocked-id arm above this one. Both also `continue` before `ran.next()`, for the same reason: those calls were not dispatched. The `find` borrows the denial message and clones it into the tool message. `StaticTool` named `terminal` would have returned the string `"terminal"` if it had run. `a_hardline_terminal_command_never_reaches_the_tool` sends `rm -rf /` as the `command` argument, then a text reply. The tool message contains `BLOCKED` and is not `"terminal"`. The outcome is `StopReason::Finished`, because that next scripted reply is text and the phase takes `ModelText`.

The session on this path is manual and interactive, so the deny-list arm and the unattended arm do not run. Hardline and `sudo -S` are the `Deny` values `check` can still return. Hardline's message has no breaker sentence. Three hardline denials in one batch would increment the same session, and the messages would still be `hardline_message`. The breaker the cron test sees is `check` with `Presence::Cron`, not this loop.

## Tests

The approval tests are the original fourteen, plus the loop test that a hardline `terminal` command never reaches the tool. Hardline still denies under YOLO and mode off. `echo "rm -rf /"` is allowed. `bash -c 'rm -rf /'` is not. `sudo -S` is a floor under YOLO. A user deny rule matches before YOLO. An allowlist entry skips the human. Cron denies `rm -rf ./build` and still allows `echo hello`. The default timeout is 300 seconds and the unattended modes are deny. Smart mode denies a `curl` upload and escalates `rm`. The third denial adds the breaker. The script stops. Two asks share one prompt and both saw a free permit. A silent prompt denies in well under two seconds and the permit is back.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch11-approvals
```

```text
Starting 67 tests across 17 binaries
     Summary [   0.133s] 67 tests run: 67 passed, 0 skipped
```

```bash
cargo run -p ch11-approvals
```

```text
verdict: deny
stopped: 1
```

The binary runs the two-call script. No key. `stopped: 1` is the fake model refusing to try `echo hello` after the hardline denial.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `skip_decision` | 249.6 ns | under 1 µs | `check` of `echo hello` |

`skip_decision` calls `skip_decision("echo hello")`, which is `check` on `Policy::manual()` with a fresh session. The string is passed through `black_box`. The timed closure is `b.iter`. Building the policy is inside the call, not a separate setup step. There is no runtime, no sleep, and no prompt. The fast path scans the bytes and returns `Allow`. It does not allocate a normalized copy.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `11`.

### Milestone demo

The `cargo run` above. The verdict is `deny`.

## Pitfalls

The fast path only runs when the deny list is empty. A deny rule of `echo` would be skipped if the fast path ignored the list. It does not. It falls through to the scan.

Quote masking deletes the inside of quotes before the hardline check. That is what lets `echo "rm -rf /"` through. `bash -c 'rm -rf /'` is a second variant: the payload is scanned raw. Forgetting that second variant is how a quoted shell one-liner escapes the floor.

`run_call` drops the permit only on `Ask`. An `Allow` or a `Deny` holds the permit until the function returns. That is the tool body, which this function does not run. A caller that then does its own long wait must not keep the permit either. The test that joins two asks is the one that checks the drop.

The breaker reads the tally after this denial has been counted. Threshold 3 means the third message carries the sentence, not the fourth.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L5.

**A1, the pipeline.** `check` is the single entry. The order is the floor, then YOLO or mode off, then the allowlist, then unattended mode, then smart, then ask.

**B1, hardline.** `rm -rf /`, `mkfs`, raw `dd`, the fork bomb, `kill -1`, and shutdown are denied with `hardline: true`. YOLO, mode off, and cron approve-mode do not move that branch. The message says so.

**B2, dangerous commands.** Recursive `rm`, a sensitive path, a `curl` upload, and inline interpreter code are the dangerous class. They are normalized (`sudo`, `nohup`, `env NAME=value`) before the match. They are not hardline. An interactive session asks. Cron denies them.

**B3, floors that are not the hardline table.** `sudo -S` and `approvals.deny` run in the floor, before YOLO. A deny-list hit is not marked hardline. It still cannot be YOLO'd past.

**A4.** `Presence::Cron`, `SingleQuery`, and `Unattended` read `cron_mode`, `single_query_mode`, and `unattended_mode`. All three default to deny. A safe command still runs. A dangerous one does not wait for a human who is not there.

**A5.** `DEFAULT_TIMEOUT` is 300 seconds. The silent prompt denies. The message includes the stop sentence and "Silence is not consent."

**A6.** The denial text is `_STOP_COMMAND`: the user has not consented, and the model must not retry or rephrase. `run_script` drops the rest of the calls. The breaker sentence is appended at `DENIAL_BREAKER_THRESHOLD`, which is 3.

**Left for later.** A8, a plugin escalating into this prompt, is chapter 12. A2, the `execute_code` script gate, is chapter 13. A9, memory and skill writes, is chapter 25. Secret redaction (E1, E2) is still absent. This chapter does not log the command.

## Chapter summary

- **Pipeline.** Hardline and the other floors first. Then YOLO, the allowlist, unattended deny, smart, and one ask.
- **Prompt.** Two dangerous calls share one mutex. The tool permit is back in the semaphore before either wait starts.
- **Fake model.** `rm -rf /` then `echo hello` produces one verdict, `deny`.
- **Time.** The skip of `echo hello` measured **249.6 ns**. The target is under 1 µs.

[personal note: the first command you put on the allowlist, and whether the floor still caught a cousin of it]

## What you implement

`my-harness/tests/ch11.rs` is the milestone. It compiles, then panics in `todo!("chapter 11")` until `chapter_11_terminal_verdict` in `my-harness/src/milestones.rs` matches this chapter. The function takes a command. For `rm -rf /` the returned text contains `BLOCKED`, and it does not contain `echo hello ran`, because the tool body never ran. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 12 is hooks. A `pre_tool_call` hook can block, rewrite arguments, or escalate into the prompt this chapter just added. The availability check stays in the registry from chapter 3.
