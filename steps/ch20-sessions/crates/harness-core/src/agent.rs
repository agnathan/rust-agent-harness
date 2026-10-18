//! Model, tools, results, repeat. Each model call and each tool call is
//! raced against the hard-stop token. Soft stop finishes the current work,
//! then one wrap-up that does not dispatch tools.

use std::time::Instant;

use crate::budget::{
    check_gate, drop_pending_steer, inject_budget_notices, Budget, CancelFlag, NoticeLatch,
    StopReason,
};
use crate::canonical::canonical_messages;
use crate::dispatch::{Dispatcher, ToolCall};
use crate::event::{AgentUsage, Event};
use crate::inbox::SessionInbox;
use crate::message::{Message, Role};
use crate::provider::Provider;
use crate::registry::ToolRegistry;
use crate::stall::StallGuard;
use crate::truncate::{truncate_head_tail, TOOL_OUTPUT_MAX_CHARS};
use crate::turn::{Driver, Phase, TurnEvent};
use crate::validate::{classify, Classified, PlannedCall, MAX_INVALID_STRIKES, MAX_JSON_RETRIES};

#[derive(Debug, Clone, PartialEq)]
pub enum TurnOutcome {
    Finished {
        events: Vec<Event>,
        usage: AgentUsage,
        text: String,
    },
    Partial {
        events: Vec<Event>,
        usage: AgentUsage,
        text: String,
    },
    Budget {
        events: Vec<Event>,
        usage: AgentUsage,
        text: String,
    },
    Cancelled {
        events: Vec<Event>,
        usage: AgentUsage,
        text: String,
        hard: bool,
    },
    Stalled {
        events: Vec<Event>,
        usage: AgentUsage,
        text: String,
    },
    InvalidToolCalls {
        events: Vec<Event>,
        usage: AgentUsage,
        text: String,
    },
}

impl TurnOutcome {
    pub fn events(&self) -> &[Event] {
        match self {
            Self::Finished { events, .. }
            | Self::Partial { events, .. }
            | Self::Budget { events, .. }
            | Self::Cancelled { events, .. }
            | Self::Stalled { events, .. }
            | Self::InvalidToolCalls { events, .. } => events,
        }
    }

    pub fn usage(&self) -> &AgentUsage {
        match self {
            Self::Finished { usage, .. }
            | Self::Partial { usage, .. }
            | Self::Budget { usage, .. }
            | Self::Cancelled { usage, .. }
            | Self::Stalled { usage, .. }
            | Self::InvalidToolCalls { usage, .. } => usage,
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Self::Finished { text, .. }
            | Self::Partial { text, .. }
            | Self::Budget { text, .. }
            | Self::Cancelled { text, .. }
            | Self::Stalled { text, .. }
            | Self::InvalidToolCalls { text, .. } => text,
        }
    }

    pub fn stop(&self) -> StopReason {
        match self {
            Self::Finished { .. } => StopReason::Finished,
            Self::Partial { .. } => StopReason::Partial,
            Self::Budget { .. } => StopReason::Budget,
            Self::Cancelled { hard: true, .. } => StopReason::Cancelled,
            Self::Cancelled { hard: false, .. } => StopReason::SoftStop,
            Self::Stalled { .. } => StopReason::Stalled,
            Self::InvalidToolCalls { .. } => StopReason::InvalidToolCalls,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Finished { .. } => "Finished",
            Self::Partial { .. } => "Partial",
            Self::Budget { .. } => "Budget",
            Self::Cancelled { .. } => "Cancelled",
            Self::Stalled { .. } => "Stalled",
            Self::InvalidToolCalls { .. } => "InvalidToolCalls",
        }
    }
}

fn finish(stop: StopReason, events: Vec<Event>, usage: AgentUsage, text: String) -> TurnOutcome {
    match stop {
        StopReason::Finished => TurnOutcome::Finished {
            events,
            usage,
            text,
        },
        StopReason::Partial => TurnOutcome::Partial {
            events,
            usage,
            text,
        },
        StopReason::Budget => TurnOutcome::Budget {
            events,
            usage,
            text,
        },
        StopReason::Cancelled => TurnOutcome::Cancelled {
            events,
            usage,
            text,
            hard: true,
        },
        StopReason::SoftStop => TurnOutcome::Cancelled {
            events,
            usage,
            text,
            hard: false,
        },
        StopReason::Stalled => TurnOutcome::Stalled {
            events,
            usage,
            text,
        },
        StopReason::InvalidToolCalls => TurnOutcome::InvalidToolCalls {
            events,
            usage,
            text,
        },
    }
}

