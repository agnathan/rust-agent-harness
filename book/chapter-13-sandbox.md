# Chapter 13: Sandboxed Execution

Local is the default, and printing a Docker command doesn't put the process in a container. The loop's only sandbox call is `check_execute_code`, inside `deny_tool`. A local interactive script is approved there, and its `terminal` calls still hit chapter 11. Cron on that same local env is denied. A container env is approved even when the presence is cron, because the env check returns before presence is considered. `Worker::spawn` doesn't call `scrub_env`, so the child inherits the parent's keys unless a caller builds the list first.

What this chapter delivers, each piece tested on its own, is that gate, the argv a Docker run would use, the scrub function, a warm worker, a timeout that kills the child it started, and a checkpoint you can rewind. The crate is `steps/ch13-sandbox`, a copy of chapter 12 plus a `sandbox` module. The package is `ch13-sandbox`. The worker binary stays named `sandbox-worker`, because the tests and the bench compile `CARGO_BIN_EXE_sandbox-worker`, and later copies keep that name.

Chapter 11 can deny a dangerous shell command, and chapter 12 can watch a call. Neither of them runs a model-written script outside this process. `execute_code` that evals in-process would be the harness trusting the model with its own address space, so the boundary is the child. `whole_script` asks once, not once per line, which is the script gate Hermes calls `check_execute_code_guard`, deferred from chapter 11 and closed here.

`Backend` describes how a job would be launched. The Docker argv drops capabilities, sets `no-new-privileges`, and sets a pid limit, and nothing in this chapter spawns Docker. `scrub_env` builds the child environment from an allowlist plus declared passthrough. `run_until` is the deadline, the timeout chapter 1 deferred, and it kills the process it spawned. A grandchild that worker started would survive, which is the gap beside Hermes's `kill_process_tree`. A warm worker accepts a job without starting a process per call.

Chapter 17 sends the terminal through this worker after the approval floor, instead of growing a second sandbox. A skill that contains a script is still a script, the scanner for that text is chapter 25, and chapter 36 restarts a failed child agent from a heartbeat without adopting this worker.

A child process is `std::process`, and the worker is a binary so the tests can kill it. A checkpoint is a list of paths and bytes, rather than a filesystem snapshot library. `check_execute_code` returns an enum, so a cron session can't fall through to allow by forgetting an arm.

`cargo run -p ch13-sandbox` runs the demo. The exercise is `my-harness/tests/ch13.rs`: cron can't run a local script, a container env approves, and a whole-script ask is one prompt.

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
> **Deferred.** Credential-file mounts (D5) and per-child git worktrees (D6, chapter 29). This chapter does not parse Python. A2 is the script gate above, not a source scan.

## What this chapter decides

`Backend::from_name` maps `"docker"` to `Docker` and everything else, including `""`, to `Local`. `Local.is_sandboxed()` is false. `Docker.is_sandboxed()` is true.

`docker_run_args("hermes-sandbox")` is the hardened `docker run` line. The tests look for `--cap-drop ALL`, `--security-opt no-new-privileges`, and `--pids-limit 2048`. They do not spawn Docker.

`scrub_env` walks the pairs you pass in. A provider credential is dropped even if it is on the passthrough list. Anything else on that list is kept, which is how `TENOR_API_KEY` survives and `RANDOM_SECRET` does not. `PATH` and `HOME` stay because they are on the allowlist.

`Worker` is the warm child. `accept` writes one line and reads one `ok` line. `run_script` is the fake model: a list of jobs, no key. The demo job is `ping`.

`run_until` writes one job and polls. If the child is still alive when the timeout passes, it kills the child and returns `Timeout`. The test uses `sleep-ms 5000` and an 80 millisecond timeout. The child does not get to finish the sleep.

`Checkpoints::new(false)` is the default shape. `take` returns `None` until you pass `true`. A store keeps `MAX_SNAPSHOTS` (20) and drops the oldest. `Manifest::checkpoint` copies up to `MAX_FILES` entries and skips a file whose content is longer than 10 MB. `rewind_into` replaces the live vec with that copy. `snapshot_dir` / `restore_dir` do the same thing for a real directory. The test uses one file in a temp dir.

`check_execute_code` ignores the script text. A non-local `env_type` is `Approved`. YOLO or mode off is `Approved`. Cron, single-query, and unattended on `local` are `Denied`. `whole_script` is `Ask`. Anything left is `Approved`. The loop calls it with `"local"`, `Presence::Interactive`, and the three flags false, so that call is `Approved`. The cron denial is a direct call in `tests/execute_code.rs`.

## 13.1 One call from the loop

The file is `steps/ch13-sandbox/crates/harness-core/src/sandbox.rs`. Its module comment is the whole job of the file.

```rust
//! Book chapter 13. Code runs in a child process. Docker argv is built and
//! not executed. The default backend is local, which is not a sandbox.
```

`lib.rs` declares `pub mod sandbox;`. The module is public, so an integration test can write `harness_core::sandbox::Worker`. The `pub use` list at the bottom of `lib.rs` does not re-export these names. You go through the module. `approval` is also `pub mod`, which is how `deny_tool` can call `crate::sandbox::check_execute_code` and how the sandbox signature can name `crate::approval::Presence`.

`Worker`, `docker_run_args`, `scrub_env`, `run_until`, and `Checkpoints` aren't in `agent.rs`. The loop's only sandbox call is inside `deny_tool`, and that call is `check_execute_code`. The child, the argv, the scrub, and the checkpoint are what the tests, the bench, and `chapter_demo` construct. When `deny_tool` returns `None`, the inherited turn still runs tools in this process.

