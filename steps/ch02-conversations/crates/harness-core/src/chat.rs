use crate::conversation::Conversation;
use crate::message::Message;
use crate::provider::Provider;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
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
    /// `None` means "not a command, send it to the model".
    pub fn parse(line: &str) -> Option<ChatCommand> {
        let rest = line.trim().strip_prefix('/')?;
        let (name, arg) = match rest.split_once(char::is_whitespace) {
            Some((n, a)) => (n, a.trim()),
            None => (rest, ""),
        };
        Some(match (name, arg) {
            ("help", _) => ChatCommand::Help,
            ("reset", _) => ChatCommand::Reset,
            ("history", _) => ChatCommand::History,
            ("quit" | "exit", _) => ChatCommand::Quit,
            ("save", a) if !a.is_empty() => ChatCommand::Save(PathBuf::from(a)),
            ("load", a) if !a.is_empty() => ChatCommand::Load(PathBuf::from(a)),
            (other, _) => ChatCommand::Unknown(other.to_string()),
        })
    }
}

/// What the REPL should do after handling one line.
#[derive(Debug, PartialEq)]
pub enum Outcome {
    Reply(String),
    Info(String),
    Quit,
}

pub const HELP: &str = "\
/help            show this help
/reset           forget the conversation (keeps the system prompt)
/history         show the conversation so far
/save <file>     save the conversation as JSON
/load <file>     load a conversation from JSON
/quit            leave";

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
        let line = line.trim();
        if line.is_empty() {
            return Outcome::Info(String::new());
        }
        match ChatCommand::parse(line) {
            Some(command) => self.run_command(command),
            None => self.ask(line).await,
        }
    }

    fn run_command(&mut self, command: ChatCommand) -> Outcome {
        match command {
            ChatCommand::Help => Outcome::Info(HELP.to_string()),
            ChatCommand::Quit => Outcome::Quit,
            ChatCommand::Reset => {
                self.conversation.reset();
                Outcome::Info("Conversation cleared.".into())
            }
            ChatCommand::History => Outcome::Info(self.render_history()),
            ChatCommand::Save(path) => match self.conversation.save(&path) {
                Ok(()) => Outcome::Info(format!("Saved to {}", path.display())),
                Err(e) => Outcome::Info(format!("error: {e}")),
            },
            ChatCommand::Load(path) => match Conversation::load(&path) {
                Ok(conv) => {
                    let n = conv.history().len();
                    self.conversation = conv;
                    Outcome::Info(format!("Loaded {n} messages from {}", path.display()))
                }
                Err(e) => Outcome::Info(format!("error: {e}")),
            },
            ChatCommand::Unknown(name) => {
                Outcome::Info(format!("Unknown command /{name}. Try /help."))
            }
        }
    }

    async fn ask(&mut self, text: &str) -> Outcome {
        if let Err(e) = self.conversation.push(Message::user(text)) {
            return Outcome::Info(format!("error: {e}"));
        }
        let trimmed = self.conversation.trim_to(self.max_tokens);
        let messages = self.conversation.to_messages();
        match self.provider.complete(&messages).await {
            Ok(reply) => {
                let content = reply.content.clone();
                // A provider that misbehaves can't corrupt our history.
                if let Err(e) = self.conversation.push(Message::assistant(reply.content)) {
                    return Outcome::Info(format!("error: {e}"));
                }
                if trimmed > 0 {
                    Outcome::Reply(format!("{content}\n[trimmed {trimmed} old messages]"))
                } else {
                    Outcome::Reply(content)
                }
            }
            Err(e) => {
                // Roll back the user turn so the user can simply retry.
                self.conversation.pop_last_user();
                Outcome::Info(format!("error: {e}"))
            }
        }
    }

    fn render_history(&self) -> String {
        let mut out = format!("[system] {}\n", self.conversation.system());
        for m in self.conversation.history() {
            out.push_str(&format!("[{}] {}\n", m.role.as_str(), m.content));
        }
        out.push_str(&format!("(~{} tokens)", self.conversation.approx_tokens()));
        out
    }
}
