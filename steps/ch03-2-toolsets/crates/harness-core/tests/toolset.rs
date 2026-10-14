use harness_core::{Catalog, EnvMap, StaticTool, ToolKind, Toolset};

fn catalog() -> Catalog {
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
    catalog
}

#[test]
fn only_toolsets_that_pass_are_listed() {
    let catalog = catalog();
    let mut env = EnvMap::new();
    env.insert("HARNESS_TIME", "1");
    env.insert("HARNESS_NOTES", "0");
    let names: Vec<_> = catalog
        .available_schemas(&env)
        .unwrap()
        .into_iter()
        .map(|schema| schema.name)
        .collect();
    assert_eq!(names, ["clock"]);
}

#[test]
fn an_empty_env_var_hides_the_toolset() {
    let catalog = catalog();
    let mut env = EnvMap::new();
    env.insert("HARNESS_TIME", "");
    env.insert("HARNESS_NOTES", "1");
    let names: Vec<_> = catalog
        .available_schemas(&env)
        .unwrap()
        .into_iter()
        .map(|schema| schema.name)
        .collect();
    assert_eq!(names, ["note"]);
}

#[test]
fn both_sets_pass() {
    let catalog = catalog();
    let mut env = EnvMap::new();
    env.insert("HARNESS_TIME", "yes");
    env.insert("HARNESS_NOTES", "1");
    let registry = catalog.available(&env).unwrap();
    assert!(registry.get("clock").is_some());
    assert!(registry.get("note").is_some());
    assert_eq!(registry.len(), 2);
}

#[test]
fn a_duplicate_name_across_passing_sets_is_an_error() {
    let mut catalog = Catalog::new();
    catalog.add(Toolset::gated(
        "a",
        "HARNESS_A",
        vec![StaticTool::arc("echo", ToolKind::Read)],
    ));
    catalog.add(Toolset::gated(
        "b",
        "HARNESS_B",
        vec![StaticTool::arc("echo", ToolKind::Read)],
    ));
    let mut env = EnvMap::new();
    env.insert("HARNESS_A", "1");
    env.insert("HARNESS_B", "1");
    let error = match catalog.available(&env) {
        Err(error) => error,
        Ok(_) => panic!("expected a duplicate-name error"),
    };
    assert!(error.to_string().contains("duplicate"), "{error}");
}
