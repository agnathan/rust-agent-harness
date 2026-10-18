# Chapter 13: Sandboxed Execution

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 12 can refuse a tool. The tool still runs in this process. A script that can spawn a process, or a command that can delete a file, needs a boundary that is not another `if` in the harness. This chapter adds a child process, the argv Docker would be given, and a checkpoint you can rewind. The default backend is local. Local is not a sandbox. That is the same default Hermes ships.

The step crate is `steps/ch13-sandbox`. It is chapter 12's harness copied forward, plus `sandbox`. The package name stays `ch13-sandbox`. The worker is a second binary, `sandbox-worker`, because the sandbox tests and the bench compile `env!("CARGO_BIN_EXE_sandbox-worker")`. Later copies keep that binary.

| Section | Step | What it adds |
|---|---|---|
| 13 | `steps/ch13-sandbox` | a child worker, Docker argv, env scrub, checkpoints |

> **How Hermes does it**
>
> I read `hermes_cli/config_defaults.py` (`terminal.backend` is `"local"`), `tools/environments/docker.py` (`_BASE_SECURITY_ARGS`, `_NO_NEW_PRIVILEGES_ARGS`, `_DEFAULT_PIDS_LIMIT`), `tools/code_execution_env.py` (`_SAFE_ENV_PREFIXES`, `_SECRET_SUBSTRINGS`), `tools/mcp_tool_config.py` (`_SAFE_ENV_KEYS`, `_build_safe_env`), `tools/env_passthrough.py` (`_is_hermes_provider_credential`, `register_env_passthrough`), `agent/deadline.py` (`kill_process_tree`), and `tools/checkpoint_manager.py` (`CheckpointManager`, `enabled=False`, `max_snapshots=20`, `max_total_size_mb=500`, `max_file_size_mb=10`, `_MAX_FILES = 50_000`) on Hermes `main` at `cf71b1aa`.
>
> Docker drops every capability, adds `DAC_OVERRIDE`, `CHOWN`, and `FOWNER` back, sets `no-new-privileges`, and limits pids. The pids default I read is `"2048"`. The child environment keeps an allowlist and drops names that contain `KEY`, `TOKEN`, `SECRET`, and the other substrings in `_SECRET_SUBSTRINGS`. A declared passthrough may add a third-party key. It may not add `OPENAI_API_KEY`. The docstring on `_is_hermes_provider_credential` names that bypass, plus `AUXILIARY_*_API_KEY` / `_BASE_URL` and `GATEWAY_RELAY_*`. Checkpoints are off until enabled. `kill_process_tree` kills the pid and its descendants.
>
> **Adopted.** Default backend local. The Docker flags above, including the three caps added back and pids `2048`. The secret substrings and the passthrough exception for a non-provider key. Checkpoints off, 20 kept, files over 10 MB skipped, 50,000 files is the cap. A timeout kills the child.
>
> **Adapted.** Nothing calls Docker. `docker_run_args` returns the argv a test can read. The worker is one process this crate compiles, not a container. `run_until` kills that direct child. Hermes's `kill_process_tree` also walks descendants (`taskkill /F /T` on Windows, a process-group kill on POSIX). This chapter does not. The checkpoint is a manifest of path and content, not a shadow git repo. Rewind copies the manifest back.
>
> **Deferred.** Credential-file mounts (D5) and per-child git worktrees (D6, chapter 27). This chapter does not parse Python. A2 is the script gate above, not a source scan.

[personal note: a job you thought was sandboxed, and which backend it actually used]

## What this chapter decides

`Backend::from_name` maps `"docker"` to `Docker` and everything else, including `""`, to `Local`. `Local.is_sandboxed()` is false.

`docker_run_args("hermes-sandbox")` is the hardened `docker run` line. The tests look for `--cap-drop ALL`, `--security-opt no-new-privileges`, and `--pids-limit 2048`. They do not spawn Docker.

`scrub_env` walks the pairs you pass in. A provider credential is dropped even if it is on the passthrough list. Anything else on that list is kept, which is how `TENOR_API_KEY` survives and `RANDOM_SECRET` does not. `PATH` and `HOME` stay because they are on the allowlist.

`Worker` is the warm child. `accept` writes one line and reads one `ok` line. `run_script` is the fake model: a list of jobs, no key. The demo job is `ping`.

