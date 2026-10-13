use crate::conversation::Conversation;
use crate::event::{AgentUsage, Event};
use crate::message::Message;
use crate::provider::Provider;
use crate::session::{SessionInbox, INBOX_BOUND};
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
            Some((name, arg)) => (name, arg.trim()),
            None => (rest, ""),
        };
        Some(match (name, arg) {
            ("help", _) => ChatCommand::Help,
            ("reset", _) => ChatCommand::Reset,
            ("history", _) => ChatCommand::History,
            ("quit" | "exit", _) => ChatCommand::Quit,
            ("save", arg) if !arg.is_empty() => ChatCommand::Save(PathBuf::from(arg)),
            ("load", arg) if !arg.is_empty() => ChatCommand::Load(PathBuf::from(arg)),
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
    pub inbox: SessionInbox,
    provider: P,
    max_tokens: usize,
    pub usage: AgentUsage,
    pub last_events: Vec<Event>,
}

impl<P: Provider> ChatSession<P> {
    pub fn new(provider: P, system: &str, max_tokens: usize) -> Self {
        Self {
            conversation: Conversation::new(system),
            inbox: SessionInbox::new(INBOX_BOUND),
            provider,
            max_tokens,
            usage: AgentUsage::default(),
            last_events: Vec::new(),
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
                Err(error) => Outcome::Info(format!("error: {error}")),
            },
            ChatCommand::Load(path) => match Conversation::load(&path) {
                Ok(conv) => {
                    let n = conv.history().len();
                    self.conversation = conv;
                    Outcome::Info(format!("Loaded {n} messages from {}", path.display()))
                }
                Err(error) => Outcome::Info(format!("error: {error}")),
            },
            ChatCommand::Unknown(name) => {
                Outcome::Info(format!("Unknown command /{name}. Try /help."))
            }
        }
    }

    async fn ask(&mut self, text: &str) -> Outcome {
        if let Err(error) = self.inbox.submit(text) {
            return Outcome::Info(format!("error: {error}"));
        }
        let text = self.inbox.pop().expect("a turn was just submitted");
        if let Err(error) = self.conversation.push(Message::user(text)) {
            return Outcome::Info(format!("error: {error}"));
        }
        let trimmed = self.conversation.trim_to(self.max_tokens);
        let messages = self.conversation.to_messages();
        match self.provider.complete_events(&messages).await {
            Ok(events) => {
                let mut content = String::new();
                let mut assistant = None;
                for event in &events {
                    match event {
                        Event::TextDelta(text) => content.clone_from(text),
                        Event::Usage(usage) => self.usage.record(usage),
                        Event::Done { message, .. } => {
                            content.clone_from(&message.content);
                            assistant = Some(message.clone());
                        }
                        Event::ToolCall { .. } | Event::Error(_) => {}
                    }
                }
                let reply = assistant.unwrap_or_else(|| Message::assistant(content.clone()));
                self.last_events = events;
                if let Err(error) = self.conversation.push(reply) {
                    return Outcome::Info(format!("error: {error}"));
                }
                if trimmed > 0 {
                    Outcome::Reply(format!("{content}\n[trimmed {trimmed} old messages]"))
                } else {
                    Outcome::Reply(content)
                }
            }
            Err(error) => {
                self.conversation.pop_last_user();
                self.last_events.clear();
                Outcome::Info(format!("error: {error}"))
            }
        }
    }

    fn render_history(&self) -> String {
        let mut out = format!("[system] {}\n", self.conversation.system());
        for message in self.conversation.history() {
            out.push_str(&format!(
                "[{}] {}\n",
                message.role.as_str(),
                message.content
            ));
        }
        out.push_str(&format!("(~{} tokens)", self.conversation.approx_tokens()));
        out
    }
}
