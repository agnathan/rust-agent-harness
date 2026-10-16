# Social posts: Chapter 5, "A Minimal MCP Server"

## X thread
1/
Chapter 5 of my "learning Rust by building a multi-agent harness" series is up: A Minimal MCP Server.

The harness decides what enters the prompt. This chapter is the other process: a fixed tool list and a gate. 🧵

2/
Hermes Agent serves a fixed list from mcp_serve.py. Registration order is list_tools order. run_mcp_server speaks stdio. It is a messaging bridge, not a catalog.

3/
I kept the fixed list and the registration order. A duplicate name is refused, which is the chapter 3 rule. Hermes overwrites a same-toolset duplicate in tools/registry.py.

This step binds no socket. HTTP is a later chapter.

4/
The gate accepts 32 calls and runs 8. The 33rd is Saturated. A permit round trip measured 79.93 ns. Eight tasks that each sleep 1 ms measured 2.307 ms.

5/
There is no model here, so there is no API key. cargo run registers echo and prints hello.

[personal note]

6/
Next, the catalog moves into this process: BM25, plugins, and a reload that swaps an Arc.

Read Chapter 5: [link]

#rustlang #AIagents

## LinkedIn
Chapter 5 of my "learning Rust by building a multi-agent harness" series is out: A Minimal MCP Server.

Chapter 4 can call an MCP server. It cannot be one. Hermes Agent's mcp_serve.py is a stdio server with a fixed tool list, registered in list order. I took that shape and left the socket for later.

The Rust in this chapter is two semaphores. One caps accepted calls at 32, including calls waiting to run. The other caps the tool body at 8. A permit is a value. Dropping it gives the slot back. Holding a mutex across the body would pin a worker. The permit does not.

The keyless demo registers echo and returns hello. A permit round trip measured 79.93 ns. Eight 1 ms sleeps measured 2.307 ms.

[personal note: one takeaway from writing this chapter]

Next: a server-side catalog. Read Chapter 5: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
