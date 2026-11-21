use harness_core::approval::{check, Policy, Session, Verdict};
use harness_core::failure::{command_deletes_runtime, self_repo_git_mutation};

#[test]
fn yolo_still_refuses_to_delete_the_running_binary() {
    let runtime = "/opt/uv/python/cpython-3.12.7-x86_64/bin/python";
    assert!(command_deletes_runtime(
        &format!("rm -rf {runtime}"),
        runtime
    ));
    assert!(command_deletes_runtime(
        &format!("find {runtime} -delete"),
        runtime
    ));
    assert!(command_deletes_runtime(
        "uv python uninstall --all",
        runtime
    ));
    assert!(command_deletes_runtime("uv python uninstall 3.12", runtime));
    assert!(!command_deletes_runtime(
        "uv python uninstall 3.11",
        runtime
    ));
    assert!(!command_deletes_runtime("rm -rf /tmp/scratch", runtime));
    let running = std::env::current_exe().unwrap().display().to_string();
    let mut session = Session::new();
    let mut policy = Policy::manual();
    policy.yolo = true;
    let verdict = check(&policy, &mut session, &format!("rm {running}"));
    assert!(
        matches!(verdict, Verdict::Deny { hardline: true, .. }),
        "{verdict:?}"
    );
}

#[test]
fn yolo_still_refuses_to_rewrite_the_live_checkout() {
    let root = env!("CARGO_MANIFEST_DIR");
    assert!(self_repo_git_mutation(
        &format!("git -C {root} checkout main"),
        root
    ));
    assert!(self_repo_git_mutation(
        &format!("sudo git -C {root} reset --hard"),
        root
    ));
    assert!(!self_repo_git_mutation(
        "git -C /tmp/other-repo checkout main",
        root
    ));
    let mut session = Session::new();
    let mut policy = Policy::manual();
    policy.yolo = true;
    let verdict = check(
        &policy,
        &mut session,
        &format!("git -C {root} reset --hard"),
    );
    assert!(matches!(verdict, Verdict::Deny { .. }), "{verdict:?}");
    let allowed = check(&policy, &mut session, "echo hello");
    assert!(allowed.is_allow());
}