Chapter 11's `deny_tool` returned `None` for `execute_code` before `check` ever saw the script. This chapter replaces that early return with a match. The function still returns `None` for every other name, and it still runs `check` for `terminal`.

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
        return match crate::sandbox::check_execute_code(
            command,
            "local",
            Presence::Interactive,
            false,
            false,
            false,
        ) {
            crate::sandbox::CodeDecision::Approved => None,
            crate::sandbox::CodeDecision::Denied { message } => Some(message),
            crate::sandbox::CodeDecision::Ask { description } => Some(format!(
                "BLOCKED: execute_code needs a whole-script approval ({description}).{STOP_COMMAND}"
            )),
        };
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

> **Rust sidebar: `Option` is the run-or-refuse bit**
> - `None` means the tool runs. `Some(message)` means it does not, and the `String` is the tool result the model will read.
> - The first `if` returns `None` for every name except `terminal` and `execute_code`. `echo` and `delegate` take that arm.
> - `get("code")` is `Option<&Value>`. `and_then` with `as_str` keeps the arm only when the JSON value is a string. `unwrap_or("")` turns a missing key, or a number, into an empty script. That empty string is still passed into `check_execute_code`. The gate does not read it.

The six arguments on the loop's call are fixed literals. `command` is the only value that comes from the tool call, and the gate ignores it. `env_type` is `"local"`. `presence` is `Presence::Interactive`. `mode_off` is `false`. `yolo` is `false`. `whole_script` is `false`. Section 13.7 walks what each of those does. With this combination the function returns `CodeDecision::Approved`, the match arm returns `None`, and the script is not refused.

The `Denied` and `Ask` arms are in the match because `CodeDecision` has three variants and a `match` has to name every one. They do not run for these literals. Cron, mode off, yolo, and `whole_script` are tested by calling `check_execute_code` directly. `deny_tool` has no parameter for them.

`drive_loop` calls `deny_tool` before it builds a `ToolCall`. A `Some` is stored and skipped. A `None` is pushed into `allowed` and later handed to `run_raced`.

```rust
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
```

`if let Some(message)` is the `Option` from `deny_tool`. For an interactive local `execute_code` the `if let` fails and the call is moved into `allowed`. The result walk looks a denial up by id and `continue`s before `ran.next()`, so a real denial stays aligned with the zip. This call is not in that vec. The dispatcher output is what gets pushed.

The demo imports `harness_core::sandbox::run_script`. The turn table, the approval fake model, and the hooks fake model each have their own `run_script`. The sandbox one is the function that talks to a `Worker`.

## 13.2 `Backend` and the Docker argv

Two unit variants and four constants sit at the top of the file.

```rust
pub const DEFAULT_PIDS_LIMIT: &str = "2048";
pub const MAX_SNAPSHOTS: usize = 20;
pub const MAX_FILE_BYTES: usize = 10 * 1024 * 1024;
pub const MAX_FILES: usize = 50_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Local,
    Docker,
}

impl Backend {
    pub fn from_name(name: &str) -> Self {
        if name.eq_ignore_ascii_case("docker") {
            Self::Docker
        } else {
            Self::Local
        }
    }

    pub fn is_sandboxed(self) -> bool {
        matches!(self, Self::Docker)
    }
}
```

> **Rust sidebar: an enum is one variant, and `Copy` duplicates it**
>
> - `Backend` is either `Local` or `Docker`.
> - `PartialEq` is what `assert_eq!(Backend::from_name("local"), Backend::Local)` uses. `Copy` means the copy is implicit: assigning the value, or passing it to `is_sandboxed(self)`, still leaves the original in place.
> - `eq_ignore_ascii_case` compares ASCII letters without allocating. `"docker"`, `"Docker"`, and `"DOCKER"` all take the `Docker` arm. `""`, `"local"`, and `"docker "` take the `else` and return `Local`.
> - `matches!(self, Self::Docker)` is true only for the `Docker` variant. `Self` here is `Backend`. `is_sandboxed` takes `self` by value. Because of `Copy`, `Backend::Local.is_sandboxed()` does not consume a variable you still need.
> - `Option`, `Result`, enums, and `Arc` are taught in [chapter 1](chapter-01-first-contact.md).

`from_name("local")` and `from_name("")` are what the test calls. Both are `Local`, and `Backend::Local.is_sandboxed()` is false, while `Backend::Docker.is_sandboxed()` is true. The test constructs that `Docker` as the variant. Any ASCII-case spelling of `docker` is the sandboxed variant, and every other string, including `"docker "` with a trailing space, is local.

```rust
pub fn docker_run_args(image: &str) -> Vec<String> {
    vec![
        "docker".into(),
        "run".into(),
        "--rm".into(),
        "--cap-drop".into(),
        "ALL".into(),
        "--cap-add".into(),
        "DAC_OVERRIDE".into(),
        "--cap-add".into(),
        "CHOWN".into(),
        "--cap-add".into(),
        "FOWNER".into(),
        "--security-opt".into(),
        "no-new-privileges".into(),
        "--pids-limit".into(),
        DEFAULT_PIDS_LIMIT.into(),
        image.into(),
    ]
}
```

> **Rust sidebar: `.into()` allocates the `String` the vec stores**
> - A string literal is `&str`. `Vec<String>` needs an owned `String` in each slot. `.into()` is `Into<String>`, and for a literal that allocates a copy. `image.into()` copies the caller's `&str`. The caller keeps its own string.
> - The vec owns sixteen strings. This function does not build a `Command` and does not call `spawn`. Returning the vec is the end of the work.

