# MCP gateway

The book builds two processes. The harness decides what enters a prompt. The MCP gateway is a standalone server: a large catalog, a filtered tool list per session, and later a highly available HTTP front. Chapters 5, 7, and 8 are written. They bind no socket. `mcp-gateway/` re-exports the chapter 8 session gateway. `Gateway::new` still starts from an empty catalog.

Priority 1 is token efficiency through dynamic toolsets. Priority 2 is high availability. The benchmark targets are at the bottom. None of the server numbers have been measured.

## Who decides what

**Harness.** It knows the task, the context budget, the prompt cache, and the other servers in the turn. It chooses what is allowed into the prompt. Ordering is append-only so a cached prefix stays byte-stable. Eviction happens at checkpoints, not by rewriting the prefix. It keeps a token budget, a per-agent view, and (once sessions and memory exist) a learned prediction of which tools this task tends to use.

**Server.** It holds the catalog: native tools, tools federated from upstream MCP servers, and tools loaded from runtime plugins. It searches that catalog itself (BM25 plus embeddings). It exposes `search_tools`, a filtered toolset per session and per client, and `set_task` for a task-shaped starter set. When the offered set changes it sends `notifications/tools/list_changed`. Schemas on the wire stay compact; the full docs are a separate request. It can offer a composite (macro) tool and a code-mode tool. Large results are paged or summarized. Per session it records schema tokens, call tokens, result tokens, and tools that were offered and never called.

The harness chapter that is already written (book chapter 6, crates `steps/ch05-*`) is the client half of this split. It measures schema cost, builds a BM25 index, reveals tools append-only, and caches `tools/list`. It does not search a remote catalog. Book chapter 5 is the process that will hold the catalog. Book chapter 7 is the server-side index.

## What I read

Hermes Agent, current `main` of [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent), plus three public gateways. Paths below are the files that were opened.

### Hermes is a client, and one small stdio server

`mcp_serve.py` is a stdio MCP server named `hermes`. The tool list is fixed and registered in this order, which is the `list_tools` order: `conversations_list`, `conversation_get`, `messages_read`, `attachments_fetch`, `events_poll`, `events_wait`, `messages_send`, `channels_list`, `permissions_list_open`, `permissions_respond`. `create_mcp_server` / `run_mcp_server` / `run_stdio_async` serve that list. `EventBridge` polls the session database. This is a messaging bridge. It does not federate other MCP servers, and it does not search a catalog.

`website/docs/user-guide/features/mcp.md` is the client. `mcp_servers` config, `tools.include` / `tools.exclude`, `discovery_concurrency` default 4 (`0` means unlimited). A server can start lazily from an on-disk schema cache. `notifications/tools/list_changed` triggers a lock-protected re-fetch. `prompts/list_changed` and `resources/list_changed` are received and not acted on. `/reload-mcp` reloads. The messaging gateway watches `config.yaml` (on the order of a minute). Connect cooldown starts at 30 seconds and doubles up to 10 minutes. The runtime toolset name is `mcp-<server>`. An empty toolset is not created. Stdio environment is filtered. OAuth refresh does not open a browser in the background.

`optional-mcps/` is an install-time catalog of Nous-approved servers. Presence in the tree is the approval, via pull request. `optional-mcps/linear/manifest.yaml` is `manifest_version: 1`, transport `http` to `https://mcp.linear.app/mcp`, auth `oauth`, described there as native MCP OAuth 2.1 with dynamic client registration over streamable HTTP. The install checklist writes `tools.include`. The docs say a large surface (Cloudflare, on the order of 3,300 tools) uses `tools.default_excluded`. There is no community submission tier in what I read.

`toolsets.py` holds the `TOOLSETS` dict, `_HERMES_CORE_TOOLS`, `resolve_toolset`, `get_toolset`, and `CLIENT_SURFACE_TOOLSETS` (`project`, `desktop_ui`, `catalog`). Toolsets can be composed. `browser_vault` is deferred by default through `tools.tool_search.defer`.

`tools/registry.py` defines `ToolEntry` (`toolset`, `schema`, `handler`, `check_fn`). `check_fn` has a TTL cache and a failure grace so a flaky probe does not strip a toolset. A second registration of the same name is rejected.

`tools/tool_search.py` is client-side progressive disclosure. Deferred tools are replaced by `tool_search`, `tool_describe`, and `tool_call`. The catalog is rebuilt from the live definitions on every assembly: it is stateless, so a session-keyed catalog would drift from what Hermes does. Core tools are not deferred unless named. `_MAX_QUERIES_PER_CALL` is 7. `_MAX_DESCRIBE_NAMES_PER_CALL` is 10. `enabled` is `auto`, `on`, or `off`.

