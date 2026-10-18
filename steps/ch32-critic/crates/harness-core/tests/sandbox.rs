use std::time::{Duration, Instant};

use harness_core::sandbox::{
    docker_run_args, is_provider_credential, restore_dir, run_script, run_until, scrub_env,
    snapshot_dir, thousand_files, Backend, Checkpoints, Manifest, ScriptJob, Worker,
    DEFAULT_PIDS_LIMIT, MAX_SNAPSHOTS,
};

fn worker() -> Worker {
    Worker::spawn(std::path::Path::new(env!("CARGO_BIN_EXE_sandbox-worker"))).expect("worker")
}

#[test]
fn the_default_backend_is_local_and_docker_argv_is_hardened() {
    assert_eq!(Backend::from_name("local"), Backend::Local);
    assert_eq!(Backend::from_name(""), Backend::Local);
    assert!(!Backend::Local.is_sandboxed());
    assert!(Backend::Docker.is_sandboxed());
    let args = docker_run_args("hermes-sandbox");
    assert!(args.windows(2).any(|pair| pair == ["--cap-drop", "ALL"]));
    assert!(args
        .windows(2)
        .any(|pair| pair == ["--security-opt", "no-new-privileges"]));
    assert!(args
        .windows(2)
        .any(|pair| pair == ["--pids-limit", DEFAULT_PIDS_LIMIT]));
    assert!(!args.iter().any(|arg| arg == "docker-run-executed"));
}

#[test]
fn scrub_keeps_path_and_a_third_party_key_and_drops_openai() {
    let env = [
        ("PATH", "/usr/bin"),
        ("OPENAI_API_KEY", "sk-test"),
        ("TENOR_API_KEY", "tenor"),
        ("RANDOM_SECRET", "nope"),
        ("HOME", "/tmp/me"),
    ];
    let scrubbed = scrub_env(&env, &["TENOR_API_KEY", "OPENAI_API_KEY"]);
    let names: Vec<&str> = scrubbed.iter().map(|(key, _)| key.as_str()).collect();
    assert!(names.contains(&"PATH"));
    assert!(names.contains(&"HOME"));
    assert!(names.contains(&"TENOR_API_KEY"));
    assert!(!names.contains(&"OPENAI_API_KEY"));
    assert!(!names.contains(&"RANDOM_SECRET"));
    assert!(is_provider_credential("AUXILIARY_OPENAI_API_KEY"));
    assert!(is_provider_credential("GATEWAY_RELAY_TOKEN"));
}

#[test]
fn a_warm_worker_accepts_a_job() {
    let mut worker = worker();
    let replies = run_script(
        &mut worker,
        &[ScriptJob {
            line: "ping".into(),
        }],
    )
    .expect("ping");
    assert_eq!(replies, vec!["ok ping".to_string()]);
}

#[test]
fn a_timeout_kills_the_child() {
    let started = Instant::now();
    let result = run_until(
        std::path::Path::new(env!("CARGO_BIN_EXE_sandbox-worker")),
        "sleep-ms 5000",
        Duration::from_millis(80),
    );
    assert!(matches!(
        result,
        Err(harness_core::sandbox::SandboxError::Timeout)
    ));
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[test]
fn checkpoints_are_off_until_enabled_and_rewind_restores() {
    let files = [("a.txt".into(), "one".into())];
    let mut off = Checkpoints::new(false);
    assert!(off.take(&files).is_none());
    let mut on = Checkpoints::new(true);
    on.take(&files).expect("snap");
    let mut live = vec![("a.txt".into(), "changed".into())];
    on.latest().expect("latest").rewind_into(&mut live);
    assert_eq!(live[0].1, "one");
    for index in 0..MAX_SNAPSHOTS + 5 {
        on.take(&[("n.txt".into(), index.to_string())]);
    }
    assert_eq!(on.len(), MAX_SNAPSHOTS);
}

#[test]
fn a_thousand_path_manifest_round_trips() {
    let files = thousand_files();
    let snap = Manifest::checkpoint(&files);
    assert_eq!(snap.files.len(), 1_000);
    let mut live = vec![("gone.txt".into(), "x".into())];
    snap.rewind_into(&mut live);
    assert_eq!(live.len(), 1_000);
    assert_eq!(live[0].0, "f0.txt");
}

#[test]
fn a_directory_snapshot_rewinds_one_file() {
    let root = std::env::temp_dir().join(format!("ch13-sandbox-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("note.txt"), "before").unwrap();
    let snap = snapshot_dir(&root).unwrap();
    std::fs::write(root.join("note.txt"), "after").unwrap();
    restore_dir(&root, &snap).unwrap();
    let text = std::fs::read_to_string(root.join("note.txt")).unwrap();
    assert_eq!(text, "before");
    let _ = std::fs::remove_dir_all(&root);
}
