# Social posts: Chapter 3, "Tools"

## X thread
1/
Chapter 3 of my "learning Rust by building a multi-agent harness" series is up: Tools.

A name, a description, a JSON Schema, and a content result. That shape is MCP's, before this harness speaks MCP. 🧵

2/
Why not jump to an MCP client? Because the agent loop should not care whether a tool is native or remote.

Hermes Agent (Nous Research) registers tools in tools/registry.py: schema, handler, availability check. Toolsets decide what the model is told about.

3/
In Rust that's a `Tool` trait stored as `Arc<dyn Tool>`.

A catalog hides a toolset unless its env var is set and a check accepts the value. The model is only told about the tools that passed.

4/
Reads in one batch run together, up to 8. A write runs alone.

On this machine, eight tools that each sleep 20 ms finished in 21.4 ms together and 170.0 ms one at a time. The criterion run, with a 1 ms sleep, was 2.362 ms vs 17.55 ms.

5/
Lookup in a 50-tool registry was 13.96 ns. Building the available list for 50 tools was 21.89 µs.

No live model call. The runs are the mock tools.

[personal note]

6/
Next, Chapter 4: an MCP client, so a tool in another process can use this same registry.

Read Chapter 3: [link]

#rustlang #AIagents

## LinkedIn
Chapter 3 of my "learning Rust by building a multi-agent harness" series is out: Tools.

The tool shape I wanted is the one MCP already uses: a name, a description, a JSON Schema for the arguments, and a content result. Nous Research's Hermes Agent registers tools that way (`tools/registry.py`) and uses toolsets so the model only sees the tools that should be available. I built the native version of that first. The MCP client is the next chapter, and it will share this registry.

Three steps:
• a `Tool` trait and a registry
• named toolsets gated on an environment variable and a check
• read-only calls run together, up to 8, and a write runs alone

Eight tools that each waited 20 ms finished in 21.4 ms together and 170.0 ms one at a time. A 50-tool lookup was 13.96 ns. I'm new to Rust, so trait objects, a boxed check, and a semaphore show up in short sidebars at the moment they're needed. Nothing in the chapter is a live model transcript.

[personal note: one takeaway from writing this chapter]

Next up: an MCP client. Read Chapter 3: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
