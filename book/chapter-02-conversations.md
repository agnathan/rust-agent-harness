# Chapter 2: Conversations

*Draft. Bracketed notes like [personal note] are where my own story goes.*

At the end of Chapter 1, my harness could do exactly one thing: send a prompt, print the reply, and forget everything. Ask it "What's my name?" right after telling it, and it has no idea, because nothing was remembered between calls.

That's not a bug in the model. Language-model APIs are *stateless*. The model remembers nothing between requests, so if you want a conversation, you send the whole history every time. "Memory", at this level, is just a list of messages that the program keeps and resends.

So this chapter is about that list, and about the one door a turn goes through before the model sees it. By the end I'll have:
- a `Conversation` type that holds a system prompt and an ordered history, and *refuses* to get into an invalid state
- a scriptable mock provider, so I can test multi-turn conversations with no network
- simple length tracking, plus trimming when the history gets too long
- saving and loading conversations as JSON
- `harness chat`: an interactive chat loop (a REPL) with `/help`, `/reset`, `/history`, `/save`, `/load` and `/quit`
- one session inbox, capacity 64, that the REPL submits to before it calls the provider
- the same `Event` values as chapter 1, including usage and cost when a server actually sent them
- OpenRouter as the default server, the same way chapter 1 configured it

Same rules as before: Rust is introduced just in time, in short sidebars. The runnable demo is `--mock`. A live call is optional, and it reads `OPENROUTER_API_KEY`.

> **How Hermes does it**
>
> Hermes writes the message-list rules down in `agent/AGENTS.md` ("Message-flow invariants"). Roles alternate (`agent/message_sanitization.py`). The system prompt is built once per session (`agent/system_prompt.py::build_system_prompt`) and stays byte-stable so a provider can cache that prefix. The only sanctioned rewrite of history is compression (`agent/context_compressor.py`). This chapter's `Conversation` is that list: private fields, a single `push` that refuses a bad alternation, and a system prompt the history cannot touch. Hermes stores the list in SQLite (`hermes_state.py`). I store JSON. The slash commands in Hermes live in one registry (`hermes_cli/commands.py`); mine are a small enum.
>
> Hermes also has one place a new turn arrives, rather than each client calling the model itself. This chapter's `SessionInbox` is that place. `harness chat` and `harness ask` submit a line, the session pops it, then the provider runs. The gateway, cron, and the editor integration later submit to this same inbox. They do not grow a second queue.

[personal note: how Chapter 1 went, whether you ran it against a real model, and what you wanted to do next]

The code for this chapter lives in `steps/ch02-conversations/`. It starts from the messages, the provider trait, and one pooled client, then adds the conversation and the inbox. A live call uses the chapter 1 rules: OpenRouter by default (`OPENROUTER_API_KEY`, model `openai/gpt-4o-mini`), and `OPENAI_API_KEY` only when `--base-url` points somewhere else.

## What Hermes taught me about conversations

Before writing code I went back to Hermes Agent to see what rules it follows for its message list. It turns out a conversation has more rules than "a list of strings", and Hermes writes them down. The developer guide for its agent core (`agent/AGENTS.md`) has a section called "Message-flow invariants". Three of them shaped this chapter.

**1. Roles alternate strictly.** Never two user messages in a row, never two assistant messages in a row. Hermes enforces this carefully because some providers reject conversations that break it, and others quietly behave worse. Its message sanitizer (`agent/message_sanitization.py`) notes that strict providers like Gemini and Claude respond to an alternation violation by hallucinating. A harness that lets bad histories through will produce odd bugs much later, far from the cause.

**2. The system prompt is stable.** Hermes builds the system prompt once per session (`agent/system_prompt.py::build_system_prompt`) and never changes it mid-conversation. The reason is money and speed. Providers cache the *prefix* of a conversation (prompt caching), so the second, third and hundredth requests can reuse work done on the first. Change the system prompt and the cached prefix no longer matches, so every following request pays full price. Hermes's root developer guide calls per-conversation prompt caching "sacred". Anything that needs to be injected mid-conversation goes into a user message or a tool result, never the system prompt.

**3. The only allowed rewrite of history is compression.** When a conversation gets too long, Hermes summarises older turns (`agent/context_compressor.py`). That's the single sanctioned way to change the past. My version this chapter is much cruder (drop the oldest exchanges), but it follows the same shape: the system prompt is never touched, and the most recent message always survives.

For context: Hermes stores conversations in SQLite (`hermes_state.py`, the `SessionDB`), and its slash commands are defined in one central registry (`hermes_cli/commands.py`). I'm using a JSON file and a simple enum, which is enough for now.

## The Conversation type

Here's the core of the chapter:

```rust
use crate::message::{Message, Role};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ConversationError {
    #[error("expected a {expected:?} message next, got {got:?}")]
    OutOfOrder { expected: Role, got: Role },
    #[error("system messages only go at the start")]
    SystemInHistory,
    #[error("could not read or write file: {0}")]
    Io(String),
    #[error("invalid conversation file: {0}")]
    Format(String),
}

/// A system prompt plus an ordered, alternating history of user/assistant turns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conversation {
    system: String,
    history: Vec<Message>,
}

impl Conversation {
    pub fn new(system: impl Into<String>) -> Self {
        Self { system: system.into(), history: Vec::new() }
    }

    pub fn system(&self) -> &str {
        &self.system
    }

    pub fn history(&self) -> &[Message] {
        &self.history
    }

    /// Which role is allowed to speak next.
    pub fn next_role(&self) -> Role {
        match self.history.last() {
            None => Role::User,
            Some(m) if m.role == Role::User => Role::Assistant,
            Some(_) => Role::User,
        }
    }

    /// Append a message, enforcing strict user/assistant alternation.
    pub fn push(&mut self, message: Message) -> Result<(), ConversationError> {
        if message.role == Role::System {
            return Err(ConversationError::SystemInHistory);
        }
        let expected = self.next_role();
        if message.role != expected {
            return Err(ConversationError::OutOfOrder { expected, got: message.role });
        }
        self.history.push(message);
        Ok(())
    }

    /// Undo a user turn that never got an answer (e.g. the API call failed).
    pub fn pop_last_user(&mut self) -> Option<Message> {
        if self.next_role() == Role::Assistant {
            self.history.pop()
        } else {
            None
        }
    }

    /// Everything the model sees: the system prompt first, then the history.
    pub fn to_messages(&self) -> Vec<Message> {
        let mut out = Vec::with_capacity(self.history.len() + 1);
        out.push(Message::system(self.system.clone()));
        out.extend(self.history.iter().cloned());
        out
    }

    /// Forget the history but keep the (stable) system prompt.
    pub fn reset(&mut self) {
        self.history.clear();
    }

    /// Rough token estimate: about 4 characters per token for English text.
    pub fn approx_tokens(&self) -> usize {
        let chars: usize = self.system.len() + self.history.iter().map(|m| m.content.len()).sum::<usize>();
        chars.div_ceil(4)
    }

    /// Drop the oldest user/assistant pairs until we fit in `max_tokens`.
    /// Never touches the system prompt and never drops the newest message.
    /// Returns how many messages were removed.
    pub fn trim_to(&mut self, max_tokens: usize) -> usize {
        let mut removed = 0;
        while self.approx_tokens() > max_tokens && self.history.len() > 2 {
            self.history.drain(0..2);
            removed += 2;
        }
        removed
    }

    pub fn save(&self, path: &Path) -> Result<(), ConversationError> {
        let json = serde_json::to_string_pretty(self).map_err(|e| ConversationError::Format(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| ConversationError::Io(e.to_string()))
    }

    /// Load and re-validate: a hand-edited file must still obey the rules.
    pub fn load(path: &Path) -> Result<Self, ConversationError> {
        let text = std::fs::read_to_string(path).map_err(|e| ConversationError::Io(e.to_string()))?;
        let raw: Conversation = serde_json::from_str(&text).map_err(|e| ConversationError::Format(e.to_string()))?;
        let mut checked = Conversation::new(raw.system);
        for message in raw.history {
            checked.push(message)?;
        }
        Ok(checked)
    }
}
```

The important design decision is that **the fields are private**. Outside this module, nobody can reach in and push a message onto `history`. The only way to add one is `push`, and `push` checks the rules. That gives me a guarantee: if you hold a `Conversation`, it's valid. Hermes enforces the same rules through careful code review and sanitizers; Rust's privacy rules let me enforce them through the type itself.

A few details:
- **The system prompt is stored separately** as a plain `String`, not as the first element of `history`. It can't be reordered, duplicated or trimmed away by accident. `to_messages` puts it in front every time, so it's byte-for-byte identical across requests, which is exactly what prompt caching needs.
- **`next_role`** answers "who speaks next?" The rule is simple: an empty history or a finished exchange means the user speaks next; an unanswered user message means the assistant does.
- **`load` re-validates.** It doesn't trust the JSON file. It rebuilds the conversation one `push` at a time, so a hand-edited file that breaks alternation is rejected with a clear error instead of producing a broken conversation.

> **Rust sidebar: Vec, ownership and borrowing**
> - `Vec<Message>` is a growable list that *owns* its messages. When the `Conversation` is dropped, so are they.
> - `history(&self) -> &[Message]` hands out a *borrow*: a read-only view of the messages that doesn't copy or give away anything. The caller can look but not change.
> - `push(&mut self, ...)` needs a *mutable* borrow. Rust allows either many readers or one writer at a time, never both. That rule is what stops "I was iterating over the history while something else appended to it" bugs at compile time.
> - `to_messages` *clones* because the provider needs a contiguous slice that starts with the system message. Cloning a few strings per request costs nothing next to a network call; I'll revisit it if it ever shows up in a profile.

> **Rust sidebar: enums and match**
> - `ConversationError` is an enum: an error is exactly one of these cases, and some cases carry data (`OutOfOrder { expected, got }`).
> - `match` must cover every case, and the compiler checks that it does. In `next_role`, `Some(m) if m.role == Role::User` is a *match guard*: a pattern plus an extra condition.
> - If I add a `Role::Tool` next chapter (spoiler: I will), every `match` that forgot about it becomes a compile error. That's how a growing codebase stays consistent.

### Counting tokens, roughly