`--cap-drop` is followed by `ALL`. Three `--cap-add` entries put `DAC_OVERRIDE`, `CHOWN`, and `FOWNER` back. `--security-opt` is followed by `no-new-privileges`. `--pids-limit` is followed by `DEFAULT_PIDS_LIMIT`, the string `"2048"`. `--rm` is in the vec. The image is last. The test passes `"hermes-sandbox"`. The function does not inspect the image.

`the_default_backend_is_local_and_docker_argv_is_hardened` locks three adjacent pairs and one absence:

```rust
assert!(args.windows(2).any(|pair| pair == ["--cap-drop", "ALL"]));
assert!(args
    .windows(2)
    .any(|pair| pair == ["--security-opt", "no-new-privileges"]));
assert!(args
    .windows(2)
    .any(|pair| pair == ["--pids-limit", DEFAULT_PIDS_LIMIT]));
assert!(!args.iter().any(|arg| arg == "docker-run-executed"));
```

`windows(2)` yields each adjacent pair, and `String` compares equal to `&str`, so the pair matches the array of literals. The three asserts fail if a flag is renamed or split from its value. `--rm` and the three `--cap-add` lines (`DAC_OVERRIDE`, `CHOWN`, `FOWNER`) are in the vec the function returns. The test locks the three pairs above, plus the absence of `docker-run-executed`, which records that the function returned the vec. `Command` appears later, and it starts `sandbox-worker`.

## 13.3 `scrub_env`

A provider credential is a name, checked before the allowlist.

```rust
pub fn is_provider_credential(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper == "OPENAI_API_KEY"
        || (upper.starts_with("AUXILIARY_")
            && (upper.contains("API_KEY") || upper.contains("BASE_URL")))
        || upper.starts_with("GATEWAY_RELAY_")
}
```

`to_ascii_uppercase` allocates, and the comparisons read that copy. `"openai_api_key"` matches the equality. The test calls `is_provider_credential` on `AUXILIARY_OPENAI_API_KEY` and `GATEWAY_RELAY_TOKEN`. The auxiliary name is true because it starts with `AUXILIARY_` and contains `API_KEY`. A name that only has the prefix, with neither `API_KEY` nor `BASE_URL`, is false. `GATEWAY_RELAY_` is true from the prefix alone, so the `TOKEN` in the test's name isn't what that clause reads.

The allowlist and the secret substrings are private. `scrub_env` is what tests call.

```rust
const SECRET_PARTS: &[&str] = &[
    "KEY",
    "TOKEN",
    "SECRET",
    "PASSWORD",
    "CREDENTIAL",
    "PASSWD",
    "AUTH",
    "DSN",
];

fn safe_name(name: &str) -> bool {
    const EXACT: &[&str] = &[
        "PATH",
        "HOME",
        "USER",
        "LANG",
        "TERM",
        "TMPDIR",
        "TMP",
        "TEMP",
        "SHELL",
        "LOGNAME",
        "PYTHONPATH",
        "VIRTUAL_ENV",
        "SYSTEMROOT",
        "COMSPEC",
        "PATHEXT",
    ];
    EXACT.contains(&name)
        || name.starts_with("LC_")
        || name.starts_with("XDG_")
        || name.starts_with("CONDA")
}

fn secret_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    SECRET_PARTS.iter().any(|part| upper.contains(part))
}

/// Allowlist, then declared passthrough, minus provider credentials.
pub fn scrub_env(
    env: &[(impl AsRef<str>, impl AsRef<str>)],
    passthrough: &[&str],
) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (key, value) in env {
        let key = key.as_ref();
        if is_provider_credential(key) {
            continue;
        }
        let passed = passthrough.contains(&key);
        if passed {
            out.push((key.to_string(), value.as_ref().to_string()));
            continue;
        }
        if secret_name(key) {
            continue;
        }
        if safe_name(key) {
            out.push((key.to_string(), value.as_ref().to_string()));
        }
    }
    out
}
```

> **Rust sidebar: `continue` skips the rest of this pair**
> - `for (key, value) in env` borrows each pair. The slice stays with the caller.
> - `continue` jumps to the next pair. A provider credential hits the first `continue` and is never pushed. The passthrough list is not consulted for that name.
> - A passed-through name is pushed and then `continue`s, so `secret_name` and `safe_name` do not see it.
> - A secret name `continue`s with no push. A safe name is pushed. A name that misses every `if` falls out of the loop body and is also absent from `out`. Silence is the drop.
> - `impl AsRef<str>` means each half of the pair can produce a `&str`. The test's pairs are `&str`. `as_ref()` borrows them. `to_string()` allocates the copy that `out` owns. `passthrough.contains(&key)` is an exact match against `&str`. `"tenor_api_key"` does not contain-match `"TENOR_API_KEY"`.

The test builds five pairs and a passthrough of `TENOR_API_KEY` and `OPENAI_API_KEY`.

`OPENAI_API_KEY` hits the provider `continue` even though the passthrough list names it. `TENOR_API_KEY` is pushed because it is on that list. It contains `KEY`, so `secret_name` would have dropped it, and that check never runs because of the `continue`. `RANDOM_SECRET` is not passed through, `secret_name` sees `SECRET`, and it is dropped. `PATH` and `HOME` reach `safe_name` and are in `EXACT`.

