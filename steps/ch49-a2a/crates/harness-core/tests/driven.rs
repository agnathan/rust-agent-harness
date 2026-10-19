//! The chapter 9 loop and the chapter 10 table agree on the phase.

use std::sync::Arc;
use std::time::Instant;

use harness_core::{
    run_loop_driven, Budget, CancelFlag, Dispatcher, EchoTool, Phase, Provider, ScriptedProvider,
    ScriptedReply, StopReason, Tool, ToolRegistry, DISPATCH_BOUND,
};
use serde_json::json;

fn echo_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    registry
}

fn phase_of(stop: StopReason) -> Phase {
    match stop {
        StopReason::Finished => Phase::Finished,
        StopReason::Partial => Phase::Partial,
        StopReason::Budget => Phase::Budget,
        StopReason::Cancelled => Phase::Cancelled,
        StopReason::SoftStop => Phase::SoftStopped,
        StopReason::Stalled => Phase::Stalled,
        StopReason::InvalidToolCalls => Phase::InvalidToolCalls,
    }
}

#[tokio::test]
async fn a_text_answer_finishes_in_the_table() {
    let provider = ScriptedProvider::new(vec![ScriptedReply::text("ok")]);
    let registry = ToolRegistry::new();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let mut messages = vec![harness_core::Message::user("status")];
    let (outcome, phase) = run_loop_driven(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(phase, phase_of(outcome.stop()));
    assert_eq!(phase, Phase::Finished);
}

#[tokio::test]
async fn a_tool_then_text_passes_through_dispatching() {
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "echo", json!({"text": "hello"})),
        ScriptedReply::text("hello"),
    ]);
    let registry = echo_registry();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let mut messages = vec![harness_core::Message::user("say hello")];
    let (outcome, phase) = run_loop_driven(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &Budget::default(),
        &CancelFlag::new(),
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::Finished);
    assert_eq!(phase, Phase::Finished);
    assert_eq!(provider.calls(), 2);
}

#[tokio::test]
async fn a_spent_budget_with_no_calls_never_enters_calling_model() {
    let provider = ScriptedProvider::new(vec![ScriptedReply::text("should not run")]);
    let registry = ToolRegistry::new();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let mut messages = vec![harness_core::Message::user("now")];
    let budget = Budget {
        max_iterations: 0,
        ..Budget::default()
    };
    let (outcome, phase) = run_loop_driven(
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
    assert_eq!(phase, Phase::Budget);
    assert_eq!(provider.calls(), 0);
}

#[tokio::test]
async fn a_soft_stop_before_the_model_ends_soft_stopped() {
    let provider = ScriptedProvider::new(vec![ScriptedReply::text("wrapping up")]);
    let registry = ToolRegistry::new();
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let cancel = CancelFlag::new();
    cancel.soft_stop();
    let mut messages = vec![harness_core::Message::user("stop")];
    let (outcome, phase) = run_loop_driven(
        &mut messages,
        &provider,
        &registry,
        &dispatcher,
        &Budget::default(),
        &cancel,
        Instant::now(),
    )
    .await;
    assert_eq!(outcome.stop(), StopReason::SoftStop);
    assert_eq!(phase, Phase::SoftStopped);
}
