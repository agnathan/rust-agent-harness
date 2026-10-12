use std::process::Command;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn mock_ask_prints_the_canned_reply() {
    let output = Command::new(env!("CARGO_BIN_EXE_ch01-harness"))
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
            r#"{"choices":[{"message":{"role":"assistant","content":"from-fake"}}]}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let base = format!("{}/v1", server.uri());
    let output = Command::new(env!("CARGO_BIN_EXE_ch01-harness"))
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
}