`safe_name` does not uppercase. `"path"` misses `EXACT` and is dropped unless it was passed through. `secret_name` does uppercase, so `"random_secret"` is still a secret. The test asserts names only. The kept values are `"/usr/bin"`, `"tenor"`, and `"/tmp/me"`.

`LC_`, `XDG_`, and `CONDA` are prefixes on the original spelling. `CONDA_TOKEN` starts with `CONDA` and contains `TOKEN`, and `secret_name` is the earlier `continue`, so the prefix doesn't save it. `safe_name` also keeps `USER`, `TMPDIR`, and the Windows names in `EXACT` when the spelling matches.

The child inherits the parent's environment, because `Worker::spawn` doesn't call `scrub_env`. The scrub is the function a caller uses when it builds an env list.

## 13.4 `Worker`

The child is one struct. Stdin and stdout are taken off the `Child` so the parent can write a job and read a line. Stderr is discarded.

```rust
#[derive(Debug)]
pub struct Worker {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
}

impl Worker {
    pub fn spawn(bin: &Path) -> io::Result<Self> {
        let mut child = Command::new(bin)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let stdin = child.stdin.take().expect("stdin");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }

    pub fn accept(&mut self, job: &str) -> io::Result<String> {
        writeln!(self.stdin, "{job}")?;
        self.stdin.flush()?;
        let mut line = String::new();
        self.stdout.read_line(&mut line)?;
        if line.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "worker closed",
            ));
        }
        Ok(line.trim_end().to_string())
    }

    pub fn quit(mut self) -> io::Result<()> {
        let _ = writeln!(self.stdin, "quit");
        let _ = self.stdin.flush();
        let _ = self.child.wait();
        Ok(())
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
```

> **Rust sidebar: `Command` starts a process, `?` returns the error, `expect` crashes**
> - `Command::new(bin)` is the program. Piped stdin and stdout are the pipes. `Stdio::null()` drops stderr. `spawn()` returns `io::Result<Child>`.
> - `?` returns the `io::Error` from `Worker::spawn` when the program fails to start. `.expect("worker")` in a test crashes instead.
> - `take()` moves `ChildStdin` out and leaves `None` on the child. `expect("stdin")` crashes if the pipe was missing. `BufReader` wraps stdout so `read_line` can collect bytes until a newline.
> - `writeln!` writes the job and a newline. `{job}` is the argument. `?` returns a write error. `flush` pushes the parent's buffer. `read_line` appends into an empty `String`, newline included.
> - `line.is_empty()` is true when the child closed stdout without a byte. That becomes `Err` with kind `UnexpectedEof` and the message `"worker closed"`. A line that is only `"\n"` is not empty. `trim_end` makes it `""`, and the function returns `Ok("")`.
> - `quit` takes `self` by value. `let _ =` discards the write, the flush, and the `wait`. The function still returns `Ok(())`.

`Drop` runs when the `Worker` goes out of scope, including at the end of `quit`, because `quit` owns `self`. `kill` signals the child and `wait` collects the exit. Both results are discarded. The warm-worker test never calls `quit`. The `Worker` drops at the end of the test, and that `kill` is what ends `sandbox-worker`. The bench calls `quit` after the samples so the child sees the `quit` line and leaves its read loop. `Drop` still runs afterwards.

`accept` blocks in `read_line` until a newline arrives or the pipe closes. Stdout on a pipe is block-buffered, so a child that writes a line and never flushes leaves the parent in that read. The worker binary flushes every reply. The parent's `flush` puts `ping\n` into the pipe before the read.

`a_warm_worker_accepts_a_job` spawns the worker, sends one `ScriptJob` whose line is `ping`, and expects `vec!["ok ping"]`. `run_script` is the loop around `accept`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptJob {
    pub line: String,
}

pub fn run_script(worker: &mut Worker, jobs: &[ScriptJob]) -> io::Result<Vec<String>> {
    let mut out = Vec::new();
    for job in jobs {
        out.push(worker.accept(&job.line)?);
    }
    Ok(out)
}
```

`&mut Worker` is required because `accept` needs `&mut self`. Jobs run in order. The first `Err` returns through `?`. The test and the demo each pass one job. An empty slice returns an empty vec and never calls `accept`. No test passes zero jobs.

## 13.5 `run_until`

`accept` waits until the child writes. `run_until` waits until the child exits, or until the timeout, whichever it sees first. The error type is local so a test can match `Timeout` with `==`-style patterns.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandboxError {
    Timeout,
    Io(String),
}

/// Send one job. If the child is still running when `timeout` passes, kill it.
pub fn run_until(bin: &Path, job: &str, timeout: Duration) -> Result<String, SandboxError> {
    let mut worker = Worker::spawn(bin).map_err(|err| SandboxError::Io(err.to_string()))?;
    writeln!(worker.stdin, "{job}").map_err(|err| SandboxError::Io(err.to_string()))?;
    worker
        .stdin
        .flush()
        .map_err(|err| SandboxError::Io(err.to_string()))?;
    let started = Instant::now();
    loop {
        if started.elapsed() > timeout {
            let _ = worker.child.kill();
            let _ = worker.child.wait();
            return Err(SandboxError::Timeout);
        }
        match worker.child.try_wait() {
            Ok(Some(_)) => {
                let mut line = String::new();
                let _ = worker.stdout.read_line(&mut line);
                return Ok(line.trim_end().to_string());
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
            Err(err) => return Err(SandboxError::Io(err.to_string())),
        }
    }
}
```

