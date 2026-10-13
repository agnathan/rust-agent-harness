use harness_core::milestone;

fn main() {
    let report = milestone(2_000, "rename a file");
    println!("tools={}", report.tools);
    println!("all_schema_tokens={}", report.all_tokens);
    println!("visible_tokens_before={}", report.visible_before);
    println!("this_turn={}", report.this_turn.join(","));
    println!("search:\n{}", report.listing);
    println!("next_turn={}", report.next_turn.join(","));
    println!("visible_tokens_after={}", report.visible_after);
    println!("core_prompt_unchanged={}", report.core_same);
}
