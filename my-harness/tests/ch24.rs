use my_harness::chapter_24_index_changes_when_viewed;

#[test]
fn loading_skills_does_not_change_the_index_bytes_when_one_is_viewed() {
    let index = "<available_skills>\nrename\n</available_skills>";
    assert!(!chapter_24_index_changes_when_viewed(index, index));
}