`map_err` stores `err.to_string()` in `SandboxError::Io`. `io::Error` does not implement `PartialEq`. A `String` does, which is why the enum derives `Eq`. Spawn failure, a failed write, a failed flush, and `try_wait` returning `Err` are the four `Io` paths. The timeout test hits none of them.

`started.elapsed() > timeout` is the first check in the loop. An elapsed time equal to `timeout` still polls. Then `try_wait` reports the process, not the pipe.

`Ok(Some(_))` means the process has exited. The status is ignored. One line is read and returned. A read error is discarded, and an empty buffer becomes `Ok("")`. This arm runs only after the child is gone.

`Ok(None)` means the process is still running. The function sleeps 5 milliseconds and loops. It does not read stdout on this arm. A finished `ok ping` sitting in the pipe stays there.

The worker reads until the line is `quit` or stdin closes. `run_until` holds the `Worker`, so stdin stays open, and it never writes `quit`. `ping` makes the child write `ok ping` and block on the next read, and `try_wait` stays `Ok(None)` until the timeout. The success arm is for a child that exits, which this worker does on `quit` (no reply) or when the parent drops stdin.

`a_timeout_kills_the_child` sends `sleep-ms 5000` with `Duration::from_millis(80)`. The child is inside that sleep when the timeout arm kills it, before `ok slept` is written. The test matches `Err(SandboxError::Timeout)` and asserts the elapsed time is under two seconds. `Drop` then kills and waits again. Those errors are discarded.

The kill is `worker.child.kill()` on the process `spawn` started, and a grandchild would keep running. `drive_loop` doesn't call `run_until`. The function is synchronous, `thread::sleep` blocks the caller, and the timeout test is a plain `#[test]`.

## 13.6 Checkpoints

A snapshot is a vec of path and content. The store is a deque of those snapshots, and it starts disabled.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    pub files: Vec<(String, String)>,
}

impl Manifest {
    pub fn checkpoint(files: &[(String, String)]) -> Self {
        let mut kept = Vec::new();
        for (path, content) in files.iter().take(MAX_FILES) {
            if content.len() <= MAX_FILE_BYTES {
                kept.push((path.clone(), content.clone()));
            }
        }
        Self { files: kept }
    }

    pub fn rewind_into(&self, live: &mut Vec<(String, String)>) {
        live.clone_from(&self.files);
    }
}

#[derive(Debug)]
pub struct Checkpoints {
    enabled: bool,
    snaps: VecDeque<Manifest>,
}

impl Checkpoints {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            snaps: VecDeque::new(),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn len(&self) -> usize {
        self.snaps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.snaps.is_empty()
    }

    pub fn take(&mut self, files: &[(String, String)]) -> Option<&Manifest> {
        if !self.enabled {
            return None;
        }
        if self.snaps.len() == MAX_SNAPSHOTS {
            self.snaps.pop_front();
        }
        self.snaps.push_back(Manifest::checkpoint(files));
        self.snaps.back()
    }

    pub fn latest(&self) -> Option<&Manifest> {
        self.snaps.back()
    }
}
```

`Checkpoints::new(false)` sets `enabled` to false and `snaps` to an empty deque, and there is no setter. `take` returns `None` before `pop_front` and before `push_back`, so the files stay unsaved. The test calls that with `a.txt` / `one` and asserts `is_none()`.

On an enabled store, `len() == MAX_SNAPSHOTS` (20) means full: `pop_front` drops the oldest, `push_back` adds the new one, and the length stays 20. Fewer than 20 skips the pop. `take` is the only push, so the deque does not grow past 20. `VecDeque` is what makes dropping the front cheap. The returned `back()` borrows the store. The test `expect`s it and drops that borrow before `latest()`.

`Manifest::checkpoint` walks `files.iter().take(MAX_FILES)`, so anything after the first 50,000 entries is never examined. An oversized file inside that window is skipped and does not free a slot past the window. `content.len()` is bytes. `<= MAX_FILE_BYTES` keeps a file of exactly `10 * 1024 * 1024` bytes. One byte more is omitted. The clones are the snapshot. The caller's slice stays as it was.

`rewind_into` is `live.clone_from(&self.files)`. Paths that existed only in `live` disappear. The test takes `a.txt` / `one`, sets live to `changed`, rewinds, and expects `"one"`. Then it takes `MAX_SNAPSHOTS + 5` more snapshots. The length is 20. The snapshot that held `one` has been popped. The rewind already ran, so `live` still has `one`. `latest` is `back()`. The test calls it on the enabled store before that overflow loop.

The 1,000-path helper builds the vec the bench and the round-trip test share.

```rust
pub fn thousand_files() -> Vec<(String, String)> {
    (0..1_000)
        .map(|index| (format!("f{index}.txt"), format!("body {index}")))
        .collect()
}
```

`(0..1_000)` is the indexes `0` through `999`. `a_thousand_path_manifest_round_trips` expects length 1,000 and `live[0].0 == "f0.txt"` after a rewind that started from `gone.txt`. `checkpoint` keeps caller order. Every body fits, so nothing is skipped. `gone.txt` is gone because `clone_from` replaced the vec.

A directory is a second pair of functions. `Checkpoints` is not involved.

```rust
pub fn snapshot_dir(dir: &Path) -> io::Result<Manifest> {
    let mut files = Vec::new();
    collect(dir, dir, &mut files)?;
    files.sort();
    Ok(Manifest { files })
}

fn collect(root: &Path, dir: &Path, files: &mut Vec<(String, String)>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect(root, &path, files)?;
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let content = fs::read_to_string(&path).unwrap_or_default();
        if content.len() <= MAX_FILE_BYTES {
            files.push((rel, content));
        }
    }
    Ok(())
}

