use std::process::Command;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn mock_ask_prints_the_canned_reply() {
    let output = Command::new(env!("CARGO_BIN_EXE_ch01-4-harness"))
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
}

#[tokio::test]
async fn ask_talks_to_a_local_fake_server() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"model":"openai/gpt-4o-mini","choices":[{"message":{"role":"assistant","content":"from-fake"}}],"usage":{"prompt_tokens":3,"completion_tokens":1,"total_tokens":4}}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let base = format!("{}/v1", server.uri());
    let output = Command::new(env!("CARGO_BIN_EXE_ch01-4-harness"))
        .args(["--base-url", &base, "--api-key", "test", "ask", "ping"])
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENAI_API_KEY")
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
}

#[test]
fn missing_openrouter_key_is_a_clear_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_ch01-4-harness"))
        .args(["ask", "ping"])
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENAI_API_KEY")
        .env_remove("OPENAI_BASE_URL")
        .output()
        .expect("run cli");
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("OPENROUTER_API_KEY"), "{stderr}");
}
