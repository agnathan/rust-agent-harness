# Chapter 44: Editor Integration

Two front doors, one mailbox, one agent task. An editor that learned the turn table, the approval floor, and the memory files would duplicate the harness inside the editor. So chat and the editor both become mail on the inbox the harness already runs. A steer typed in the editor and a steer typed in chat then share that mailbox. So the two can't fill separate queues and race.

A correction typed in the editor is a steer, reflection sees the turn after it finishes. `translate` must not call the model, which is why this adapter matters to a continuously improving harness. The adapter's job is to deliver the update once and to keep a provider call off the translation path. A translate that invoked the provider would hide a model call inside a protocol function, and the turn table would never record it.

The crate is `steps/ch44-acp`, a copy of chapter 43 plus `acp`, the package is `ch44-acp`. Chapter 45 copies this harness onward. `cargo run -p ch44-acp --bin ch44-acp` runs the demo and prints `acp: end_turn`. The exercise is `my-harness/tests/ch44.rs`, where one ACP update becomes one inbox submit.

The editor sends an `Update`, and `translate` turns it into a `Mail` that carries only the text, the same mailbox chapter 35 fills with a turn, a steer, or an interrupt. The cancelled flag stays off that mail. `map_permission` turns an editor option id into the approval vocabulary chapter 11 already returns. So the editor doesn't grow a second allow or deny. `Inbox::submit` accepts that mail until 64, `Streams` writes the result and a log line back on separate vecs. `stop_reason` maps the outcome onto the reason the editor shows. `FakeModel` completes one turn after the mail is queued. The MCP server stays the other program: an editor that wants tools connects as an MCP client, or it asks the harness, which already is one, and this adapter doesn't register tools.

At the translate, the update and the mail are separate types, and the submit is the bounded send from chapter 35. A stream is a writer the test replaces with a buffer.

| Section | What it walks |
|---|---|
| 44.1 | `INBOX_CAP`, `StopReason`, `stop_reason` |
| 44.2 | `Approval` and `map_permission` |
| 44.3 | `Update`, `Mail`, `translate` |
| 44.4 | `Submit` and `Inbox` |
| 44.5 | `Streams`, `write_result`, `log_line` |
| 44.6 | `FakeModel` and `demo` |
| 44.7 | what this chapter doesn't build |

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

`translate` copies the update text into a `Mail`, and the `cancelled` flag stays off that mail. `Inbox::submit` accepts mail until the cap, and the 65th call returns `Full` without pushing, where `INBOX_CAP` is 64.

`stop_reason(false)` is `EndTurn`, spelled `end_turn`, and `stop_reason(true)` is `Cancelled`, spelled `cancelled`. `write_result` pushes one JSON object onto `stdout`, `log_line` pushes onto `stderr`, and the log string isn't written into the JSON.

`map_permission` returns `Once`, `Session`, or `Always` for the three allow ids, returns `Deny` for `deny` and `deny_always`, and returns `Deny` for any other id.

The demo submits one mail, discards a fake reply, logs `session started` on stderr, writes an `end_turn` result on stdout, and returns `acp: end_turn`.

## 44.1 `INBOX_CAP`, `StopReason`, and `stop_reason`

The new module is `acp.rs`, `cron.rs` is the chapter 43 copy, `vision.rs` is still the chapter 41 copy. `pub mod acp` is what `chapter_demo.rs` imports. The constant sits above the types:

```rust
pub const INBOX_CAP: usize = 64;
```

`inbox.rs` in the same crate defines `INBOX_BOUND` as 64, the bound chapter 2 and chapter 35 used for `SessionInbox`. Both `INBOX_CAP` and `INBOX_BOUND` are 64, and they aren't aliases. The adapter defines its own constant instead of using `crate::inbox::INBOX_BOUND`, so changing one leaves the other. That smell is left in the file. The test and the bench both pass `INBOX_CAP` into `Inbox::new`.

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

The strings from `as_str` are what the JSON result inserts, and the two reasons differ, `end_turn` against `cancelled`. They aren't the ACP SDK's identifiers, because that SDK isn't linked. The test `cancel_and_end_turn_are_different_reasons` expects `stop_reason(false).as_str()` to be `end_turn` and `stop_reason(true).as_str()` to be `cancelled`. So a cancelled turn doesn't report `end_turn`.

`Update` stores the same bool, the demo passes `update.cancelled` into `stop_reason` after `translate`. `translate` doesn't read the flag, so the stop reason and the mail are two outputs of one update.