pub fn restore_dir(dir: &Path, snap: &Manifest) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    for (rel, content) in &snap.files {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }
    Ok(())
}
```

`snapshot_dir` recurses into directories and reads each file as a string. `strip_prefix(root)` makes the path relative, `unwrap_or(&path)` keeps the original if the prefix is missing, and backslashes become slashes. `read_to_string` failure becomes `""`, and that empty string is kept when it is under the cap. A longer file is left out. `collect` does not stop at `MAX_FILES`. That cap is on `Manifest::checkpoint`, which this function does not call. `files.sort()` then orders by path, then content. `checkpoint` does not sort.

`restore_dir` creates the directory and writes each manifest entry, creating parents along the way. It leaves files that are absent from the manifest. `rewind_into` replaces a vec. `restore_dir` overwrites the paths it has.

`a_directory_snapshot_rewinds_one_file` uses a temp directory named with the process id. `note.txt` goes from `before` to `after` and back to `before`, and the directory is removed before the test and again at the end.

The 500 MB total cap is Hermes's. This manifest is in memory. `take` caps the count at 20. It does not add up bytes.

## 13.7 Which scripts the gate allows

The gate is where the script would run. The source text is a parameter so the signature matches the call shape, and the body does not look at it.

```rust
pub const EXECUTE_CODE_DESCRIPTION: &str = "execute_code script execution. The script can spawn subprocesses or mutate files without passing through terminal command approval; approval is one-shot for this run.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeDecision {
    Approved,
    Denied { message: String },
    Ask { description: &'static str },
}

/// `check_execute_code_guard` from `tools/approval.py`.
/// An isolated `env_type` (anything other than `local`) is already sandboxed.
/// YOLO and mode off approve. Cron, `-q`, and unattended deny.
/// `whole_script` is the gateway/ask path: one prompt for the script.
/// An interactive CLI approves here; the script's own `terminal` calls still hit chapter 11.
pub fn check_execute_code(
    code: &str,
    env_type: &str,
    presence: crate::approval::Presence,
    mode_off: bool,
    yolo: bool,
    whole_script: bool,
) -> CodeDecision {
    let _ = code;
    if env_type != "local" {
        return CodeDecision::Approved;
    }
    if yolo || mode_off {
        return CodeDecision::Approved;
    }
    if matches!(
        presence,
        crate::approval::Presence::Cron
            | crate::approval::Presence::SingleQuery
            | crate::approval::Presence::Unattended
    ) {
        return CodeDecision::Denied {
            message: "BLOCKED: execute_code runs arbitrary local Python (including subprocess calls that bypass shell-string approval checks).".to_string(),
        };
    }
    if whole_script {
        return CodeDecision::Ask {
            description: EXECUTE_CODE_DESCRIPTION,
        };
    }
    CodeDecision::Approved
}
```

> **Rust sidebar: the first matching `if` returns, and `let _ = code` keeps the argument**
> - `let _ = code;` reads the argument and throws it away, which silences the unused-variable warning. `print(1)` and `""` take the same path. The script is not scanned.
> - The parameter is written `crate::approval::Presence`. `sandbox.rs` has no `use` of `approval`. The enum is `Interactive`, `Cron`, `SingleQuery`, `Unattended`, and it is `Copy`.
> - `Approved` carries nothing. `Denied` owns a `String`, so the enum is `Clone` and not `Copy`. `Ask` holds `&'static str`, the constant `EXECUTE_CODE_DESCRIPTION`. `PartialEq` is what `assert_eq!` uses.
> - Each `return` leaves the function. The last line has no `return` keyword. It is the value when every `if` was false.

The order of those returns is the policy, and the first match wins. Cron inside a non-local env is approved because the env check returns before presence is considered.

| Check, in order | Result |
|---|---|
| `env_type` is anything other than the exact bytes `"local"` | `Approved`, including cron |
| `yolo` or `mode_off` | `Approved` |
| presence is cron, single-query, or unattended | `Denied`, with the local-Python `BLOCKED` sentence |
| `whole_script` | `Ask`, one description, no wait inside this function |
| whatever is left | `Approved` |

`let _ = code` throws the script away before any of those checks, so `print(1)` and an empty string take the same path. `deny_tool` still reads `arguments["code"]` so the key stays `code`. `"container"`, `"ssh"`, `""`, and `"Local"` are all approved on the first row, because this comparison doesn't fold case the way `Backend::from_name` does. The test's second call is `check_execute_code("print(1)", "container", Presence::Cron, false, false, false)`, and it expects `Approved`.

A local cron session with yolo on, or with mode off, is approved on the second row. The test calls the mode-off pair, `check_execute_code("print(1)", "local", Presence::Interactive, true, false, false)`, so the `||` fires while `yolo` is false. Both flags share that return. The denial runs only when the env was `"local"` and both flags were false, and it sits above `whole_script`, so a local cron call with `whole_script` true is still `Denied`. `Cron`, `SingleQuery`, and `Unattended` share that one `Denied`. `Interactive` falls through. The message is:

`BLOCKED: execute_code runs arbitrary local Python (including subprocess calls that bypass shell-string approval checks).`

`STOP_COMMAND` isn't on that message. The test matches the prefix `BLOCKED: execute_code runs arbitrary local Python`.

