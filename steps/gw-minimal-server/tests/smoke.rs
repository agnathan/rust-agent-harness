use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use gw_minimal::{CallGate, Gateway, GatewayError, ACCEPT_BOUND, EXECUTE_BOUND};
use serde_json::json;

#[test]
fn a_new_gateway_lists_nothing() {
    let gateway = Gateway::new();
    assert!(gateway.is_empty());
    let view = gateway.session_tools("session-1");
    assert_eq!(view.session_id, "session-1");
    assert!(view.tools.is_empty());
}

#[tokio::test]
async fn echo_returns_text_and_a_duplicate_is_refused() {
    let mut gateway = Gateway::new();
    gateway
        .register("echo", "Return the text argument.")
        .unwrap();
    let err = gateway.register("echo", "again").unwrap_err();
    assert_eq!(
        err,
        GatewayError::Duplicate {
            name: "echo".into()
        }
    );
    let text = gateway
        .call("echo", &json!({"text": "hello"}))
        .await
        .unwrap();
    assert_eq!(text, "hello");
    let missing = gateway.call("nope", &json!({})).await.unwrap_err();
    assert_eq!(
        missing,
        GatewayError::Unknown {
            name: "nope".into()
        }
    );
}

#[tokio::test]
async fn thirty_two_are_accepted_and_eight_run() {
    let gate = CallGate::new(ACCEPT_BOUND, EXECUTE_BOUND);
    let running = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let mut tasks = Vec::new();
    for _ in 0..ACCEPT_BOUND {
        let gate = gate.clone();
        let running = Arc::clone(&running);
        let peak = Arc::clone(&peak);
        tasks.push(tokio::spawn(async move {
            gate.run(|| async move {
                let now = running.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(30)).await;
                running.fetch_sub(1, Ordering::SeqCst);
            })
            .await
            .unwrap();
        }));
    }
    tokio::time::sleep(Duration::from_millis(5)).await;
    let refused = gate.run(|| async {}).await.expect_err("33rd call");
    assert_eq!(
        refused,
        GatewayError::Saturated {
            bound: ACCEPT_BOUND
        }
    );
    for task in tasks {
        task.await.unwrap();
    }
    assert!(peak.load(Ordering::SeqCst) <= EXECUTE_BOUND);
    assert!(peak.load(Ordering::SeqCst) >= 2);
}