Models have a context limit, and long conversations cost more, so the harness needs to know roughly how big the history is. Exact token counts depend on each model's tokenizer. For now I use the well-known approximation of about four characters per token. `approx_tokens` is honest about what it is, and `trim_to` uses it to drop the oldest *pair* of messages at a time (a user message plus its reply), so the history still starts with a user turn and still alternates.

This is the simplest possible form of what Hermes does far more carefully. Its compressor first prunes old tool outputs, then picks safe cut points, then has an auxiliary model write a summary, and it has a deterministic fallback if the summary fails. That comes much later. What matters now is the boundary: trimming happens *inside* `Conversation`, so the rest of the code never sees an invalid history.

## A mock that can hold a conversation

Chapter 1's `MockProvider` always gave the same answer. To test a multi-turn chat I need a mock that gives *different* answers in order, and can also fail on cue:

```rust
// ---------- Mock provider for tests ----------

/// Plays back scripted replies in order, then falls back to a default reply.
/// Records every conversation it was sent.
pub struct MockProvider {
    script: Mutex<VecDeque<Result<String, String>>>,
    default_reply: String,
    pub seen: Mutex<Vec<Vec<Message>>>,
}

impl MockProvider {
    /// Always answers with `reply`.
    pub fn new(reply: impl Into<String>) -> Self {
        Self { script: Mutex::new(VecDeque::new()), default_reply: reply.into(), seen: Mutex::new(Vec::new()) }
    }

    /// Answers with each reply in turn (then "(mock) no more replies").
    pub fn scripted<I, S>(replies: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mock = Self::new("(mock) no more replies");
        mock.script.lock().unwrap().extend(replies.into_iter().map(|r| Ok(r.into())));
        mock
    }

    /// Queue a failure, to test error handling.
    pub fn then_fail(self, message: impl Into<String>) -> Self {
        self.script.lock().unwrap().push_back(Err(message.into()));
        self
    }

    pub fn calls(&self) -> usize {
        self.seen.lock().unwrap().len()
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        self.seen.lock().unwrap().push(messages.to_vec());
        match self.script.lock().unwrap().pop_front() {
            Some(Ok(reply)) => Ok(Message::assistant(reply)),
            Some(Err(message)) => Err(ProviderError::Api { status: 500, body: message }),
            None => Ok(Message::assistant(self.default_reply.clone())),
        }
    }
}

// A shared reference to a provider is also a provider.
#[async_trait]
impl<P: Provider + ?Sized> Provider for std::sync::Arc<P> {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        (**self).complete(messages).await
    }
}

#[async_trait]
impl<P: Provider + ?Sized> Provider for Box<P> {
    async fn complete(&self, messages: &[Message]) -> Result<Message, ProviderError> {
        (**self).complete(messages).await
    }
}
```

`MockProvider::scripted(["Hi Daniel!", "You said your name is Daniel."])` answers the first two calls in order, and `.then_fail("overloaded")` queues an error. It still records every conversation it was sent, which turns out to be the most useful part: my tests can check *exactly what the model would have seen*.

The two `impl ... for Arc<P>` / `Box<P>` blocks at the end mean "a shared pointer to a provider is also a provider". That lets a test keep one handle to the mock (to inspect `seen`) while the chat session owns another. In the snapshot those wrappers also forward `complete_detailed` and `complete_events`. If they only forwarded `complete`, a scripted usage would disappear, because the trait's default event path would rebuild the reply with no usage and no model id.

The snapshot's mock also takes a latency, for the overhead benches, and `with_usage(model, usage)` attaches a scripted model id and a `Usage` (including `cost`). That usage is test data. `MockProvider::new` emits text then done, and no usage event.

> **Rust sidebar: a light touch of generics and lifetimes**
> - `scripted<I, S>(replies: I)` is *generic*. It accepts anything that can be iterated (`I: IntoIterator`) whose items can become strings (`S: Into<String>`). Arrays, vectors and iterators all work.
> - `impl<P: Provider + ?Sized> Provider for Arc<P>` is a *blanket implementation*. `?Sized` lets `P` be a trait object like `dyn Provider`.
> - Lifetimes appear here only implicitly. `fn history(&self) -> &[Message]` really means "the returned slice can't outlive `self`". The compiler infers that, so I didn't have to write `'a`. In Chapter 1, `ChatRequest<'a>` was the one place I had to spell it out.

## The chat session, separate from the terminal

The temptation was to write the chat loop straight into `main`: read a line, call the model, print the result. I didn't, because that code would be impossible to test. Instead, the logic lives in the library as a `ChatSession` that takes one line of input and returns an `Outcome`. The terminal part is a thin shell around it:

```rust
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
            ChatCommand::Unknown(name) => Outcome::Info(format!("Unknown command /{name}. Try /help.")),
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
                // Roll back the user turn so the user can simply retry.
                self.conversation.pop_last_user();
                self.last_events.clear();
                Outcome::Info(format!("error: {error}"))
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
```

