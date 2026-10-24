# Social posts: Chapter 23, "Learned Tools"

## X thread
1/
Chapter 23 is up: Learned Tools. Server track.

The offered tool list is installed before the request. session_tools reads it. The score does not run on that read. 🧵

2/
Hermes Agent's code_execution toolset runs Python that calls tools (toolsets.py). SANDBOX_ALLOWED_TOOLS is seven names. DEFAULT_MAX_TOOL_CALLS is 50 (tools/code_execution_tool.py). This chapter's script is lines of `call tool json`, not Python.

3/
A memory mention is worth 3. A past use that shares a task token is worth 1. A zero stays out. Composite and macro names expand to steps. They do not run the steps.

4/
Copying 32 names measured 76.1 ns. Scoring 1,000 rows measured 149.64 µs. The demo offer is read_file. No key. No socket.

5/
That is chapter 23. The next harness chapter is skills: a cache-stable index of SKILL.md files.

Read Chapter 23: [link]

#rustlang #AIagents

## LinkedIn
Chapter 23 is out: Learned Tools.

This is the server crate. It vendors the chapter 15 gateway instead of path-depending on it, then installs a predicted tool list before the request. session_tools copies that list. It does not score memory or the usage log on the read.

Hermes Agent allows seven tools inside code execution and stops a script at 50 calls. The Rust script is `call tool json` lines, checked against that allowlist and against the offer. A composite or a macro is a saved name. Expanding it does not run the steps.

Copying 32 names measured 76.1 ns. Scoring 1,000 rows measured 149.64 µs. The demo prints read_file and ok. No key.

Next: skills, on the harness. Read Chapter 23: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
