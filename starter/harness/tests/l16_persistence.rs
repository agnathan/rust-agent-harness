use harness::l02_message::*;
use harness::l16_persistence::*;
#[test]
fn roundtrip() {
    let s = Store::open_in_memory().unwrap();
    s.create_session("s1", "first").unwrap();
    s.create_session("s2", "second").unwrap();
    let a = s.append("s1", &Message::user("find the bug")).unwrap();
    let b = s
        .append("s1", &Message::assistant("bug found in parser"))
        .unwrap();
    s.append("s2", &Message::user("unrelated")).unwrap();
    assert!(b > a);
    let msgs = s.load("s1").unwrap();
    assert_eq!(
        msgs,
        vec![
            Message::user("find the bug"),
            Message::assistant("bug found in parser")
        ]
    );
    assert_eq!(
        s.search("bug").unwrap(),
        vec![
            ("s1".to_string(), "find the bug".to_string()),
            ("s1".into(), "bug found in parser".into())
        ]
    );
    assert!(s.load("nope").unwrap().is_empty());
}
#[test]
fn duplicate_session_errors() {
    let s = Store::open_in_memory().unwrap();
    s.create_session("s1", "x").unwrap();
    assert!(s.create_session("s1", "y").is_err());
}
#[test]
fn file_store_persists() {
    let p = std::env::temp_dir().join(format!("harness-test-{}.db", std::process::id()));
    let _ = std::fs::remove_file(&p);
    {
        let s = Store::open(p.to_str().unwrap()).unwrap();
        s.create_session("s", "t").unwrap();
        s.append("s", &Message::user("persist me")).unwrap();
    }
    let s = Store::open(p.to_str().unwrap()).unwrap();
    assert_eq!(s.load("s").unwrap()[0].content, "persist me");
    let _ = std::fs::remove_file(&p);
}