Things worth pointing out:
- **`ChatCommand::parse` returns `Option<ChatCommand>`.** `None` means "this isn't a command, send it to the model". `Some(Unknown(..))` means "it looked like a command, but I don't know it". Splitting these cases means a typo like `/hsitory` gets a helpful message instead of being sent to the model as a question.
- **`Outcome`** tells the caller what to do: print a reply, print some information, or quit. The session never prints anything itself, which is why a test can drive it line by line.
- **Failure rolls back.** If the model call fails, the user message I just added is removed again with `pop_last_user`. Without that, the history would end with an unanswered user turn, the user's retry would be a *second* user turn in a row, and alternation would break. I only noticed this while writing the test for it, which is the best argument for writing that test.
- **`ChatSession<P: Provider>` is generic.** In tests `P` is `Arc<MockProvider>`; in the CLI it's `Box<dyn Provider>`, chosen at runtime. The session's code is the same for both.
- **The provider is called with a borrow.** `complete_events(&messages)` takes `&[Message]`. The session builds one `Vec` so the system prompt can sit in front of the history, then lends that vec to the provider. It does not clone the history a second time on the way out.

## One session inbox

`ask` does not call the provider with the line it was given. It submits that line to a `SessionInbox`, then pops. On a quiet REPL the queue was empty, so the pop is the line just submitted. If 64 turns are already waiting, `submit` returns an error and the provider is not called. The bound is the backpressure. The queue does not grow past it.

```rust
pub const INBOX_BOUND: usize = 64;

pub fn submit(&self, prompt: impl Into<String>) -> Result<(), InboxError> {
    let mut queue = self.queue.lock().expect("inbox");
    if queue.len() >= self.bound {
        return Err(InboxError::Full { bound: self.bound });
    }
    queue.push_back(prompt.into());
    Ok(())
}
```

The inbox is a `Mutex<VecDeque<String>>`. Push and pop are the whole critical section.

> **Rust sidebar: a sync mutex inside an async function**
> - `ask` is `async` because the provider call awaits the model. The queue itself does not wait for the model, so it does not need an async lock.
> - `std::sync::Mutex` is the right lock when the section is a couple of pointer writes. Holding it across an `.await` would be the bug. This `submit` returns before any `.await`.
> - `VecDeque` is a ring buffer. `push_back` plus `pop_front` is a queue. A `Vec` would also work, and would make "drop the oldest" shift every element.

Chapters 29 and 32–34 reuse this inbox. Actors, the gateway, cron, and the editor submit turns here. This chapter only has the REPL, and the REPL still goes through the queue so those later callers do not need a second type.

A plain mock reply has no usage, so `usage.calls` stays 0. When a test attaches a scripted `Usage` with `cost: Some(0.25)`, `ask` records it on `session.usage`. `0.25` is a test constant. It is not a bill from OpenRouter.

## The REPL

For reading lines I used `rustyline`, a readline-style library. It gives arrow-key editing and an up-arrow history for free, and it also works when input is piped in rather than typed, which matters for testing. The CLI gains a `chat` subcommand and a `--system` flag. The server flags are the chapter 1 client: OpenRouter is the default, `--api-key` has no env binding of its own (the program picks `OPENROUTER_API_KEY` or `OPENAI_API_KEY`), and `--model` defaults to `openai/gpt-4o-mini`.

```rust
#[derive(Parser)]
#[command(
    name = "harness",
    about = "A tiny agent harness",
    after_long_help = "The default server is OpenRouter (https://openrouter.ai/api/v1).\n\
A live call reads OPENROUTER_API_KEY. Another --base-url reads OPENAI_API_KEY\n\
instead, which is how OpenAI, Ollama, and a local fake server are configured.\n\
--api-key overrides either variable. --mock never uses a key."
)]
struct Cli {
    #[arg(long, env = "OPENAI_BASE_URL", default_value = OPENROUTER_BASE_URL)]
    base_url: String,

    /// API key. Overrides OPENROUTER_API_KEY or OPENAI_API_KEY.
    #[arg(long)]
    api_key: Option<String>,

    #[arg(long, env = "HARNESS_MODEL", default_value = DEFAULT_MODEL)]
    model: String,

    #[arg(long, default_value_t = 60)]
    timeout_secs: u64,

    #[arg(long)]
    mock: bool,

    #[arg(long, default_value = "You are a helpful assistant.")]
    system: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Send one prompt and print the reply
    Ask { prompt: String },
    /// Start an interactive chat
    Chat {
        /// Rough history budget in tokens before old turns are trimmed
        #[arg(long, default_value_t = 8_000)]
        max_tokens: usize,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", Event::error(&error).report_line());
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn make_provider(cli: &Cli) -> Result<Box<dyn Provider>, ProviderError> {
    if cli.mock {
        return Ok(Box::new(MockProvider::new("(mock) I heard you.")));
    }
    let openrouter_api_key = std::env::var("OPENROUTER_API_KEY").ok();
    let openai_api_key = std::env::var("OPENAI_API_KEY").ok();
    let endpoint = resolve_endpoint(EndpointInput {
        base_url: &cli.base_url,
        api_key_flag: cli.api_key.as_deref(),
        openrouter_api_key: openrouter_api_key.as_deref(),
        openai_api_key: openai_api_key.as_deref(),
        model: &cli.model,
        http_referer: cli.http_referer.as_deref(),
        title: cli.title.as_deref(),
    })?;
    Ok(Box::new(OpenAiCompatible::with_timeout(
        &endpoint,
        Duration::from_secs(cli.timeout_secs.max(1)),
    )))
}

async fn chat(provider: Box<dyn Provider>, system: &str, max_tokens: usize) -> Result<(), ReadlineError> {
    let mut session = ChatSession::new(provider, system, max_tokens);
    let mut editor = rustyline::DefaultEditor::new()?;
    println!("Chatting. Type /help for commands, /quit to leave.");

    loop {
        let line = match editor.readline("you> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(e) => return Err(e),
        };
        let _ = editor.add_history_entry(line.as_str());

        match session.handle_line(&line).await {
            Outcome::Reply(text) => {
                println!("assistant> {text}");
                for event in &session.last_events {
                    println!("{}", event.report_line());
                }
            }
            Outcome::Info(text) if text.is_empty() => {}
            Outcome::Info(text) => println!("{text}"),
            Outcome::Quit => break,
        }
    }
    println!("Bye.");
    Ok(())
}
```