`run_until` writes one job and polls. If the child is still alive when the timeout passes, it kills the child and returns `Timeout`. The test uses `sleep-ms 5000` and an 80 millisecond timeout. The child does not get to finish the sleep.

`Checkpoints::new(false)` is the default shape. `take` returns `None` until you pass `true`. A store keeps `MAX_SNAPSHOTS` (20) and drops the oldest. `Manifest::checkpoint` copies up to `MAX_FILES` entries and skips a file whose content is longer than 10 MB. `rewind_into` replaces the live vec with that copy. `snapshot_dir` / `restore_dir` do the same thing for a real directory. The test uses one file in a temp dir.

## Rust: a child is a `Command`

`std::process::Command` starts a program and gives you its `Child`. Piped stdin and stdout are how the harness talks to it without a shell. `writeln` and `flush` matter: a buffered write that you never flush looks like a hung child. `Drop` kills the child so a test that returns early does not leave `sandbox-worker` running.

The demo finds the worker next to its own executable. `cargo run` puts both binaries in the same directory. The tests and the bench use `CARGO_BIN_EXE_sandbox-worker`, which Cargo sets when it builds those targets.

## Walking `check_execute_code` and the worker

Chapter 11's `deny_tool` returned `None` for `execute_code`. A script could spawn a process without ever passing the shell floor. This chapter replaces that branch. The loop still calls `deny_tool` before it builds a `ToolCall`. For `execute_code` it now calls `sandbox::check_execute_code` with the `code` argument, `env_type` `"local"`, `Presence::Interactive`, and the three flags false.

```rust
pub fn check_execute_code(
    code: &str,
    env_type: &str,
    presence: crate::approval::Presence,
    mode_off: bool,
    yolo: bool,
    whole_script: bool,
) -> CodeDecision
```

The body ignores `code`. Hermes's gate is about where the script runs, not about scanning the source. The order is the order in `tools/approval.py` `check_execute_code_guard`:

1. `env_type != "local"` returns `Approved`. A container or a remote sandbox is already the boundary. Cron inside that env is still approved. The test uses `"container"`.
2. `yolo` or `mode_off` returns `Approved`.
3. `Presence::Cron`, `SingleQuery`, or `Unattended` returns `Denied`. The message starts with `BLOCKED: execute_code runs arbitrary local Python (including subprocess calls that bypass shell-string approval checks).` That sentence is Hermes's. The test matches the prefix.
4. `whole_script` returns `Ask` with `EXECUTE_CODE_DESCRIPTION`, the one-shot description of a script that can spawn processes. Gateway and ask mode take this path. The loop's default call passes `whole_script: false`, so an interactive CLI does not ask here.
5. Otherwise `Approved`. The script's own `terminal` calls still go through chapter 11.

`tests/execute_code.rs` calls the function directly for the cron denial, the container approval, mode-off approval, and the whole-script ask. The default loop path does not deny an interactive local script, on purpose.

The worker is a long-lived child. One line in, one `ok` line out. `sleep-ms N` waits, then writes `ok slept`. `quit` ends the loop. The sandbox bench and `tests/sandbox.rs` spawn it with `env!("CARGO_BIN_EXE_sandbox-worker")`. That environment variable exists only when the package declares a bin of that name. The bin is `src/bin/worker.rs`, name `sandbox-worker`. A warm accept is the bench `warm_accept`. Checkpoint and rewind of 1,000 paths are the other two ids. The Docker argv test still requires `--cap-drop ALL`, `--security-opt no-new-privileges`, and a pids limit, and the default backend is still local.

> **Rust sidebar: `env!` runs at compile time**
>
> `env!("CARGO_BIN_EXE_sandbox-worker")` is not a runtime lookup. The compiler pastes the path into the binary. If the `[[bin]]` is missing, the test crate does not compile. That is why every later copy of this harness keeps the worker binary even when the chapter is about providers or memory. `option_env!` would compile and return `None`; the tests want the failure to be loud.

## Tests

Seven tests. Local is the default and the Docker argv contains the three flags. Scrub keeps `PATH`, `HOME`, and `TENOR_API_KEY`, and drops `OPENAI_API_KEY` and `RANDOM_SECRET`. A warm worker answers `ok ping`. A five-second sleep is killed in under two seconds. Checkpoints do nothing while disabled, rewind restores `one`, and the store stays at 20. A 1,000-path manifest round-trips. A temp-dir file goes from `after` back to `before`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch13-sandbox
```

```text
Starting 84 tests across 22 binaries
     Summary [   0.161s] 84 tests run: 84 passed, 0 skipped
