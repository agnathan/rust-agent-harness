use crate::conversation::Conversation;
use crate::provider::Provider;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum ChatCommand {
    Help,
    Reset,
    History,
    Quit,
    Save(PathBuf),
    Load(PathBuf),
    Unknown(String),
}

impl ChatCommand {
    pub fn parse(line: &str) -> Option<ChatCommand> {
        // None when the line is not a slash command.
        // /save and /load need a path; without one they are Unknown.
        let _ = line;
        todo!("chapter 02")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Reply(String),
    Info(String),
    Quit,
}

pub struct ChatSession<P: Provider> {
    pub conversation: Conversation,
    provider: P,
    max_tokens: usize,
}

impl<P: Provider> ChatSession<P> {
    pub fn new(provider: P, system: &str, max_tokens: usize) -> Self {
        Self {
            conversation: Conversation::new(system),
            provider,
            max_tokens,
        }
    }

    pub async fn handle_line(&mut self, line: &str) -> Outcome {
        // Slash commands, or one user turn plus the provider reply.
        // A failed provider call removes the user turn that was just pushed.
        // /reset clears history and returns Info("Conversation cleared.").
        // /quit returns Outcome::Quit.
        let _ = line;
        todo!("chapter 02")
    }
}
