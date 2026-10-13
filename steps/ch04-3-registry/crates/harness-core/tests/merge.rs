use std::sync::Arc;
use std::time::{Duration, Instant};

use harness_core::{
    fixture_binary, register_remote, Dispatcher, EchoTool, McpSession, StaticTool, Tool, ToolCall,
    ToolKind, ToolRegistry, DISPATCH_BOUND,
};
use serde_json::json;

#[tokio::test]
async fn remote_tools_share_the_registry_with_a_native_tool() {
    let session = Arc::new(McpSession::stdio(fixture_binary()).await.expect("connect"));
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .expect("native echo");
    // The fixture also has echo. That duplicate is the error we want to see
    // before swapping the native name.
    let error = register_remote(&mut registry, Arc::clone(&session))
        .await
        .expect_err("duplicate echo");
    assert!(error.to_string().contains("duplicate"), "{error}");

    let mut registry = ToolRegistry::new();
    registry
        .register(StaticTool::arc("ping", ToolKind::Read))
        .unwrap();
    register_remote(&mut registry, Arc::clone(&session))
        .await
        .expect("register");
    assert!(registry.get("ping").is_some());
    assert_eq!(registry.get("echo").unwrap().kind(), ToolKind::Read);
    assert_eq!(registry.get("note").unwrap().kind(), ToolKind::Write);
    let text = registry
        .get("echo")
        .unwrap()
        .call(json!({"text": "from-registry"}))
        .await
        .expect("call")
        .content[0]
        .text
        .clone();
    assert_eq!(text, "from-registry");
}

#[tokio::test]
async fn eight_remote_reads_overlap_and_a_write_does_not() {
    let session = Arc::new(
        McpSession::stdio_with(fixture_binary(), &[("FIXTURE_DELAY_MS", "40")])
            .await
            .expect("connect"),
    );
    let mut registry = ToolRegistry::new();
    register_remote(&mut registry, session)
        .await
        .expect("register");
    let reads: Vec<_> = (0..8)
        .map(|_| ToolCall {
            name: "echo".into(),
            arguments: json!({"text": "x"}),
        })
        .collect();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let started = Instant::now();
    let outputs = dispatcher.run(&registry, &reads).await;
    let concurrent = started.elapsed();
    assert!(outputs.iter().all(|output| output.result.is_ok()));
    let started = Instant::now();
    let _ = dispatcher.run_sequential(&registry, &reads).await;
    let sequential = started.elapsed();
    assert!(
        concurrent < Duration::from_millis(200),
        "concurrent {concurrent:?} should be about one 40 ms call"
    );
    assert!(
        sequential > Duration::from_millis(250),
        "sequential {sequential:?} should be about eight 40 ms calls"
    );

    let mixed = vec![
        ToolCall {
            name: "echo".into(),
            arguments: json!({"text": "before"}),
        },
        ToolCall {
            name: "note".into(),
            arguments: json!({"text": "n"}),
        },
        ToolCall {
            name: "echo".into(),
            arguments: json!({"text": "after"}),
        },
    ];
    let outputs = Dispatcher::new(DISPATCH_BOUND).run(&registry, &mixed).await;
    assert_eq!(
        outputs[0].result.as_ref().unwrap().content[0].text,
        "before"
    );
    assert_eq!(
        outputs[1].result.as_ref().unwrap().content[0].text,
        "noted:n"
    );
    assert_eq!(outputs[2].result.as_ref().unwrap().content[0].text, "after");
}
