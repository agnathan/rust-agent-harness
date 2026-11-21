use std::sync::Arc;
use std::time::{Duration, Instant};

use harness_core::{
    check_gate, eight_delay_calls, fixture_binary, register_remote, run_loop, run_saved_evals,
    AgentSession, Budget, CancelFlag, DelayRead, Dispatcher, EchoTool, McpSession, Provider,
    ScriptedProvider, ScriptedReply, StopReason, Tool, ToolRegistry, DISPATCH_BOUND,
};
use serde_json::json;

fn echo_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    registry
}

#[tokio::test]
async fn a_tool_call_then_a_final_answer() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let registry = echo_registry();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let mut session = AgentSession::new();
    session.inbox.submit("say hello").unwrap();
    let outcome = session
        .turn_from_inbox(
            &provider,
            &registry,
            &dispatcher,
            &Budget::default(),
            &CancelFlag::new(),
        )
        .await
        .unwrap();
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(provider.calls(), 2);
    let tool = session
        .messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert_eq!(tool.content, "hello");
    assert_eq!(session.messages.last().unwrap().content, "hello");
}

#[tokio::test]
async fn cancel_before_the_next_model_call() {
    let dispatcher = Dispatcher::default();
    let cancel = CancelFlag::new();
    let hooked = cancel.clone();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(CancelOnCall { flag: hooked }) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "cancel_on_call", json!({})),
        ScriptedReply::text("should not run"),
    ]);
    let mut messages = vec![harness_core::Message::user("stop")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &Budget::default(),
        &cancel,
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Cancelled);
    assert_eq!(provider.calls(), 1);
}

struct CancelOnCall {
    flag: CancelFlag,
}

#[async_trait::async_trait]
impl Tool for CancelOnCall {
    fn schema(&self) -> harness_core::ToolSchema {
        harness_core::ToolSchema {
            name: "cancel_on_call".into(),
            description: "Sets the cancellation flag.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> harness_core::ToolKind {
        harness_core::ToolKind::Read
    }

    async fn call(
        &self,
        _arguments: serde_json::Value,
    ) -> Result<harness_core::ToolResult, harness_core::ToolError> {
        self.flag.cancel();
        Ok(harness_core::ToolResult::text("cancelled"))
    }
}

#[tokio::test]
async fn a_token_budget_stops_before_the_next_call() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "partial"})).with_usage(
            harness_core::Usage {
                prompt_tokens: 40,
                completion_tokens: 60,
                total_tokens: 100,
                cost: Some(0.25),
            },
        ),
        ScriptedReply::text("budget answer"),
    ]);
    let registry = echo_registry();
    let dispatcher = Dispatcher::default();
    let budget = Budget {
        max_tokens: Some(50),
        ..Budget::default()
    };
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &budget,
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Budget);
    assert_eq!(outcome.label(), "Budget");
    assert_eq!(outcome.text(), "budget answer");
    assert_eq!(provider.calls(), 2);
    assert_eq!(outcome.usage().total_tokens, 100);
    assert_eq!(outcome.usage().cost_usd, 0.25);
}

#[tokio::test]
async fn a_wrap_up_that_asks_for_tools_returns_partial() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "partial"})).with_usage(
            harness_core::Usage {
                prompt_tokens: 40,
                completion_tokens: 60,
                total_tokens: 100,
                cost: Some(0.25),
            },
        ),
        ScriptedReply::tool("call-2", "echo", json!({"text": "again"})),
    ]);
    let registry = echo_registry();
    let budget = Budget {
        max_tokens: Some(50),
        ..Budget::default()
    };
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &budget,
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Partial);
    assert_eq!(outcome.label(), "Partial");
    assert_eq!(provider.calls(), 2);
    assert_eq!(
        messages
            .iter()
            .filter(|message| message.tool_call_id.is_some())
            .count(),
        1
    );
}

#[tokio::test]
async fn a_cost_budget_and_a_time_budget_stop_the_same_way() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "partial"})).with_usage(
            harness_core::Usage {
                prompt_tokens: 1,
                completion_tokens: 1,
                total_tokens: 2,
                cost: Some(0.25),
            },
        ),
        ScriptedReply::text("should not run"),
    ]);
    let registry = echo_registry();
    let dispatcher = Dispatcher::default();
    let budget = Budget {
        max_cost_usd: Some(0.1),
        ..Budget::default()
    };
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &budget,
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Budget);
    assert_eq!(provider.calls(), 2);
    assert_eq!(outcome.text(), "should not run");

    let provider = ScriptedProvider::new(vec![ScriptedReply::text("should not run")]);
    let budget = Budget {
        max_duration: Some(Duration::from_millis(0)),
        ..Budget::default()
    };
    let started = Instant::now() - Duration::from_millis(5);
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &budget,
        &CancelFlag::new(),
        started,
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Budget);
    assert_eq!(provider.calls(), 0);
}

#[tokio::test]
async fn eight_reads_in_one_turn_stay_inside_the_bound() {
    let delay = DelayRead::new(Duration::from_millis(30));
    let peak = Arc::clone(&delay.peak);
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(delay) as Arc<dyn Tool>).unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tools(eight_delay_calls()),
        ScriptedReply::text("done"),
    ]);
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let started = Instant::now();
    let mut messages = vec![harness_core::Message::user("fan out")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    let elapsed = started.elapsed();
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert!(peak.load(std::sync::atomic::Ordering::SeqCst) <= DISPATCH_BOUND);
    assert!(
        peak.load(std::sync::atomic::Ordering::SeqCst) > 1,
        "reads should overlap"
    );
    assert!(
        elapsed < Duration::from_millis(200),
        "elapsed {elapsed:?} should be about one 30 ms wave, not eight"
    );
}

#[tokio::test]
async fn the_saved_eval_set_passes_on_the_mock() {
    let scores = run_saved_evals().await;
    assert!(scores.iter().all(|score| score.passed), "{scores:?}");
    assert_eq!(scores.len(), 3);
}

#[tokio::test]
async fn an_mcp_echo_uses_the_same_loop() {
    let session = Arc::new(McpSession::stdio(fixture_binary()).await.expect("fixture"));
    let mut registry = ToolRegistry::new();
    register_remote(&mut registry, session)
        .await
        .expect("register");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let mut messages = vec![harness_core::Message::user("say hello")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert_eq!(tool.content, "hello");
}

#[tokio::test]
async fn an_http_mcp_echo_uses_the_same_loop() {
    let fixture = harness_core::HttpFixture::spawn(fixture_binary())
        .await
        .expect("http");
    let session = Arc::new(
        McpSession::streamable_http(&fixture.url)
            .await
            .expect("connect"),
    );
    let mut registry = ToolRegistry::new();
    register_remote(&mut registry, session)
        .await
        .expect("register");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let mut messages = vec![harness_core::Message::user("say hello")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert_eq!(tool.content, "hello");
}

#[test]
fn the_gate_is_a_plain_check() {
    let cancel = CancelFlag::new();
    let budget = Budget::default();
    let usage = harness_core::AgentUsage::default();
    assert_eq!(check_gate(&cancel, &budget, &usage, Instant::now()), None);
    cancel.cancel();
    assert_eq!(
        check_gate(&cancel, &budget, &usage, Instant::now()),
        Some(StopReason::Cancelled)
    );
}
