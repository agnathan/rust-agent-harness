use clap::{Parser, Subcommand};
use harness_core::{Message, MockProvider, OpenAiCompatible, Provider, ProviderError};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "harness", about = "A tiny agent harness")]
struct Cli {
    /// Base URL of an OpenAI-compatible API.
    #[arg(
        long,
        env = "OPENAI_BASE_URL",
        default_value = "https://api.openai.com/v1"
    )]
    base_url: String,

    /// API key. A local fake server accepts any value.
    #[arg(long, env = "OPENAI_API_KEY", default_value = "")]
    api_key: String,

    /// Model name.
    #[arg(long, env = "HARNESS_MODEL", default_value = "gpt-4o-mini")]
    model: String,

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
        Box::new(OpenAiCompatible::new(
            &cli.base_url,
            &cli.api_key,
            &cli.model,
        ))
    };

    match cli.command {
        Command::Ask { prompt } => {
            let messages = vec![
                Message::system("You are a helpful assistant."),
                Message::user(prompt),
            ];
            let reply = provider.complete(&messages).await?;
            println!("{}", reply.content);
        }
    }
    Ok(())
}
