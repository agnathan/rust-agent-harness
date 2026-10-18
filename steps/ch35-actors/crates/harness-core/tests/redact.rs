use harness_core::{redact_for_egress, redact_sensitive_text, redacting_formatter};

#[test]
fn stored_text_drops_the_key_and_egress_drops_a_bare_bearer() {
    let scrubbed = redact_sensitive_text(
        "openai sk-abcdefghijklmnopqrstuvwxyz and OPENAI_API_KEY=super-secret-value",
    );
    assert!(scrubbed.contains("sk-***"), "{scrubbed}");
    assert!(!scrubbed.contains("super-secret-value"), "{scrubbed}");
    assert!(scrubbed.contains("OPENAI_API_KEY=***"), "{scrubbed}");
    let logged = redacting_formatter("Authorization: Bearer abcdefghijklmnopqrstuvwxyz012345");
    assert!(logged.contains("Authorization: ***"), "{logged}");
    assert!(
        !logged.contains("abcdefghijklmnopqrstuvwxyz012345"),
        "{logged}"
    );
    let egress = redact_for_egress("see Bearer abcdefghijklmnopqrstuvwxyz012345 now");
    assert!(egress.contains("Bearer [redacted]"), "{egress}");
    assert!(
        !egress.contains("abcdefghijklmnopqrstuvwxyz012345"),
        "{egress}"
    );
}
