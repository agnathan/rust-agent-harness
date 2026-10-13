use std::process::ExitCode;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use harness_core::{
    fixture_binary, register_remote, AgentSession, Budget, CancelFlag, Dispatcher, McpSession,
    ScriptedProvider, ScriptedReply, StopReason, DISPATCH_BOUND,
};
use serde_json::json;

#[derive(Parser)]
#[command(name = "harness")]
struct Cli {
    /// Run the offline fixture. The model replies are scripted.
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
    /// Ask. `--mock` runs the saved echo script against the fixture.
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
            let mcp = Arc::new(
                McpSession::stdio(fixture_binary())
                    .await
                    .map_err(|error| error.to_string())?,
            );
            let mut registry = harness_core::ToolRegistry::new();
            register_remote(&mut registry, mcp)
                .await
                .map_err(|error| error.to_string())?;
            let provider = ScriptedProvider::new(vec![
                ScriptedReply::tool("call-1", "echo", json!({"text": "hello"})),
                ScriptedReply::text("hello"),
            ]);
            let mut session = AgentSession::new();
            session
                .inbox
                .submit(prompt)
                .map_err(|error| error.to_string())?;
            let outcome = session
                .turn_from_inbox(
                    &provider,
                    &registry,
                    &Dispatcher::new(DISPATCH_BOUND),
                    &Budget::default(),
                    &CancelFlag::new(),
                )
                .await?;
            for event in &outcome.events {
                println!("{}", event.report_line());
            }
            if let Some(tool) = session
                .messages
                .iter()
                .find(|message| message.tool_call_id.is_some())
            {
                println!("tool-result: echo {}", tool.content);
            }
            match outcome.stop {
                StopReason::Finished => Ok(()),
                StopReason::Cancelled => Err("cancelled".into()),
                StopReason::Budget => Err("budget".into()),
            }
        }
    }
}
