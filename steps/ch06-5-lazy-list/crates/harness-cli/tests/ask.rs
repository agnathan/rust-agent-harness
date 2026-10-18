use std::process::Command;

use wiremock::matchers::{body_string_contains, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn mock_ask_prints_the_tool_demo() {
    let output = Command::new(env!("CARGO_BIN_EXE_ch06-5-harness"))
        .args(["--mock", "--tools", "200", "ask", "rename a file"])
        .output()
        .expect("run cli");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("tools=200"), "{stdout}");
    assert!(stdout.contains("this_turn=search_tools"), "{stdout}");
    assert!(stdout.contains("core_prompt_unchanged=true"), "{stdout}");
    assert!(stdout.contains("event: tool-call search_tools"), "{stdout}");
    assert!(stdout.contains("event: done"), "{stdout}");
    assert!(!stdout.contains("event: usage"), "{stdout}");
}

#[test]
fn missing_openrouter_key_is_a_clear_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_ch06-5-harness"))
        .args(["ask", "rename a file"])
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENAI_API_KEY")
        .env_remove("OPENAI_BASE_URL")
        .output()
        .expect("run cli");
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("event: error"), "{stderr}");
    assert!(stderr.contains("OPENROUTER_API_KEY"), "{stderr}");
}

#[tokio::test]
async fn live_ask_sends_only_the_search_tool() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(body_string_contains("search_tools"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"model":"openai/gpt-4o-mini","choices":[{"message":{"role":"assistant","content":"from-fake"}}],"usage":{"prompt_tokens":3,"completion_tokens":1,"total_tokens":4,"cost":0.25}}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let base = format!("{}/v1", server.uri());
    let output = Command::new(env!("CARGO_BIN_EXE_ch06-5-harness"))
        .args([
            "--base-url",
            &base,
            "--api-key",
            "test",
            "ask",
            "rename a file",
        ])
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
    assert!(stdout.contains("cost: 0.25"), "{stdout}");
    assert!(
        stdout.contains("event: usage prompt=3 completion=1 total=4 cost=0.25"),
        "{stdout}"
    );

    let requests = server.received_requests().await.expect("requests");
    let body = String::from_utf8(requests[0].body.clone()).unwrap();
    assert!(body.contains("search_tools"), "{body}");
    assert!(!body.contains("files_list_file_0"), "{body}");
}