`ask` uses the same session, so a one-shot prompt also goes through the inbox and comes back as events. The printer is the chapter 1 one: the reply, then the model, tokens, and cost when the server sent them, then one `event:` line each. A missing OpenRouter key is an error on stderr, including `event: error`, and the process exits non-zero.

The env strings are bound to locals before `.as_deref()`. Calling `.as_deref()` on the temporary from `var().ok()` would dangle, and the compiler says so.

The loop itself is a few lines:
1. Read a line.
2. Hand it to the session, which submits it to the inbox.
3. Print the reply and the event lines.

Ctrl-C and Ctrl-D (`Interrupted` / `Eof`) end the chat cleanly instead of crashing.

> **Rust sidebar: std::io and terminals**
> - Rust's standard library can read lines itself: `std::io::stdin().lines()` gives an iterator of `Result<String, io::Error>`. That's fine for piped input, but it gives a bare, uneditable prompt on a terminal.
> - `rustyline` wraps the terminal properly and falls back to plain line reading when stdin isn't a terminal.
> - Either way, reading input can fail, so it returns a `Result` and the `?` operator passes the error up.
> - `println!` writes to standard output and `eprintln!` to standard error. Errors go to stderr so they don't mix with output you might pipe somewhere else.

## Tests

`tests/conversation.rs` covers the rules. `tests/session.rs` covers the inbox. `tests/chat.rs` drives the session, including a full inbox that must not call the provider. The CLI crate has its own tests: mock output, a missing OpenRouter key, and a local fake that returns `cost`.

```rust
use harness_core::{Conversation, ConversationError, Message, Role};

#[test]
fn roles_must_alternate() {
    let mut conv = Conversation::new("Be brief.");
    conv.push(Message::user("hi")).unwrap();
    assert_eq!(
        conv.push(Message::user("hello?")),
        Err(ConversationError::OutOfOrder { expected: Role::Assistant, got: Role::User })
    );
    conv.push(Message::assistant("Hello!")).unwrap();
    assert_eq!(conv.push(Message::system("new rules")), Err(ConversationError::SystemInHistory));
    assert_eq!(conv.next_role(), Role::User);
}

#[test]
fn system_prompt_stays_first_and_unchanged() {
    let mut conv = Conversation::new("You are a careful assistant.");
    for i in 0..3 {
        conv.push(Message::user(format!("q{i}"))).unwrap();
        conv.push(Message::assistant(format!("a{i}"))).unwrap();
    }
    conv.trim_to(5);
    conv.reset();
    let msgs = conv.to_messages();
    assert_eq!(msgs[0], Message::system("You are a careful assistant."));
    assert_eq!(msgs.len(), 1);
}

#[test]
fn trimming_drops_oldest_pairs_but_keeps_latest() {
    let mut conv = Conversation::new("sys");
    for i in 0..5 {
        conv.push(Message::user(format!("question number {i} {}", "x".repeat(40)))).unwrap();
        conv.push(Message::assistant(format!("answer number {i} {}", "y".repeat(40)))).unwrap();
    }
    conv.push(Message::user("latest")).unwrap();
    let before = conv.approx_tokens();
    let removed = conv.trim_to(40);
    assert!(removed > 0 && removed % 2 == 0);
    assert!(conv.approx_tokens() < before);
    assert_eq!(conv.history().last().unwrap().content, "latest");
    assert_eq!(conv.history()[0].role, Role::User, "still starts with a user turn");
}

#[test]
fn save_and_load_round_trip() {
    let path = std::env::temp_dir().join(format!("conv-{}.json", std::process::id()));
    let mut conv = Conversation::new("sys");
    conv.push(Message::user("hi")).unwrap();
    conv.push(Message::assistant("hello")).unwrap();
    conv.save(&path).unwrap();
    assert_eq!(Conversation::load(&path).unwrap(), conv);

    // A hand-edited file that breaks the rules is rejected.
    std::fs::write(&path, r#"{"system":"s","history":[{"role":"assistant","content":"first?"}]}"#).unwrap();
    assert!(matches!(Conversation::load(&path), Err(ConversationError::OutOfOrder { .. })));
    std::fs::remove_file(&path).ok();
}
```

`tests/chat.rs` drives a whole multi-turn session through the scripted mock:

