// Offline MCP server for the chapter 4 tests.
//
// Default mode speaks MCP on stdin/stdout. `--http` binds `127.0.0.1:0`,
// prints `MCP_URL=http://127.0.0.1:<port>/mcp` on stderr, and serves
// streamable HTTP. Nothing here dials an external host.
//
// `echo` is read-only. `note` is a write. `grow` reveals `extra` and sends
// `notifications/tools/list_changed`. `FIXTURE_DELAY_MS` makes `echo` wait,
// so a test can see overlapping calls.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rmcp::handler::server::tool::ToolCallContext;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, ContentBlock, Implementation, ListToolsResult,
    PaginatedRequestParams, ProtocolVersion, ResultType, ServerCapabilities, ServerConfig,
};
use rmcp::service::RequestContext;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use rmcp::{
    schemars, tool, tool_router, ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
struct TextArgs {
    /// Text to echo or store.
    text: String,
}

#[derive(Clone)]
struct Fixture {
    extra: Arc<AtomicBool>,
}

impl Fixture {
    fn new(extra: Arc<AtomicBool>) -> Self {
        Self { extra }
    }
}

#[tool_router]
impl Fixture {
    #[tool(
        description = "Return the text argument unchanged.",
        annotations(read_only_hint = true)
    )]
    async fn echo(&self, Parameters(TextArgs { text }): Parameters<TextArgs>) -> String {
        if let Ok(ms) = std::env::var("FIXTURE_DELAY_MS") {
            if let Ok(ms) = ms.parse::<u64>() {
                if ms > 0 {
                    tokio::time::sleep(Duration::from_millis(ms)).await;
                }
            }
        }
        text
    }

    #[tool(description = "Store a note. A write.")]
    fn note(&self, Parameters(TextArgs { text }): Parameters<TextArgs>) -> String {
        format!("noted:{text}")
    }

    #[tool(
        description = "Reveal the extra tool.",
        annotations(read_only_hint = true)
    )]
    fn grow(&self) -> String {
        self.extra.store(true, Ordering::SeqCst);
        "grew".into()
    }

    #[tool(
        description = "An extra tool, hidden until grow.",
        annotations(read_only_hint = true)
    )]
    fn extra(&self) -> String {
        "extra".into()
    }
}

impl ServerHandler for Fixture {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_tool_list_changed()
                .build(),
        )
        .with_server_info(Implementation::new("mcp-fixture", "0.1.0"))
        .with_instructions("Offline test server.")
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let mut tools = Self::tool_router().list_all();
        if !self.extra.load(Ordering::SeqCst) {
            tools.retain(|tool| tool.name != "extra");
        }
        let supports_cache_hints = context
            .protocol_version()
            .is_some_and(|version| version >= ProtocolVersion::V_2026_07_28);
        Ok(ListToolsResult {
            result_type: Some(ResultType::COMPLETE),
            tools,
            meta: None,
            next_cursor: None,
            ttl_ms: supports_cache_hints.then_some(0),
            cache_scope: supports_cache_hints.then_some(rmcp::model::CacheScope::Public),
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::CallToolResponse, McpError> {
        if request.name == "grow" {
            self.extra.store(true, Ordering::SeqCst);
            let peer = context.peer.clone();
            tokio::spawn(async move {
                let _ = peer.notify_tool_list_changed().await;
            });
            return Ok(CallToolResult::success(vec![ContentBlock::text("grew")]).into());
        }
        if request.name == "extra" && !self.extra.load(Ordering::SeqCst) {
            return Err(McpError::invalid_params("tool not found", None));
        }
        let context = ToolCallContext::new(self, request, context);
        Self::tool_router().call(context).await
    }

    fn get_tool(&self, name: &str) -> Option<rmcp::model::Tool> {
        if name == "extra" && !self.extra.load(Ordering::SeqCst) {
            return None;
        }
        Self::tool_router().get(name).cloned()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http = std::env::args().any(|arg| arg == "--http");
    if http {
        serve_http().await
    } else {
        let service = Fixture::new(Arc::new(AtomicBool::new(false)))
            .serve(rmcp::transport::stdio())
            .await?;
        service.waiting().await?;
        Ok(())
    }
}

async fn serve_http() -> anyhow::Result<()> {
    let extra = Arc::new(AtomicBool::new(false));
    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(false)
        .with_json_response(true);
    let service = StreamableHttpService::new(
        {
            let extra = Arc::clone(&extra);
            move || Ok(Fixture::new(Arc::clone(&extra)))
        },
        LocalSessionManager::default().into(),
        config,
    );
    let router = axum::Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    eprintln!("MCP_URL=http://{addr}/mcp");
    axum::serve(listener, router).await?;
    Ok(())
}
