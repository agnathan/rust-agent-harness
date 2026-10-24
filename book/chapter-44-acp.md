# Chapter 44: Editor Integration

Chapter 43 can claim a scheduled job and submit the next one to the inbox. An editor is a different client. It speaks the Agent Client Protocol: a stream of updates, permission requests, and a place to write the result. Teaching the editor about the turn table, the approval floor, and the memory files would duplicate the harness inside the editor. This chapter translates one ACP update into one submit on the session inbox. The crate is `steps/ch44-acp`, a copy of chapter 43 plus `acp`. The package is `ch44-acp`.

`Update` is what the editor sent. `Mail` is what the inbox already understands, the turn, steer, and interrupt from chapter 35. `translate` is the function between them. `Approval` and `map_permission` turn an editor permission into the approval vocabulary chapter 11 already returns, so the editor does not grow a second allow or deny. `Submit` and `Inbox` are the queue, still capped at 64. `Streams` writes the result and a log line back. `stop_reason` maps the harness outcome onto the reason the editor shows. `FakeModel` completes one turn. The demo runs that path. The chapter looks at what a Copilot-style client sends and at the adapter this repo does not vendor. The behavior under test is the inbox, not a live editor session.

Chapter 45 copies this harness and adds a blackboard. The editor stays a client of the inbox the way the chat gateway is a client of the inbox. Two front doors, one mailbox, one agent task. That is why a steer from the editor and a steer from chat cannot fill two queues and race. The MCP server remains the other program. An editor that wants tools connects as an MCP client, or it asks the harness, which already is one. This adapter does not register tools.

The connection to learning is the same as the chat gateway's. A correction typed in the editor is a steer in the mailbox. Reflection sees the turn after it finishes. The adapter's job is to deliver the update once and not to call the model on the translation path. A translate that invoked the provider would hide a model call inside a protocol function, and the turn table would never record it.

Rust, at the translate: the update and the mail are enums, and the mapping is a match that is exhaustive for the updates this chapter handles. The inbox submit is the bounded send from chapter 35. A stream is a writer the test replaces with a buffer.

`cargo run -p ch44-acp --bin ch44-acp` runs the demo. The exercise is `my-harness/tests/ch44.rs`: one ACP update becomes one inbox submit.

**In this chapter**

- 44.1 `INBOX_CAP`, `StopReason`, and `stop_reason`
- 44.2 `Approval` and `map_permission`
- 44.3 `Update`, `Mail`, and `translate`
- 44.4 `Submit` and `Inbox`
- 44.5 `Streams`, `write_result`, and `log_line`
- 44.6 `FakeModel` and `demo`
- 44.7 The Copilot client, the missing adapter, and this inbox

| Section | What it walks |
|---|---|
| 44.1 | `INBOX_CAP`, `StopReason`, `stop_reason` |
| 44.2 | `Approval` and `map_permission` |
| 44.3 | `Update`, `Mail`, `translate` |
| 44.4 | `Submit` and `Inbox` |
| 44.5 | `Streams`, `write_result`, `log_line` |
| 44.6 | `FakeModel` and `demo` |
| 44.7 | the Copilot client, the missing adapter, and this inbox |

