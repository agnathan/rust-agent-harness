use clap::{Parser, Subcommand};
use harness_core::{
    ChatSession, Message, MockProvider, OpenAiCompatible, Outcome, Provider, ProviderError,
};
use rustyline::error::ReadlineError;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "harness", about = "A tiny agent harness")]
struct Cli {
    /// Base URL of an OpenAI-compatible API
    #[arg(
        long,
        env = "OPENAI_BASE_URL",
        default_value = "https://api.openai.com/v1"
    )]
    base_url: String,

    /// API key (local servers like Ollama accept any value)
    #[arg(long, env = "OPENAI_API_KEY", default_value = "")]
    api_key: String,

    /// Model name
    #[arg(long, env = "HARNESS_MODEL", default_value = "gpt-4o-mini")]
    model: String,

    /// Use the offline mock provider instead of a real API
    #[arg(long)]
    mock: bool,

    /// System prompt
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
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn make_provider(cli: &Cli) -> Box<dyn Provider> {
    if cli.mock {
        Box::new(MockProvider::new("(mock) I heard you."))
    } else {
        Box::new(OpenAiCompatible::new(
            &cli.base_url,
            &cli.api_key,
            &cli.model,
        ))
    }
}

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let provider = make_provider(&cli);
    match cli.command {
        Command::Ask { prompt } => {
            let messages = vec![Message::system(cli.system.as_str()), Message::user(prompt)];
            let reply: Result<Message, ProviderError> = provider.complete(&messages).await;
            println!("{}", reply?.content);
        }
        Command::Chat { max_tokens } => chat(provider, &cli.system, max_tokens).await?,
    }
    Ok(())
}

async fn chat(
    provider: Box<dyn Provider>,
    system: &str,
    max_tokens: usize,
) -> Result<(), ReadlineError> {
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
            Outcome::Reply(text) => println!("assistant> {text}"),
            Outcome::Info(text) if text.is_empty() => {}
            Outcome::Info(text) => println!("{text}"),
            Outcome::Quit => break,
        }
    }
    println!("Bye.");
    Ok(())
}
