use my_harness::chapter_12_plugin_decision;

#[test]
fn force_cannot_install_a_dangerous_plugin() {
    assert_eq!(chapter_12_plugin_decision(401, false), "confirm");
    assert_eq!(chapter_12_plugin_decision(1, true), "block");
}
