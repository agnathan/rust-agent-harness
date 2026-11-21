//! Book chapter 49. A2A is the trait at the process boundary.
//! An agent in this process is not this trait. The pool bound is 8.
//! Outbound text is `crate::redact::redact_for_egress`. There is no second scrub.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub const CALL_BOUND: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum A2aError {
    InProcess,
    Bound,
    Cancelled,
    Unreachable,
}

pub trait ExternalAgent {
    fn send(&self, message: &str, cancel: &AtomicBool) -> Result<String, A2aError>;
}

#[derive(Debug, Default)]
pub struct HttpPool {
    pub urls: Mutex<Vec<String>>,
    pub bodies: Mutex<Vec<String>>,
}

impl HttpPool {
    pub fn record(&self, url: &str, body: &str) {
        self.urls.lock().expect("pool").push(url.to_string());
        self.bodies.lock().expect("pool").push(body.to_string());
    }

    pub fn calls(&self) -> usize {
        self.urls.lock().expect("pool").len()
    }
}

pub struct Pool {
    in_flight: AtomicUsize,
}

impl Pool {
    pub fn new() -> Self {
        Self {
            in_flight: AtomicUsize::new(0),
        }
    }

    pub fn in_flight(&self) -> usize {
        self.in_flight.load(Ordering::SeqCst)
    }

    fn acquire(&self) -> bool {
        loop {
            let now = self.in_flight.load(Ordering::SeqCst);
            if now >= CALL_BOUND {
                return false;
            }
            if self
                .in_flight
                .compare_exchange(now, now + 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return true;
            }
        }
    }

    fn release(&self) {
        self.in_flight.fetch_sub(1, Ordering::SeqCst);
    }

    /// Test helper: take one permit and keep it.
    pub fn hold(&self) -> bool {
        self.acquire()
    }

    pub fn release_one(&self) {
        self.release();
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self::new()
    }
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

pub struct RemotePeer {
    pub name: &'static str,
    pub url: &'static str,
    pub http: Arc<HttpPool>,
    pub model: FakeModel,
}

impl ExternalAgent for RemotePeer {
    fn send(&self, message: &str, cancel: &AtomicBool) -> Result<String, A2aError> {
        if self.url.starts_with("local://") {
            return Err(A2aError::InProcess);
        }
        if !(self.url.starts_with("http://") || self.url.starts_with("https://")) {
            return Err(A2aError::Unreachable);
        }
        if cancel.load(Ordering::SeqCst) {
            return Err(A2aError::Cancelled);
        }
        self.http.record(self.url, message);
        if cancel.load(Ordering::SeqCst) {
            return Err(A2aError::Cancelled);
        }
        Ok(self.model.complete(message))
    }
}

pub const INBOUND_PREFIX: &str = "[A2A inbound — untrusted external input]\n";

pub fn wrap_inbound(peer: &str, text: &str) -> String {
    format!("{INBOUND_PREFIX}{peer}\n{text}")
}

pub fn call(
    pool: &Pool,
    peer: &RemotePeer,
    message: &str,
    cancel: &AtomicBool,
) -> Result<String, A2aError> {
    if peer.url.starts_with("local://") {
        return Err(A2aError::InProcess);
    }
    if !pool.acquire() {
        return Err(A2aError::Bound);
    }
    let result = peer.send(&crate::redact::redact_for_egress(message), cancel);
    pool.release();
    result
}

/// Set the flag and wait until `arrived` is true. The in-flight loop is the peer.
pub fn cancel(flag: &AtomicBool) {
    flag.store(true, Ordering::SeqCst);
}

pub struct AgentCard {
    pub name: &'static str,
    pub skills: &'static [&'static str],
}

pub fn card() -> AgentCard {
    AgentCard {
        name: "harness",
        skills: &["research"],
    }
}

pub fn demo() -> String {
    let pool = Pool::new();
    let peer = RemotePeer {
        name: "researcher",
        url: "http://research.example:9900",
        http: Arc::new(HttpPool::default()),
        model: FakeModel { reply: "ok" },
    };
    let reply = call(&pool, &peer, "summarize", &AtomicBool::new(false)).expect("call");
    format!("a2a: {reply}")
}

/// Spin until `flag` is set. Used by the cancel bench and the cancel test.
pub fn wait_cancel(flag: &AtomicBool, started: &AtomicBool) {
    started.store(true, Ordering::SeqCst);
    while !flag.load(Ordering::SeqCst) {
        thread::yield_now();
    }
}