`whole_script` returns `Ask` with `description` set to `EXECUTE_CODE_DESCRIPTION`. The test calls `check_execute_code("print(1)", "local", Presence::Interactive, false, false, true)` and compares the description to the constant. The function returns that description without waiting for a human. The `BLOCKED` sentence with `STOP_COMMAND` is the `Ask` arm in `deny_tool`. The loop passes `whole_script: false`, so that arm isn't the path an ordinary turn takes. The arguments `deny_tool` does pass, local and interactive with all three flags false, land on the last row, which is why a local interactive `execute_code` in the loop is `Approved` and `deny_tool` returns `None`.

A `terminal` call inside the script is a later tool call. `deny_tool` sends that name to `check` on `Policy::manual()`, and `check_execute_code` doesn't call `check`. `tests/execute_code.rs` is the call that passes `Presence::Cron`. The loop doesn't.

## 13.8 The worker binary and the demo

The bin is `src/bin/worker.rs`, named `sandbox-worker` in `Cargo.toml`. It is a separate crate from the library. It links nothing from `harness_core`. One line in, one `ok` line out.

```rust
//! Long-lived child. One line in, one `ok` line out. `sleep-ms N` waits first.

use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let stdin = io::stdin();
    let mut out = io::stdout();
    for line in stdin.lock().lines() {
        let line = line.unwrap_or_default();
        if line == "quit" {
            break;
        }
        if let Some(raw) = line.strip_prefix("sleep-ms ") {
            let millis = raw.parse::<u64>().unwrap_or(0);
            thread::sleep(Duration::from_millis(millis));
            let _ = writeln!(out, "ok slept");
        } else {
            let _ = writeln!(out, "ok {line}");
        }
        let _ = out.flush();
    }
}
```

`stdin.lock().lines()` yields each line without its newline. A read error becomes `""`. That empty line is not `quit` and misses the sleep prefix, so the else arm writes `ok `. `quit` breaks before any write, `main` ends, and `Worker::quit`'s `wait` returns.

`strip_prefix("sleep-ms ")` borrows the rest of the line. `parse::<u64>()` fails to `0`. The reply is `ok slept`. The timeout test parses `5000`, so the child is inside `thread::sleep` when it is killed and `ok slept` has not been written. Any other line, including `ping`, writes `ok {line}`.

`out.flush()` runs after both reply arms and not on `quit`, because `break` already left the loop. Without that flush, `accept` waits on a block-buffered pipe.

> **Rust sidebar: `env!` runs at compile time**
> - `env!("CARGO_BIN_EXE_sandbox-worker")` is not a runtime lookup. The compiler pastes the path into the test binary and the bench binary. If the `[[bin]]` named `sandbox-worker` is missing, those crates do not compile.
> - That is why every later copy of this harness keeps the worker binary even when the chapter is about providers or memory. `option_env!` would compile and return `None`. The tests want the failure to be loud.
> - The demo binary does not use `env!`. Cargo sets `CARGO_BIN_EXE_<name>` for tests, benches, and examples. `chapter_demo` finds the worker next to its own executable instead.

```rust
fn worker_path() -> PathBuf {
    let mut path = std::env::current_exe().expect("exe");
    path.pop();
    let name = if cfg!(windows) {
        "sandbox-worker.exe"
    } else {
        "sandbox-worker"
    };
    path.push(name);
    path
}

fn main() {
    let mut worker = Worker::spawn(&worker_path()).expect("worker");
    let replies = run_script(
        &mut worker,
        &[ScriptJob {
            line: "ping".into(),
        }],
    )
    .expect("accept");
    println!("reply: {}", replies[0]);
    assert_eq!(replies[0], "ok ping");
    worker.quit().expect("quit");
}
```

`current_exe` is the `ch13-sandbox` binary. `pop` drops that file name. `cfg!(windows)` is false on ubuntu, so the pushed name is `sandbox-worker`. `cargo run` puts both binaries in one directory. The print is `reply: ok ping`, then `assert_eq` checks that line, then `quit` writes `quit` and waits. No API key. No Docker. The tests and the bench skip `worker_path` and pass `env!("CARGO_BIN_EXE_sandbox-worker")` to `spawn` or `run_until`.

## 13.9 What each test locks

Eight tests are the new surface. Seven are in `tests/sandbox.rs`.

`the_default_backend_is_local_and_docker_argv_is_hardened` locks `from_name("local")`, `from_name("")`, both `is_sandboxed` results, the three flag pairs, and the absence of `docker-run-executed`.

`scrub_keeps_path_and_a_third_party_key_and_drops_openai` locks the five names and the two `is_provider_credential` calls.

`a_warm_worker_accepts_a_job` locks `["ok ping"]`. `a_timeout_kills_the_child` locks `Err(Timeout)` in under two seconds.

`checkpoints_are_off_until_enabled_and_rewind_restores` locks a disabled `take` of `None`, rewind of `one`, and length 20. `a_thousand_path_manifest_round_trips` locks length 1,000 and `f0.txt`.

`a_local_script_is_denied_for_cron_and_approved_in_a_container` locks the cron denial, the container approval, the mode-off approval, and the `whole_script` ask.

Inherited tests from the copied harness make up the rest of the `cargo nextest` summary.

## 13.10 What the three benches time

The bench file is `benches/sandbox.rs`. The names `warm_accept`, `checkpoint_1000`, and `rewind_1000` are the `bench_function` strings. The measured means stay in the table further down. This section is the source those names time.

```rust
fn warm_accept(c: &mut Criterion) {
    let bin = Path::new(env!("CARGO_BIN_EXE_sandbox-worker"));
    let mut worker = Worker::spawn(bin).expect("worker");
    c.bench_function("warm_accept", |b| {
        b.iter(|| {
            let reply = worker.accept(black_box("ping")).expect("accept");
            black_box(reply);
        });
    });
    worker.quit().expect("quit");
}
```

