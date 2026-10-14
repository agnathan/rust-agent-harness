//! Model, tools, results, repeat. The gate runs before every model call.

use std::time::Instant;

use crate::budget::{check_gate, Budget, CancelFlag, StopReason};
use crate::dispatch::{Dispatcher, ToolCall};
use crate::event::{AgentUsage, Event};
use crate::inbox::SessionInbox;
use crate::message::Message;
use crate::provider::Provider;
use crate::registry::ToolRegistry;

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
    loop {
        if let Some(stop) = check_gate(cancel, budget, &usage, started) {
            return TurnOutcome {
                events,
                usage,
                stop,
            };
        }
        let batch = provider.complete(messages).await;
        let mut assistant = None;
        let mut saw_usage = false;
        for event in batch {
            if let Event::Usage(reported) = &event {
                usage.record(reported);
                saw_usage = true;
            }
            if let Event::Done { message, .. } = &event {
                assistant = Some(message.clone());
            }
            events.push(event);
        }
        if !saw_usage {
            usage.count_call_without_usage();
        }
        let Some(assistant) = assistant else {
            events.push(Event::Error("provider returned no done event".into()));
            return TurnOutcome {
                events,
                usage,
                stop: StopReason::Budget,
            };
        };
        let requested = assistant.tool_calls.clone();
        messages.push(assistant);
        if requested.is_empty() {
            return TurnOutcome {
                events,
                usage,
                stop: StopReason::Finished,
            };
        }
        let calls: Vec<ToolCall> = requested
            .iter()
            .map(|call| ToolCall {
                name: call.name.clone(),
                arguments: call.arguments.clone(),
            })
            .collect();
        let outputs = dispatcher.run(registry, &calls).await;
        for (request, output) in requested.iter().zip(outputs) {
            let text = match output.result {
                Ok(result) => result
                    .content
                    .first()
                    .map(|block| block.text.clone())
                    .unwrap_or_default(),
                Err(error) => format!("error: {error}"),
            };
            messages.push(Message::tool(request.id.clone(), text));
        }
    }
}
