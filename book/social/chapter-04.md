# Social posts: Chapter 4, "MCP Client"

## X thread
1/
Chapter 4 of my "learning Rust by building a multi-agent harness" series is up: MCP Client.

A tool in another process now uses the same registry as a tool in this one. 🧵

2/
Hermes Agent (Nous Research) connects in tools/mcp_tool.py. Stdio is a command and args. Streamable HTTP is a url. Discovered tools go into the same registry as the built-ins. notifications/tools/list_changed refreshes that list, and mcp_tool_health.py coalesces a burst into one refresh plus one follow-up.

3/
I built the client with rmcp 3.5.1. The test server is in the repo and stays on 127.0.0.1. echo is read-only. note has no readOnlyHint, so the dispatcher treats it as a write. grow reveals a fourth tool and sends list_changed.

4/
One stdio echo was 71.29 µs. The same call over streamable HTTP was 215.7 µs. Fifty echo calls with no sleep were 1.908 ms through the bound of 8, and 3.603 ms one at a time. Eight reads that each slept 40 ms did overlap.

A cold tools/list was 82.87 µs. The cache hit was 168.0 ns. Every list_changed drops the cache. I did not coalesce a burst. Hermes does.

5/
Chapter 6's 10,000-tool benches still use a generator. A real list enters that cache through cards_from_listed. The demo is offline. --mock runs the fixture. Without a key the CLI prints the missing OPENROUTER_API_KEY error and stops. I did not paste a live model transcript.

[personal note]

6/
Next, the agent loop: the model picks a tool, native or MCP, and the result comes back as a message.

Read Chapter 4: [link]

#rustlang #AIagents

## LinkedIn
Chapter 4 of my "learning Rust by building a multi-agent harness" series is out: MCP Client.

The registry from chapter 3 could only call a function in this process. MCP is how a file server or a browser advertises tools from another one. Nous Research's Hermes Agent already does this in tools/mcp_tool.py: stdio from a command, streamable HTTP from a url, and the discovered tools land in the same registry as the built-ins. A list_changed notification refreshes that list, and a burst is coalesced so the client does not re-list on every tick.

I built the client with rmcp. The server in the tests is a fixture in the repo. It never dials an external host. echo is marked read-only. note omits the hint, and the dispatcher therefore runs it alone. grow reveals one more tool and sends the notification. The cache drops on that notification. It does not coalesce yet.

One stdio echo measured 71.29 µs. Streamable HTTP on loopback measured 215.7 µs. Fifty zero-delay calls were 1.908 ms together and 3.603 ms one at a time. A cold tools/list was 82.87 µs, and the hit was 168.0 ns. These crates need rustc 1.88 because that is what rmcp 3.5.1 asks for.

The thousands-of-tools chapter still generates its catalog for the benches. A real list enters that cache as cards, without that crate linking rmcp. The demo is offline. I did not paste a live model transcript.

[personal note: one takeaway from writing this chapter]

Next up: a minimal MCP server (chapter 5), then thousands of tools (chapter 6). Read Chapter 4: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
