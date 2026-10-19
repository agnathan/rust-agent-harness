use my_harness::chapter_25_force_installs_dangerous;

#[test]
fn a_dangerous_community_skill_is_blocked_even_with_force() {
    assert!(!chapter_25_force_installs_dangerous());
}
