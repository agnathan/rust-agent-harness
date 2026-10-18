# Chapter 42: Editor Integration

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 41 claims a job that nobody is watching. An editor is watching. This chapter is ACP, the Agent Client Protocol, as another client of the session inbox. An update becomes one message. A finished turn and a cancelled turn are different stop reasons. The JSON-RPC line is stdout. A log line is stderr. An unknown permission denies.

The step crate is `steps/ch42-acp`. The book number is 40. The package is `ch42-acp`.

| Section | Step | What it adds |
|---|---|---|
| 42 | `steps/ch42-acp` | one inbox message per update, `end_turn` or `cancelled`, stdout kept for the protocol |

> **How Hermes does it**
>
> I read `acp_adapter/entry.py` (the module docstring), `acp_adapter/session.py` (`_acp_stderr_print`), `acp_adapter/server.py` (the `PromptResponse` at the end of the turn), and `acp_adapter/permissions.py` (`_OPTION_ID_TO_HERMES`, `_map_outcome_to_hermes`) on Hermes `main` at `e31c674f`.
>
> The entry point says stdout is reserved for ACP JSON-RPC and logging goes to stderr. `_acp_stderr_print` sends incidental agent output to stderr. The turn returns `stop_reason` `cancelled` when the turn was cancelled, and `end_turn` otherwise. Permission option ids map `allow_once` to once, `allow_session` to session, `allow_always` to always, and both `deny` and `deny_always` to deny. An outcome that is missing, or an option id the request did not offer, denies.
>
> **Adopted.** Stdout for the result line, stderr for the log. `end_turn` and `cancelled`. The option-id map, and deny for an unknown id. The inbox cap is 64, from chapter 2.
>
> **Adapted.** The message is a `Mail` on a `VecDeque`. There is no ACP SDK and no stdio process. `FakeModel` sees the update text. The bench translates one update onto an empty inbox and does not encode JSON.
>
> **Deferred.** A real editor session, usage tokens on the response, and the permission prompt UI. The catalog has no ACP row. E1 and E2 are still absent.

[personal note: an editor update that was just another message on the inbox]

## What this chapter decides

`translate` copies the update text into a `Mail`. `Inbox::submit` accepts it until 64. The 65th is `Full`.

`stop_reason(false)` is `EndTurn` (`end_turn`). `stop_reason(true)` is `Cancelled` (`cancelled`).

`write_result` pushes one JSON object onto `stdout`. `log_line` pushes onto `stderr`. The log text is not part of the JSON line.

`map_permission` returns `Once`, `Session`, `Always`, or `Deny`. `deny_always` is `Deny`. Any other id is `Deny`.

`demo` submits `hello`, calls `FakeModel`, logs `session started` on stderr, writes `end_turn` on stdout, and returns `acp: end_turn`. No key.

## Rust: the protocol line is a format

`write_result` builds one string. It does not write a file descriptor. The two vecs are the stand-in for stdout and stderr, so a test can see that they did not mix. The bench never calls `write_result`. It clones `hello` into a `Mail` and pushes that onto a fresh inbox.

## Tests

Five tests. The two stop reasons stringify to `end_turn` and `cancelled`. A log line stays on stderr and out of the JSON. `allow_once`, `allow_session`, and `allow_always` map to once, session, and always. `deny`, `deny_always`, and `allow_forever` deny. One update becomes a mail, the inbox holds 64, and the next submit is `Full`. The demo is `acp: end_turn`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch42-acp
```

```text
     Summary [   0.006s] 5 tests run: 5 passed, 0 skipped
```

```bash
cargo run -p ch42-acp
```

```text
acp: end_turn
```

The binary translates one update and ends the turn. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `translate_update` | 119.9 ns | translate an ACP update onto the session queue under 1 ms | clone `hello` and push it |

`translate_update` builds the `Update` (`hello`, not cancelled) before the bench. It uses `iter_batched`. The setup closure calls `Inbox::new(64)` and is not timed. The timed routine calls `translate` with that update passed through `black_box`, then `submit` of the mail, then `black_box` of `Submit`. The string clone and the push are inside the sample. The inbox is empty, so the result is `Accepted`. `write_result`, `log_line`, `map_permission`, and `FakeModel` are not called. The `Full` path is not taken.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38989567446](https://github.com/agnathan/rust-agent-harness/actions/runs/38989567446), section `40`. The flags were `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `acp: end_turn`.

## Pitfalls

The bench is the clone and the push. Encoding the JSON line is the test. A number that includes a real stdio write is a different program.

Stdout and stderr are vecs. A later chapter that writes the process handles has to keep the log off the protocol stream. Mixing them breaks the editor, which parses every stdout line as JSON.

An unknown permission denies. Mapping it to allow would be the bug `_map_outcome_to_hermes` exists to prevent.

The 65th message is `Full`. ACP does not grow the inbox to make room.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). The catalog has no ACP row.

**Permission bridge.** `allow_once`, `allow_session`, and `allow_always` keep their three outcomes. `deny` and `deny_always` deny. An option id the adapter does not know denies. That is the fail-closed map in `acp_adapter/permissions.py`.

**Streams.** The result is one JSON line. The log is a different vec. A test checks the log is absent from stdout.

**Stop.** A cancelled turn does not report `end_turn`.

**Left for later.** The editor process, and redaction of a secret inside an update. E1 and E2 are still absent.

## Chapter summary

- **Client.** An ACP update is a mail on the inbox of 64.
- **Stop.** `end_turn` or `cancelled`.
- **Streams.** JSON on stdout. Logs on stderr.
- **Permission.** Unknown denies.
- **Model.** `FakeModel` answers `ok`. The demo needs no key.
- **Time.** One translate measured **119.9 ns**.

[personal note: a protocol line that stayed a single string]

## What's next

Chapter 43 is the blackboard. Agents claim shared state. The lock is not held across a model call.