## 44.2 `Approval` and `map_permission`

I didn't copy this table from `acp_adapter/permissions.py`, because that file isn't in the checkout, and unknown option ids deny.

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

The three allow ids keep three outcomes, `deny` and `deny_always` share `Deny` through a `|` pattern, and the `_` arm is also `Deny`. `allow_forever` is the test's unknown id and it denies, an empty string denies. `Allow_once` denies, because the match is case-sensitive. The allow arms stay first, since putting `_` ahead of them would make those arms unreachable. A typo in an editor's option id hits `_` and denies. Returning `Once` from `_` would let an unrecognized id run the tool once.

The Copilot client answers `session/request_permission` with outcome `cancelled` and doesn't look at an option id. This function is the adapter's local map. So a later UI can pass an option id and get a closed set of answers. The ids it names are `allow_once`, `allow_session`, `allow_always`, `deny`, and `deny_always`. `map_permission` isn't called by `translate`, `submit`, or `demo`, so the permission decision and the inbox submit stay separate. The test `unknown_permissions_deny` lists those five ids and `allow_forever`.

After the call, the caller can't tell `deny` from `deny_always` from `allow_forever`. `Deny` is one variant for two input strings plus the default arm. The distinction lives only in the input `&str`. `Once`, `Session`, and `Always` stay three variants because a later caller might persist an always differently from a once, and this file doesn't persist them.

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

`Update` is what the editor side would hand the adapter, and both fields are public. `Mail` is the inbox payload, and it has only `text`. `translate` clones the text and drops the flag, so one update in becomes one mail out, and the function doesn't submit. The caller submits, and that split is what the bench times, `translate` and then `submit`.

The test builds `Update { text: "hello", cancelled: false }`, translates it, and expects `Mail { text: "hello" }`. A cancelled update with the same text would produce the same mail, and the cancel is visible only to `stop_reason`.

The Copilot client does the other shape. Each `session/update` chunk is appended, and `_run_prompt` returns `"".join(text_parts)` after the prompt request finishes. So several chunks become one string there. Here, one `Update` is already one string. `translate` doesn't join a list. A caller who has ten chunks and calls `translate` ten times gets ten mails, if the inbox has room.

> **Rust sidebar: borrow the update, own the mail**
>
> `translate` takes `&Update` so the caller still has `cancelled` afterward, which is why `demo` can pass `update.text` to the model and `update.cancelled` to `stop_reason` after `translate` has returned. The clone that does happen is `update.text.clone()`, one owned `String` inside the `Mail`, and the inbox then owns that mail while the update keeps its own `String`.

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

`new` stores the cap and reserves that much space, `with_capacity` doesn't fill the deque, `len` starts at 0, and `is_empty` is true. A cap of 0 would make the first `submit` return `Full`, and the tests use `INBOX_CAP`.

`submit` compares `len` to `cap` before `push_back`. So at 64 the 65th mail isn't stored, `Full` is the only signal. The function doesn't drop the oldest mail to make room. The caller moved the `Mail` in, so on the `Full` path that mail is dropped when the function returns. The parameter goes out of scope without being pushed. A caller who needs the text after a refusal has to clone it before `submit`.

`front` borrows the oldest mail, `SessionInbox::submit` in `inbox.rs` pushes a `String` and returns `Result<(), InboxError>`. This `submit` pushes a `Mail` and returns `Submit`. The names match and the types don't, and one ACP update becomes one of these submits. The milestone `chapter_44_update_submits_once(1)` asserts the count equals 1. The step crate's path for one update is `translate` then a single `submit` that returns `Accepted` on an empty inbox.

The test `an_update_lands_on_the_inbox` translates `hello`, expects the mail, submits it, and reads `front` as `hello`. It then submits `INBOX_CAP - 1` more mails, each `Accepted`, then one more `Mail { text: "more" }` and expects `Full`. So `len` stays `INBOX_CAP`, `is_empty` is false. The first mail is still `hello`. `Full` didn't push and nothing popped. The fill loop is `for n in 0..(INBOX_CAP - 1)`, which is 0 through 62, sixty-three integers. So 1 + 63 = 64, and those strings sit behind `hello`. `front` still returns `hello` until some other function, not written here, removes the head.

`push_back` adds at the tail and `front` reads the head, so a queue of edits stays in arrival order. `VecDeque::with_capacity` allocates the buffer up front so the first 64 pushes don't reallocate when the cap is 64. The `Full` path doesn't use that spare capacity. Length, not capacity, is the guard.

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

