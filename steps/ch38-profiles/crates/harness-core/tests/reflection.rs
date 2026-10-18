use std::time::{Duration, Instant};

use harness_core::reflection::{
    execute, spawn_reviewer, Loop, ReviewError, ReviewRun, ScriptedModel, MEMORY_NUDGE_INTERVAL,
};

#[test]
fn the_tenth_user_turn_forks_a_reviewer_with_memory() {
    let mut loop_ = Loop::new();
    for _ in 0..(MEMORY_NUDGE_INTERVAL - 1) {
        loop_.note_user_turn();
    }
    assert!(loop_.finalize().is_none());
    loop_.note_user_turn();
    let fork = loop_.finalize().expect("fork");
    assert!(fork.review_memory);
    assert!(fork.tools.contains(&"memory"));
    assert!(fork.tools.contains(&"skill_view"));
    assert_eq!(fork.memory_nudge_interval, 0);
    assert_eq!(fork.skill_nudge_interval, 0);
    assert!(loop_.finalize().is_none());
}

#[test]
fn a_skill_nudge_does_not_hand_the_reviewer_memory() {
    let mut loop_ = Loop::new();
    for _ in 0..10 {
        loop_.note_tool_iter();
    }
    let fork = loop_.finalize().expect("fork");
    assert!(!fork.review_memory);
    assert!(!fork.tools.contains(&"memory"));
    assert!(fork.tools.contains(&"skill_manage"));
}

#[test]
fn the_reviewer_cannot_call_terminal_and_can_write_a_note() {
    let fork = spawn_reviewer(true);
    let model = ScriptedModel::new(["terminal ls", "memory prefer short notes"]);
    let mut notes = Vec::new();
    let refused = execute(&fork, &model, &mut notes, None);
    assert!(matches!(refused, Err(ReviewError::NotAllowed(tool)) if tool == "terminal"));
    assert!(notes.is_empty());
    assert_eq!(model.calls(), 1);

    let model = ScriptedModel::new(["memory prefer short notes"]);
    let done = execute(&fork, &model, &mut notes, None).expect("memory");
    assert_eq!(done, ["prefer short notes"]);
    assert_eq!(notes, ["prefer short notes"]);
}

#[test]
fn cancel_before_the_next_tool_stops_the_review() {
    let fork = spawn_reviewer(true);
    fork.cancel();
    let model = ScriptedModel::new(["memory should not land"]);
    let mut notes = Vec::new();
    let stopped = execute(&fork, &model, &mut notes, None);
    assert!(matches!(stopped, Err(ReviewError::Cancelled)));
    assert!(notes.is_empty());
}

#[test]
fn cancelling_a_running_review_returns_quickly() {
    let run = ReviewRun::spawn_spinner();
    let started = Instant::now();
    run.cancel_and_join();
    assert!(started.elapsed() < Duration::from_millis(50));
}

#[test]
fn the_demo_note_is_the_scripted_line() {
    assert_eq!(harness_core::reflection::demo(), "prefer short notes");
}