> **How Hermes does it**
>
> I looked for the ACP server adapter. `pyproject.toml` lists `acp_adapter` in the setuptools package include, and the script entry `hermes-acp` is `acp_adapter.entry:main`. `hermes_cli/main_agent_cmds.py` does `from acp_adapter.entry import main as acp_main` inside `cmd_acp`. `hermes_cli/_launchers.py` maps `hermes-acp` to `acp_adapter.entry`. On Hermes `main` at `9e01b44` the directory `/tmp/hermes/acp_adapter` is not in the tree. I checked that path. There is no `entry.py`, `session.py`, `server.py`, or `permissions.py` to quote. The import fails closed in `cmd_acp`: an `ImportError` prints that the protocol packages are missing and exits 1. `hermes_cli/subcommands/acp.py` only builds the argparse subcommand. Its help text says the server is for editor integration with VS Code, Zed, and JetBrains. It does not translate an update.
>
> What is in the tree is the Copilot ACP client, `agent/copilot_acp_client.py`, and the gateway's idle-session closer, `gateway/session_acp_lifecycle.py`. `_handle_server_message` treats `method == "session/update"` by reading `params.update`. If `sessionUpdate` is `agent_message_chunk`, it appends `content.text` to `text_parts`. If it is `agent_thought_chunk`, it appends to `reasoning_parts`. That is a list of chunks, not one inbox submit. `session/request_permission` is answered with `{"outcome": {"outcome": "cancelled"}}`. I did not find `allow_once`, `allow_session`, or `allow_always` in that function. Stdout is pumped as JSON lines into a `queue.Queue`. Stderr lines go to a `deque` of the last 40. `gateway/session_acp_lifecycle.py` ends an idle local session whose policy source is `acp`, with reason `acp_disconnect`. It does not translate a session update into a prompt.
>
> The session inbox this harness already has is `inbox.rs`: `INBOX_BOUND` is 64, and `SessionInbox::submit` returns `InboxError::Full` when the queue is at the bound. This chapter's `Inbox` is a separate `VecDeque` with the same cap. `acp.rs` does not call `SessionInbox::submit`.
>
> **Adopted.** The cap of 64, from the session inbox. Stdout for a JSON-RPC result line and stderr for a log line, which is how `copilot_acp_client.py` already splits the Copilot process streams. A cancelled permission in that client is a refusal. Unknown permissions in this chapter deny as well.
>
> **Adapted.** One `Update` becomes one `Mail` through `translate`, then one `Inbox::submit`. The Copilot client appends every chunk into a list and joins them after `session/prompt`. Stop reasons here are `end_turn` and `cancelled`. The permission map is `allow_once`, `allow_session`, `allow_always`, `deny`, and `deny_always`, written in this file because the adapter package is not in the checkout. `FakeModel` sees the update text. The bench translates one update onto an empty inbox and does not encode JSON.
>
> **Deferred.** A real editor session, the `acp_adapter` process, usage tokens on the response, and the permission prompt UI. Wiring `Mail` into `SessionInbox` is not this file. E1 and E2 are chapter 20. This file does not call them.

> **Guardrails.** The catalog has no ACP row. The permission map fails closed. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

## What this chapter decides

`translate` copies the update text into a `Mail`. The `cancelled` flag is not part of the mail. `Inbox::submit` accepts that mail until the cap. The 65th call returns `Full` and does not push. `INBOX_CAP` is 64.

`stop_reason(false)` is `EndTurn`, spelled `end_turn`. `stop_reason(true)` is `Cancelled`, spelled `cancelled`. `write_result` pushes one JSON object onto `stdout`. `log_line` pushes onto `stderr`. The log string is not written into the JSON.

`map_permission` returns `Once`, `Session`, or `Always` for the three allow ids. `deny` and `deny_always` return `Deny`. Any other id returns `Deny`.

`demo` submits one mail, discards a fake reply, logs `session started` on stderr, writes an `end_turn` result on stdout, and returns `acp: end_turn`.

## 44.1 `INBOX_CAP`, `StopReason`, and `stop_reason`

`acp.rs` is the new module. `cron.rs` is the chapter 43 copy. `vision.rs` is still the chapter 41 copy. `pub mod acp` is what `chapter_demo.rs` imports. The constant sits above the types:

```rust
pub const INBOX_CAP: usize = 64;
```

`inbox.rs` in the same crate defines `INBOX_BOUND` as 64. Chapter 2 and chapter 35 used that bound for `SessionInbox`. This constant is a second name with the same number, on the adapter's own queue. Changing one does not change the other. The test and the bench both pass `INBOX_CAP` into `Inbox::new`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    EndTurn,
    Cancelled,
}

impl StopReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EndTurn => "end_turn",
            Self::Cancelled => "cancelled",
        }
    }
}

pub fn stop_reason(cancelled: bool) -> StopReason {
    if cancelled {
        StopReason::Cancelled
    } else {
        StopReason::EndTurn
    }
}
```

`as_str` takes `self` by value. `Copy` makes that cheap. The strings are the spellings written into the JSON result. They are not the ACP SDK's identifiers, because that SDK is not linked. The test `cancel_and_end_turn_are_different_reasons` expects `stop_reason(false).as_str()` to be `end_turn` and `stop_reason(true).as_str()` to be `cancelled`. A cancelled turn does not report `end_turn`.

`Update` stores the same bool. `demo` passes `update.cancelled` into `stop_reason` after `translate`. `translate` does not read the flag. The stop reason and the mail are two outputs of one update.

### Rust: `match` on a `Copy` enum

`match` has to name both variants. A new variant would fail to compile until `as_str` learned a string for it. Taking `self` by value on a `Copy` type does not move the original if the caller still needs it. `stop_reason` returns the enum. The caller can call `as_str` and still hold the enum, because the copy went into the method. `PartialEq` is why a test could also compare `stop_reason(true)` to `StopReason::Cancelled`. The test compares the strings instead.

## 44.2 `Approval` and `map_permission`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approval {
    Once,
    Session,
    Always,
    Deny,
}

/// Unknown option ids deny. `deny_always` is a deny.
pub fn map_permission(option_id: &str) -> Approval {
    match option_id {
        "allow_once" => Approval::Once,
        "allow_session" => Approval::Session,
        "allow_always" => Approval::Always,
        "deny" | "deny_always" => Approval::Deny,
        _ => Approval::Deny,
    }
}
```

