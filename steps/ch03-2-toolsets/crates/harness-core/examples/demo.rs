use harness_core::{Catalog, EnvMap, StaticTool, ToolKind, Toolset};

/// Two toolsets. The notes set is gated off, so the model would not be told about it.
fn main() {
    let mut catalog = Catalog::new();
    catalog.add(Toolset::gated(
        "time",
        "HARNESS_TIME",
        vec![StaticTool::arc("clock", ToolKind::Read)],
    ));
    catalog.add(
        Toolset::gated(
            "notes",
            "HARNESS_NOTES",
            vec![StaticTool::arc("note", ToolKind::Write)],
        )
        .with_check(|value| value == "1"),
    );
    let mut env = EnvMap::new();
    env.insert("HARNESS_TIME", "1");
    let schemas = catalog.available_schemas(&env).expect("schemas");
    println!("available: {}", schemas.len());
    for schema in &schemas {
        println!("tool: {} — {}", schema.name, schema.description);
    }
    println!("omitted: note (HARNESS_NOTES is unset)");
}
