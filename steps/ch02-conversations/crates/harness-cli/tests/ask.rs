use std::process::Command;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn harness() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch02-harness"));
    command
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENAI_API_KEY")
        .env_remove("OPENAI_BASE_URL")
        .env_remove("HARNESS_MODEL");
    command
}

#[test]
fn mock_ask_prints_events_and_no_invented_usage() {
    let output = harness()
        .args(["--mock", "ask", "Say hello"])
        .output()
        .expect("run cli");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("(mock) I heard you."), "{stdout}");
    assert!(
        stdout.contains("event: text-delta (mock) I heard you."),
        "{stdout}"
    );
    assert!(stdout.contains("event: done"), "{stdout}");
    assert!(!stdout.contains("event: usage"), "{stdout}");
    assert!(!stdout.contains("cost:"), "{stdout}");
}

#[test]
fn missing_openrouter_key_is_a_clear_error() {
    let output = harness().args(["ask", "ping"]).output().expect("run cli");
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("OPENROUTER_API_KEY"), "{stderr}");
    assert!(stderr.contains("event: error"), "{stderr}");
}

#[tokio::test]
async fn ask_prints_scripted_cost_from_a_local_fake() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"model":"openai/gpt-4o-mini","choices":[{"message":{"role":"assistant","content":"from-fake"}}],"usage":{"prompt_tokens":3,"completion_tokens":1,"total_tokens":4,"cost":0.25}}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let base = format!("{}/v1", server.uri());
    let output = harness()
        .args(["--base-url", &base, "--api-key", "test", "ask", "ping"])
        .output()
        .expect("run cli");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("from-fake"), "{stdout}");
    assert!(stdout.contains("model: openai/gpt-4o-mini"), "{stdout}");
    assert!(
        stdout.contains("tokens: prompt=3 completion=1 total=4"),
        "{stdout}"
    );
    assert!(stdout.contains("cost: 0.25"), "{stdout}");
    assert!(
        stdout.contains("agent: calls=1 prompt=3 completion=1 total=4 cost=0.25"),
        "{stdout}"
    );
    assert!(
        stdout.contains("event: usage prompt=3 completion=1 total=4 cost=0.25"),
        "{stdout}"
    );
}