The three allow ids keep three outcomes. `deny` and `deny_always` share `Deny` through a `|` pattern. The `_` arm is also `Deny`. `allow_forever` is the test's unknown id, and it denies. An empty string denies. `Allow_once` denies, because the match is case-sensitive.

I did not copy this table from `acp_adapter/permissions.py`. That file is not in the checkout. The Copilot client answers `session/request_permission` with outcome `cancelled` and does not look at an option id. This function is the adapter's local map so a later UI can pass an option id and get a closed set of answers. Unknown stays deny so a new option id from an editor does not become an allow.

`map_permission` is not called by `translate`, `submit`, or `demo`. The permission decision and the inbox submit are separate. The test `unknown_permissions_deny` lists the five known ids and `allow_forever`.

### Rust: or-patterns and `_`

`"deny" | "deny_always"` is one arm. Both strings produce the same value. `_` matches every other `&str`. Putting `_` before the allow arms would make those arms unreachable and the compiler would say so. The allow arms are first. A typo in an editor's option id hits `_` and denies. That is the fail-closed choice. Returning `Once` from `_` would let an unrecognized id run the tool once.

## 44.3 `Update`, `Mail`, and `translate`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pub text: String,
    pub cancelled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mail {
    pub text: String,
}

pub fn translate(update: &Update) -> Mail {
    Mail {
        text: update.text.clone(),
    }
}
```

`Update` is what the editor side would hand the adapter. Both fields are public. `Mail` is the inbox payload, and it has only `text`. `translate` clones the text and drops the flag on the floor. One update in, one mail out. The function does not submit. The caller submits. That split is what the bench times: `translate`, then `submit`.

The test builds `Update { text: "hello", cancelled: false }`, translates it, and expects `Mail { text: "hello" }`. A cancelled update with the same text would produce the same mail. The cancel is visible only to `stop_reason`.

The Copilot client does the other shape. Each `session/update` chunk is appended, and `_run_prompt` returns `"".join(text_parts)` after the prompt request finishes. Several chunks become one string there. Here, one `Update` is already one string, and `translate` does not join a list. A caller who has ten chunks and calls `translate` ten times gets ten mails, if the inbox has room.

### Rust: clone the `String`, borrow the update

`translate` takes `&Update` so the caller still has `cancelled` afterward. `demo` uses `update.text` for the model and `update.cancelled` for the stop reason after `translate` has returned. If `translate` took `Update` by value, `demo` would have to clone the update first or split the fields before the call. The clone that does happen is `update.text.clone()`, one owned `String` inside the `Mail`. The inbox then owns that mail. The update's `String` stays with the update.

`PartialEq` on both structs is why the test writes `assert_eq!(mail, Mail { text: "hello".into() })`.

## 44.4 `Submit` and `Inbox`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Submit {
    Accepted,
    Full,
}

pub struct Inbox {
    q: VecDeque<Mail>,
    cap: usize,
}

impl Inbox {
    pub fn new(cap: usize) -> Self {
        Self {
            q: VecDeque::with_capacity(cap),
            cap,
        }
    }

    pub fn submit(&mut self, mail: Mail) -> Submit {
        if self.q.len() >= self.cap {
            return Submit::Full;
        }
        self.q.push_back(mail);
        Submit::Accepted
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    pub fn front(&self) -> Option<&Mail> {
        self.q.front()
    }
}
```

`new` stores the cap and reserves that much space. `with_capacity` does not fill the deque. `len` starts at 0. `is_empty` is true. A cap of 0 would make the first `submit` return `Full`. The tests use `INBOX_CAP`.

`submit` compares `len` to `cap` before `push_back`. At 64, the 65th mail is not stored. The returned `Full` is the only signal. The function does not drop the oldest mail to make room. The caller still owns the `Mail` only if they did not move it in. They did move it in. On the `Full` path the mail is dropped when the function returns, because the parameter goes out of scope without being pushed. A caller who needs the text after a refusal has to clone it before `submit`.