`write_result` formats one line, and for `EndTurn` the line is:

```text
{"jsonrpc":"2.0","result":{"stopReason":"end_turn"}}
```

For `Cancelled` the stop reason field is `cancelled`. The raw string `r#"..."#` keeps the braces, each doubled `{{` or `}}` becomes one brace in the output. `{}` is the format placeholder for `reason.as_str()`. Nothing appends a newline, and the line has no `id` field and no usage object. `as_str` returns a spelling with no quote in it, so the inserted text doesn't break the JSON for these two reasons. Neither `end_turn` nor `cancelled` contains a quote.

`log_line` copies the argument onto stderr and doesn't look at stdout. The test `logs_stay_off_stdout` logs `session started`, writes an end-turn result, and expects stderr to be that one line, while stdout has length 1, contains `end_turn`, and doesn't contain `session started`. A second `write_result` of the cancelled reason makes `stdout[1]` contain `cancelled`. The two vecs stay separate, so an editor that parsed every stdout line as JSON would see two result objects and wouldn't see the log.

`copilot_acp_client.py` pumps `proc.stdout` into the JSON inbox and `proc.stderr` into `stderr_tail`, and this struct is the in-memory picture of that split. Nothing here writes a process handle, and `println!` isn't called. A later chapter that writes the real handles has to keep the log on stderr. Mixing them would put `session started` on the protocol stream. The fields are public, so a test can read `streams.stdout[0]` without a getter, and the demo starts from `Streams::default()`.

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

The order is fixed: the demo builds the streams and an inbox of 64, builds an update whose text is `hello` and whose flag is false, translates it, and submits the mail. It asks `FakeModel` for a reply and drops `ok`, logs on stderr, computes the stop reason from the flag, which is `EndTurn`, writes the JSON, and returns `acp: end_turn`. `format!("acp: {}", reason.as_str())` inserts `end_turn`.

The `Submit` value from `submit` isn't checked. So on an empty inbox the result is `Accepted`, the demo doesn't read `front`. The printed line is the stop reason rather than the mail text. If `cancelled` were true, the same function would still submit one mail and would return `acp: cancelled`. The test `demo_ends_the_turn` locks the false flag's line, so `demo()` equals `acp: end_turn`. A change to the JSON line doesn't change that return value, because `demo` drops `streams` at the end of the function. `logs_stay_off_stdout` builds its own `Streams`.

`chapter_demo.rs` prints that string:

```rust
use harness_core::acp::demo;

fn main() {
    println!("{}", demo());
}
```

`cargo run` needs `--bin ch44-acp`, because the package also builds `ch44_mcp_fixture` and `sandbox-worker`. The example binary prints eval lines, and no key is read. `FakeModel::complete` ignores the prompt, the same way the cron and service fakes do. This one is `harness_core::acp::FakeModel`, the cron module has its own `FakeModel`, and they are different types in different modules.

## 44.7 What this chapter doesn't build

`cmd_acp` in `hermes_cli/main_agent_cmds.py` is the launcher, and it copies a few flags into the environment, then:

```python
from acp_adapter.entry import main as acp_main
acp_main([flag for attr, flag in _ACP_FLAGS if getattr(args, attr, False)])
```

The `except ImportError` prints `The ACP server can't start: its protocol packages are missing from this install.` and exits 1. The directory `/tmp/hermes/acp_adapter` isn't in the tree, so that import is the path a checkout of this commit would take. `pyproject.toml` still has `hermes-acp = "acp_adapter.entry:main"` and still lists `"acp_adapter"` in `[tool.setuptools.packages.find]`. The package name is declared and the sources aren't in the tree. So there is no `permissions.py` and no `PromptResponse` there to quote.

`hermes_cli/subcommands/acp.py` attaches the subcommand, with the help string `Run Hermes Agent as an ACP (Agent Client Protocol) server`, a description that names VS Code, Zed, and JetBrains, and the flags `--version`, `--check`, `--setup`, `--setup-browser`, and `--yes`. None of those flags map an option id or write a stop reason.

The ACP code in the tree that reads a session update is the Copilot client `agent/copilot_acp_client.py`, a client of the `copilot --acp` process. So an editor has no server in this file to connect to. `_session` spawns the process, starts a thread that JSON-decodes each stdout line into a `queue.Queue` named `inbox`, and starts another thread that appends stderr lines to a `deque` of maxlen 40. The word `inbox` there is the queue of protocol messages, not a session prompt queue. `_request` writes one JSON-RPC object to the process stdin and waits on that queue. The Copilot client appends chunks, and `_handle_server_message` reads `params.update`, then `content.text`, then `sessionUpdate`:

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

