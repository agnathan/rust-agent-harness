use harness::l02_message::*;
use harness::l03_ownership::*;
fn msgs() -> Vec<Message> {
    vec![
        Message::system("sys"),
        Message::user("one"),
        Message::assistant("three"),
        Message::user("xx"),
        Message::assistant("fives"),
    ]
}
#[test]
fn chars() {
    assert_eq!(total_chars(&msgs()), 3 + 3 + 5 + 2 + 5);
}
#[test]
fn trim_keeps_system() {
    let mut m = msgs();
    trim_history(&mut m, 2);
    assert_eq!(
        m.iter().map(|m| m.content.as_str()).collect::<Vec<_>>(),
        vec!["sys", "xx", "fives"]
    );
    let mut short = msgs();
    trim_history(&mut short, 10);
    assert_eq!(short.len(), 5);
    let mut nosys = vec![Message::user("a"), Message::user("b"), Message::user("c")];
    trim_history(&mut nosys, 1);
    assert_eq!(nosys[0].content, "c");
    assert_eq!(nosys.len(), 1);
}
#[test]
fn transcript() {
    let mut c = Conversation::new("sys");
    c.push(Message::user("hi"));
    assert_eq!(into_transcript(c), "system: sys\nuser: hi");
}
#[test]
fn longest() {
    let m = msgs();
    assert_eq!(longest_message(&m).unwrap().content, "three");
    assert!(longest_message(&[]).is_none());
}
