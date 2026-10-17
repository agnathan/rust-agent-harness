# Chapter 12: Hooks and Plugins

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 11 can refuse a command. It cannot refuse an arbitrary tool because a plugin said so, and it cannot rewrite the arguments on the way in. This chapter adds that slot. A hook runs before the tool, and another runs after. A third sees the messages that are about to be sent, by borrow. A memory backend records the turn on a queue. The queue is the trait chapter 19 will fill with files.

The step crate is `steps/ch10-hooks`. The book number is 12. The package is `ch10-hooks`. It depends on `ch09-approvals`.

| Section | Step | What it adds |
|---|---|---|
| 12 | `steps/ch10-hooks` | `ToolHook`, `ApiHook`, `MemoryBackend` |

> **How Hermes does it**
>
> I read `agent/tool_executor.py` (`_pre_tool_block`, `_dispatch_authorized_once`) and `hermes_cli/plugins.py` (`_get_pre_tool_call_directive_details`, `_dispatch_pre_tool_call_hooks`, `_resolve_block_from_details`) on Hermes `main` at `cf71b1aa`. `_pre_tool_block` returns `(block_message, final_args)`. The directive scan is block, then approve, then modify. A block wins over an approve even when the approve was registered first. A hook exception does not block. An approve calls `tools/approval.py` `request_tool_approval`. If that gate errors, the call is blocked.
>
> `pre_api_request` is a lifecycle hook. I saw it invoked next to `pre_llm_call` in `tests/hermes_cli/test_shared_metrics_model_attribution.py`. `agent/memory_provider.py` `MemoryProvider.sync_turn` is the non-blocking write of one finished turn. `queue_prefetch` is the background half.
>
> **Adopted.** The `(block, args)` pair. Block over approve. Modify patches merge into the arguments the later hooks see. A hook error is skipped. Escalation that is not approved becomes a block, and a gate failure is `BLOCKED: plugin approval gate failed`. The memory call records the turn and returns.
>
> **Adapted.** `post_tool_call` returns `Option<String>`, a replacement result. I did not port Hermes's post-hook payload. `pre_api_request` takes `&[Message]` so the hook cannot be the thing that clones the conversation. `MemoryBackend::enqueue_turn` is a `Mutex<VecDeque>` capped at 64, not Honcho or Hindsight. A hook may name a toolset to enable. `set_thread_tool_whitelist` in `hermes_cli/plugins.py` narrows tools; this enable is the other direction, and a failed availability check still hides the tool.
>
> **Deferred.** The reviewer that runs after a turn, and `on_pre_compress`, are chapter 24. The file backend is chapter 19. Untrusted-output wrapping is chapter 32 (F4).

[personal note: a hook that fired on a tool you thought was still going to run]

## What this chapter decides

`dispatch_pre` walks the hooks in order.

| Action | Effect |
|---|---|
| `Modify` | Shallow-merge the patch into the arguments |
| `Block` | Return that message now. Later hooks do not run |
| `Approve` | Remember the first one. A later block still wins |
| error | Skip the hook |

`Approve` does not run the tool. The caller passes the reason through `escalation_block`, which calls chapter 11's `check`. Cron mode denies a dangerous command, so the hook's escalate becomes the denial text. `Ask` is also a block in this function: the synchronous path has no human. Chapter 11's `run_call` is the wait. `gate_failed` is the string used when the gate cannot be asked.

`dispatch_post` lets each hook replace the result string. The last `Some` wins.

`run_api_hook` passes the slice through. The test stores `messages.as_ptr()` and checks it against the vec the caller still owns.

`MemoryQueue::enqueue_turn` pushes `user assistant` and drops the oldest job past 64. It does not open a file.

`Registry::available` is true only when the tool's check passed and its toolset was enabled by a hook. Enabling `notes` does not make `secret` available when the check failed.

`run_script` is the fake model. A block ends the list.

## Rust: a trait object is the plugin

`ToolHook` is a trait. `Arc<dyn ToolHook>` is one hook you can put in a vec next to a different struct. The trait methods return owned data (`Result`, `Option<String>`) or a borrow tied to `&self` (`enable_toolset`). That is what makes the trait object-safe. A method that returned `impl Something` would not be.

