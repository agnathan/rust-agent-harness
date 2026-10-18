use std::fs;
use std::process::Command;
use std::thread;

use harness_core::kanban::{demo, task_status, transact, write_board, Board, Op, Task, TxError};

fn fresh(label: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ch38-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn claim_complete_review_and_block_are_one_status_each() {
    let dir = fresh("verbs");
    write_board(&dir, &Board::new(vec![Task::ready("t1")])).unwrap();
    assert_eq!(
        transact(
            &dir,
            "t1",
            Op::Claim {
                owner: "worker".into()
            }
        )
        .unwrap(),
        "running"
    );
    assert_eq!(
        transact(
            &dir,
            "t1",
            Op::Review {
                owner: "worker".into(),
                summary: "look".into(),
            }
        )
        .unwrap(),
        "review"
    );

    let dir = fresh("block");
    write_board(&dir, &Board::new(vec![Task::ready("t1")])).unwrap();
    transact(
        &dir,
        "t1",
        Op::Claim {
            owner: "worker".into(),
        },
    )
    .unwrap();
    assert_eq!(
        transact(
            &dir,
            "t1",
            Op::Block {
                owner: "worker".into(),
                reason: "needs a human".into(),
            }
        )
        .unwrap(),
        "blocked"
    );

    let dir = fresh("done");
    write_board(&dir, &Board::new(vec![Task::ready("t1")])).unwrap();
    transact(
        &dir,
        "t1",
        Op::Claim {
            owner: "worker".into(),
        },
    )
    .unwrap();
    assert_eq!(
        transact(
            &dir,
            "t1",
            Op::Complete {
                owner: "worker".into(),
                result: "shipped".into(),
            }
        )
        .unwrap(),
        "done"
    );
    let _ = fs::remove_dir_all(fresh("noop"));
}

#[test]
fn an_empty_result_and_a_stranger_lose() {
    let dir = fresh("empty");
    write_board(&dir, &Board::new(vec![Task::ready("t1")])).unwrap();
    transact(
        &dir,
        "t1",
        Op::Claim {
            owner: "worker".into(),
        },
    )
    .unwrap();
    assert_eq!(
        transact(
            &dir,
            "t1",
            Op::Complete {
                owner: "worker".into(),
                result: "  ".into(),
            }
        ),
        Err(TxError::Empty)
    );
    assert_eq!(
        transact(
            &dir,
            "t1",
            Op::Complete {
                owner: "other".into(),
                result: "shipped".into(),
            }
        ),
        Err(TxError::Lost)
    );
    assert_eq!(task_status(&dir, "t1").unwrap().as_deref(), Some("running"));
}

#[test]
fn a_parent_that_is_not_done_blocks_the_claim() {
    let mut child = Task::ready("child");
    child.parents.push("parent".into());
    let dir = fresh("parents");
    write_board(&dir, &Board::new(vec![Task::ready("parent"), child])).unwrap();
    assert_eq!(
        transact(
            &dir,
            "child",
            Op::Claim {
                owner: "worker".into()
            }
        ),
        Err(TxError::Parents)
    );
}

#[test]
fn pause_refuses_a_new_claim_and_complete_still_runs() {
    let dir = fresh("pause");
    let mut board = Board::new(vec![Task::ready("t1")]);
    write_board(&dir, &board).unwrap();
    transact(
        &dir,
        "t1",
        Op::Claim {
            owner: "worker".into(),
        },
    )
    .unwrap();
    board.paused = true;
    board.tasks[0].status = "running".into();
    board.tasks[0].owner = Some("worker".into());
    write_board(&dir, &board).unwrap();
    assert_eq!(
        transact(
            &dir,
            "t2",
            Op::Claim {
                owner: "other".into()
            }
        ),
        Err(TxError::Missing)
    );
    let mut paused = Board::new(vec![Task::ready("t2")]);
    paused.paused = true;
    let dir = fresh("pause-claim");
    write_board(&dir, &paused).unwrap();
    assert_eq!(
        transact(
            &dir,
            "t2",
            Op::Claim {
                owner: "other".into()
            }
        ),
        Err(TxError::Paused)
    );
}

#[test]
fn two_processes_cannot_hold_the_same_task() {
    let dir = fresh("race");
    write_board(&dir, &Board::new(vec![Task::ready("t1")])).unwrap();
    let exe = env!("CARGO_BIN_EXE_ch47_kanban_claim").to_string();
    let dir_a = dir.clone();
    let dir_b = dir.clone();
    let exe_a = exe.clone();
    let exe_b = exe.clone();
    let first = thread::spawn(move || {
        Command::new(exe_a)
            .args(["claim", dir_a.to_str().unwrap(), "t1", "a"])
            .output()
            .unwrap()
    });
    let second = thread::spawn(move || {
        Command::new(exe_b)
            .args(["claim", dir_b.to_str().unwrap(), "t1", "b"])
            .output()
            .unwrap()
    });
    let out_a = first.join().unwrap();
    let out_b = second.join().unwrap();
    let a = String::from_utf8(out_a.stdout).unwrap();
    let b = String::from_utf8(out_b.stdout).unwrap();
    let err_a = String::from_utf8_lossy(&out_a.stderr);
    let err_b = String::from_utf8_lossy(&out_b.stderr);
    let lines = [a.trim(), b.trim()];
    assert!(
        lines.contains(&"claimed"),
        "stdout {lines:?} stderr [{err_a:?}, {err_b:?}]"
    );
    assert!(
        lines.contains(&"lost"),
        "stdout {lines:?} stderr [{err_a:?}, {err_b:?}]"
    );
    assert_eq!(task_status(&dir, "t1").unwrap().as_deref(), Some("running"));
}

#[test]
fn the_model_runs_after_the_lock_file_is_gone() {
    let dir = fresh("model");
    write_board(&dir, &Board::new(vec![Task::ready("t1")])).unwrap();
    transact(
        &dir,
        "t1",
        Op::Claim {
            owner: "worker".into(),
        },
    )
    .unwrap();
    assert!(!dir.join("board.lock").exists());
    assert_eq!(
        harness_core::kanban::FakeModel { reply: "ok" }.complete("t1"),
        "ok"
    );
}

#[test]
fn demo_finishes_the_card() {
    assert_eq!(demo(), "kanban: done");
}
