use my_harness::chapter_14_yolo_allows;

#[test]
fn yolo_still_refuses_the_runtime_and_the_checkout() {
    assert!(!chapter_14_yolo_allows("rm-the-running-binary"));
    assert!(!chapter_14_yolo_allows("git-reset-hard-of-this-checkout"));
    assert!(chapter_14_yolo_allows("echo hello"));
}