`Worker::spawn` runs before `b.iter`. Each sample writes `ping`, flushes, and reads `ok ping`. Process startup is outside the sample. The job is `ping`, so the worker does not sleep. `black_box` is `std::hint::black_box`. It makes the optimizer treat the job string and the returned line as used. `quit` runs after the samples.

```rust
fn checkpoint_1000(c: &mut Criterion) {
    let files = thousand_files();
    c.bench_function("checkpoint_1000", |b| {
        b.iter(|| {
            let snap = Manifest::checkpoint(black_box(&files));
            black_box(snap);
        });
    });
}

fn rewind_1000(c: &mut Criterion) {
    let snap = Manifest::checkpoint(&thousand_files());
    c.bench_function("rewind_1000", |b| {
        b.iter(|| {
            let mut live = Vec::new();
            snap.rewind_into(black_box(&mut live));
            black_box(live);
        });
    });
}
```

`checkpoint_1000` builds the 1,000 pairs before the samples. Each body is `body N`. The timed closure clones the pairs that fit the size cap. Building the input vec is outside the sample. Every pair fits, so the snapshot length is 1,000.

`rewind_1000` builds the snapshot before the samples. Each sample starts from `Vec::new()` and `clone_from`s the snapshot into it. That copy is the sample. `criterion_group!(benches, warm_accept, checkpoint_1000, rewind_1000)` registers the three functions, and `criterion_main!` is the bench binary's entry.

## Tests

Eight tests. Local is the default and the Docker argv contains the three flags. Scrub keeps `PATH`, `HOME`, and `TENOR_API_KEY`, and drops `OPENAI_API_KEY` and `RANDOM_SECRET`. A warm worker answers `ok ping`. A five-second sleep is killed in under two seconds. Checkpoints do nothing while disabled, rewind restores `one`, and the store stays at 20. A 1,000-path manifest round-trips. A temp-dir file goes from `after` back to `before`. Cron denies a local script, a container approves that same cron call, mode off approves, and `whole_script` asks once.

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

The means are `mean.point_estimate` from [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `13`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

Local is the default. Printing Docker argv does not put the process in a container. `is_sandboxed` is the bit a later chapter should check before it tells the model the command is isolated.

`accept` deadlocks if the child does not flush. The worker flushes every line. A child you write later has to do the same.

`run_until` kills the process it spawned. A grandchild the worker started would survive. That is the gap next to Hermes's `kill_process_tree`.

Checkpoints are off. `take` on a disabled store returns `None` and does not remember the files. A caller that ignores `None` will rewind nothing and think it restored.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L7.

**D1.** `Backend::Local` is the default. It is not a security boundary. The Docker backend is opt-in, and this chapter does not start it.

**D2.** When the backend is Docker, the argv includes `--cap-drop ALL`, `--security-opt no-new-privileges`, and `--pids-limit 2048`. The caps Hermes adds back (`DAC_OVERRIDE`, `CHOWN`, `FOWNER`) are on that argv too.

**D3 and D4.** `scrub_env` is the allowlist plus passthrough. `OPENAI_API_KEY`, `AUXILIARY_*_API_KEY`, `AUXILIARY_*_BASE_URL`, and `GATEWAY_RELAY_*` are dropped even if a caller lists them. A third-party key on the passthrough list is kept. Names containing `KEY`, `TOKEN`, `SECRET`, `PASSWORD`, `CREDENTIAL`, `PASSWD`, `AUTH`, or `DSN` are dropped unless they were passed through.

**I6.** `run_until` is the timeout. It kills the child it started. It does not walk a process tree.

**N1.** `Checkpoints` starts disabled. The caps are 20 snapshots, 10 MB per file, and 50,000 files. The 500 MB total cap is Hermes's; this manifest is in memory and does not track a store size yet.

**A2 is this chapter.** `check_execute_code` is the script gate. Cron, single-query, and unattended deny a local script. An isolated env approves. `whole_script` asks once. An interactive CLI approves here; `terminal` inside the script still hits chapter 11.

**Left for later.** File-path checks from chapter 3 stay in force. They are defense in depth, which is what `agent/file_safety.py` says. D5 and D6 are not this chapter. Secret redaction of the job line (E1, E2) is chapter 20.

## Chapter summary

- **Default.** Local. Docker is an argv.
- **Child.** A warm worker answers `ok ping`. A timeout kills a sleep.
- **Env.** `PATH` stays. `OPENAI_API_KEY` does not, even on the passthrough list.
- **Checkpoint.** Off until enabled. Rewind copies the manifest back.
- **Gate.** The loop's local interactive `execute_code` is `Approved`. Cron on `local` is `Denied` when you call `check_execute_code`.
- **Time.** A warm accept measured **30.49 µs**. Checkpoint of 1,000 paths measured **50.37 µs**. Rewind measured **83.79 µs**.

## What you implement

`my-harness/tests/ch13.rs` is the milestone. It compiles, then panics in `todo!("chapter 13")` until `chapter_13_execute_code` returns `"deny"` when the presence is cron and the env is local, and `"allow"` when the presence is cron and the env is not local. 

## What's next

Chapter 14 classifies a failure and decides whether to retry. A timeout from this chapter is one of those classes, and the retry has to stay inside the concurrency bound. Empty replies and a repetition loop, deferred since chapter 9, land there too.
