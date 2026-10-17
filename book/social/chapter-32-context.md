# Social posts: Chapter 32, "Managing Context"

## X thread
1/
Chapter 32 is up: Managing Context.

A context file is scanned before it enters the prompt. A project file that matches is replaced with a blocked sentence. The frozen snapshot is an Arc. Compression runs only when the transcript is over the cap. 🧵

2/
Hermes Agent does this in agent/prompt_builder.py. The cap is 20,000 characters, head 70 percent, tail 20 percent. A user-authored SOUL.md still loads. High-risk tool output is wrapped in agent/tool_dispatch_helpers.py, and an embedded closer is neutralized. The skill marker is [SKILL_PRUNED: …].

3/
I kept four of the threat patterns, not the whole library. The side model is a queue of strings. Under the cap it is not called. The hot path clones the Arc.

4/
Sharing that snapshot measured 15.12 ns. The demo prints "shared: clean". No key.

[personal note]

5/
Next, each agent gets a task and a mailbox. The inbox holds 64, and a full one pushes back.

Read Chapter 32: [link]

#rustlang #AIagents

## LinkedIn
Chapter 32 is out: Managing Context.

The prompt is a choice. Hermes Agent scans files such as AGENTS.md before they enter it, replaces a match with a blocked sentence, and caps a huge file at 20,000 characters while keeping a head and a tail. Tool output from the web, the browser, or an MCP server is wrapped, and a forged closer is rewritten so it cannot end the block early. I kept those rules. The frozen snapshot is an Arc the hot path only clones. Compression, and the memory flush in front of it, run when the transcript is over the cap, and they do not rebuild that snapshot.

Sharing the snapshot measured 15.12 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: one task per agent, and a mailbox that can say it is full. Read Chapter 32: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
