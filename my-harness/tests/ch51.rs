use my_harness::chapter_51_arm_succeeded;

#[test]
fn each_arm_reports_success() {
    assert!(chapter_51_arm_succeeded("every"));
    assert!(chapter_51_arm_succeeded("static"));
    assert!(chapter_51_arm_succeeded("dynamic"));
}