Each chunk is appended, and `_run_prompt` joins `text_parts` after `session/prompt` returns, so many updates become one string. This chapter's `translate` does the opposite, because the `Update` already holds one `text` and one call returns one `Mail`. A thought chunk and a message chunk aren't two types here. For `session/request_permission` the same function writes back `{"outcome": {"outcome": "cancelled"}}` and doesn't read an option id, which is why section 44.2 keeps a local table. The Copilot client has no `Once`.

`gateway/session_acp_lifecycle.py` covers a different gap: ACP v0.9 has no per-session destroy. So the last viewer leaving an idle ACP session ends it. `ACP_END_REASON` is `acp_disconnect`. `end_idle_acp_session` returns false when the session still has subscribers, when the receipt isn't a local session, or when `policy.source` isn't `acp`, and otherwise it calls `end_idle_local_session` with that reason. `end_idle_viewer_session` keeps the ACP reason for ACP rows and uses the viewer's reason for everyone else. None of these functions call `translate` or push a prompt, so ending a session isn't an inbox submit.

`SessionInbox` in `inbox.rs` is the chapter 2 queue. Its `submit` takes `impl Into<String>`, locks a `Mutex<VecDeque<String>>`, and returns `Err(InboxError::Full { bound })` when the length is at the bound, while `new` caps the allocation with `bound.min(64)`. `acp::Inbox` is a different type from `SessionInbox`: its `submit` takes a `Mail` by value, needs `&mut self`, returns `Submit::Full` with no bound field, and doesn't lock. `acp.rs` doesn't call `SessionInbox::submit`. The cap it passes is the adapter's own constant from section 44.1, `demo` never mentions `SessionInbox`, and a later wiring could take `mail.text` and call `SessionInbox::submit`, which this file doesn't.

The demo builds that result and then drops the streams. After `translate` and `submit`, the ignored `Submit` on an empty inbox was `Accepted`, and the line `write_result` pushed is:

```text
{"jsonrpc":"2.0","result":{"stopReason":"end_turn"}}
```

`format!("acp: {}", reason.as_str())` is `acp: end_turn`, and the JSON isn't in that returned string. A second `write_result` of the cancelled reason pushes another line. The test checks it after the end-turn line is already in the vec:.

```text
{"jsonrpc":"2.0","result":{"stopReason":"cancelled"}}
```

The `format!` has no trailing newline. So a parser that splits stdout on `\n` would see this object only after something else wrote the newline, and this function doesn't write one. The two lines in the test are two `String`s rather than one buffer split on newlines. `stdout[0]` contains `end_turn` and doesn't contain `session started`, `stdout[1]` contains `cancelled`, and `stderr` stays a one-element vec.

`ch44-harness-cli` depends on `ch44-acp` by path, and `ch44-acp` doesn't depend on `ch43-cron`. `lib.rs` lists `pub mod acp` and, above it, `pub mod cron`, so a test in this package can still call `harness_core::cron::run_job`, and the ACP tests don't. There is no `use crate::cron` inside `acp.rs`, so the adapter doesn't scan a prompt and doesn't claim a job. `cargo run -p ch44-acp --bin ch44-acp` selects `chapter_demo.rs`, while the example target `demo` still runs `run_saved_evals` and wouldn't print `acp: end_turn`.

## Tests

The new tests live in `steps/ch44-acp/crates/harness-core/tests/acp.rs`, and the package also builds the copied harness tests, including `cron.rs` and `vision.rs`. The nextest line in the build section is that package run.

The stop-reason test `cancel_and_end_turn_are_different_reasons` expects the two `as_str` spellings. `logs_stay_off_stdout` is the stream split in section 44.5, including a second stdout line that contains `cancelled`. `unknown_permissions_deny` expects `allow_once` to be `Once`, `allow_session` to be `Session`, `allow_always` to be `Always`, both deny ids to be `Deny`, and `allow_forever` to be `Deny`.

`an_update_lands_on_the_inbox` is one translate of `hello`, one accepted submit, a front of `hello`, and then a fill to `INBOX_CAP` that expects the next submit to be `Full` with `len` still 64, and `demo_ends_the_turn` expects `demo()` to equal `acp: end_turn`.

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

The binary translates one update and ends the turn, and it reads no key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `translate_update` | 132.4 ns | one update onto an empty inbox | `translate` then `submit` |

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

