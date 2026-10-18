use my_harness::chapter_32_critic_does_the_work;

#[test]
fn the_critic_reads_the_summary_and_does_not_do_the_work() {
    assert!(!chapter_32_critic_does_the_work());
}
