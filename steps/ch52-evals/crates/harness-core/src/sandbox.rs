//! Book chapter 13. Code runs in a child process. Docker argv is built and
//! not executed. The default backend is local, which is not a sandbox.

use std::collections::VecDeque;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::{Duration, Instant};

pub const DEFAULT_PIDS_LIMIT: &str = "2048";
pub const MAX_SNAPSHOTS: usize = 20;
pub const MAX_FILE_BYTES: usize = 10 * 1024 * 1024;
pub const MAX_FILES: usize = 50_000;

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

pub fn is_provider_credential(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper == "OPENAI_API_KEY"
        || (upper.starts_with("AUXILIARY_")
            && (upper.contains("API_KEY") || upper.contains("BASE_URL")))
        || upper.starts_with("GATEWAY_RELAY_")
}

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

pub fn thousand_files() -> Vec<(String, String)> {
    (0..1_000)
        .map(|index| (format!("f{index}.txt"), format!("body {index}")))
        .collect()
}

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
