# Social posts: Chapter 24, "The Reflection Loop"

## X thread
1/
Chapter 24 is up: The Reflection Loop.

Every 10 user turns, and every 10 tool calls, turn finalize builds a reviewer. The reviewer can use skill tools. It gets memory only when the memory nudge fired. Cancel is a flag. 🧵

2/
Hermes Agent sets those intervals in agent/agent_init.py (both default to 10). The fork's own intervals are set to 0 in agent/background_review.py so the reviewer does not nudge itself. A skill-nudge review never gets the memory tool.

3/
The whitelist is skills, plus read_file and search_files. terminal stays off. Hermes will wait up to 2 seconds for the thread to notice cancel. This chapter sets the flag and joins. It does not sit out that 2 seconds.

4/
Building the fork measured 60.16 ns. Cancelling the spinner measured 819.8 µs. The demo note is "prefer short notes". No key.

[personal note]

5/
Next is curation: stale, then archived, off the request path, with the same scan and approval gate as a skill write.

Read Chapter 24: [link]

#rustlang #AIagents

## LinkedIn
Chapter 24 is out: The Reflection Loop.

A finished turn can teach the next one, and that work does not belong inside the turn the user is waiting on. Hermes Agent nudges a background review every 10 user turns and every 10 tool iterations, then forks a reviewer whose own nudge intervals are 0. Memory is on that fork only when the memory nudge fired. I did the same: turn finalize builds the fork, terminal is refused, and cancel is a flag the worker checks.

Building the fork measured 60.16 ns. Joining a cancelled review measured 819.8 µs.

[personal note: one takeaway from writing this chapter]

Next: a curator that archives unused skills off the request path. Read Chapter 24: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
