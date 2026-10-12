//! Lesson 15: a message bus (direct + topics), a blackboard, and an orchestrator-worker team.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};

#[derive(Debug, Clone, PartialEq)]
pub enum Address {
    Agent(String),
    Topic(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Envelope {
    pub from: String,
    pub to: Address,
    pub corr_id: u64,
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub enum RouteError {
    UnknownAgent(String),
    AgentGone(String),
}

#[derive(Default)]
pub struct Router {
    agents: Mutex<HashMap<String, mpsc::Sender<Envelope>>>,
    topics: Mutex<HashMap<String, broadcast::Sender<Envelope>>>,
}

impl Router {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
    /// Register an agent inbox with `capacity`; returns the receiving end.
    pub fn register(&self, name: &str, capacity: usize) -> mpsc::Receiver<Envelope> {
        // Hint: mpsc::channel; insert the Sender into the map (lock().unwrap())
        todo!("lesson 15")
    }
    /// Subscribe to a topic (create its broadcast channel with capacity 64 if new).
    pub fn subscribe(&self, topic: &str) -> broadcast::Receiver<Envelope> {
        // Hint: entry(..).or_insert_with(..).subscribe()
        todo!("lesson 15")
    }

    /// Agent: deliver to its inbox (await = backpressure). Unknown -> UnknownAgent, closed -> AgentGone.
    /// Topic: broadcast; returns number of receivers (0 if none / topic unknown).
    /// IMPORTANT: clone the Sender and drop the lock BEFORE awaiting (never hold a std Mutex across .await).
    pub async fn send(&self, env: Envelope) -> Result<usize, RouteError> {
        // Hint: match env.to.clone()
        todo!("lesson 15")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    Claimed(String),
    Done(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoardTask {
    pub id: u64,
    pub title: String,
    pub status: TaskStatus,
}

/// A shared task board (Hermes's kanban, in-process).
#[derive(Default)]
pub struct Blackboard {
    tasks: Mutex<Vec<BoardTask>>,
}

impl Blackboard {
    /// Add a Todo task; ids start at 1 and increase.
    pub fn add(&self, title: &str) -> u64 {
        // Hint: id = len + 1
        todo!("lesson 15")
    }
    /// Atomically claim the first Todo task for `worker` (status -> Claimed(worker)); return a copy.
    pub fn claim(&self, worker: &str) -> Option<BoardTask> {
        // Hint: iter_mut().find(..)
        todo!("lesson 15")
    }
    /// Complete a task that is Claimed. Err("no such task") / Err("not claimed").
    pub fn complete(&self, id: u64, result: &str) -> Result<(), String> {
        // Hint: find by id, check status
        todo!("lesson 15")
    }
    pub fn snapshot(&self) -> Vec<BoardTask> {
        self.tasks.lock().unwrap().clone()
    }
}

/// Orchestrator-worker over the Router:
/// - register "orchestrator" plus one inbox per worker name; spawn a task per worker that replies to
///   each Envelope with body "<worker>:<body>" (same corr_id) addressed to Agent("orchestrator")
/// - send task i (corr_id = i) to workers[i % workers.len()]
/// - collect tasks.len() replies, return bodies ordered by corr_id
pub async fn orchestrate(router: Arc<Router>, workers: &[&str], tasks: Vec<String>) -> Vec<String> {
    // Hint: each worker: while let Some(env) = rx.recv().await { router.send(reply).await }
    todo!("lesson 15")
}
