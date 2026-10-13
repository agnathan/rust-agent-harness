use clap::{Parser, Subcommand};
use harness_core::{
    format_cost, resolve_endpoint, AgentUsage, ChatSession, EndpointInput, Event, MockProvider,
    OpenAiCompatible, Outcome, Provider, ProviderError, DEFAULT_MODEL, OPENROUTER_BASE_URL,
};
use rustyline::error::ReadlineError;
use std::process::ExitCode;
use std::time::Duration;

#[derive(Parser)]
#[command(
    name = "harness",
    about = "A tiny agent harness",
    after_long_help = "The default server is OpenRouter (https://openrouter.ai/api/v1).\n\
A live call reads OPENROUTER_API_KEY. Another --base-url reads OPENAI_API_KEY\n\
instead, which is how OpenAI, Ollama, and a local fake server are configured.\n\
--api-key overrides either variable. --mock never uses a key.\n\
\n\
`chat` submits each line to the session inbox, then the session pops it and\n\
calls the provider. The same inbox is what later chapters' gateway, cron, and\n\
editor integration submit to."
)]
struct Cli {
    /// Base URL of an OpenAI-compatible API. Default is OpenRouter.
    #[arg(long, env = "OPENAI_BASE_URL", default_value = OPENROUTER_BASE_URL)]
    base_url: String,

    /// API key. Overrides OPENROUTER_API_KEY or OPENAI_API_KEY.
    #[arg(long)]
    api_key: Option<String>,

    /// OpenRouter model id, such as openai/gpt-4o-mini or anthropic/claude-3.5-sonnet.
    #[arg(long, env = "HARNESS_MODEL", default_value = DEFAULT_MODEL)]
    model: String,

    /// Optional OpenRouter HTTP-Referer attribution header.
    #[arg(long, env = "OPENROUTER_HTTP_REFERER")]
    http_referer: Option<String>,

    /// Optional OpenRouter X-Title attribution header.
    #[arg(long, env = "OPENROUTER_TITLE")]
    title: Option<String>,

    /// Whole-request timeout in seconds. A timeout is not retried.
    #[arg(long, default_value_t = 60)]
    timeout_secs: u64,

    /// Use the offline mock provider instead of HTTP.
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
    /// Send one prompt and print the reply.
    Ask { prompt: String },
    /// Start an interactive chat.
    Chat {
        /// Rough history budget in tokens before old turns are trimmed.
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

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let provider = make_provider(&cli)?;
    match cli.command {
        Command::Ask { prompt } => {
            let mut session = ChatSession::new(provider, &cli.system, 8_000);
            match session.handle_line(&prompt).await {
                Outcome::Reply(_) => print_events(&session.last_events),
                Outcome::Info(text) => {
                    if let Some(message) = text.strip_prefix("error: ") {
                        return Err(message.to_string().into());
                    }
                    if !text.is_empty() {
                        println!("{text}");
                    }
                }
                Outcome::Quit => {}
            }
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
            Err(error) => return Err(error),
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
            Outcome::Info(text) => {
                if let Some(message) = text.strip_prefix("error: ") {
                    eprintln!("{}", Event::error(message).report_line());
                    eprintln!("{text}");
                } else {
                    println!("{text}");
                }
            }
            Outcome::Quit => break,
        }
    }
    println!("Bye.");
    Ok(())
}

fn print_events(events: &[Event]) {
    let mut ledger = AgentUsage::default();
    let mut content = String::new();
    let mut model = None;
    for event in events {
        match event {
            Event::TextDelta(text) => content.clone_from(text),
            Event::Done {
                message,
                model: answered,
            } => {
                content.clone_from(&message.content);
                model.clone_from(answered);
            }
            Event::Usage(usage) => ledger.record(usage),
            Event::ToolCall { .. } | Event::Error(_) => {}
        }
    }
    println!("{content}");
    if let Some(model) = model {
        println!("model: {model}");
    }
    if ledger.calls > 0 {
        println!(
            "tokens: prompt={} completion={} total={}",
            ledger.prompt_tokens, ledger.completion_tokens, ledger.total_tokens
        );
        if ledger.cost_known {
            println!("cost: {}", format_cost(ledger.cost_usd));
        }
        let cost = if ledger.cost_known {
            format!(" cost={}", format_cost(ledger.cost_usd))
        } else {
            String::new()
        };
        println!(
            "agent: calls={} prompt={} completion={} total={}{cost}",
            ledger.calls, ledger.prompt_tokens, ledger.completion_tokens, ledger.total_tokens
        );
    }
    for event in events {
        println!("{}", event.report_line());
    }
}
