use harness_core::profiles::{
    demo, FakeModel, Profile, Profiles, ResolveError, UnscopedSecretError,
};

#[test]
fn multiplex_without_scope_fails_closed() {
    let mut store = Profiles::new(true);
    store.add(Profile::new(
        "writer",
        "you write",
        [("API_KEY", "sk-writer")],
    ));
    store.set_process("API_KEY", "sk-process");
    let err = store.get_secret("API_KEY").unwrap_err();
    assert_eq!(
        err,
        UnscopedSecretError {
            name: "API_KEY".to_string()
        }
    );
    assert!(matches!(
        store.resolve("writer", "API_KEY"),
        Err(ResolveError::Unscoped(_))
    ));
}

#[test]
fn scope_reads_only_that_profile() {
    let mut store = Profiles::new(true);
    store.add(Profile::new(
        "writer",
        "you write",
        [("API_KEY", "sk-writer")],
    ));
    store.add(Profile::new(
        "reader",
        "you read",
        [("API_KEY", "sk-reader")],
    ));
    store.set_process("API_KEY", "sk-process");
    store.set_scope(Some("writer"));
    assert_eq!(
        store.get_secret("API_KEY").unwrap().as_deref(),
        Some("sk-writer")
    );
    store.set_scope(Some("reader"));
    assert_eq!(
        store.get_secret("API_KEY").unwrap().as_deref(),
        Some("sk-reader")
    );
    assert_eq!(store.get_secret("MISSING").unwrap(), None);
}

#[test]
fn prompt_does_not_carry_the_secret() {
    let mut store = Profiles::new(true);
    store.add(Profile::new(
        "writer",
        "you write",
        [("API_KEY", "sk-writer")],
    ));
    store.set_scope(Some("writer"));
    let resolved = store.resolve("writer", "API_KEY").unwrap();
    assert!(resolved.has_secret);
    assert!(!resolved.prompt.contains("sk-writer"));
    assert!(!store.secret_leaked_into_prompt("writer"));
    let reply = FakeModel { reply: "ok" }.complete(&resolved.prompt);
    assert_eq!(reply, "ok");
    assert!(!resolved.prompt.contains("sk-"));
}

#[test]
fn inactive_multiplex_reads_the_process_map() {
    let mut store = Profiles::new(false);
    store.set_process("API_KEY", "sk-process");
    assert_eq!(
        store.get_secret("API_KEY").unwrap().as_deref(),
        Some("sk-process")
    );
    store.add(Profile::new(
        "writer",
        "you write",
        [("API_KEY", "sk-writer")],
    ));
    store.set_scope(Some("writer"));
    assert_eq!(
        store.get_secret("API_KEY").unwrap().as_deref(),
        Some("sk-writer")
    );
    store.set_scope(None);
    assert_eq!(
        store.get_secret("API_KEY").unwrap().as_deref(),
        Some("sk-process")
    );
}

#[test]
fn single_profile_scope_falls_through_on_a_miss() {
    let mut store = Profiles::new(false);
    store.add(Profile::new(
        "writer",
        "you write",
        [("API_KEY", "sk-writer")],
    ));
    store.set_process("OTHER", "from-process");
    store.set_scope(Some("writer"));
    assert_eq!(
        store.get_secret("OTHER").unwrap().as_deref(),
        Some("from-process")
    );
}

#[test]
fn demo_names_the_profile() {
    assert_eq!(demo(), "profile: writer");
}
