# Social posts: Chapter 17, "Built-in Tools II: Files, Terminal, Todo, and Clarify"

## X thread
1/
Chapter 17 is up: Built-in Tools II. Files, a terminal, a todo list, and clarify.

A path that leaves the workspace root is refused. A write of a file this task has not read is refused. The terminal asks the approval floor before it starts a process. 🧵

2/
Hermes Agent stamps a file on read and blocks a write when the stamp is missing or the mtime moved (tools/file_state.py, tools/file_tools_write_guards.py). A relative path outside the root warns there. Here it is an error. patch replaces exactly one old_string.

3/
The terminal calls the same deny_tool as chapter 11, then the chapter 13 worker. A hardline command never reaches the binary. Output is cut on line length, then line count, then bytes. The foreground timeout stops at 600 seconds.

4/
todo is a list in memory, merged by id, with a revision that rejects a stale writer. clarify pauses the turn for one queued reply. A child agent may not call it. `17/patch_apply` measured 255.1 µs. `17/todo_merge` measured 131.2 ns. No key. No prompt.

5/
Next is provider adapters: an Anthropic body, a Bedrock body, an OpenRouter route, and key rotation on the same client.

Read Chapter 17: [link]

#rustlang #AIagents

## LinkedIn
Chapter 17 is out: Built-in Tools II, Files, Terminal, Todo, and Clarify.

Search was not enough. The harness needed to read a workspace without walking out of it, and to run a command without growing a second approval path. A write of a file this task has not read is `stale_write_blocked`. The terminal calls `deny_tool` first. `rm -rf /` comes back blocked, and the worker is not spawned.

Hermes Agent keeps the todo list as agent state and refuses `clarify` for a child. Both landed here. Tests use a scripted model and a queued reply, not a person at a keyboard. `17/patch_apply` measured 255.1 µs. `17/todo_merge` measured 131.2 ns.

Next: native provider adapters and key rotation. Read Chapter 17: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