`front` borrows the oldest mail. `SessionInbox::submit` in `inbox.rs` pushes a `String` and returns `Result<(), InboxError>`. This `submit` pushes a `Mail` and returns `Submit`. The names match. The types do not. One ACP update becomes one of these submits. The milestone `chapter_44_update_submits_once(1)` asserts the count equals 1. The step crate's path for one update is `translate` then a single `submit` that returns `Accepted` on an empty inbox.

The test `an_update_lands_on_the_inbox` translates `hello`, expects the mail, submits it, and reads `front` as `hello`. It then submits `INBOX_CAP - 1` more mails, each `Accepted`, then one more `Mail { text: "more" }` and expects `Full`. `len` stays `INBOX_CAP`. `is_empty` is false. The first mail is still `hello`, because `Full` did not push and nothing popped.

### Rust: `VecDeque` and a moved `Mail`

`push_back` adds at the tail. `front` reads the head. A queue of edits stays in arrival order. `VecDeque::with_capacity` allocates the buffer up front so the first 64 pushes do not reallocate, if the cap is 64. The `Full` path does not use that spare capacity. Length, not capacity, is the guard.

`Mail` is not `Copy`. `submit` takes ownership. `translate` returns a new `Mail`, and that value is the argument. The bench does those two calls inside the timed closure.

## 44.5 `Streams`, `write_result`, and `log_line`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Streams {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
}

pub fn write_result(streams: &mut Streams, reason: StopReason) {
    streams.stdout.push(format!(
        r#"{{"jsonrpc":"2.0","result":{{"stopReason":"{}"}}}}"#,
        reason.as_str()
    ));
}

pub fn log_line(streams: &mut Streams, line: &str) {
    streams.stderr.push(line.to_string());
}
```

`Default` builds two empty vecs. `write_result` formats one line. For `EndTurn` the line is:

```text
{"jsonrpc":"2.0","result":{"stopReason":"end_turn"}}
```

For `Cancelled` the stop reason field is `cancelled`. The raw string `r#"..."#` keeps the braces. Each doubled `{{` or `}}` becomes one brace in the output. `{}` is the format placeholder for `reason.as_str()`. There is no newline appended. There is no `id` field and no usage object.

`log_line` copies the argument onto stderr. It does not look at stdout. The test `logs_stay_off_stdout` logs `session started`, writes an end-turn result, and expects stderr to be that one line. Stdout has length 1, contains `end_turn`, and does not contain `session started`. A second `write_result` of the cancelled reason makes `stdout[1]` contain `cancelled`. The two vecs stay separate. An editor that parsed every stdout line as JSON would see two result objects and would not see the log.

`copilot_acp_client.py` pumps `proc.stdout` into the JSON inbox and `proc.stderr` into `stderr_tail`. This struct is the in-memory picture of that split. Nothing here writes a process handle. `println!` is not called. A later chapter that writes the real handles has to keep the log on stderr. Mixing them would put `session started` on the protocol stream.

### Rust: raw strings and doubled braces

`format!` treats `{` as the start of a placeholder. A JSON object written in a normal string has to escape every brace. The raw string `r#"..."#` does not interpret backslashes, and doubled braces are still the format escape. `as_str` returns a value with no quotes in it, so the inserted text does not break the JSON for these two reasons. A stop reason that contained a quote would. Neither spelling does.

`Streams` derives `Default`, so `Streams::default()` is the demo's starting point. The fields are public. A test can read `streams.stdout[0]` without a getter.

## 44.6 `FakeModel` and `demo`

```rust
pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

pub fn demo() -> String {
    let mut streams = Streams::default();
    let mut inbox = Inbox::new(INBOX_CAP);
    let update = Update {
        text: "hello".to_string(),
        cancelled: false,
    };
    let mail = translate(&update);
    inbox.submit(mail);
    let _ = FakeModel { reply: "ok" }.complete(&update.text);
    log_line(&mut streams, "session started");
    let reason = stop_reason(update.cancelled);
    write_result(&mut streams, reason);
    format!("acp: {}", reason.as_str())
}
```

The order is fixed. Build the streams and an inbox of 64. Build an update whose text is `hello` and whose flag is false. Translate it. Submit the mail. Ask `FakeModel` for a reply and drop `ok`. Log on stderr. Compute the stop reason from the flag, which is `EndTurn`. Write the JSON. Return `acp: end_turn`, because `format!("acp: {}", reason.as_str())` inserts `end_turn`.

`submit`'s `Submit` value is not checked. On an empty inbox the result is `Accepted`. `demo` does not read `front`. The printed line is the stop reason, not the mail text. If `cancelled` were true, the same function would still submit one mail and would return `acp: cancelled`. The test `demo_ends_the_turn` locks the false flag's line: `demo()` equals `acp: end_turn`.

