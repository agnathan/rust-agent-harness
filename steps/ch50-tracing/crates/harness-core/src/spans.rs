//! Book chapter 50. A span per event. The field is the event name, not the prompt.
//! A research team of mock agents writes into one memory across runs.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

#[derive(Clone, Default)]
pub struct CountingSubscriber {
    next: Arc<AtomicU64>,
}

impl CountingSubscriber {
    pub fn new() -> Self {
        Self {
            next: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn spans_opened(&self) -> u64 {
        self.next.load(Ordering::Relaxed).saturating_sub(1)
    }
}

impl Subscriber for CountingSubscriber {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &Attributes<'_>) -> Id {
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        Id::from_u64(id.max(1))
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, _event: &Event<'_>) {}

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

/// Content-free field. The prompt is not a span field.
pub fn safe_field(text: &str) -> &'static str {
    let _ = text;
    "request"
}

pub fn emit(name: &'static str, sink: &mut Vec<&'static str>) {
    sink.push(name);
    let span = tracing::info_span!("event", name);
    let _entered = span.enter();
}

pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

/// Chapter 1 events, then a gateway child span. No prompt text is stored.
pub fn mock_turn(model: &FakeModel, prompt: &str, sink: &mut Vec<&'static str>) -> String {
    let root = tracing::info_span!("turn", event = safe_field(prompt));
    let _root = root.enter();
    emit("request", sink);
    let reply = model.complete(prompt);
    emit("response", sink);
    let gateway = tracing::info_span!("gateway", event = "dispatch");
    let _gateway = gateway.enter();
    emit("gateway", sink);
    reply
}

pub struct Memory {
    notes: Mutex<Vec<String>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            notes: Mutex::new(Vec::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.notes.lock().expect("memory").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn last(&self) -> Option<String> {
        self.notes.lock().expect("memory").last().cloned()
    }

    pub fn push(&self, note: String) {
        self.notes.lock().expect("memory").push(note);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

/// `n` mock agents append one note each. The second call sees the earlier notes.
pub fn research_team(n: usize, memory: &Memory, model: &FakeModel) -> usize {
    thread::scope(|scope| {
        for index in 0..n {
            let known = memory.last();
            scope.spawn(move || {
                let prompt = known.as_deref().unwrap_or("first");
                let note = model.complete(prompt);
                memory.push(format!("{index}:{note}"));
            });
        }
    });
    memory.len()
}

pub fn demo() -> String {
    let model = FakeModel { reply: "ok" };
    let mut sink = Vec::new();
    let subscriber = CountingSubscriber::new();
    let opened = subscriber.clone();
    let reply =
        tracing::subscriber::with_default(subscriber, || mock_turn(&model, "hello", &mut sink));
    let memory = Memory::new();
    research_team(100, &memory, &model);
    let again = research_team(1, &memory, &FakeModel { reply: "kept" });
    format!(
        "trace: {reply} events={} spans={} team={again}",
        sink.len(),
        opened.spans_opened()
    )
}