```rust
use harness_core::{ChatCommand, ChatSession, MockProvider, Outcome, Role};
use std::sync::Arc;

#[test]
fn parses_slash_commands() {
    assert_eq!(ChatCommand::parse("/help"), Some(ChatCommand::Help));
    assert_eq!(ChatCommand::parse("  /quit "), Some(ChatCommand::Quit));
    assert_eq!(ChatCommand::parse("/save chat.json"), Some(ChatCommand::Save("chat.json".into())));
    assert_eq!(ChatCommand::parse("/save"), Some(ChatCommand::Unknown("save".into())));
    assert_eq!(ChatCommand::parse("hello /help"), None);
}

#[tokio::test]
async fn multi_turn_chat_sends_full_history() {
    let mock = Arc::new(MockProvider::scripted(["Hi Daniel!", "You said your name is Daniel."]));
    let mut chat = ChatSession::new(mock.clone(), "Be friendly.", 4_000);

    assert_eq!(chat.handle_line("Hi, I'm Daniel").await, Outcome::Reply("Hi Daniel!".into()));
    assert_eq!(chat.handle_line("What's my name?").await, Outcome::Reply("You said your name is Daniel.".into()));

    let seen = mock.seen.lock().unwrap();
    let second_call = &seen[1];
    let roles: Vec<Role> = second_call.iter().map(|m| m.role).collect();
    assert_eq!(roles, [Role::System, Role::User, Role::Assistant, Role::User]);
    assert_eq!(second_call[0].content, "Be friendly.");
}

#[tokio::test]
async fn failed_call_rolls_back_the_user_turn() {
    let mock = Arc::new(MockProvider::scripted(["first"]).then_fail("overloaded"));
    let mut chat = ChatSession::new(mock.clone(), "sys", 4_000);
    chat.handle_line("one").await;
    let out = chat.handle_line("two").await;
    assert!(matches!(out, Outcome::Info(ref s) if s.contains("overloaded")));
    assert_eq!(chat.conversation.history().len(), 2, "the failed user turn was removed");
    // Retrying works because the history still alternates.
    assert!(matches!(chat.handle_line("two again").await, Outcome::Reply(_)));
}

#[tokio::test]
async fn reset_and_quit() {
    let mut chat = ChatSession::new(MockProvider::new("ok"), "sys", 4_000);
    chat.handle_line("hello").await;
    assert_eq!(chat.handle_line("/reset").await, Outcome::Info("Conversation cleared.".into()));
    assert!(chat.conversation.history().is_empty());
    assert_eq!(chat.handle_line("/quit").await, Outcome::Quit);
}
```

The test I like most is `multi_turn_chat_sends_full_history`. It checks that the *second* request contained `[system, user, assistant, user]`, with the system prompt unchanged at the front. That's this chapter's core claim in one assertion: memory works because the full history is resent, and the prefix stays stable.

## Did it work?

`cargo nextest run -p ch02-harness-core` runs 22 tests: 7 chat, 4 conversation, 9 provider, and 2 session. The summary from the run on this machine:

```text
     Summary [   0.059s] 22 tests run: 22 passed, 0 skipped
```

The CLI crate has 3 tests of its own (mock events, a missing OpenRouter key, and a local fake that returns `cost`).

Then a scripted session, with the input piped in and `--mock` so nothing leaves the machine. Each reply is followed by the event lines. The mock did not report usage, so there is no `event: usage` line:

```text
$ printf 'hello there\n/history\n/save /tmp/c2.json\n/reset\n/history\n/load /tmp/c2.json\n/bogus\nsecond message\n/quit\n' \
    | cargo run -p ch02-harness-cli -- --mock chat
Chatting. Type /help for commands, /quit to leave.
you> assistant> (mock) I heard you.
event: text-delta (mock) I heard you.
event: done
you> [system] You are a helpful assistant.
[user] hello there
[assistant] (mock) I heard you.
(~15 tokens)
you> Saved to /tmp/c2.json
you> Conversation cleared.
you> [system] You are a helpful assistant.
(~7 tokens)
you> Loaded 2 messages from /tmp/c2.json
you> Unknown command /bogus. Try /help.
you> assistant> (mock) I heard you.
event: text-delta (mock) I heard you.
event: done
you> Bye.
```

The saved file is the JSON `save` actually wrote:

```json
{
  "system": "You are a helpful assistant.",
  "history": [
    {
      "role": "user",
      "content": "hello there"
    },
    {
      "role": "assistant",
      "content": "(mock) I heard you."
    }
  ]
}
```

### Milestone demo

The demo is the mock. I did not make a live call.

```bash
cargo run -p ch02-harness-core --example demo
cargo run -p ch02-harness-cli -- --mock ask "Say hello"
```

The example walks two scripted turns through the inbox and prints the ledger. `calls=0` and `cost_known=false` because this mock did not attach a usage. That is the point: the harness does not invent a bill.

```text
you> Hello
assistant> Hi.
event: text-delta Hi.
event: done
you> Do you remember me?
assistant> Still here.
event: text-delta Still here.
event: done
agent: calls=0 cost_known=false
```

`--mock ask` prints the same shape:

```text
(mock) I heard you.
event: text-delta (mock) I heard you.
event: done
```

A live chat is the same `cargo run` without `--mock`, with `OPENROUTER_API_KEY` set. Another server is `--base-url` plus `OPENAI_API_KEY`. I did not make that live call. A missing OpenRouter key fails before any request:

```text
event: error missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
error: missing OPENROUTER_API_KEY: pass --api-key, or run with --mock
```

