use std::process::ExitCode;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use harness_core::{
    fixture_binary, missing_live_key, prepare_live_evals, register_remote, run_saved_evals,
    run_stress_evals, AgentSession, Budget, CancelFlag, Dispatcher, EchoTool, LiveProvider,
    McpSession, NoteTool, OpenAiCompatible, ScriptedProvider, ScriptedReply, StopReason, Tool,
    ToolRegistry, DEFAULT_MODEL, DISPATCH_BOUND, OPENROUTER_BASE_URL,
};
use serde_json::json;

#[derive(Parser)]
#[command(name = "harness")]
struct Cli {
    /// Offline script. This is the default. `--live` is the opt-in.
    #[arg(long)]
    mock: bool,
    /// Call OpenRouter with echo and note. CI does not pass this.
    #[arg(long)]
    live: bool,
    /// Run the stress evals against OpenRouter. Without a key, exit before any socket.
    #[arg(long)]
    live_evals: bool,
    /// API key. Overrides OPENROUTER_API_KEY. Used only with `--live`.
    #[arg(long)]
    api_key: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Ask. The mock is the default. `--live` calls OpenRouter with echo and note.
    Ask { prompt: String },
    /// Run the saved set and the stress set. Mock unless `--live-evals` is set.
    Evals,
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
            if cli.live && cli.mock {
                return Err("pass either --mock or --live".into());
            }
            if cli.live {
                return live_ask(prompt, cli.api_key.as_deref()).await;
            }
            mock_ask(prompt).await
        }
        Command::Evals => {
            if cli.live_evals {
                let openrouter_api_key = std::env::var("OPENROUTER_API_KEY").ok();
                let key = cli
                    .api_key
                    .clone()
                    .filter(|value| !value.is_empty())
                    .or(openrouter_api_key);
                prepare_live_evals(key.as_deref())?;
                return live_evals(key.as_deref().unwrap_or("")).await;
            }
            mock_evals().await
        }
    }
}

async fn mock_evals() -> Result<(), String> {
    for score in run_saved_evals().await {
        println!(
            "eval: {} {}",
            score.name,
            if score.passed { "pass" } else { "fail" }
        );
        if !score.passed {
            return Err(score.detail);
        }
    }
    for score in run_stress_evals().await {
        println!(
            "eval: {} {}",
            score.name,
            if score.passed { "pass" } else { "fail" }
        );
        if !score.passed {
            return Err(score.detail);
        }
    }
    Ok(())
}

async fn live_evals(key: &str) -> Result<(), String> {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .map_err(|error| error.to_string())?;
    registry
        .register(Arc::new(NoteTool) as Arc<dyn Tool>)
        .map_err(|error| error.to_string())?;
    let schemas = registry.schemas();
    let provider = LiveProvider {
        client: OpenAiCompatible::new(OPENROUTER_BASE_URL, key, DEFAULT_MODEL),
        schemas,
    };
    let outcome = run_session("say hello using the echo tool".into(), &provider, &registry).await?;
    for event in outcome.events() {
        println!("{}", event.report_line());
    }
    println!("stop: {}", outcome.label());
    Ok(())
}

async fn mock_ask(prompt: String) -> Result<(), String> {
    let mcp = Arc::new(
        McpSession::stdio(fixture_binary())
            .await
            .map_err(|error| error.to_string())?,
    );
    let mut registry = ToolRegistry::new();
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
    let tool = session
        .messages
        .iter()
        .find(|message| message.tool_call_id.is_some())
        .map(|message| message.content.clone());
    print_outcome(outcome.events(), &tool);
    match outcome.stop() {
        StopReason::Finished => Ok(()),
        StopReason::Cancelled => Err("cancelled".into()),
        StopReason::SoftStop => Err("soft stop".into()),
        StopReason::Budget => Err("budget".into()),
        StopReason::Partial => Err("partial".into()),
        StopReason::InvalidToolCalls => Err("invalid tool calls".into()),
        StopReason::Stalled => Err("stalled".into()),
    }
}

async fn run_session(
    prompt: String,
    provider: &dyn harness_core::Provider,
    registry: &ToolRegistry,
) -> Result<harness_core::TurnOutcome, String> {
    let mut session = AgentSession::new();
    session
        .inbox
        .submit(prompt)
        .map_err(|error| error.to_string())?;
    session
        .turn_from_inbox(
            provider,
            registry,
            &Dispatcher::new(DISPATCH_BOUND),
            &Budget::default(),
            &CancelFlag::new(),
        )
        .await
}

fn print_outcome(events: &[harness_core::Event], tool: &Option<String>) {
    for event in events {
        println!("{}", event.report_line());
    }
    if let Some(text) = tool {
        println!("tool-result: echo {text}");
    }
}

async fn live_ask(prompt: String, api_key_flag: Option<&str>) -> Result<(), String> {
    let openrouter_api_key = std::env::var("OPENROUTER_API_KEY").ok();
    if let Some(error) = missing_live_key(api_key_flag, openrouter_api_key.as_deref()) {
        return Err(error);
    }
    let key = api_key_flag
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .or(openrouter_api_key)
        .unwrap_or_default();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .map_err(|error| error.to_string())?;
    registry
        .register(Arc::new(NoteTool) as Arc<dyn Tool>)
        .map_err(|error| error.to_string())?;
    let schemas = registry.schemas();
    let provider = LiveProvider {
        client: OpenAiCompatible::new(OPENROUTER_BASE_URL, &key, DEFAULT_MODEL),
        schemas,
    };
    let outcome = run_session(prompt, &provider, &registry).await?;
    for event in outcome.events() {
        println!("{}", event.report_line());
    }
    Ok(())
}
