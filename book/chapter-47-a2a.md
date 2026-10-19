# Chapter 47: A2A for External Agents

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 46 shares a skill inside this process. An agent in another process is not a waiter on that board and not a subscriber on that bus. Hermes uses A2A for that boundary. MCP stays the tool boundary. This chapter is the external call: a trait, a pool of 8, and a cancel flag.

The step crate is `steps/ch47-a2a`. The book number is 45. The package is `ch47-a2a`.

| Section | Step | What it adds |
|---|---|---|
| 47 | `steps/ch47-a2a` | `ExternalAgent` for a remote peer, the bound of 8, cancel |

> **How Hermes does it**
>
> I read `website/docs/user-guide/messaging/a2a.md`, `plugins/platforms/a2a/adapter.py` (the method table), `plugins/platforms/a2a/tools.py` (`a2a_call`), `plugins/platforms/a2a/security.py` (`wrap_inbound`, `redact_outbound`), `gateway/session_a2a.py` (`build_forward_policy`), and `agent/redact.py` (`redact_for_egress`) on Hermes `main` at `e31c674f`.
>
> The A2A doc says to use delegation or kanban for agents on the same machine, and A2A to cross a process, a machine, or a framework. The adapter method table maps `_rpc_tasks_cancel` to `CancelTask` and `tasks/cancel`. `a2a_call` sends a task to a configured peer or a URL. `wrap_inbound` frames every inbound message, including text that starts with `/`, as untrusted. `redact_outbound` calls `redact_for_egress` and then masks e-mail. `build_forward_policy` only lets the forwarding handler mint an A2A policy. Session-create JSON cannot.
>
> **Adopted.** A `local://` peer is refused before a permit is taken. The ninth in-flight call is `Bound`. Cancel is checked before the fake HTTP record. Outbound text is scrubbed. Inbound text is wrapped, slash and all.
>
> **Adapted.** There is no socket and no JSON-RPC server. `HttpPool` records the URL in a `Vec`. `FakeModel` is the reply. Cancel is an `AtomicBool`. The bench is the flag reaching a spinning thread, not a network timeout.
>
> **Deferred.** The agent card over HTTP, SSE streaming, and push callbacks. The pool bound is the same 8 the harness uses for tools.

[personal note: a URL that was not allowed to mean "in this process"]

## What this chapter decides

`ExternalAgent::send` is the trait at the boundary. `RemotePeer` implements it. A URL that starts with `local://` returns `InProcess` and records nothing.

`Pool` holds at most 8. `call` acquires a permit, scrubs the message, calls `send`, and releases the permit. The ninth `call` returns `Bound` without recording a URL.

`cancel` stores `true`. `send` returns `Cancelled` when the flag is already set, before `HttpPool::record`. `wait_cancel` spins until the flag is set. That is the bench.

`wrap_inbound` prefixes the peer and the text with `[A2A inbound — untrusted external input]`. `redact_for_egress` masks `Bearer` and `sk-`.

`card` names the agent `harness` and the skill `research`. It is a struct, not an HTTP response.

`demo` calls `http://research.example:9900` and prints `a2a: ok`. No key.

## Rust: the trait is the process edge

`ExternalAgent` is the function an outside agent has to implement. A supervisor specialist is not this trait. The compiler does not stop someone from passing a local URL, so `call` checks the prefix and returns `InProcess` before `acquire`. The permit is a `compare_exchange` on an `AtomicUsize`. Release always runs after `send`, including when `send` returns `Cancelled`.

## Tests

Six tests. `local://supervisor` is `InProcess`, the pool stays at 0, and the URL list stays empty. Eight `hold` calls fill the pool and the next `call` is `Bound`. After one release, a call records the http URL and returns `ok`. A flag that starts true never records the URL, and the permit is released. `cancel` unblocks `wait_cancel`. The scrubbed bearer does not contain `sk-live`, and `/shutdown` stays inside the inbound frame. The card name is `harness`. The demo is `a2a: ok`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch47-a2a
```

```text
     Summary [   0.006s] 6 tests run: 6 passed, 0 skipped
```

```bash
cargo run -p ch47-a2a
```

```text
a2a: ok
```

The binary calls one fake peer. No key. No socket.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `cancel_inflight` | 48.33 µs | cancel reaches an in-flight call under 50 ms | set the flag and join the waiter |

`cancel_inflight` uses `iter_batched`. The setup closure spawns a thread that runs `wait_cancel`, waits until that thread has stored `started`, and is not timed. The timed routine calls `cancel` and `join`. The flag store and the waiter's exit are inside the sample. `HttpPool`, `FakeModel`, `redact_for_egress`, and the permit pool are not called. There is no sleep and no socket.

The table is `mean.point_estimate` from Benchmarks run [38991844877](https://github.com/agnathan/rust-agent-harness/actions/runs/38991844877) on ubuntu-latest (`bench-run.json`, 48326.683465522576 ns). The flags are `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `a2a: ok`.

## Pitfalls

The bench is a flag and a join. A number that includes HTTP is a client this crate does not open.

`local://` returns before `acquire`. Counting it against the pool would let an in-process name consume a remote permit.

The ninth call is `Bound` only while eight permits are held. A test that releases inside `call` never sees 8, because `call` releases before it returns. `hold` is what keeps them.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Boundary.** A2A is for a URL outside this process. `local://` is `InProcess`. The team patterns still decide a call between two agents here.

**Egress (E2).** `call` scrubs before `send`. `redact_for_egress` fails closed in Hermes. This crate's two patterns do not. A body with neither marker is unchanged.

**Inbound frame.** `wrap_inbound` keeps a leading `/shutdown` inside the frame so it is not a local slash command. That is the rule in `security.py`.

**Cancel.** The flag is checked before the URL is recorded. A set flag does not append to `HttpPool`.

**Pool.** The bound is 8, the same number as tool dispatch. The ninth call is `Bound`.

**Left for later.** HMAC push callbacks, the agent-card HTTP route, and e-mail masking. The forward-policy mint in `session_a2a.py` is not reimplemented.

## Chapter summary

- **Trait.** `ExternalAgent` is the outside process.
- **Pool.** Eight permits. The ninth is `Bound`.
- **Cancel.** A flag. The bench joins the waiter.
- **Model.** `FakeModel` answers `ok`. The demo needs no key and no socket.
- **Time.** One cancel measured **48.33 µs**.

[personal note: a permit that came back after cancel]

## What's next

Chapter 48 puts a span on the events from chapter 1, and a research team of 100 mock agents that keeps a note across runs.
