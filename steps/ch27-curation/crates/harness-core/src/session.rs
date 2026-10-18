//! One MCP session. Section 1 opens it on a child process's stdin and stdout.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use rmcp::model::CallToolRequestParams;
use rmcp::service::NotificationContext;
use rmcp::transport::TokioChildProcess;
use rmcp::{ClientHandler, RoleClient, ServiceExt};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    /// `readOnlyHint`. Absent means the tool is treated as a write.
    pub read_only: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("spawn {path}: {source}")]
    Spawn {
        path: String,
        source: std::io::Error,
    },
    #[error("mcp: {0}")]
    Protocol(String),
}

#[derive(Clone)]
struct Watch {
    changed: Arc<AtomicBool>,
}

impl ClientHandler for Watch {
    fn on_tool_list_changed(
        &self,
        _context: NotificationContext<RoleClient>,
    ) -> impl std::future::Future<Output = ()> + Send + '_ {
        self.changed.store(true, Ordering::SeqCst);
        std::future::ready(())
    }
}

/// Path to this package's fixture binary. Tests see Cargo's
/// `CARGO_BIN_EXE_ch27_mcp_fixture`. Examples and benches find it beside the
/// profile directory, which is where Cargo writes the bin.
pub fn fixture_binary() -> PathBuf {
    if let Some(path) = option_env!("CARGO_BIN_EXE_ch27_mcp_fixture") {
        return PathBuf::from(path);
    }
    let mut path = std::env::current_exe().expect("current exe");
    path.pop();
    if path
        .file_name()
        .is_some_and(|name| name == "examples" || name == "deps")
    {
        path.pop();
    }
    let file = if cfg!(windows) {
        "ch27_mcp_fixture.exe"
    } else {
        "ch27_mcp_fixture"
    };
    path.push(file);
    path
}

/// A fixture process serving streamable HTTP on `127.0.0.1`. Drop kills it.
pub struct HttpFixture {
    child: Child,
    pub url: String,
}

impl HttpFixture {
    pub async fn spawn(program: impl AsRef<Path>) -> Result<Self, SessionError> {
        let program = program.as_ref();
        let mut child = Command::new(program)
            .arg("--http")
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|source| SessionError::Spawn {
                path: program.display().to_string(),
                source,
            })?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| SessionError::Protocol("no stderr".into()))?;
        let mut lines = BufReader::new(stderr).lines();
        let line = tokio::time::timeout(std::time::Duration::from_secs(5), lines.next_line())
            .await
            .map_err(|_| SessionError::Protocol("fixture http timed out".into()))?
            .map_err(|error| SessionError::Protocol(error.to_string()))?
            .ok_or_else(|| SessionError::Protocol("fixture http closed stderr".into()))?;
        let url = line
            .strip_prefix("MCP_URL=")
            .ok_or_else(|| SessionError::Protocol(format!("bad fixture line: {line}")))?
            .to_string();
        Ok(Self { child, url })
    }
}

impl Drop for HttpFixture {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

pub struct McpSession {
    inner: rmcp::service::RunningService<RoleClient, Watch>,
    changed: Arc<AtomicBool>,
    list_calls: AtomicU64,
}

impl McpSession {
    pub async fn stdio(program: impl AsRef<Path>) -> Result<Self, SessionError> {
        Self::stdio_with(program, &[]).await
    }

    pub async fn stdio_with(
        program: impl AsRef<Path>,
        env: &[(&str, &str)],
    ) -> Result<Self, SessionError> {
        let program = program.as_ref();
        let changed = Arc::new(AtomicBool::new(false));
        let mut command = Command::new(program);
        for (key, value) in env {
            command.env(key, value);
        }
        let transport = TokioChildProcess::new(command).map_err(|source| SessionError::Spawn {
            path: program.display().to_string(),
            source,
        })?;
        let inner = Watch {
            changed: Arc::clone(&changed),
        }
        .serve(transport)
        .await
        .map_err(|error| SessionError::Protocol(error.to_string()))?;
        Ok(Self {
            inner,
            changed,
            list_calls: AtomicU64::new(0),
        })
    }

    pub async fn streamable_http(url: &str) -> Result<Self, SessionError> {
        let changed = Arc::new(AtomicBool::new(false));
        let transport = rmcp::transport::StreamableHttpClientTransport::from_uri(url.to_string());
        let inner = Watch {
            changed: Arc::clone(&changed),
        }
        .serve(transport)
        .await
        .map_err(|error| SessionError::Protocol(error.to_string()))?;
        Ok(Self {
            inner,
            changed,
            list_calls: AtomicU64::new(0),
        })
    }

    pub fn list_calls(&self) -> u64 {
        self.list_calls.load(Ordering::SeqCst)
    }

    /// True once since the last [`Self::take_list_changed`].
    pub fn take_list_changed(&self) -> bool {
        self.changed.swap(false, Ordering::SeqCst)
    }

    pub async fn list_tools(&self) -> Result<Vec<RemoteTool>, SessionError> {
        self.list_calls.fetch_add(1, Ordering::SeqCst);
        let tools = self
            .inner
            .list_all_tools()
            .await
            .map_err(|error| SessionError::Protocol(error.to_string()))?;
        let mut listed = Vec::with_capacity(tools.len());
        for tool in tools {
            let read_only = tool
                .annotations
                .as_ref()
                .and_then(|note| note.read_only_hint)
                .unwrap_or(false);
            listed.push(RemoteTool {
                name: tool.name.to_string(),
                description: tool.description.unwrap_or_default().to_string(),
                input_schema: Value::Object(tool.input_schema.as_ref().clone()),
                read_only,
            });
        }
        listed.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(listed)
    }

    pub async fn call_text(&self, name: &str, arguments: Value) -> Result<String, SessionError> {
        let arguments = match arguments {
            Value::Object(map) => map,
            _ => serde_json::Map::new(),
        };
        let result = self
            .inner
            .call_tool(CallToolRequestParams::new(name.to_string()).with_arguments(arguments))
            .await
            .map_err(|error| SessionError::Protocol(error.to_string()))?;
        Ok(result
            .content
            .first()
            .and_then(|block| block.as_text())
            .map(|text| text.text.clone())
            .unwrap_or_default())
    }

    pub async fn shutdown(self) -> Result<(), SessionError> {
        self.inner
            .cancel()
            .await
            .map(|_| ())
            .map_err(|error| SessionError::Protocol(error.to_string()))
    }
}