```

```bash
cargo run -p ch13-sandbox
```

```text
reply: ok ping
```

The binary spawns `sandbox-worker`, sends `ping`, and prints the line it read. No key. No Docker.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `warm_accept` | 30.49 µs | under 50 ms | one `ping` to a worker that is already running |
| `checkpoint_1000` | 50.37 µs | under 10 ms | copy a 1,000-path manifest |
| `rewind_1000` | 83.79 µs | under 10 ms | copy that manifest back onto a live vec |

`warm_accept` spawns the worker before `b.iter`. Each sample writes `ping`, flushes, and reads `ok ping`. Process startup is excluded. There is no sleep in the job.

`checkpoint_1000` builds the 1,000 `(path, body)` pairs before the bench. Each body is `body N`. The timed closure clones the pairs that fit the size cap. The initial vec is excluded.

`rewind_1000` builds the snapshot before the bench. Each sample starts from an empty live vec and `clone_from`s the snapshot into it. That copy is the sample.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `13`.

### Milestone demo

The `cargo run` above. The reply is `ok ping`.

## Pitfalls

Local is the default. Printing Docker argv does not put the process in a container. `is_sandboxed` is the bit a later chapter should check before it tells the model the command is isolated.

`accept` deadlocks if the child does not flush. The worker flushes every line. A child you write later has to do the same.

`run_until` kills the process it spawned. A grandchild the worker started would survive. That is the gap next to Hermes's `kill_process_tree`.

Checkpoints are off. `take` on a disabled store returns `None` and does not remember the files. A caller that ignores `None` will rewind nothing and think it restored.

The 10 MB skip drops the file from the manifest. Rewind will not put it back, because it was never saved.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L7.

**D1.** `Backend::Local` is the default. It is not a security boundary. The Docker backend is opt-in, and this chapter does not start it.

**D2.** When the backend is Docker, the argv includes `--cap-drop ALL`, `--security-opt no-new-privileges`, and `--pids-limit 2048`. The caps Hermes adds back (`DAC_OVERRIDE`, `CHOWN`, `FOWNER`) are on that argv too.

**D3 and D4.** `scrub_env` is the allowlist plus passthrough. `OPENAI_API_KEY`, `AUXILIARY_*_API_KEY`, `AUXILIARY_*_BASE_URL`, and `GATEWAY_RELAY_*` are dropped even if a caller lists them. A third-party key on the passthrough list is kept. Names containing `KEY`, `TOKEN`, `SECRET`, `PASSWORD`, `CREDENTIAL`, `PASSWD`, `AUTH`, or `DSN` are dropped unless they were passed through.

**I6.** `run_until` is the timeout. It kills the child it started. It does not walk a process tree.

**N1.** `Checkpoints` starts disabled. The caps are 20 snapshots, 10 MB per file, and 50,000 files. The 500 MB total cap is Hermes's; this manifest is in memory and does not track a store size yet.

**A2 is this chapter.** `check_execute_code` is the script gate. Cron, single-query, and unattended deny a local script. An isolated env approves. `whole_script` asks once. An interactive CLI approves here; `terminal` inside the script still hits chapter 11.

**Left for later.** File-path checks from chapter 3 stay in force. They are defense in depth, which is what `agent/file_safety.py` says. D5 and D6 are not this chapter. Secret redaction of the job line (E1, E2) is chapter 18.

## Chapter summary

- **Default.** Local. Docker is an argv.
- **Child.** A warm worker answers `ok ping`. A timeout kills a sleep.
- **Env.** `PATH` stays. `OPENAI_API_KEY` does not, even on the passthrough list.
- **Checkpoint.** Off until enabled. Rewind copies the manifest back.
- **Time.** A warm accept measured **30.49 µs**. Checkpoint of 1,000 paths measured **50.37 µs**. Rewind measured **83.79 µs**.

[personal note: the first time the worker stayed up after the test, and which drop killed it]

## What's next

Chapter 14 classifies a failure and decides whether to retry. A timeout from this chapter is one of those classes, and the retry has to stay inside the concurrency bound. Empty replies and a repetition loop, deferred since chapter 9, land there too.
