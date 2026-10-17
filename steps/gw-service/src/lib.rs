//! Book chapter 38. The MCP gateway service, in this process. Replicas are
//! names. A session id picks one. Nothing here listens on a socket.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

pub const SAMPLING_MAX_RPM: usize = 10;
pub const SAMPLING_WINDOW_SECONDS: u64 = 60;
pub const ELICIT_TIMEOUT_SECONDS: u64 = 300;
pub const OSV_TIMEOUT_SECONDS: u64 = 10;
pub const OSV_CACHE_TTL_SECONDS: u64 = 3600;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reject {
    Draining,
    Quota,
    NoReplica,
    BadToken,
}

pub struct Service {
    replicas: Vec<(String, bool)>,
    routes: HashMap<String, String>,
    config: Arc<Config>,
    inflight_quota: HashMap<String, u32>,
    quota: u32,
    audit: Vec<String>,
    spans: Vec<String>,
    inflight: Vec<(u64, String)>,
    next_id: u64,
    draining: bool,
}

impl Service {
    pub fn new(names: &[&str], quota: u32) -> Self {
        Self {
            replicas: names
                .iter()
                .map(|name| ((*name).to_string(), true))
                .collect(),
            routes: HashMap::new(),
            config: Arc::new(Config { generation: 1 }),
            inflight_quota: HashMap::new(),
            quota,
            audit: Vec::new(),
            spans: Vec::new(),
            inflight: Vec::new(),
            next_id: 1,
            draining: false,
        }
    }

    pub fn config(&self) -> Arc<Config> {
        Arc::clone(&self.config)
    }

    pub fn reload(&mut self, generation: u64) -> Arc<Config> {
        self.config = Arc::new(Config { generation });
        Arc::clone(&self.config)
    }

    pub fn route(&self, session: &str) -> Option<&str> {
        self.routes.get(session).map(String::as_str)
    }

    pub fn place(&mut self, session: &str) -> Option<String> {
        if let Some(current) = self.routes.get(session) {
            if self.alive(current) {
                return Some(current.clone());
            }
        }
        let next = self.replicas.iter().find(|(_, alive)| *alive)?.0.clone();
        self.routes.insert(session.to_string(), next.clone());
        Some(next)
    }

    pub fn kill(&mut self, name: &str) {
        for (replica, alive) in &mut self.replicas {
            if replica == name {
                *alive = false;
            }
        }
        let sessions: Vec<String> = self
            .routes
            .iter()
            .filter(|(_, replica)| replica.as_str() == name)
            .map(|(session, _)| session.clone())
            .collect();
        for session in sessions {
            self.routes.remove(&session);
            self.place(&session);
        }
    }

    pub fn healthy(&self) -> bool {
        self.replicas.iter().any(|(_, alive)| *alive)
    }

    pub fn ready(&self) -> bool {
        self.healthy() && !self.draining
    }

    pub fn accept(&mut self, tenant: &str, session: &str, header: &str) -> Result<u64, Reject> {
        if !bearer_ok(header) {
            self.audit.push(format!("deny {tenant}"));
            return Err(Reject::BadToken);
        }
        if self.draining {
            self.audit.push(format!("drain {tenant}"));
            return Err(Reject::Draining);
        }
        let used = *self.inflight_quota.entry(tenant.to_string()).or_insert(0);
        if used >= self.quota {
            self.audit.push(format!("quota {tenant}"));
            return Err(Reject::Quota);
        }
        if self.place(session).is_none() {
            return Err(Reject::NoReplica);
        }
        self.inflight_quota
            .insert(tenant.to_string(), used.saturating_add(1));
        let id = self.next_id;
        self.next_id += 1;
        self.inflight.push((id, session.to_string()));
        self.spans.push(format!("gateway.call {id}"));
        self.audit.push(format!("accept {tenant} {session}"));
        Ok(id)
    }

    pub fn finish(&mut self, id: u64, tenant: &str) -> bool {
        let Some(index) = self.inflight.iter().position(|(call, _)| *call == id) else {
            return false;
        };
        self.inflight.remove(index);
        if let Some(used) = self.inflight_quota.get_mut(tenant) {
            *used = used.saturating_sub(1);
        }
        true
    }

    pub fn start_drain(&mut self) {
        self.draining = true;
    }

    pub fn inflight(&self) -> usize {
        self.inflight.len()
    }

    pub fn drained(&self) -> bool {
        self.draining && self.inflight.is_empty()
    }

    pub fn audit(&self) -> &[String] {
        &self.audit
    }

    pub fn spans(&self) -> &[String] {
        &self.spans
    }

    fn alive(&self, name: &str) -> bool {
        self.replicas
            .iter()
            .any(|(replica, alive)| replica == name && *alive)
    }
}

pub fn bearer_ok(header: &str) -> bool {
    let Some(token) = header.strip_prefix("Bearer ") else {
        return false;
    };
    !token.is_empty() && !token.contains(' ')
}

pub struct Sampler {
    stamps: VecDeque<u64>,
}

impl Sampler {
    pub fn new() -> Self {
        Self {
            stamps: VecDeque::new(),
        }
    }

    /// Sliding window of 60 seconds. The 11th request inside the window is refused.
    pub fn allow(&mut self, now: u64) -> bool {
        self.stamps
            .retain(|stamp| now.saturating_sub(*stamp) < SAMPLING_WINDOW_SECONDS);
        if self.stamps.len() >= SAMPLING_MAX_RPM {
            return false;
        }
        self.stamps.push_back(now);
        true
    }
}

impl Default for Sampler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elicit {
    Approval { timeout_seconds: u64 },
    Declined,
}

pub fn elicit(url_mode: bool) -> Elicit {
    if url_mode {
        Elicit::Declined
    } else {
        Elicit::Approval {
            timeout_seconds: ELICIT_TIMEOUT_SECONDS,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsvVerdict {
    Clean,
    Blocked(String),
    FailOpen,
    Skipped,
}

pub struct OsvCache {
    entries: HashMap<String, (u64, Option<String>)>,
}

impl OsvCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// `command` is checked only for `npx` and `uvx`. Network failure is not cached.
    /// A regular CVE id is ignored. `MAL-*` blocks. `timeout_seconds` is the
    /// Hermes budget and is not slept.
    pub fn check(
        &mut self,
        command: &str,
        package: &str,
        now: u64,
        network_ok: bool,
        ids: &[impl AsRef<str>],
    ) -> OsvVerdict {
        let _ = OSV_TIMEOUT_SECONDS;
        if command != "npx" && command != "uvx" {
            return OsvVerdict::Skipped;
        }
        let key = format!("{command}:{package}");
        if let Some((expiry, cached)) = self.entries.get(&key) {
            if *expiry > now {
                return match cached {
                    Some(id) => OsvVerdict::Blocked(id.clone()),
                    None => OsvVerdict::Clean,
                };
            }
        }
        if !network_ok {
            return OsvVerdict::FailOpen;
        }
        let malware = ids
            .iter()
            .map(|id| id.as_ref().to_string())
            .find(|id| id.starts_with("MAL-"));
        self.entries.insert(
            key,
            (now.saturating_add(OSV_CACHE_TTL_SECONDS), malware.clone()),
        );
        match malware {
            Some(id) => OsvVerdict::Blocked(id),
            None => OsvVerdict::Clean,
        }
    }
}

impl Default for OsvCache {
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

pub fn demo() -> String {
    let mut service = Service::new(&["a", "b"], 4);
    let route = service.place("s1").expect("a live replica");
    let _ = FakeModel { reply: "ok" }.complete(&route);
    format!("route: {route}")
}