The `Update` is built before the bench, with text `hello` and `cancelled` false, and `iter_batched` runs the setup closure outside the timer. `BatchSize::SmallInput` asks Criterion to clone that setup across a small batch, the timed closure takes the inbox by value, and `black_box(&update)` keeps the compiler from lifting the clone of `hello` out of the sample, while `black_box(submitted)` keeps it from deleting the `submit`. Each iteration gets another empty inbox from `Inbox::new(INBOX_CAP)`, so the measured work is one clone of `hello` and one `push_back`, and `Submit::Full` isn't a value this bench returns while the cap is 64.

The means are `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573), section `44`, for bench ids `44/translate_update`, `42/translate_update`, and `40/translate_update`. The command is `cargo bench -p ch44-acp --bench acp --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`, and how to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The bench is the clone and the push, while encoding the JSON line is `write_result`, which belongs to the stream test, and a sample that included it would be formatting a string as well as cloning `hello`. A sample that wrote a real stdout handle would be a different program. This one uses `iter_batched` so the empty inbox isn't part of the measured work, the `Update` is built outside the closure, and the text `hello` is cloned from the outer update on every sample.

Stdout and stderr are vecs, so `log_line` can't reach `stdout` unless someone later concatenates them, and the test checks that `session started` is absent from `stdout[0]`. A `format!` that interpolated the log into the JSON would fail that test. The result line also has no trailing newline, so a parser that splits one buffer on `\n` wouldn't see the object until something else wrote the newline.

`submit` on a full inbox drops the `Mail`, the 65th text is gone, and nothing pops the front to make room. `front` still returns the first accepted mail, and `len` stays 64.

`translate` ignores `cancelled`, so a caller who only submits the mail and never calls `stop_reason` will enqueue a cancelled turn as ordinary text. The demo does both, the return value follows the flag, and the mail doesn't.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). It has no ACP row. I did not find an ACP guardrail id to attach this adapter to. The behaviors below are the ones this file actually implements.

**Permission bridge.** `allow_once`, `allow_session`, and `allow_always` keep their three outcomes. `deny` and `deny_always` deny. An option id the function does not name denies. The `acp_adapter` package that might have held this table is not in the Hermes tree I checked. `agent/copilot_acp_client.py` answers `session/request_permission` with `cancelled`.

**Streams.** The result is one JSON line on `stdout`. The log is a different vec. `logs_stay_off_stdout` checks that the log is absent from the protocol line.

**Stop.** `stop_reason(true)` is `cancelled`. `stop_reason(false)` is `end_turn`. `as_str` is what `write_result` and `demo` insert.

**Inbox.** `INBOX_CAP` is 64, the same number as `INBOX_BOUND`. One `translate` plus one `submit` is one update. The next submit past the cap is `Full`.

**Left for later.** The editor process, the missing `acp_adapter` directory, and redaction of a secret inside an update. E1 and E2 are chapter 20. `Mail` is not passed through `redact_for_egress`.

## What you implement

The milestone is `my-harness/tests/ch44.rs`.

```rust
use my_harness::chapter_44_update_submits_once;

#[test]
fn one_acp_update_becomes_one_inbox_submit() {
    assert_eq!(chapter_44_update_submits_once(1), 1);
}
```

The stub panics on purpose.

```rust
pub fn chapter_44_update_submits_once(updates: usize) -> usize {
    let _ = updates;
    todo!("chapter 44")
}
```

The line `todo!("chapter 44")` is that panic. The test passes `1` and wants `1` back, because one update becomes one submit, and in the step crate that is `translate` of one `Update`, then one `Inbox::submit` that returns `Accepted` while the queue is under `INBOX_CAP`. The stub stays `todo!`.

## Chapter summary

- **Client:** `translate` copies one update into one `Mail`, and `Inbox::submit` takes that mail until 64.
- **Stop:** the flag `translate` doesn't store becomes `end_turn` or `cancelled`.
- **Streams:** the JSON result stays on stdout, and the log stays on stderr.
- **Permission:** the three allow ids keep their outcomes, and `deny`, `deny_always`, and any unknown id deny.
- **Model:** `FakeModel` answers `ok`, and the demo line is `acp: end_turn`.
- **Time:** `translate_update` has mean `132.4 ns`, which is `mean.point_estimate` from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573).

## What's next

Chapter 45 posts a fact on a shared board, wakes the agent waiting on it, and releases the lock before the model runs.
