use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use harness_core::{
    drop_pending_steer, run_loop, Budget, CancelFlag, Dispatcher, Provider, ScriptedProvider,
    ScriptedReply, SessionInbox, StopReason, Tool, ToolError, ToolKind, ToolRegistry, ToolResult,
    ToolSchema,
};
use serde_json::{json, Value};

struct SlowTool;

#[async_trait]
impl Tool for SlowTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "slow".into(),
            description: "Sleeps until the token fires.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        tokio::time::sleep(Duration::from_secs(30)).await;
        Ok(ToolResult::text("too late"))
    }
}

struct SoftOnCall {
    flag: CancelFlag,
}

#[async_trait]
impl Tool for SoftOnCall {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "soft_on_call".into(),
            description: "Asks for a soft stop.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        self.flag.soft_stop();
        Ok(ToolResult::text("finished the call"))
    }
}

struct SlowProvider {
    inner: ScriptedProvider,
}

#[async_trait]
impl Provider for SlowProvider {
    async fn complete(&self, messages: &[harness_core::Message]) -> Vec<harness_core::Event> {
        tokio::time::sleep(Duration::from_secs(30)).await;
        self.inner.complete(messages).await
    }

    fn calls(&self) -> u64 {
        self.inner.calls()
    }
}

#[tokio::test]
async fn a_hard_stop_aborts_an_in_flight_model_call() {
    let cancel = CancelFlag::new();
    let flag = cancel.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(30)).await;
        flag.hard_stop();
    });
    let provider = SlowProvider {
        inner: ScriptedProvider::new(vec![ScriptedReply::text("should not finish")]),
    };
    let started = Instant::now();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &ToolRegistry::new(),
        &Dispatcher::default(),
        &Budget::default(),
        &cancel,
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Cancelled);
    assert_eq!(provider.calls(), 0);
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[tokio::test]
async fn a_hard_stop_aborts_an_in_flight_tool_call() {
    let cancel = CancelFlag::new();
    let flag = cancel.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(30)).await;
        flag.hard_stop();
    });
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(SlowTool) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "slow", json!({})),
        ScriptedReply::text("should not run"),
    ]);
    let started = Instant::now();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &cancel,
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Cancelled);
    assert_eq!(provider.calls(), 1);
    let tool = messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .unwrap();
    assert_eq!(tool.content, "error: cancelled");
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[tokio::test]
async fn a_soft_stop_finishes_the_tool_then_one_wrap_up() {
    let cancel = CancelFlag::new();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(SoftOnCall {
            flag: cancel.clone(),
        }) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "soft_on_call", json!({})),
        ScriptedReply::text("wrapping up"),
        ScriptedReply::text("should not run"),
    ]);
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &cancel,
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::SoftStop);
    assert_eq!(provider.calls(), 2);
    assert_eq!(messages.last().unwrap().content, "wrapping up");
    assert_eq!(
        messages
            .iter()
            .find(|message| message.tool_call_id.is_some())
            .unwrap()
            .content,
        "finished the call"
    );
}

#[tokio::test]
async fn a_soft_wrap_up_that_asks_for_tools_does_not_run_them() {
    let cancel = CancelFlag::new();
    cancel.soft_stop();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(SlowTool) as Arc<dyn Tool>)
        .unwrap();
    let provider = ScriptedProvider::new(vec![ScriptedReply::tool("call-1", "slow", json!({}))]);
    let started = Instant::now();
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &cancel,
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::SoftStop);
    assert_eq!(provider.calls(), 1);
    assert!(messages
        .iter()
        .all(|message| message.tool_call_id.is_none()));
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[test]
fn hard_stop_drops_a_pending_steer_and_soft_stop_does_not() {
    let inbox = SessionInbox::default();
    inbox.submit("keep going").unwrap();
    let cancel = CancelFlag::new();
    cancel.soft_stop();
    drop_pending_steer(&cancel, &inbox);
    assert_eq!(inbox.pop().unwrap(), "keep going");

    inbox.submit("drop me").unwrap();
    cancel.hard_stop();
    drop_pending_steer(&cancel, &inbox);
    assert!(inbox.pop().is_none());

    cancel.clear_soft();
    assert!(!cancel.is_soft_stop());
    assert!(cancel.is_cancelled());
}
