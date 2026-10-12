use harness::l01_hello::*;
#[test]
fn banner() {
    assert_eq!(agent_banner("hermes", 3), "hermes v3 ready");
}
#[test]
fn words() {
    assert_eq!(count_words("  read the   file now "), 4);
    assert_eq!(count_words(""), 0);
}
#[test]
fn slash() {
    assert!(is_slash_command("  /stop"));
    assert!(!is_slash_command("hello /stop"));
}
