# Social posts: Chapter 37, "Balancing and Federation"

## X thread
1/
Chapter 37 is up: Balancing and Federation. This one is the server track.

A tool can have several upstreams. pick reads outstanding counters and returns an index. Least outstanding wins. Discovery connects at most 4. The pick does not open a connection. 🧵

2/
I searched Hermes Agent at 9e01b44 for a least-outstanding pick, a backend pool, and federation. I did not find that picker. What I did find is tools/mcp_tool_discovery.py: _DISCOVERY_CONNECT_CONCURRENCY = 4. mcp.discovery_concurrency defaults to 4. 0 means unlimited there. A non-integer or a negative value falls back to 4.

3/
The pool is this chapter's adaptation. Outstanding is an atomic. Power of two keeps the quieter of two indexes the caller passes. Federation is a map from tool name to server names, in memory. Six overlapping connects stay at 4. Unlimited 0 is not implemented. No socket.

4/
The bench is pick_least: four counters, 3, 1, 4, and 2, and the quietest index. The mean is 3.74 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The demo prints "pick: echo via alpha". No key.

5/
Next is chapter 38, back on the harness: profiles. Several personas, a secret map each, and a secret read that fails closed when no scope is installed.

Read Chapter 37: [link]

#rustlang #AIagents

## LinkedIn
Chapter 37 is out: Balancing and Federation. Server track.

Chapter 23's learned gateway is vendored into the crate. The new module is federation. I looked through Hermes Agent for a least-outstanding backend pool and did not find one. The behavior I took from the tree is discovery concurrency: four MCP servers connect at once, from tools/mcp_tool_discovery.py and the mcp.discovery_concurrency default. Hermes treats 0 as unlimited. This gate does not. The pick itself is local. Least outstanding returns the quietest index. The quieter of two indexes is the other policy. Neither one opens a socket. echo on alpha and beta, both idle, picks alpha.

The mean for pick_least is 3.74 ns. The demo prints "pick: echo via alpha".

Next: profiles on the harness. Each persona keeps its own secrets, and an unscoped read fails closed. Read Chapter 37: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
