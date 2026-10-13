use harness_core::canonical_json;

fn main() {
    let raw = r#"{"b":1,"a":2}"#;
    let canonical = canonical_json(raw).expect("json");
    println!("raw: {raw}");
    println!("canonical: {canonical}");
    let again = canonical_json(raw).expect("json");
    println!(
        "byte-identical: {}",
        canonical.as_bytes() == again.as_bytes()
    );
}