`tools/tool_search_catalog.py` is BM25 over the deferrable definitions, with a snowball stemmer, `CHARS_PER_TOKEN = 4.0`, and a byte-stable listing. `BRIDGE_TOOL_NAMES` are reserved.

`tools/mcp_tool_discovery.py` sets `_DISCOVERY_CONNECT_CONCURRENCY = 4`.

`tools/mcp_schema_cache.py` persists a cache keyed by server name plus a fingerprint of command, args, url, and tool filters. `config_fingerprint` uses `json.dumps(..., sort_keys=True)` and `sha256[:16]`. The file is `mcp_schema_cache.json` under the Hermes home cache.

`tools/mcp_oauth.py` is an OAuth 2.1 **client** (PKCE, client ID metadata or RFC 7591 dynamic client registration). Hermes is not the authorization server. The gateway's OAuth chapter is the resource-server side.

`gateway/drain_control.py` drains the **messaging** gateway. A marker file `{HERMES_HOME}/.drain_request.json` requests drain. `DRAIN_REQUEST_MAX_AGE_SECONDS` is 3600. A malformed file is still treated as drain-active. That is not an MCP-server drain.

`gateway/hosted_room_replicas.py` stores replicas of hosted Group Chat rooms, with `promote_replica` at epoch + 1, `demote_room`, and `MAX_REPLICA_ROOMS = 256`. Those are storage primitives for chat rooms, not MCP server replicas.

### Public gateways that do federate

These are the designs that sit in front of many MCP servers. The notes are from each project's README, not from running them.