`chapter_demo.rs` prints that string:

```rust
use harness_core::acp::demo;

fn main() {
    println!("{}", demo());
}
```

`cargo run` needs `--bin ch44-acp`. The package also builds `ch44_mcp_fixture` and `sandbox-worker`. The example binary prints eval lines. No key is read. `FakeModel::complete` ignores the prompt, the same way the cron and service fakes do. This one is `harness_core::acp::FakeModel`. The cron module has its own `FakeModel`. They are different types in different modules.

### Rust: the format of the demo line

`format!("acp: {}", reason.as_str())` is the whole return value. The colon and the space are literal. `as_str` supplies `end_turn` or `cancelled` and nothing else. The test compares the full string. A change to the JSON line does not change `demo`'s return value. The JSON is on `streams`, and `demo` drops `streams` at the end of the function. The test of the streams is `logs_stay_off_stdout`, which builds its own `Streams`.

## 44.7 The Copilot client, the missing adapter, and this inbox

`cmd_acp` in `hermes_cli/main_agent_cmds.py` is the launcher. It copies a few flags into the environment, then:

```python
from acp_adapter.entry import main as acp_main
acp_main([flag for attr, flag in _ACP_FLAGS if getattr(args, attr, False)])
```

The `except ImportError` prints `The ACP server can't start: its protocol packages are missing from this install.` and exits 1. I listed `/tmp/hermes/acp_adapter` and the directory is absent, so that import is the path a checkout of this commit would take. `pyproject.toml` still has `hermes-acp = "acp_adapter.entry:main"` and still lists `"acp_adapter"` in `[tool.setuptools.packages.find]`. The package name is declared. The sources are not in the tree I read. I am not going to invent `permissions.py` or a `PromptResponse` from a file that is not there.

`hermes_cli/subcommands/acp.py` attaches the subcommand. The help string is `Run Hermes Agent as an ACP (Agent Client Protocol) server`, and the description names VS Code, Zed, and JetBrains. The flags are `--version`, `--check`, `--setup`, `--setup-browser`, and `--yes`. None of them map an option id or write a stop reason.

The ACP code that does translate a session update is `agent/copilot_acp_client.py`, and it is a client of the `copilot --acp` process, not a server an editor connects to. `_session` spawns the process, starts a thread that JSON-decodes each stdout line into a `queue.Queue` named `inbox`, and starts another thread that appends stderr lines to a `deque` of maxlen 40. The word `inbox` there is the queue of protocol messages, not a session prompt queue. `_request` writes one JSON-RPC object to the process stdin and waits on that queue.

`_handle_server_message` is the translation. For `session/update` it reads `params.update`, then `content.text`, then `sessionUpdate`:

```python
if method == "session/update":
    update = (msg.get("params") or {}).get("update") or {}
    content = update.get("content") or {}
    chunk_text = str(content.get("text") or "") if isinstance(content, dict) else ""
    sinks = {"agent_message_chunk": text_parts, "agent_thought_chunk": reasoning_parts}
    if chunk_text and (sink := sinks.get(str(update.get("sessionUpdate") or "").strip())) is not None:
        sink.append(chunk_text)
    return True
```

Each chunk is appended. `_run_prompt` joins `text_parts` after `session/prompt` returns. Many updates become one string. This chapter's `translate` does the opposite shape: the `Update` already holds one `text`, and one call returns one `Mail`. A thought chunk and a message chunk are not two types here. There is one text field.

For `session/request_permission` the same function writes back `{"outcome": {"outcome": "cancelled"}}` and does not read an option id. `map_permission` is the local table this crate can test without that process. `allow_once` is `Once`, `allow_session` is `Session`, `allow_always` is `Always`, and both deny spellings plus every other string are `Deny`. The Copilot client has no `Once`. Its permission answer is the cancelled outcome.

`gateway/session_acp_lifecycle.py` documents a different gap: ACP v0.9 has no per-session destroy, so the last viewer leaving an idle ACP session ends it. `ACP_END_REASON` is `acp_disconnect`. `end_idle_acp_session` returns false when the session still has subscribers, when the receipt is not a local session, or when `policy.source` is not `acp`. Otherwise it calls `end_idle_local_session` with that reason. `end_idle_viewer_session` keeps the ACP reason for ACP rows and uses the viewer's reason for everyone else. None of these functions call `translate` or push a prompt. Ending a session is not an inbox submit.