[personal note: a real chat session, e.g. testing whether the model remembers something said three turns earlier]

## What we built

- **`Conversation`:** a system prompt plus an alternating history, with the rules enforced by the type itself. Private fields, a single `push` gate, and re-validation on load.
- **A stable system prompt** kept separate from history and resent identically every time, so it's friendly to prompt caching, as Hermes insists.
- **Rough token tracking and pair-wise trimming** that never drops the system prompt or the newest message.
- **JSON save and load.**
- **A scriptable `MockProvider`** that can also fail on cue, and can attach a scripted usage when a test needs one.
- **`SessionInbox`:** one bounded queue (64) in front of the provider. A full inbox refuses the turn. The gateway, cron, and the editor will submit here later.
- **`ChatSession`:** testable chat logic, separate from the terminal, with rollback when a model call fails. It records `Event` values and, when a server sent them, tokens and cost.
- **`harness chat` and `harness ask`:** OpenRouter by default, `--mock` with no key, and the same event lines as chapter 1.

New Rust along the way: private fields as a way to enforce rules, borrowing slices, match guards, `Option` vs `Result`, generics with trait bounds, blanket implementations, a sync mutex inside an async function, and the basics of terminal input.

[personal note: which Rust idea finally made sense in this chapter, and which still feels shaky]

## Build, run, and benchmark

From the repository root. The library is `ch02-harness-core`. The binary crate is `ch02-harness-cli`, and the executable name is `ch02-harness`.

```bash
cargo build -p ch02-harness-core
cargo build -p ch02-harness-cli
cargo nextest run -p ch02-harness-core
cargo nextest run -p ch02-harness-cli
cargo run -p ch02-harness-core --example demo
cargo run -p ch02-harness-cli -- --mock ask "Say hello"
printf 'hello there\n/history\n/save /tmp/c2.json\n/reset\n/history\n/load /tmp/c2.json\n/bogus\nsecond message\n/quit\n' \
  | cargo run -p ch02-harness-cli -- --mock chat
cargo bench -p ch02-harness-core --bench inbox -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

`--mock` is the offline provider. A live chat is the same command without `--mock`:

```bash
OPENROUTER_API_KEY=sk-or-... \
  cargo run -p ch02-harness-cli -- chat
```

I did not make that live call. `--help` on this machine:

```text
A tiny agent harness

Usage: ch02-harness [OPTIONS] <COMMAND>

Commands:
  ask   Send one prompt and print the reply
  chat  Start an interactive chat
  help  Print this message or the help of the given subcommand(s)

