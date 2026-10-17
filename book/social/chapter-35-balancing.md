# Social posts: Chapter 35, "Balancing and Federation"

## X thread
1/
Chapter 35 is up: Balancing and Federation. Server track.

One tool can have several upstreams. The pick is local: least outstanding, or the quieter of two indexes. Discovery connects four at a time. No socket. 🧵

2/
Hermes Agent caps MCP discovery at 4. tools/mcp_tool_discovery.py names _DISCOVERY_CONNECT_CONCURRENCY. mcp.discovery_concurrency can override it, and 0 means unlimited. I did not find a least-outstanding balancer in that client.

3/
The pool is in memory. A finished call drops the outstanding count. Six connects with a short hold never show more than four in flight. Fake replies. No key.

4/
One least-outstanding pick measured 2.502 ns. The demo prints "pick: echo via alpha".

[personal note]

5/
Next is profiles: a persona per config, and secrets that do not enter the prompt.

Read Chapter 35: [link]

#rustlang #AIagents

## LinkedIn
Chapter 35 is out: Balancing and Federation.

This one is the server. Hermes Agent limits how many MCP servers it connects at once: the default is 4, and a config value of 0 means unlimited. It does not pick among backends by outstanding load. That pick is the pool in this chapter. Least outstanding reads four atomics and returns an index. Power of two choices returns the quieter of the two indexes you handed it. Federation is a map from a tool name to the servers that offer it. Nothing opens a socket.

One pick measured 2.502 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: profiles, and secrets that stay out of the prompt. Read Chapter 35: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
