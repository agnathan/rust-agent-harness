use harness_core::{generate, mention_count, schema_cost};

fn main() {
    for count in [50, 1_000, 10_000] {
        let cards = generate(count);
        let cost = schema_cost(&cards);
        let mentions = mention_count(&cards, "rename a file");
        println!(
            "tools={count} bytes={} tokens={} mentions_rename_file={mentions}",
            cost.bytes, cost.tokens
        );
    }
}
