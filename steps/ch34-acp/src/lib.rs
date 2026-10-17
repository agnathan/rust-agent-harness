//! Book chapter 40. An editor speaks ACP. The update becomes one inbox
//! message. Protocol bytes go to stdout. Logs go to stderr.

use std::collections::VecDeque;

pub const INBOX_CAP: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    EndTurn,
    Cancelled,
}

impl StopReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EndTurn => "end_turn",
            Self::Cancelled => "cancelled",
        }
    }
}

pub fn stop_reason(cancelled: bool) -> StopReason {
    if cancelled {
        StopReason::Cancelled
    } else {
        StopReason::EndTurn
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approval {
    Once,
    Session,
    Always,
    Deny,
}

/// Unknown option ids deny. `deny_always` is a deny.
pub fn map_permission(option_id: &str) -> Approval {
    match option_id {
        "allow_once" => Approval::Once,
        "allow_session" => Approval::Session,
        "allow_always" => Approval::Always,
        "deny" | "deny_always" => Approval::Deny,
        _ => Approval::Deny,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pub text: String,
    pub cancelled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mail {
    pub text: String,
}

pub fn translate(update: &Update) -> Mail {
    Mail {
        text: update.text.clone(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Submit {
    Accepted,
    Full,
}

pub struct Inbox {
    q: VecDeque<Mail>,
    cap: usize,
}

impl Inbox {
    pub fn new(cap: usize) -> Self {
        Self {
            q: VecDeque::with_capacity(cap),
            cap,
        }
    }

    pub fn submit(&mut self, mail: Mail) -> Submit {
        if self.q.len() >= self.cap {
            return Submit::Full;
        }
        self.q.push_back(mail);
        Submit::Accepted
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    pub fn front(&self) -> Option<&Mail> {
        self.q.front()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Streams {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
}

pub fn write_result(streams: &mut Streams, reason: StopReason) {
    streams.stdout.push(format!(
        r#"{{"jsonrpc":"2.0","result":{{"stopReason":"{}"}}}}"#,
        reason.as_str()
    ));
}

pub fn log_line(streams: &mut Streams, line: &str) {
    streams.stderr.push(line.to_string());
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
    let mut streams = Streams::default();
    let mut inbox = Inbox::new(INBOX_CAP);
    let update = Update {
        text: "hello".to_string(),
        cancelled: false,
    };
    let mail = translate(&update);
    inbox.submit(mail);
    let _ = FakeModel { reply: "ok" }.complete(&update.text);
    log_line(&mut streams, "session started");
    let reason = stop_reason(update.cancelled);
    write_result(&mut streams, reason);
    format!("acp: {}", reason.as_str())
}