`SessionInbox` in `inbox.rs` is the chapter 2 queue. `INBOX_BOUND` is 64. `submit` takes `impl Into<String>`, locks a `Mutex<VecDeque<String>>`, and returns `Err(InboxError::Full { bound })` when `len >= bound`. `new` caps the allocation with `bound.min(64)`. `acp::Inbox` is a different type. Its `submit` takes a `Mail` by value, needs `&mut self`, returns `Submit::Full` with no bound field, and does not lock. `INBOX_CAP` is also 64. The numbers match. The types do not. `demo` constructs `Inbox::new(INBOX_CAP)` and never mentions `SessionInbox`. A later wiring could take `mail.text` and call `SessionInbox::submit`. This file does not.

Count the test `an_update_lands_on_the_inbox`. The first `submit` is the translated `hello`. The loop is `for n in 0..(INBOX_CAP - 1)`, which is 0 through 62, sixty-three integers. 1 + 63 = 64. `len` equals `INBOX_CAP`. `is_empty` is false. The next `Mail { text: "more" }` hits `len >= cap` and returns `Full`. `front` is still `hello` because `Full` does not `push_back` and nothing calls `pop_front`. `SessionInbox` would have returned `InboxError::Full` at the same count. The milestone passes `1` to `chapter_44_update_submits_once` and wants `1`. That is one update, one submit, not the fill-up. The fill-up is the cap test. The milestone is the one-to-one path `translate` then `submit` on an empty inbox, which is also what `translate_update` measures.

`demo` builds the streams and then drops them. After `translate` and `submit`, the inbox holds one mail, `hello`, and `Submit` was `Accepted` even though `demo` ignores the enum. `FakeModel { reply: "ok" }.complete("hello")` returns `ok` and the `let _` drops it. `log_line` leaves `stderr == ["session started"]`. `update.cancelled` is false, so `stop_reason` is `EndTurn` and `as_str` is `end_turn`. `write_result` pushes this exact line:

```text
{"jsonrpc":"2.0","result":{"stopReason":"end_turn"}}
```

`format!("acp: {}", reason.as_str())` is `acp: end_turn`. The JSON is not in that string. `streams` goes out of scope at the closing brace of `demo`. The test `demo_ends_the_turn` only sees the returned `String`. The test `logs_stay_off_stdout` rebuilds `Streams` and checks the vecs, including a second `write_result(stop_reason(true))` whose line contains `cancelled` and does not replace the first line.

`ch44-harness-cli` depends on `ch44-acp` by path. `ch44-acp` does not depend on `ch43-cron`. `pub mod cron` is the copied module. `cargo run -p ch44-acp --bin ch44-acp` selects `chapter_demo.rs`. The example target `demo` still runs `run_saved_evals` and would not print `acp: end_turn`.

### Rust: two inboxes, one number

`INBOX_CAP` and `INBOX_BOUND` are both `usize` constants equal to 64. Rust will not stop you from changing one. A comment in `acp.rs` does not mention `inbox.rs`. The shared number is a decision recorded in this chapter, not a const alias. `use crate::inbox::INBOX_BOUND` inside `acp.rs` would have made them the same item. The file defines its own. The bench passes `INBOX_CAP` into `Inbox::new`, so a reader who changes `INBOX_BOUND` and reruns `translate_update` is still allocating 64.

`iter_batched` with `BatchSize::SmallInput` asks Criterion to clone the setup value across a small batch of timed iterations. The setup closure is `|| Inbox::new(INBOX_CAP)`. The timed closure takes that inbox by value, translates, and submits. The `Update` is captured by reference from the outer `let`. `black_box(&update)` keeps the compiler from lifting the clone of `hello` out of the sample. `black_box(submitted)` keeps it from deleting the `submit` call once it sees that nobody reads `Accepted`.

`write_result` of `StopReason::Cancelled` pushes a second kind of line, and the test checks it after the end-turn line is already in the vec:

```text
{"jsonrpc":"2.0","result":{"stopReason":"cancelled"}}
```

There is no trailing newline in the `format!`. A parser that splits stdout on `\n` would see this object only after something else wrote the newline. This function does not. The two lines in the test are two `String`s, not one buffer split on newlines. `stdout[0]` contains `end_turn` and does not contain `session started`. `stdout[1]` contains `cancelled`. `stderr` stays a one-element vec. That is the whole contract of `Streams` in this file.