fn last_text(messages: &[Message]) -> String {
    messages
        .iter()
        .rev()
        .find(|message| message.role == Role::Assistant && message.tool_calls.is_empty())
        .map(|message| message.content.clone())
        .unwrap_or_default()
}

pub struct AgentSession {
    pub inbox: SessionInbox,
    pub messages: Vec<Message>,
}

impl AgentSession {
    pub fn new() -> Self {
        Self {
            inbox: SessionInbox::default(),
            messages: Vec::new(),
        }
    }

    pub async fn turn_from_inbox(
        &mut self,
        provider: &dyn Provider,
        registry: &ToolRegistry,
        dispatcher: &Dispatcher,
        budget: &Budget,
        cancel: &CancelFlag,
    ) -> Result<TurnOutcome, String> {
        let prompt = self
            .inbox
            .pop()
            .ok_or_else(|| "session inbox is empty".to_string())?;
        self.messages.push(Message::user(prompt));
        Ok(run_loop_with_inbox(
            &mut self.messages,
            provider,
            registry,
            dispatcher,
            budget,
            cancel,
            Instant::now(),
            Some(&self.inbox),
        )
        .await)
    }
}

impl Default for AgentSession {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn run_loop(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
) -> TurnOutcome {
    run_loop_with_inbox(
        messages, provider, registry, dispatcher, budget, cancel, started, None,
    )
    .await
}

/// Same loop as [`run_loop`], plus the phase the table ended on.
pub async fn run_loop_driven(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
) -> (TurnOutcome, Phase) {
    let mut driver = Driver::new();
    let outcome = drive_loop(
        messages,
        provider,
        registry,
        dispatcher,
        budget,
        cancel,
        started,
        None,
        &mut driver,
    )
    .await;
    (outcome, driver.phase())
}

fn note(driver: &mut Driver, event: TurnEvent) {
    driver.advance(event).unwrap_or_else(|err| {
        panic!(
            "illegal turn move from {:?} on {:?} — the loop and the table disagree",
            err.from, err.event
        )
    });
}

#[allow(clippy::too_many_arguments)]
pub async fn run_loop_with_inbox(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
    inbox: Option<&SessionInbox>,
) -> TurnOutcome {
    let mut driver = Driver::new();
    drive_loop(
        messages,
        provider,
        registry,
        dispatcher,
        budget,
        cancel,
        started,
        inbox,
        &mut driver,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
async fn drive_loop(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    registry: &ToolRegistry,
    dispatcher: &Dispatcher,
    budget: &Budget,
    cancel: &CancelFlag,
    started: Instant,
    inbox: Option<&SessionInbox>,
    driver: &mut Driver,
) -> TurnOutcome {
    let mut events = Vec::new();
    let mut usage = AgentUsage::default();
    let mut json_retries = 0u32;
    let mut invalid_strikes = 0u32;
    let mut stall = StallGuard::new();
    let mut iteration = 0u64;
    let mut notices = NoticeLatch::default();
    let mut started_phase = false;
    loop {
        if cancel.is_cancelled() {
            if let Some(inbox) = inbox {
                drop_pending_steer(cancel, inbox);
            }
            if !started_phase {
                note(driver, TurnEvent::Start);
            }
            note(driver, TurnEvent::HardCancel);
            return finish(StopReason::Cancelled, events, usage, last_text(messages));
        }
        if let Some(inbox) = inbox {
            append_steers(messages, inbox);
        }
        inject_budget_notices(
            messages,
            budget,
            &usage,
            started.elapsed(),
            cancel,
            &mut notices,
        );
        if cancel.is_soft_stop() {
            if !started_phase {
                note(driver, TurnEvent::Start);
            }
            note(driver, TurnEvent::SoftStop);
            return soft_wrap_up(messages, provider, events, usage, cancel, driver).await;
        }
        if let Some(stop) = check_gate(cancel, budget, &usage, started) {
            if stop == StopReason::Budget && usage.calls > 0 {
                if !started_phase {
                    note(driver, TurnEvent::Start);
                }
                note(driver, TurnEvent::BudgetHit);
                return budget_wrap_up(messages, provider, events, usage, cancel, driver).await;
            }
            if stop == StopReason::Budget && !started_phase {
                note(driver, TurnEvent::BudgetHit);
                return finish(stop, events, usage, last_text(messages));
            }
            if stop == StopReason::Budget {
                note(driver, TurnEvent::BudgetAbort);
                return finish(stop, events, usage, last_text(messages));
            }
            if !started_phase {
                note(driver, TurnEvent::Start);
            }
            note(driver, TurnEvent::HardCancel);
            return finish(stop, events, usage, last_text(messages));
        }
        if !started_phase {
            note(driver, TurnEvent::Start);
            started_phase = true;
        }
        let token = cancel.token();
        let send = canonical_messages(messages);
        let batch = tokio::select! {
            biased;
            _ = token.cancelled() => {
                note(driver, TurnEvent::HardCancel);
                return finish(StopReason::Cancelled, events, usage, last_text(messages));
            }
            batch = provider.complete(&send) => batch,
        };
        let Some(assistant) = absorb(&batch, &mut events, &mut usage) else {
            events.push(Event::Error("provider returned no done event".into()));
            note(driver, TurnEvent::NoDone);
            return finish(StopReason::Budget, events, usage, last_text(messages));
        };
        let requested = assistant.tool_calls.clone();
        if requested.is_empty() {
            messages.push(assistant);
            note(driver, TurnEvent::ModelText);
            return finish(StopReason::Finished, events, usage, last_text(messages));
        }
        let known = known_names(registry);
        let classified = classify(requested, &known);
        if classified.truncated {
            push_assistant(messages, assistant, &classified);
            push_errors(messages, &classified);
            note(driver, TurnEvent::Truncated);
            return finish(StopReason::Partial, events, usage, last_text(messages));
        }
        if classified.malformed {
            json_retries += 1;
            if json_retries < MAX_JSON_RETRIES {
                continue;
            }
            json_retries = 0;
        } else {
            json_retries = 0;
        }
        let runnable: Vec<PlannedCall> = classified
            .calls
            .iter()
            .filter(|call| call.error.is_none())
            .cloned()
            .collect();
        if runnable.is_empty() {
            push_assistant(messages, assistant, &classified);
            push_errors(messages, &classified);
            if classified.malformed {
                continue;
            }
            invalid_strikes += 1;
            if invalid_strikes >= MAX_INVALID_STRIKES {
                note(driver, TurnEvent::InvalidStrikeout);
                return finish(
                    StopReason::InvalidToolCalls,
                    events,
                    usage,
                    last_text(messages),
                );
            }
            continue;
        }
        invalid_strikes = 0;
        note(driver, TurnEvent::ModelToolCalls);
        push_assistant(messages, assistant, &classified);
        let mut allowed = Vec::new();
        let mut blocked = Vec::new();
        let mut denied: Vec<(String, String)> = Vec::new();
        let mut approval_session = crate::approval::Session::new();
        for call in runnable {
            if let Some(message) =
                crate::approval::deny_tool(&mut approval_session, &call.name, &call.arguments)
            {
                denied.push((call.id.clone(), message));
                continue;
            }
            if call.name == "delegate" {
                if usage.subagent_calls >= budget.max_subagent_iterations {
                    blocked.push(call);
                    continue;
                }
                usage.subagent_calls += 1;
            }
            allowed.push(call);
        }
        let blocked_ids: Vec<String> = blocked.iter().map(|call| call.id.clone()).collect();
        let calls: Vec<ToolCall> = allowed
            .iter()
            .map(|call| ToolCall {
                name: call.name.clone(),
                arguments: call.arguments.clone(),
            })
            .collect();
        let outputs = dispatcher
            .run_raced(registry, &calls, &cancel.token())
            .await;
        let mut ran = allowed.into_iter().zip(outputs);
        let mut halt = false;
        for planned in &classified.calls {
            if let Some(error) = &planned.error {
                messages.push(Message::tool(planned.id.clone(), error.clone()));
                continue;
            }
            if blocked_ids.iter().any(|id| id == &planned.id) {
                messages.push(Message::tool(
                    planned.id.clone(),
                    "error: subagent iteration cap",
                ));
                continue;
            }
            if let Some((_, message)) = denied.iter().find(|(id, _)| id == &planned.id) {
                messages.push(Message::tool(planned.id.clone(), message.clone()));
                continue;
            }
            let Some((_, output)) = ran.next() else {
                messages.push(Message::tool(
                    planned.id.clone(),
                    "error: tool result missing",
                ));
                continue;
            };
            let text = truncate_head_tail(&tool_text(&output), TOOL_OUTPUT_MAX_CHARS);
            let failed =
                output.result.is_err() || matches!(&output.result, Ok(result) if result.is_error);
            let action = stall.observe(
                iteration,
                &planned.name,
                &planned.arguments.to_string(),
                &text,
                failed,
            );
            if action.halt {
                halt = true;
            }
            if planned.name == "execute_code" && !failed {
                usage.refund_iteration();
            }
            messages.push(Message::tool(planned.id.clone(), action.text));
        }
        iteration += 1;
        if halt {
            note(driver, TurnEvent::Stall);
            return finish(StopReason::Stalled, events, usage, last_text(messages));
        }
        note(driver, TurnEvent::ToolsDone);
    }
}

pub fn append_steer_text(messages: &mut [Message], steer: &str) -> bool {
    if steer.is_empty() {
        return false;
    }
    let Some(tool) = messages
        .iter_mut()
        .rev()
        .find(|message| message.role == Role::Tool)
    else {
        return false;
    };
    tool.content.push_str("\n\n");
    tool.content.push_str(steer);
    true
}

fn append_steers(messages: &mut [Message], inbox: &SessionInbox) {
    if !messages.iter().any(|message| message.role == Role::Tool) {
        return;
    }
    let mut lines = Vec::new();
    while let Some(line) = inbox.pop() {
        if !line.is_empty() {
            lines.push(line);
        }
    }
    if lines.is_empty() {
        return;
    }
    append_steer_text(messages, &lines.join("\n"));
}

fn known_names(registry: &ToolRegistry) -> Vec<String> {
    registry
        .schemas()
        .into_iter()
        .map(|schema| schema.name)
        .collect()
}

fn push_assistant(messages: &mut Vec<Message>, mut assistant: Message, classified: &Classified) {
    assistant.tool_calls = classified
        .calls
        .iter()
        .map(|call| crate::message::RequestedTool {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: call.arguments.clone(),
            raw_arguments: None,
        })
        .collect();
    messages.push(assistant);
}

fn push_errors(messages: &mut Vec<Message>, classified: &Classified) {
    for call in &classified.calls {
        let text = call
            .error
            .clone()
            .unwrap_or_else(|| "error: invalid tool call".to_string());
        messages.push(Message::tool(call.id.clone(), text));
    }
}

fn tool_text(output: &crate::dispatch::CallOutput) -> String {
    match &output.result {
        Ok(result) => result
            .content
            .first()
            .map(|block| block.text.clone())
            .unwrap_or_default(),
        Err(error) => format!("error: {error}"),
    }
}

async fn budget_wrap_up(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    mut events: Vec<Event>,
    mut usage: AgentUsage,
    cancel: &CancelFlag,
    driver: &mut Driver,
) -> TurnOutcome {
    let token = cancel.token();
    let send = canonical_messages(messages);
    let batch = tokio::select! {
        biased;
        _ = token.cancelled() => {
            note(driver, TurnEvent::HardCancel);
            return finish(StopReason::Cancelled, events, usage, last_text(messages));
        }
        batch = provider.complete(&send) => batch,
    };
    let Some(assistant) = absorb(&batch, &mut events, &mut usage) else {
        events.push(Event::Error("provider returned no done event".into()));
        note(driver, TurnEvent::NoDone);
        return finish(StopReason::Budget, events, usage, last_text(messages));
    };
    if assistant.tool_calls.is_empty() {
        let text = assistant.content.clone();
        messages.push(assistant);
        note(driver, TurnEvent::WrapUpText);
        return finish(StopReason::Budget, events, usage, text);
    }
    note(driver, TurnEvent::WrapUpToolCalls);
    finish(StopReason::Partial, events, usage, last_text(messages))
}

async fn soft_wrap_up(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    mut events: Vec<Event>,
    mut usage: AgentUsage,
    cancel: &CancelFlag,
    driver: &mut Driver,
) -> TurnOutcome {
    let token = cancel.token();
    let send = canonical_messages(messages);
    let batch = tokio::select! {
        biased;
        _ = token.cancelled() => {
            note(driver, TurnEvent::HardCancel);
            return finish(StopReason::Cancelled, events, usage, last_text(messages));
        }
        batch = provider.complete(&send) => batch,
    };
    let Some(assistant) = absorb(&batch, &mut events, &mut usage) else {
        events.push(Event::Error("provider returned no done event".into()));
        note(driver, TurnEvent::SoftWrapIgnoredTools);
        return finish(StopReason::SoftStop, events, usage, last_text(messages));
    };
    if assistant.tool_calls.is_empty() {
        messages.push(assistant);
        note(driver, TurnEvent::WrapUpText);
    } else {
        note(driver, TurnEvent::SoftWrapIgnoredTools);
    }
    finish(StopReason::SoftStop, events, usage, last_text(messages))
}

fn absorb(batch: &[Event], events: &mut Vec<Event>, usage: &mut AgentUsage) -> Option<Message> {
    let mut assistant = None;
    let mut saw_usage = false;
    for event in batch {
        if let Event::Usage(reported) = event {
            usage.record(reported);
            saw_usage = true;
        }
        if let Event::Done { message, .. } = event {
            assistant = Some(message.clone());
        }
        events.push(event.clone());
    }
    if !saw_usage {
        usage.count_call_without_usage();
    }
    assistant
}
