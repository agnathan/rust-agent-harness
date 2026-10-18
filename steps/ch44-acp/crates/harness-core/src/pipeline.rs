//! Book chapter 28. Four stages run in order. Each one receives the same
//! `Arc`. The transcript is not cloned between stages.

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub text: String,
}

pub struct Context {
    pub messages: Vec<Message>,
    pub notes: Mutex<Vec<String>>,
}

pub struct Stage {
    pub name: &'static str,
    pub reply: String,
}

pub const STAGE_NAMES: [&str; 4] = ["outline", "draft", "critique", "revise"];

pub fn run(stages: &[Stage], context: Arc<Context>) -> Arc<Context> {
    for stage in stages {
        context
            .notes
            .lock()
            .expect("notes")
            .push(format!("{}: {}", stage.name, stage.reply));
    }
    context
}

/// What the next stage is given. The pointer is the same allocation.
pub fn hand_off(context: Arc<Context>) -> Arc<Context> {
    context
}

pub fn demo() -> String {
    let context = Arc::new(Context {
        messages: vec![Message {
            role: "user".into(),
            text: "Write a note on rename.".into(),
        }],
        notes: Mutex::new(Vec::new()),
    });
    let stages = [
        Stage {
            name: "outline",
            reply: "three steps".into(),
        },
        Stage {
            name: "draft",
            reply: "the draft".into(),
        },
        Stage {
            name: "critique",
            reply: "shorter".into(),
        },
        Stage {
            name: "revise",
            reply: "the revision".into(),
        },
    ];
    let out = run(&stages, Arc::clone(&context));
    assert!(Arc::ptr_eq(&context, &out));
    let last = out
        .notes
        .lock()
        .expect("notes")
        .last()
        .cloned()
        .unwrap_or_default();
    last
}