`map_permission` is pure. It does not read `Update` and it does not submit. Calling it six times in `unknown_permissions_deny` builds no inbox. `Approval` is `Copy`, so the returned value is the test's to compare. `Once`, `Session`, and `Always` are three variants because a later caller might persist "always" differently from "once". This file does not persist them. `Deny` is one variant for two input strings plus the default arm, so the caller cannot tell `deny` from `deny_always` from `allow_forever` after the call. The distinction lives only in the input `&str`.

`lib.rs` lists `pub mod acp` and, above it, `pub mod cron`. Both are public. A test in this package can still call `harness_core::cron::run_job`. The ACP tests do not. Copying the chapter 43 crate forward is what keeps `cron.rs` compiling here. There is no `use crate::cron` inside `acp.rs`. The adapter does not scan a prompt and does not claim a job.

`FakeModel` in this module takes `&str` and returns `self.reply.to_string()`. `demo` passes `&update.text`, which is `hello`, and drops the result. The model is constructed so the chapter has a keyless reply the way the earlier chapters do. Routing does not depend on the reply. The stop reason does not depend on the reply. The mail text is the update text, cloned before the model runs.

`front` returns `Option<&Mail>`. On an empty inbox it is `None`. After the demo's `submit`, it would be `Some` of a mail whose text is `hello`, and `cancelled` is not stored on that mail. `len` would be 1. `is_empty` would be false. `demo` does not call those methods. The test `an_update_lands_on_the_inbox` does, and it checks `front` before the fill loop so the assertion is about the translated text rather than about mail number 63. Mail numbers `0` through `62` are the strings of those integers. They sit behind `hello`. Nothing in `acp.rs` pops them. A reader who wants the next edit calls `front` again and still sees `hello` until some other function, not written here, removes the head.

The bench never fills the queue. Each timed iteration receives an inbox from `Inbox::new`, submits once, and drops that inbox at the end of the closure. The next iteration gets another empty inbox from the setup. `Submit::Full` is not a value the bench can return unless `INBOX_CAP` were 0. It is 64. The measured work is one clone of `hello` and one `push_back` of a one-element deque.

## Tests

The new tests are `steps/ch44-acp/crates/harness-core/tests/acp.rs`. The package also builds the copied harness tests, including `cron.rs` and `vision.rs`. The nextest line below is the package run. I am not typing a pass count for it.

`cancel_and_end_turn_are_different_reasons` expects the two `as_str` spellings.

`logs_stay_off_stdout` is the stream split in section 44.5, including a second stdout line that contains `cancelled`.

`unknown_permissions_deny` expects `allow_once` to be `Once`, `allow_session` to be `Session`, `allow_always` to be `Always`, both deny ids to be `Deny`, and `allow_forever` to be `Deny`.

`an_update_lands_on_the_inbox` is one translate of `hello`, one accepted submit, a front of `hello`, then fills the inbox to `INBOX_CAP` and expects the next submit to be `Full` with `len` still 64.

`demo_ends_the_turn` expects `demo()` to equal `acp: end_turn`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch44-acp
```

```text
    Starting 229 tests across 49 binaries
     Summary [   0.498s] 229 tests run: 229 passed, 0 skipped
