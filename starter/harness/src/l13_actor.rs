//! Lesson 13: actor-per-agent. A tokio task owns the agent state; others talk to it via a bounded mpsc inbox.
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

pub enum AgentMsg {
    Ask {
        text: String,
        reply: oneshot::Sender<String>,
    },
    Steer(String),
    Stop,
}

#[derive(Debug, PartialEq)]
pub enum SendError {
    /// Inbox full (backpressure!)
    Busy,
    /// Actor has stopped
    Gone,
}

#[derive(Clone)]
pub struct AgentHandle {
    pub name: String,
    tx: mpsc::Sender<AgentMsg>,
}

/// Spawn the actor. Loop over the inbox:
/// - Steer(t): remember t (Vec<String>)
/// - Ask: answer = brain(&text, &steers); clear steers; send answer on reply (ignore send errors); count it
/// - Stop or inbox closed: exit
/// The JoinHandle resolves to the number of Asks answered.
pub fn spawn_agent<F>(name: &str, capacity: usize, mut brain: F) -> (AgentHandle, JoinHandle<usize>)
where
    F: FnMut(&str, &[String]) -> String + Send + 'static,
{
    // Hint: let (tx, mut rx) = mpsc::channel(capacity); tokio::spawn(async move { ... })
    todo!("lesson 13")
}

impl AgentHandle {
    /// Send Ask and await the reply. Err(Gone) if the actor is gone or dropped the reply.
    pub async fn ask(&self, text: &str) -> Result<String, SendError> {
        // Hint: oneshot::channel(); tx.send(..).await; rx.await
        todo!("lesson 13")
    }
    /// Non-blocking steer: Busy if the inbox is full, Gone if closed.
    pub fn try_steer(&self, text: &str) -> Result<(), SendError> {
        // Hint: tx.try_send; match mpsc::error::TrySendError::{Full, Closed}
        todo!("lesson 13")
    }
    pub async fn stop(&self) {
        let _ = self.tx.send(AgentMsg::Stop).await;
    }
}