Options:
      --base-url <BASE_URL>
          Base URL of an OpenAI-compatible API. Default is OpenRouter

          [env: OPENAI_BASE_URL=]
          [default: https://openrouter.ai/api/v1]

      --api-key <API_KEY>
          API key. Overrides OPENROUTER_API_KEY or OPENAI_API_KEY

      --model <MODEL>
          OpenRouter model id, such as openai/gpt-4o-mini or anthropic/claude-3.5-sonnet

          [env: HARNESS_MODEL=]
          [default: openai/gpt-4o-mini]

      --http-referer <HTTP_REFERER>
          Optional OpenRouter HTTP-Referer attribution header

          [env: OPENROUTER_HTTP_REFERER=]

      --title <TITLE>
          Optional OpenRouter X-Title attribution header

          [env: OPENROUTER_TITLE=]

      --timeout-secs <TIMEOUT_SECS>
          Whole-request timeout in seconds. A timeout is not retried

          [default: 60]

      --mock
          Use the offline mock provider instead of HTTP

      --system <SYSTEM>
          System prompt

          [default: "You are a helpful assistant."]

  -h, --help
          Print help (see a summary with '-h')

The default server is OpenRouter (https://openrouter.ai/api/v1).
A live call reads OPENROUTER_API_KEY. Another --base-url reads OPENAI_API_KEY
instead, which is how OpenAI, Ollama, and a local fake server are configured.
--api-key overrides either variable. --mock never uses a key.

`chat` submits each line to the session inbox, then the session pops it and
calls the provider. The same inbox is what later chapters' gateway, cron, and
editor integration submit to.
```

The two builds, on the same 4-vCPU Intel Xeon guest (KVM, 16 GiB, Linux, rustc 1.99.0, cargo-nextest 0.9.148):

```text
   Compiling ch02-harness-core v0.2.0 (/workspace/steps/ch02-conversations/crates/harness-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
```

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
```

The library tests:

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.30s
────────────
 Nextest run ID b59894ab-d22a-4cce-b5fd-a22d70da72fb with nextest profile: default
    Starting 22 tests across 5 binaries
        PASS [   0.003s] ( 1/22) ch02-harness-core::chat multi_turn_chat_sends_full_history
        PASS [   0.004s] ( 2/22) ch02-harness-core::chat a_full_inbox_does_not_call_the_provider
        PASS [   0.004s] ( 3/22) ch02-harness-core::chat failed_call_rolls_back_the_user_turn
        PASS [   0.004s] ( 4/22) ch02-harness-core::chat mock_events_have_no_usage
        PASS [   0.003s] ( 5/22) ch02-harness-core::chat reset_and_quit
        PASS [   0.004s] ( 6/22) ch02-harness-core::chat parses_slash_commands
        PASS [   0.005s] ( 7/22) ch02-harness-core::conversation roles_must_alternate
        PASS [   0.005s] ( 8/22) ch02-harness-core::chat scripted_usage_lands_on_the_session_ledger
        PASS [   0.005s] ( 9/22) ch02-harness-core::conversation save_and_load_round_trip
        PASS [   0.003s] (10/22) ch02-harness-core::conversation system_prompt_stays_first_and_unchanged
        PASS [   0.003s] (11/22) ch02-harness-core::conversation trimming_drops_oldest_pairs_but_keeps_latest
        PASS [   0.003s] (12/22) ch02-harness-core::provider another_base_url_uses_the_openai_key
        PASS [   0.004s] (13/22) ch02-harness-core::provider mock_events_are_text_then_done
        PASS [   0.005s] (14/22) ch02-harness-core::provider mock_provider_answers_and_records
        PASS [   0.007s] (15/22) ch02-harness-core::provider local_fake_reports_cost_and_the_answered_model
        PASS [   0.005s] (16/22) ch02-harness-core::provider openrouter_is_the_default_and_names_its_key
        PASS [   0.004s] (17/22) ch02-harness-core::provider request_serializes_to_openai_shape
        PASS [   0.004s] (18/22) ch02-harness-core::provider response_deserializes_from_openai_shape
        PASS [   0.003s] (19/22) ch02-harness-core::session a_full_inbox_refuses_the_next_turn
        PASS [   0.003s] (20/22) ch02-harness-core::provider scripted_cost_accumulates_without_a_network
        PASS [   0.003s] (21/22) ch02-harness-core::session submit_then_pop_is_fifo
        PASS [   0.046s] (22/22) ch02-harness-core::provider mock_latency_is_configurable
────────────
     Summary [   0.059s] 22 tests run: 22 passed, 0 skipped
```

The CLI tests:

```text
────────────
 Nextest run ID 66c64a1d-93cf-4c0b-a00e-d508d7cf8d7b with nextest profile: default
    Starting 3 tests across 2 binaries
        PASS [   0.006s] (1/3) ch02-harness-cli::ask mock_ask_prints_events_and_no_invented_usage
        PASS [   0.006s] (2/3) ch02-harness-cli::ask missing_openrouter_key_is_a_clear_error
        PASS [   0.008s] (3/3) ch02-harness-cli::ask ask_prints_scripted_cost_from_a_local_fake
────────────
     Summary [   0.009s] 3 tests run: 3 passed, 0 skipped
```

One mock ask, and the two-turn example, are the blocks in the milestone section above. The piped chat wrote the JSON shown there.

The bench, with a 1 second warm-up, 2 seconds of measurement, and 10 samples, matching the Benchmarks workflow:

```text
    Finished `bench` profile [optimized] target(s) in 2.50s
     Running benches/inbox.rs (target/release/deps/inbox-4f44420eb33935fe)
Benchmarking inbox_submit
Benchmarking inbox_submit: Warming up for 1.0000 s
Benchmarking inbox_submit: Collecting 10 samples in estimated 2.0000 s (33M iterations)
Benchmarking inbox_submit: Analyzing
inbox_submit            time:   [32.057 ns 33.626 ns 35.008 ns]

Benchmarking chat_turn_overhead
Benchmarking chat_turn_overhead: Warming up for 1.0000 s
Benchmarking chat_turn_overhead: Collecting 10 samples in estimated 2.0000 s (2.2M iterations)
Benchmarking chat_turn_overhead: Analyzing
chat_turn_overhead      time:   [599.91 ns 616.27 ns 632.80 ns]
```

The `time:` line is Criterion's slope. The means below are the mean point estimates from `target/criterion/inbox_submit/new/estimates.json` and `target/criterion/chat_turn_overhead/new/estimates.json` on this same run.

| Benchmark | Mean | Target | Machine |
|---|---|---|---|
| `inbox_submit` | 32.78 ns | < 1 µs | 4 vCPU Intel Xeon (KVM), 16 GiB, Linux, rustc 1.99.0, sample size 10 |
| `chat_turn_overhead` | 628.2 ns | < 1 ms | same |

`inbox_submit` is one `submit` into an empty inbox. The setup of the inbox is outside the measurement. `chat_turn_overhead` is one mock turn: submit, pop, append the user line, call the provider, append the reply. Model latency is zero. Both are under the budget.

CI runs these tests on Ubuntu, macOS, and Windows. Criterion for this bench, and the chapter 1 benches, runs on Ubuntu in the [Benchmarks workflow](https://github.com/agnathan/rust-agent-harness/actions/workflows/benchmarks.yml). The median, low, and high land in the job summary and in `criterion-summary-<commit>.json`. A push to `main` appends the run to the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/).

## What's next

So far the model can only talk. In **Chapter 3: Tools**, it gets a way to *act* without this chapter's chat loop becoming the agent loop yet. A tool is MCP-shaped: a name, a description, a JSON Schema for the arguments, and a content result. A registry holds them. Named toolsets turn on only when their environment variables and checks pass, and the model is told only about the tools that passed. Read-only calls in one batch run together, up to a bound of 8. A write runs alone. The loop that asks the model which tool to call, and the `tool` messages that come back, wait until the agent loop.
