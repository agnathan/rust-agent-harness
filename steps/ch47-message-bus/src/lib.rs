//! Book chapter 47. Peers publish on a bounded channel.
//! A full subscriber is backpressure. Twenty bot messages cool down.

use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

pub const BUS_CAP: usize = 64;
/// Hermes `gateway/bot_loop_guard.py` admits 20 bot messages, then cools down.
pub const MAX_BOT_EVENTS: u32 = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub topic: String,
    pub body: String,
}

struct Sub {
    name: String,
    topic: String,
    tx: SyncSender<Envelope>,
}

pub struct Bus {
    subs: Vec<Sub>,
    bot_events: u32,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            subs: Vec::new(),
            bot_events: 0,
        }
    }

    pub fn subscribe(
        &mut self,
        name: impl Into<String>,
        topic: impl Into<String>,
    ) -> Receiver<Envelope> {
        let (tx, rx) = sync_channel(BUS_CAP);
        self.subs.push(Sub {
            name: name.into(),
            topic: topic.into(),
            tx,
        });
        rx
    }

    pub fn bot_events(&self) -> u32 {
        self.bot_events
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Publish {
    Delivered(usize),
    Slow(String),
    Cooldown,
    Disconnected(String),
}

/// Mask a bearer residue and an `sk-` token. Other text is unchanged.
pub fn redact_for_egress(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if let Some(rest) = text[i..].strip_prefix("Bearer ") {
            out.push_str("Bearer [redacted]");
            let token_len = rest
                .find(|ch: char| ch.is_whitespace())
                .unwrap_or(rest.len());
            i += "Bearer ".len() + token_len;
            continue;
        }
        if text[i..].starts_with("sk-") {
            out.push_str("[redacted]");
            let token_len = text[i..]
                .find(|ch: char| ch.is_whitespace())
                .unwrap_or(text.len() - i);
            i += token_len;
            continue;
        }
        let ch = text[i..].chars().next().expect("char");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

pub fn publish(bus: &mut Bus, envelope: Envelope) -> Publish {
    deliver(bus, envelope)
}

/// A bot-authored publish. The 21st in this process is `Cooldown` and is not sent.
pub fn publish_bot(bus: &mut Bus, envelope: Envelope) -> Publish {
    if bus.bot_events >= MAX_BOT_EVENTS {
        return Publish::Cooldown;
    }
    let result = deliver(bus, envelope);
    if matches!(result, Publish::Delivered(_)) {
        bus.bot_events += 1;
    }
    result
}

fn deliver(bus: &Bus, envelope: Envelope) -> Publish {
    let mut delivered = 0usize;
    for sub in &bus.subs {
        if sub.topic != envelope.topic {
            continue;
        }
        match sub.tx.try_send(envelope.clone()) {
            Ok(()) => delivered += 1,
            Err(TrySendError::Full(_)) => return Publish::Slow(sub.name.clone()),
            Err(TrySendError::Disconnected(_)) => {
                return Publish::Disconnected(sub.name.clone());
            }
        }
    }
    Publish::Delivered(delivered)
}

pub fn send_to(bus: &Bus, name: &str, envelope: Envelope) -> Publish {
    let Some(sub) = bus.subs.iter().find(|sub| sub.name == name) else {
        return Publish::Disconnected(name.to_string());
    };
    match sub.tx.try_send(envelope) {
        Ok(()) => Publish::Delivered(1),
        Err(TrySendError::Full(_)) => Publish::Slow(sub.name.clone()),
        Err(TrySendError::Disconnected(_)) => Publish::Disconnected(sub.name.clone()),
    }
}

pub fn submit(bus: &mut Bus, topic: &str, body: &str) -> Publish {
    publish(
        bus,
        Envelope {
            topic: topic.to_string(),
            body: redact_for_egress(body),
        },
    )
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

pub fn demo() -> String {
    let mut bus = Bus::new();
    let rx = bus.subscribe("researcher", "work");
    let published = submit(&mut bus, "work", "ping");
    assert_eq!(published, Publish::Delivered(1));
    let mail = rx.recv().expect("mail");
    let reply = FakeModel { reply: "ok" }.complete(&mail.body);
    format!("bus: {} {reply}", mail.body)
}
