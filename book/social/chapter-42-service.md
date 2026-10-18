# Social posts: Chapter 42, "MCP Gateway Service"

## X thread
1/
Chapter 42 is up: MCP Gateway Service. Server track.

A session id sticks to a replica. Kill that replica and the next lookup moves the session. A drain finishes the calls already accepted. No socket. 🧵

2/
Hermes Agent at 9e01b44 is the MCP client. tools/mcp_tool_sampling.py defaults max_rpm to 10 and keeps timestamps newer than now minus 60 seconds. URL-mode elicitation is declined. Form mode uses a 300 second approval timeout. The sampling LLM timeout defaults to 30 seconds. This sampler does not sleep it.

3/
tools/osv_check.py sets _TIMEOUT to 10 and _CACHE_TTL_S from OSV_CHECK_CACHE_TTL, default 3600. It keeps MAL-* ids, ignores a regular CVE, and on a network error returns without caching. tools/mcp_oauth.py is the OAuth 2.1 client. This crate checks a Bearer token. It does not issue one.

4/
The demo prints "route: a". The bench id is 42/route_session. The old ids 40/route_session and 38/route_session are aliases. The mean is 19.7 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573).

[personal note]

5/
Next is chapter 43, cron: claim a job once, scan the prompt, and keep the lifecycle check on the job.

Read Chapter 42: [link]

#rustlang #AIagents

## LinkedIn
Chapter 42 is out: MCP Gateway Service.

This is the server track. The crate vendors chapter 37 and adds a service. Hermes Agent, at 9e01b44, does not ship the authorization server for that process. tools/mcp_oauth.py is an OAuth 2.1 client. Sampling in tools/mcp_tool_sampling.py defaults to 10 requests a minute over a 60 second window. Form-mode elicitation goes through approval with a 300 second timeout. URL mode is declined. Before an npx or uvx launch, tools/osv_check.py asks about MAL-* advisories, ignores ordinary CVEs, caches a verdict for 3600 seconds, and allows the launch when the network fails without storing that failure.

The service routes a session id to a replica name, moves those sessions when the replica is killed, swaps config with an Arc, counts a per-tenant concurrent quota, writes an audit line and a span name, and drains by finishing in-flight calls. It does not bind a socket.

The demo prints "route: a". The bench id is 42/route_session, and 40/route_session and 38/route_session remain aliases.

[personal note]

Next: chapter 43, cron. Read Chapter 42: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
