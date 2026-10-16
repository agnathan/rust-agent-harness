use streaming::{buffer_join, demo_reply, fold_text, Assembler, Delta, ThinkScrubber};

#[test]
fn a_closed_think_block_is_removed_and_a_split_tag_does_not_leak() {
    let mut scrubber = ThinkScrubber::new();
    let mut out = scrubber.feed("<think>secret</think>hello");
    out.push_str(&scrubber.flush());
    assert_eq!(out, "hello");

    let mut scrubber = ThinkScrubber::new();
    let mut out = String::new();
    out.push_str(&scrubber.feed("<thi"));
    out.push_str(&scrubber.feed("nk>secret</think>ok"));
    out.push_str(&scrubber.flush());
    assert_eq!(out, "ok");
    assert!(!out.contains("secret"));
}

#[test]
fn cjk_tags_are_stripped_and_an_inline_mention_is_kept_until_it_closes() {
    let mut scrubber = ThinkScrubber::new();
    let mut out = scrubber.feed("<思考>秘密</思考>ok");
    out.push_str(&scrubber.flush());
    assert_eq!(out, "ok");

    let mut scrubber = ThinkScrubber::new();
    let mut out = scrubber.feed("see <think> later");
    out.push_str(&scrubber.flush());
    assert_eq!(out, "see <think> later");

    let mut scrubber = ThinkScrubber::new();
    let _ = scrubber.feed("\n<think>secret");
    assert_eq!(scrubber.flush(), "");
}

#[test]
fn a_read_starts_during_the_stream_and_a_write_waits() {
    let mut assembler = Assembler::new();
    assembler.push(Delta::Text("hi".into()));
    assembler.push(Delta::Tool {
        name: "echo".into(),
        read_only: true,
    });
    assembler.push(Delta::Tool {
        name: "write".into(),
        read_only: false,
    });
    assert_eq!(assembler.started, vec!["echo"]);
    assert_eq!(assembler.finish(), "hi");
    assert_eq!(assembler.started, vec!["echo", "write"]);
}

#[test]
fn the_demo_drops_the_split_think_block() {
    let (reply, started) = demo_reply();
    assert_eq!(reply, "hello");
    assert_eq!(started, vec!["echo"]);
}

#[test]
fn folding_plain_chunks_matches_a_buffer() {
    let chunks: Vec<String> = (0..100).map(|_| "hello ".into()).collect();
    assert_eq!(fold_text(&chunks), buffer_join(&chunks));
}
