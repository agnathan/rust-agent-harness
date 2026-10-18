//! Model, tools, results, repeat. Each model call and each tool call is
//! raced against the hard-stop token. Soft stop finishes the current work,
//! then one wrap-up that does not dispatch tools.

use std::time::Instant;

use crate::budget::{check_gate, Budget, CancelFlag, StopReason};
use crate::dispatch::{Dispatcher, ToolCall};
use crate::event::{AgentUsage, Event};
use crate::inbox::SessionInbox;
use crate::message::Message;
use crate::provider::Provider;
use crate::registry::ToolRegistry;
use crate::stall::StallGuard;
use crate::validate::{classify, Classified, PlannedCall, MAX_INVALID_STRIKES, MAX_JSON_RETRIES};

#[derive(Debug, Clone, PartialEq)]
pub struct TurnOutcome {
    pub events: Vec<Event>,
    pub usage: AgentUsage,
    pub stop: StopReason,
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
        Ok(run_loop(
            &mut self.messages,
            provider,
            registry,
            dispatcher,
            budget,
            cancel,
            Instant::now(),
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
    let mut events = Vec::new();
    let mut usage = AgentUsage::default();
    let mut json_retries = 0u32;
    let mut invalid_strikes = 0u32;
    let mut stall = StallGuard::new();
    let mut iteration = 0u64;
    loop {
        if cancel.is_cancelled() {
            return TurnOutcome {
                events,
                usage,
                stop: StopReason::Cancelled,
            };
        }
        if cancel.is_soft_stop() {
            return soft_wrap_up(messages, provider, events, usage, cancel).await;
        }
        if let Some(stop) = check_gate(cancel, budget, &usage, started) {
            return TurnOutcome {
                events,
                usage,
                stop,
            };
        }
        let token = cancel.token();
        let batch = tokio::select! {
            biased;
            _ = token.cancelled() => {
                return TurnOutcome {
                    events,
                    usage,
                    stop: StopReason::Cancelled,
                };
            }
            batch = provider.complete(messages) => batch,
        };
        let Some(assistant) = absorb(&batch, &mut events, &mut usage) else {
            events.push(Event::Error("provider returned no done event".into()));
            return TurnOutcome {
                events,
                usage,
                stop: StopReason::Budget,
            };
        };
        let requested = assistant.tool_calls.clone();
        if requested.is_empty() {
            messages.push(assistant);
            return TurnOutcome {
                events,
                usage,
                stop: StopReason::Finished,
            };
        }
        let known = known_names(registry);
        let classified = classify(requested, &known);
        if classified.truncated {
            push_assistant(messages, assistant, &classified);
            push_errors(messages, &classified);
            return TurnOutcome {
                events,
                usage,
                stop: StopReason::Partial,
            };
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
                return TurnOutcome {
                    events,
                    usage,
                    stop: StopReason::InvalidToolCalls,
                };
            }
            continue;
        }
        invalid_strikes = 0;
        push_assistant(messages, assistant, &classified);
        let calls: Vec<ToolCall> = runnable
            .iter()
            .map(|call| ToolCall {
                name: call.name.clone(),
                arguments: call.arguments.clone(),
            })
            .collect();
        let outputs = dispatcher
            .run_raced(registry, &calls, &cancel.token())
            .await;
        let mut ran = runnable.into_iter().zip(outputs);
        let mut halt = false;
        for planned in &classified.calls {
            if let Some(error) = &planned.error {
                messages.push(Message::tool(planned.id.clone(), error.clone()));
                continue;
            }
            let Some((_, output)) = ran.next() else {
                messages.push(Message::tool(
                    planned.id.clone(),
                    "error: tool result missing",
                ));
                continue;
            };
            let text = tool_text(&output);
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
            messages.push(Message::tool(planned.id.clone(), action.text));
        }
        iteration += 1;
        if halt {
            return TurnOutcome {
                events,
                usage,
                stop: StopReason::Stalled,
            };
        }
    }
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

async fn soft_wrap_up(
    messages: &mut Vec<Message>,
    provider: &dyn Provider,
    mut events: Vec<Event>,
    mut usage: AgentUsage,
    cancel: &CancelFlag,
) -> TurnOutcome {
    let token = cancel.token();
    let batch = tokio::select! {
        biased;
        _ = token.cancelled() => {
            return TurnOutcome {
                events,
                usage,
                stop: StopReason::Cancelled,
            };
        }
        batch = provider.complete(messages) => batch,
    };
    let Some(assistant) = absorb(&batch, &mut events, &mut usage) else {
        events.push(Event::Error("provider returned no done event".into()));
        return TurnOutcome {
            events,
            usage,
            stop: StopReason::SoftStop,
        };
    };
    if assistant.tool_calls.is_empty() {
        messages.push(assistant);
    }
    TurnOutcome {
        events,
        usage,
        stop: StopReason::SoftStop,
    }
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
