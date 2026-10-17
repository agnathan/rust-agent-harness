# Social posts: Chapter 38, "MCP Gateway Service"

## X thread
1/
Chapter 38 is up: MCP Gateway Service. Server track.

A session id sticks to a replica. Kill that replica and the session moves. A drain finishes the calls already accepted. No socket. 🧵

2/
Hermes Agent is the MCP client here, not an OAuth resource server. tools/mcp_tool_sampling.py defaults sampling to 10 requests a minute and declines URL-mode elicitation. tools/osv_check.py blocks MAL-* advisories on npx and uvx, ignores regular CVEs, and fails open on a network error without caching it.

3/
The bearer check in this crate is a non-empty token. Quotas are concurrent and per tenant. Hot reload is an Arc swap, the same shape as chapter 7. Fake replies. No key.

4/
One session lookup measured 11.89 ns. The demo prints "route: a".

[personal note]

5/
Next is cron: claim a job once, scan the prompt, and do not let a job restart the gateway.

Read Chapter 38: [link]

#rustlang #AIagents

## LinkedIn
Chapter 38 is out: MCP Gateway Service.

This is the server track. Hermes Agent does not ship a gateway OAuth server. It is the MCP client. Sampling is capped at 10 requests a minute. Form-mode elicitation goes through approval, with a 300 second timeout. URL mode is declined. Before an npx or uvx launch, osv_check.py asks about MAL-* malware advisories, ignores ordinary CVEs, caches a verdict for an hour, and allows the launch when the network fails without remembering that failure.

The service in this chapter routes a session id to a replica name, moves those sessions when the replica is killed, swaps config with an Arc, counts a per-tenant concurrent quota, writes an audit line and a span name, and drains by finishing in-flight calls. It does not bind a socket, and it does not issue OAuth tokens. A missing Bearer token is refused.

One route measured 11.89 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: cron. Read Chapter 38: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
