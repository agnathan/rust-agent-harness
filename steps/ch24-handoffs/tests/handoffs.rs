use handoffs::{filter_transcript, Board, HandoffError, Message, MAX_HANDOFFS};

fn message(role: &str, text: &str) -> Message {
    Message {
        role: role.into(),
        text: text.into(),
    }
}

#[test]
fn scratch_is_dropped_and_the_fifth_handoff_is_refused() {
    let board = Board::new("outline");
    let messages = vec![
        message("user", "Write a note."),
        message("scratch", "draft junk"),
        message("assistant", "three steps"),
    ];
    let filtered = board.handoff("draft", &messages).unwrap();
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|message| message.role != "scratch"));
    assert_eq!(board.owner(), "draft");
    for index in 1..MAX_HANDOFFS {
        board.handoff(&format!("agent-{index}"), &messages).unwrap();
    }
    assert_eq!(board.count(), MAX_HANDOFFS);
    assert!(matches!(
        board.handoff("one-more", &messages),
        Err(HandoffError::Limit)
    ));
    assert_eq!(board.owner(), "agent-3");
}

#[test]
fn an_empty_filtered_transcript_does_not_move_ownership() {
    let board = Board::new("outline");
    let only_scratch = vec![message("scratch", "nope")];
    assert!(matches!(
        board.handoff("draft", &only_scratch),
        Err(HandoffError::Empty)
    ));
    assert_eq!(board.owner(), "outline");
    assert_eq!(board.count(), 0);
}

#[test]
fn filter_keeps_order() {
    let messages = vec![
        message("user", "a"),
        message("scratch", "b"),
        message("assistant", "c"),
    ];
    let filtered = filter_transcript(&messages);
    assert_eq!(filtered[0].text, "a");
    assert_eq!(filtered[1].text, "c");
}

#[test]
fn the_demo_shows_the_next_owner() {
    assert_eq!(handoffs::demo(), "draft sees 2");
}