[IBM/mcp-context-forge](https://github.com/IBM/mcp-context-forge) (ContextForge, PyPI `mcp-contextforge-gateway`) is a registry and proxy. It federates MCP, A2A, and REST/gRPC, and it lists governance, discovery, observability, OpenTelemetry, Redis-backed federation and caching, rate limiting, auth, retries, and plugins.

[stacklok/toolhive](https://github.com/stacklok/toolhive) runs each MCP server in its own container, with an identity and access policy. Its README claims semantic tool search can reduce token usage by up to 85 percent. That figure is theirs. This book will measure tokens per completed task instead of adopting it. ToolHive also ships a Kubernetes operator, OpenTelemetry, and audit logs.

[docker/mcp-gateway](https://github.com/docker/mcp-gateway) is one gateway in front of many servers: a catalog, dynamic discovery of tools, prompts, and resources, OAuth, secrets through Docker Desktop, and container isolation. The protocol it points at is the [MCP spec](https://spec.modelcontextprotocol.io/).

## Adopted, adapted, deferred

**Adopted for the skeleton.** A gateway is its own crate, not a mode of the harness. `Gateway::new` starts from an empty `Catalog`. `session_tools` returns a `SessionView` that offers nothing. No listener. The harness remains the thing that builds the prompt.

**Adapted, when those chapters are written.**

- Hermes searches on the client (`tool_search_catalog.py`) and rebuilds the catalog every assembly. The server will own the index so a 10,000-tool search does not ride along inside the agent process. The harness keeps the append-only, cache-safe reveal it already has in chapter 6. A session-keyed view is a deliberate difference from Hermes's stateless rebuild: the server sends `list_changed` when the view changes, which is the notification Hermes already honors.
- Hermes `tools.include` / `tools.exclude` and `optional-mcps` are install-time filters. `set_task` and per-session toolsets are the runtime version of that filter.
- Compact schemas, with the long text fetched on request, follow the same idea as `tool_describe` (Hermes caps describe at 10 names and search at 7 queries). The server can page a result instead of only truncating it in the harness.
- ContextForge and ToolHive are the precedent for one process in front of many upstream servers, with Redis for shared session and rate-limit state. Docker's gateway is the precedent for a catalog plus dynamic discovery. The Rust server uses `rmcp` and axum when the service chapter is written. The skeleton does not depend on them, and it stays on `rust-version` 1.85.
- Pooling uses least-outstanding-requests or power-of-two-choices per tool, with a concurrency limit and a queue. That is the same bound the harness already uses (8), applied on the server to each backend.
- Retries are only for tools marked idempotent. Hermes's client reconnect cooldown (30 seconds, doubling to 10 minutes) is a client policy; the server's breaker and hedge are separate and not copied from that cooldown.
- Drain is adapted from `gateway/drain_control.py`: stop accepting, finish in-flight calls. The marker-file mechanism and the 3600-second max age belong to the messaging gateway. The MCP server will drain on a signal and a readiness flip. A malformed-file fail-safe is not copied, because this server will not use that file.
- Hosted-room replica promotion is not the MCP replica design. MCP replicas are mostly stateless, routed by session id, with shared state in Redis or something like it.
- OAuth 2.1 on the server follows the MCP spec (the resource server). Hermes's `mcp_oauth.py` is the client that will talk to it. This server does not implement the client flow.

**In chapters 5, 7, 8, and 15.** A 32/8 call gate, BM25 over native, plugin, and in-memory upstream tools, an `Arc` reload, `search_tools`, `set_task`, `describe`, a `list_changed` flag, paging, schema/call/result tokens, and per-tool permits, idempotent retries, a breaker, and 429/503 backpressure.

**In chapter 21.** A usage score installed as the offered set, composite and macro names, and a line-script code mode capped at 50 calls on the sandbox allowlist. The score does not run inside `session_tools`.

**In chapter 35.** Least-outstanding and power-of-two picks over an in-memory per-tool pool, federation as a map from tool name to server names, and a discovery gate of 4. No socket.

**In chapter 38.** Session-id routing, failover when a replica is marked dead, an `Arc` config reload, per-tenant concurrent quotas, an audit vec, span names, health and readiness, a bearer check, and a drain. Sampling, form-mode elicitation, and the `MAL-*` check live in the same crate. No socket.

**Deferred with the unwritten chapters.** A real embedder, live upstream connections, a Python sandbox for code mode, a bound listener, Redis, OAuth token issuance, and a trace exporter. The server still does not bind a socket.

## Reading order

Server-track chapters are marked *(server)*. Each one sits after the harness chapter it builds on. Step directories for the written harness chapters stay `steps/ch05-*` (book chapter 6) and `steps/ch06-*` (book chapter 9). `steps/gw-minimal-server`, `steps/gw-catalog`, `steps/gw-session-toolsets`, `steps/gw-limits`, `steps/gw-learned-tools`, `steps/gw-balancing`, `steps/gw-service`, and `steps/gw-capstone` exist. None of them bind a socket.

| Ch | Track | After | What it adds |
|---|---|---|---|
| 4 | harness | — | MCP client. Written. |
| 5 | *(server)* | chapter 4 | Minimal MCP server. Skeleton only. Replaces the old "MCP Server" row. |
| 6 | harness | — | Thousands of Tools. Written. The client half of the catalog. |
| 7 | *(server)* | chapter 6 | Dynamic catalog and server-side BM25 plus embeddings. |
| 8 | *(server)* | chapter 7 | Per-session toolsets, `set_task`, `list_changed`, compact schemas, token telemetry. |
| 9 | harness | — | The agent loop. Written. Section crates stay `ch06-*`. |
| 14 | harness | — | When Things Fail. |
| 15 | *(server)* | chapter 14 | Timeouts, idempotent-only retries, circuit breakers, queues, concurrency limits. |
| 18–20 | harness | — | Sessions, memory, recall. |
| 21 | *(server)* | chapter 20 | Learned tool prediction and composite tools. Written. Prediction stays off the request path except for the offered set. |
| 34 | harness | — | Supervision and restart. Cancellation is already in chapter 9. |
| 35 | *(server)* | chapter 34 | Least-outstanding / power-of-two-choices, per-tool pools, federation of upstream MCP. Written. In memory. No socket. |
| 37 | harness | — | The chat gateway: platform adapters into the session inbox. Not this server. |
| 38 | *(server)* | chapter 37 | Session-id routing, an `Arc` reload, quotas, audit, drain, health. Written. In memory. No socket. |
| 46 | harness | — | Seeing Inside. Spans across the harness and the gateway. |
| 47 | *(server)* | chapter 46 | Load test and the token-efficiency capstone. Written. In memory. No socket. |

## Benchmark targets

Chapters 5, 7, and 8 quote the means from their own benches. The targets below that those chapters did not measure are still targets.

- **Tokens per completed task**, three arms (every tool, a static set, a dynamic set), each with task success. Chapter 8 collects the telemetry. Chapter 47 is the capstone that reports it.
- **Search latency over 10,000 tools** on the server, and the cost of a catalog reload that swaps an `Arc` without blocking an in-flight call. Chapter 7. The client BM25 search in chapter 6 already measured 146.8 µs. That number is the harness, not the server.
- **Throughput and p99** under load, with oha or something like it. Chapters 38 and 47 do not bind a socket, so oha is not run. Chapter 47 measures 100 in-memory dispatches and a nearest-rank p99 inside that process.
- **Replica-kill failover time**, and drain that finishes in-flight calls. Chapter 38.
- **Catalog reload cost.** Chapter 7.

Chapter 5 accepts 32 calls and executes 8. A permit round trip measured 45.53 ns. Eight 1 ms sleeps measured 2.129 ms.
