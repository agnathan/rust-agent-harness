use std::sync::{Arc, Mutex};

use pipeline::{hand_off, run, Context, Message, Stage, STAGE_NAMES};

fn context(n: usize) -> Arc<Context> {
    Arc::new(Context {
        messages: (0..n)
            .map(|i| Message {
                role: "user".into(),
                text: format!("message {i}"),
            })
            .collect(),
        notes: Mutex::new(Vec::new()),
    })
}

#[test]
fn four_stages_share_the_arc_and_keep_message_order() {
    let context = context(3);
    let stages: Vec<Stage> = STAGE_NAMES
        .iter()
        .map(|name| Stage {
            name,
            reply: (*name).into(),
        })
        .collect();
    let out = run(&stages, Arc::clone(&context));
    assert!(Arc::ptr_eq(&context, &out));
    assert_eq!(out.messages.len(), 3);
    assert_eq!(out.messages[0].text, "message 0");
    let notes = out.notes.lock().unwrap();
    assert_eq!(
        notes.as_slice(),
        [
            "outline: outline",
            "draft: draft",
            "critique: critique",
            "revise: revise",
        ]
    );
}

#[test]
fn hand_off_is_the_same_pointer() {
    let context = context(100);
    let next = hand_off(Arc::clone(&context));
    assert!(Arc::ptr_eq(&context, &next));
    assert_eq!(next.messages.len(), 100);
}

#[test]
fn the_demo_ends_on_the_revision() {
    assert_eq!(pipeline::demo(), "revise: the revision");
}
