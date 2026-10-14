use std::process::ExitCode;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use harness_core::{fixture_binary, McpSession, ToolListCache};

#[derive(Parser)]
#[command(name = "harness")]
struct Cli {
    /// Run the offline fixture demo.
    #[arg(long)]
    mock: bool,
    /// API key. Overrides OPENROUTER_API_KEY. This binary still does not call OpenRouter.
    #[arg(long)]
    api_key: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Ask. `--mock` runs the fixture. Without it, a missing OpenRouter key is an error.
    Ask { prompt: String },
}

fn live_refusal(api_key_flag: Option<&str>, openrouter_api_key: Option<&str>) -> String {
    let key = api_key_flag
        .filter(|value| !value.is_empty())
        .or_else(|| openrouter_api_key.filter(|value| !value.is_empty()));
    if key.is_none() {
        "missing OPENROUTER_API_KEY: pass --api-key, or run with --mock".into()
    } else {
        "this demo does not place a live OpenRouter call".into()
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Err(error) = run(cli).await {
        eprintln!("event: error {error}");
        eprintln!("error: {error}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

async fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Ask { prompt } => {
            if !cli.mock {
                let openrouter_api_key = std::env::var("OPENROUTER_API_KEY").ok();
                return Err(live_refusal(
                    cli.api_key.as_deref(),
                    openrouter_api_key.as_deref(),
                ));
            }
            let _ = prompt;
            let session = Arc::new(
                McpSession::stdio(fixture_binary())
                    .await
                    .map_err(|error| error.to_string())?,
            );
            let mut cache = ToolListCache::new(Arc::clone(&session));
            let tools = cache.list().await.map_err(|error| error.to_string())?;
            println!(
                "tools: {}",
                tools
                    .iter()
                    .map(|tool| tool.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let echo = session
                .call_text("echo", serde_json::json!({"text": "hello"}))
                .await
                .map_err(|error| error.to_string())?;
            println!("echo: {echo}");
            println!("event: text-delta {echo}");
            println!("event: done");
            drop(cache);
            drop(session);
            Ok(())
        }
    }
}
