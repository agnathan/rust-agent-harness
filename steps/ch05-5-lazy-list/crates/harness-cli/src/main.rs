use clap::Parser;
use harness_core::{
    format_cost, milestone, openai_tool, resolve_endpoint, search_tool_card, AgentUsage,
    EndpointInput, Event, Message, OpenAiCompatible, DEFAULT_MODEL, OPENROUTER_BASE_URL,
};
use std::process::ExitCode;
use std::time::Duration;

#[derive(Parser)]
#[command(
    name = "harness",
    about = "Load tools on demand",
    after_long_help = "The default server is OpenRouter (https://openrouter.ai/api/v1).\n\
A live call reads OPENROUTER_API_KEY. Another --base-url reads OPENAI_API_KEY\n\
instead. --api-key overrides either variable. --mock never uses a key.\n\
\n\
--mock ask runs the tool-loading demo offline: a synthetic catalog stands in\n\
for MCP tools/list until chapter 4's client exists. It does not call a model."
)]
struct Cli {
    /// Base URL of an OpenAI-compatible API. Default is OpenRouter.
    #[arg(long, env = "OPENAI_BASE_URL", default_value = OPENROUTER_BASE_URL)]
    base_url: String,

    /// API key. Overrides OPENROUTER_API_KEY or OPENAI_API_KEY.
    #[arg(long)]
    api_key: Option<String>,

    /// OpenRouter model id, such as openai/gpt-4o-mini.
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

    /// Run the offline tool-loading demo. No model call.
    #[arg(long)]
    mock: bool,

    /// How many synthetic tools the mock demo generates.
    #[arg(long, default_value_t = 2_000)]
    tools: usize,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Search the catalog, or send one prompt.
    Ask { prompt: String },
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

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let Command::Ask { prompt } = &cli.command;
    if cli.mock {
        print_milestone(prompt, cli.tools);
        return Ok(());
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
    let provider =
        OpenAiCompatible::with_timeout(&endpoint, Duration::from_secs(cli.timeout_secs.max(1)));
    let card = search_tool_card();
    let tools = vec![openai_tool(
        &card.name,
        &card.description,
        &card.input_schema,
    )];
    let messages = vec![
        Message::system(
            "You can call search_tools. Matching tool schemas are added on the next turn. Do not assume any other tool is loaded.",
        ),
        Message::user(prompt),
    ];
    let events = provider.complete_with_tools(&messages, tools).await?;
    print_events(&events);
    Ok(())
}

fn print_milestone(query: &str, tools: usize) {
    let report = milestone(tools, query);
    println!("tools={}", report.tools);
    println!("all_schema_tokens={}", report.all_tokens);
    println!("visible_tokens_before={}", report.visible_before);
    println!("this_turn={}", report.this_turn.join(","));
    println!("search:\n{}", report.listing);
    println!("next_turn={}", report.next_turn.join(","));
    println!("visible_tokens_after={}", report.visible_after);
    println!("core_prompt_unchanged={}", report.core_same);
    let args = serde_json::json!({ "query": query });
    println!(
        "{}",
        Event::ToolCall {
            id: "search".to_string(),
            name: "search_tools".to_string(),
            arguments: args.to_string(),
        }
        .report_line()
    );
    let first = report
        .listing
        .lines()
        .next()
        .unwrap_or("no tools")
        .to_string();
    println!("{}", Event::TextDelta(first).report_line());
    println!(
        "{}",
        Event::Done {
            message: Message::assistant("mock tool search"),
            model: None,
        }
        .report_line()
    );
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
