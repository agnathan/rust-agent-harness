use clap::{Parser, Subcommand};
use harness_core::{
    resolve_endpoint, DetailedCompletion, EndpointInput, Message, MockProvider, OpenAiCompatible,
    Provider, ProviderError, DEFAULT_MODEL, OPENROUTER_BASE_URL,
};
use std::process::ExitCode;

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

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Send one prompt and print the reply.
    Ask { prompt: String },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> Result<(), ProviderError> {
    let provider: Box<dyn Provider> = if cli.mock {
        Box::new(MockProvider::new("(mock) I heard you."))
    } else {
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
        Box::new(OpenAiCompatible::with_timeout(
            &endpoint,
            std::time::Duration::from_secs(cli.timeout_secs.max(1)),
        ))
    };

    match cli.command {
        Command::Ask { prompt } => {
            let messages = vec![
                Message::system("You are a helpful assistant."),
                Message::user(prompt),
            ];
            let detailed = provider.complete_detailed(&messages).await?;
            print_completion(&detailed);
        }
    }
    Ok(())
}

fn print_completion(detailed: &DetailedCompletion) {
    println!("{}", detailed.message.content);
    if let Some(model) = &detailed.model {
        println!("model: {model}");
    }
    if let Some(usage) = &detailed.usage {
        println!(
            "tokens: prompt={} completion={} total={}",
            usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
        );
    }
}