`ApiHook::pre_api_request` takes `&[Message]`. A slice is a pointer and a length. The 1,000 messages in the bench are built once, outside the timer. The hook reads the length.

## Tests

Eight tests. A modify sticks when a later hook blocks, and that block beats an earlier approve. The first approve is the one kept, and a failing hook is skipped. A post hook replaces the result. Cron escalation denies, and the dead-gate string names the tool. Enabling a toolset does not revive a failed check. The API hook sees the same pointer. The queue length is 1 after one turn. The empty hook returns nothing, and a block stops the script before `echo`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch10-hooks
```

```text
Starting 8 tests across 3 binaries
     Summary [   0.006s] 8 tests run: 8 passed, 0 skipped
```

```bash
cargo run -p ch10-hooks
```

```text
result: BLOCKED: hook refused rm
stopped: 1
```

The binary installs one hook that blocks `rm`, then a script of `rm` and `echo`. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `empty_pre_post` | 7.005 ns | under 0.1 ms | one empty before hook and one empty after hook |
| `api_request_hook` | 1.378 ns | under 0.1 ms | `pre_api_request` on 1,000 borrowed messages |
| `memory_enqueue` | 33.36 ns | under 1 ms | one turn pushed onto the queue |

`empty_pre_post` calls `NopHook::pre_tool_call` and `post_tool_call` inside `b.iter`. The JSON arguments `{}` are built before the bench. No tool runs.

`api_request_hook` builds a vec of 1,000 `Message` values before the bench. Each content is `hello`. The timed closure calls `run_api_hook`, which calls `pre_api_request` on that slice. The hook reads `len`. It does not clone the vec. Building the 1,000 messages is excluded.

`memory_enqueue` locks the queue, drops the front if the length is already 64, and pushes one formatted string. The queue is built before the bench. The string format is inside the sample. Nothing is written to disk.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38983232549](https://github.com/agnathan/rust-agent-harness/actions/runs/38983232549), section `12`.

### Milestone demo

The `cargo run` above. The result is the block message. `stopped` is 1.

## Pitfalls

`dispatch_pre` returns on the first block. A modify that was registered on a hook after that block never runs. Put the rewrite on an earlier hook if the block should still see it. The test does it that way.

An approve stored in `Dispatch.approve` is not a block. `run_script` ignores it and runs the tool. The escalation test calls `escalation_block` on purpose. Forgetting that call is how a plugin's "ask the human" becomes a silent allow.

The queue cap drops the oldest job. A caller that treats `len()` as "every turn since the process started" will be wrong after 64 enqueues. Chapter 19 is the durable copy.

`available` requires both the check and the enabled set. A hook that enables a set the tool is not in does nothing for that tool.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L5.

**Plugin veto.** `PreAction::Block` is the `pre_tool_call` veto. The message is the tool result. The script does not continue.

**A8.** `PreAction::Approve` is the escalation. `escalation_block` runs chapter 11's pipeline. A denial keeps the "has NOT consented" text. `gate_failed` is the fail-closed string when the gate cannot be called.

**Hook failure.** `Err` from `pre_tool_call` is skipped. The next hook still runs. A broken plugin does not become a deny, and it does not become an allow of a later block.

**Registry check.** `Registry::available` stays false when the check failed. The hook cannot enable a tool chapter 3 would have hidden.

**Left for later.** The post-turn reviewer is chapter 24. Memory files and the threat scan are chapter 19 (F5, F6). Secret redaction (E1, E2) is still absent. The hook message is not scrubbed.

## Chapter summary

- **Before.** Modify, then block wins, then the first approve is kept. A failed hook is skipped.
- **After.** A hook may replace the result string.
- **Borrow.** The API hook sees the message slice. It does not get its own copy.
- **Queue.** One turn enqueues. The file write is chapter 19.
- **Time.** The empty pair measured **7.005 ns**. The API hook measured **1.378 ns**. The enqueue measured **33.36 ns**.

[personal note: which hook you expected to be a block, and what it rewrote instead]

## What's next

Chapter 13 runs code outside this process. The approval gate from chapter 11 still sees the script before the child starts. The sandbox is off unless you ask for it.
