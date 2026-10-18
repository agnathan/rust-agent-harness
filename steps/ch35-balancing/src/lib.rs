//! Book chapter 35. One tool can have several upstreams. The pick is local:
//! least outstanding, or the quieter of two indexes. Discovery connects at
//! most four upstreams at once. Nothing here opens a socket.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

/// Hermes `mcp.discovery_concurrency` default, from `tools/mcp_tool_discovery.py`.
pub const DISCOVERY_CONCURRENCY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    LeastOutstanding,
    PowerOfTwo { a: usize, b: usize },
}

pub struct Backend {
    pub server: String,
    pub reply: String,
    outstanding: AtomicUsize,
}

impl Backend {
    pub fn new(server: impl Into<String>, reply: impl Into<String>, outstanding: usize) -> Self {
        Self {
            server: server.into(),
            reply: reply.into(),
            outstanding: AtomicUsize::new(outstanding),
        }
    }

    pub fn outstanding(&self) -> usize {
        self.outstanding.load(Ordering::Relaxed)
    }
}

pub struct Pool {
    backends: Vec<Backend>,
}

impl Pool {
    pub fn new(backends: Vec<Backend>) -> Self {
        Self { backends }
    }

    pub fn len(&self) -> usize {
        self.backends.len()
    }

    pub fn is_empty(&self) -> bool {
        self.backends.is_empty()
    }

    pub fn pick(&self, policy: Policy) -> Option<usize> {
        if self.backends.is_empty() {
            return None;
        }
        Some(match policy {
            Policy::LeastOutstanding => self
                .backends
                .iter()
                .enumerate()
                .min_by_key(|(_, backend)| backend.outstanding())
                .map(|(index, _)| index)
                .unwrap_or(0),
            Policy::PowerOfTwo { a, b } => {
                let left = a % self.backends.len();
                let right = b % self.backends.len();
                if self.backends[left].outstanding() <= self.backends[right].outstanding() {
                    left
                } else {
                    right
                }
            }
        })
    }

    /// Pick, count one outstanding request, return the scripted reply, then drop the count.
    pub fn call(&self, policy: Policy) -> Option<String> {
        let index = self.pick(policy)?;
        let backend = &self.backends[index];
        backend.outstanding.fetch_add(1, Ordering::Relaxed);
        let reply = format!("{} via {}", backend.reply, backend.server);
        backend.outstanding.fetch_sub(1, Ordering::Relaxed);
        Some(reply)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upstream {
    pub server: String,
    pub tools: Vec<String>,
}

/// Tool name to the servers that offer it. Order follows the upstream list.
pub fn federate(upstreams: &[Upstream]) -> HashMap<String, Vec<String>> {
    let mut pools: HashMap<String, Vec<String>> = HashMap::new();
    for upstream in upstreams {
        for tool in &upstream.tools {
            pools
                .entry(tool.clone())
                .or_default()
                .push(upstream.server.clone());
        }
    }
    pools
}

pub fn pool_for(tool: &str, upstreams: &[Upstream]) -> Pool {
    let servers = federate(upstreams).remove(tool).unwrap_or_default();
    Pool::new(
        servers
            .into_iter()
            .map(|server| Backend::new(server.clone(), tool, 0))
            .collect(),
    )
}

struct Gate {
    inflight: Mutex<usize>,
    max_seen: Mutex<usize>,
    cv: Condvar,
}

impl Gate {
    fn enter(&self) {
        let mut inflight = self.inflight.lock().expect("inflight");
        while *inflight >= DISCOVERY_CONCURRENCY {
            inflight = self.cv.wait(inflight).expect("inflight");
        }
        *inflight += 1;
        let mut max_seen = self.max_seen.lock().expect("max");
        *max_seen = (*max_seen).max(*inflight);
    }

    fn leave(&self) {
        let mut inflight = self.inflight.lock().expect("inflight");
        *inflight = inflight.saturating_sub(1);
        self.cv.notify_one();
    }
}

/// Connect `n` upstreams with at most `DISCOVERY_CONCURRENCY` in flight.
/// `hold` is the test's overlap. A zero hold is still bounded. Returns max in flight.
pub fn discover(n: usize, hold: Duration) -> usize {
    let gate = Gate {
        inflight: Mutex::new(0),
        max_seen: Mutex::new(0),
        cv: Condvar::new(),
    };
    thread::scope(|scope| {
        for _ in 0..n {
            scope.spawn(|| {
                gate.enter();
                if !hold.is_zero() {
                    thread::sleep(hold);
                }
                gate.leave();
            });
        }
    });
    let seen = *gate.max_seen.lock().expect("max");
    seen
}

/// Scripted upstream replies. `call` does not open a socket.
pub struct FakeModel {
    pub reply: String,
}

impl FakeModel {
    pub fn complete(&self) -> String {
        self.reply.clone()
    }
}

pub fn demo() -> String {
    let upstreams = [
        Upstream {
            server: "alpha".into(),
            tools: vec!["echo".into()],
        },
        Upstream {
            server: "beta".into(),
            tools: vec!["echo".into()],
        },
    ];
    let pool = pool_for("echo", &upstreams);
    let model = FakeModel {
        reply: "echo".into(),
    };
    let picked = pool
        .call(Policy::LeastOutstanding)
        .unwrap_or_else(|| model.complete());
    format!("pick: {picked}")
}