```

```bash
cargo run -p ch44-acp --bin ch44-acp
```

```text
acp: end_turn
```

The binary translates one update and ends the turn. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `translate_update` | 132.4 ns | one update onto an empty inbox | `translate` then `submit` |

The function name is `translate_update`. The bench id is `44/translate_update`. The old ids `42/translate_update` and `40/translate_update` are aliases in `benchmarks/descriptions.json`. The function name did not change when the id moved.

```rust
fn translate_update(c: &mut Criterion) {
    let update = Update {
        text: "hello".to_string(),
        cancelled: false,
    };
    c.bench_function("translate_update", |b| {
        b.iter_batched(
            || Inbox::new(INBOX_CAP),
            |mut inbox| {
                let mail = translate(black_box(&update));
                let submitted = inbox.submit(mail);
                black_box(submitted);
            },
            BatchSize::SmallInput,
        );
    });
}
```

The `Update` is built before the bench, with text `hello` and `cancelled` false. `iter_batched` runs the setup closure outside the timer. That closure calls `Inbox::new(64)` and is not timed. The timed routine calls `translate` with the update passed through `black_box`, then `submit` of the mail, then `black_box` of the `Submit` value. The string clone and the push are inside the sample. The inbox is empty at the start of each sample, so the result is `Accepted`. `write_result`, `log_line`, `map_permission`, and `FakeModel` are not called. The `Full` path is not taken. The descriptions file says the budget is under a millisecond. The mean cell is the measurement.

The mean is `132.4 ns`. It is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `44`. The id is `44/translate_update`. The old ids `42/translate_update` and `40/translate_update` are aliases in `benchmarks/descriptions.json`. `.github/scripts/run_bench_section.sh` runs `cargo bench -p ch44-acp --bench acp --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10` for that section. The same flags are in `steps/ch44-acp/README.md`. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `acp: end_turn`.

## Pitfalls

The bench is the clone and the push. Encoding the JSON line is `write_result`, and that function is the stream test. A sample that included `write_result` would be formatting a string as well as cloning `hello`. A sample that wrote a real stdout handle would be a different program. This one uses `iter_batched` so the empty inbox is not part of the measured work. Building the `Update` is also outside the closure. The text `hello` is cloned from the outer update on every sample.

Stdout and stderr are vecs. `log_line` cannot reach `stdout` unless someone later concatenates the vecs. The test checks that `session started` is absent from `stdout[0]`. A `format!` that interpolated the log into the JSON would fail that test.

`submit` on a full inbox drops the `Mail`. The 65th text is gone. ACP does not pop the front to make room. `front` still returns the first accepted mail. `len` stays 64.

`translate` ignores `cancelled`. A caller who only submits the mail and never calls `stop_reason` will enqueue a cancelled turn as ordinary text. `demo` does both. The return value follows the flag. The mail does not.

`map_permission` is case-sensitive, and `_` denies. `deny_always` is a deny, not a distinct variant. There is no `Allow` catch-all.

`Inbox` is not `SessionInbox`. Both cap at 64. Submitting a `Mail` does not push a string onto the chapter 2 queue. The milestone counts submits of this inbox. `chapter_44_update_submits_once(1)` wants 1.

The Copilot client joins many `session/update` chunks into one prompt result. Calling `translate` once per chunk will fill this inbox one mail at a time. Sixty-five chunks on an empty inbox of 64 yield one `Full`.

`demo` drops `Streams`. The JSON it built does not appear in the printed line. The printed line is `acp: end_turn`.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). It has no ACP row. I did not find an ACP guardrail id to attach this adapter to. The behaviors below are the ones this file actually implements.

**Permission bridge.** `allow_once`, `allow_session`, and `allow_always` keep their three outcomes. `deny` and `deny_always` deny. An option id the function does not name denies. The `acp_adapter` package that might have held this table is not in the Hermes tree I checked. `agent/copilot_acp_client.py` answers `session/request_permission` with `cancelled`.

**Streams.** The result is one JSON line on `stdout`. The log is a different vec. `logs_stay_off_stdout` checks that the log is absent from the protocol line.

**Stop.** `stop_reason(true)` is `cancelled`. `stop_reason(false)` is `end_turn`. `as_str` is what `write_result` and `demo` insert.

**Inbox.** `INBOX_CAP` is 64, the same number as `INBOX_BOUND`. One `translate` plus one `submit` is one update. The next submit past the cap is `Full`.

**Left for later.** The editor process, the missing `acp_adapter` directory, and redaction of a secret inside an update. E1 and E2 are chapter 20. `Mail` is not passed through `redact_for_egress`.

## What you implement

`my-harness/tests/ch44.rs` is the milestone.

```rust
use my_harness::chapter_44_update_submits_once;

#[test]
fn one_acp_update_becomes_one_inbox_submit() {
    assert_eq!(chapter_44_update_submits_once(1), 1);
}
```

The stub panics.

```rust
pub fn chapter_44_update_submits_once(updates: usize) -> usize {
    let _ = updates;
    todo!("chapter 44")
}
```

`todo!("chapter 44")` is the panic. The test passes `1` and wants `1` back: one update becomes one submit. In the step crate that is `translate` of one `Update`, then one `Inbox::submit` that returns `Accepted` while the queue is under `INBOX_CAP`. The stub stays `todo!`. Do not paste `acp.rs` into `my-harness/`.

## Chapter summary

- **Client.** `translate` copies one update into one `Mail`. `Inbox::submit` takes that mail until 64.
- **Stop.** `end_turn` or `cancelled`, from the flag `translate` does not store.
- **Streams.** JSON on stdout. Logs on stderr.
- **Permission.** The three allow ids keep their outcomes. `deny`, `deny_always`, and any unknown id deny.
- **Model.** `FakeModel` answers `ok`. The demo line is `acp: end_turn`.
- **Time.** `translate_update` mean `132.4 ns`. That is `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The Criterion `time:` line is the slope.

## What's next

Chapter 45 is the blackboard. Agents claim shared state. The lock is not held across a model call.
