# Social posts: Chapter 19, "Memory Files"

## X thread
1/
Chapter 19 is up: Memory Files.

MEMORY.md is the agent's notes. USER.md is the user profile. Entries are separated by a section mark. The snapshot in the prompt is taken when the store opens. A write during the session does not change it. 🧵

2/
Hermes Agent budgets memory at 2,200 characters and the user profile at 1,375 (tools/memory_tool.py). The delimiter is a section sign on its own line (tools/memory_tool_store.py). A strict scan blocks the write. authorized_keys is ssh_backdoor. A zero-width space is invisible unicode.

3/
The approval gate stays off. Chapter 23 is where a human accepts a memory write. This chapter writes when add is called, and it refuses a poisoned entry before the file changes.

4/
Enqueue of one turn measured 54.59 ns. The demo file says "likes tea". The snapshot prints frozen. No key.

[personal note]

5/
Next is recall: FTS5 over the session file, and a search that runs only when the tool is called.

Read Chapter 19: [link]

#rustlang #AIagents

## LinkedIn
Chapter 19 is out: Memory Files.

The memory tool finally has files. MEMORY.md and USER.md, entries split by a section mark, with the character budgets Hermes Agent uses. The system-prompt snapshot is an Arc taken at open. Writing an entry updates the file and leaves that Arc alone, so the prompt prefix does not change mid-session.

A strict scan blocks authorized_keys, a zero-width space, a context exfiltration phrase, and a hardcoded key. The write-approval gate stays off until the skills chapter. One enqueue measured 54.59 ns.

[personal note: one takeaway from writing this chapter]

Next: searching past sessions. Read Chapter 19: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
