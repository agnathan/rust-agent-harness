# Social posts: Chapter 23, "Learned Tools"

## X thread
1/
Chapter 23 is up: Learned Tools.

This one is the server. A score over past uses and memory installs the offered set before the request. session_tools only reads that set. 🧵

2/
Hermes Agent does not rank tools from memory. It does have composite toolsets (toolsets.py, is_composite) and a code_execution toolset. SANDBOX_ALLOWED_TOOLS is seven names. DEFAULT_MAX_TOOL_CALLS is 50 (tools/code_execution_tool.py).

3/
A composite and a macro expand to steps. They do not run them. Code mode here is lines of "call tool json", not Python. A tool outside the allowlist or the offer never reaches the backend.

4/
Applying a 32-name offer measured 79.01 ns. Scoring 1,000 usage rows measured 153.1 µs, and session_tools does not call it. The demo offers read_file. No key. No socket.

[personal note]

5/
Next is skills: a directory of SKILL.md and an index that stays put for the session.

Read Chapter 23: [link]

#rustlang #AIagents

## LinkedIn
Chapter 23 is out: Learned Tools.

The session toolset could already reveal a name after a search. It did not remember which tools a task tends to use. The gateway now scores a usage log plus memory and a session note, stores the names, and lets the request path copy that list. The score itself stays off session_tools.

Hermes Agent's closest pieces are composite toolsets and the code_execution tool, which only stubs an allowlist of seven tools and stops a script at 50 calls. I kept both limits. The script is a line of "call tool json" rather than a Python sandbox. Applying 32 names measured 79.01 ns.

[personal note: one takeaway from writing this chapter]

Next: using skills. Read Chapter 23: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
