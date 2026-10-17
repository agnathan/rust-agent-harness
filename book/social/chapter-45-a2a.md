# Social posts: Chapter 45, "A2A for External Agents"

## X thread
1/
Chapter 45 is up: A2A for External Agents.

A2A is for a URL outside this process. local:// is refused before it takes a permit. The pool holds 8. Cancel is a flag the in-flight call can see. 🧵

2/
Hermes Agent says to use delegation or kanban on the same machine. plugins/platforms/a2a/adapter.py maps tasks/cancel to CancelTask. security.py frames every inbound message, including one that starts with a slash, and redacts outbound text.

3/
This crate records URLs in an HttpPool and does not open a socket. The card name is harness, skill research. Fake replies. No key.

4/
Cancel of an in-flight waiter measured 48.33 µs. The demo prints "a2a: ok".

[personal note]

5/
Next is seeing inside: a span per event, and a field that is not the prompt.

Read Chapter 45: [link]

#rustlang #AIagents

## LinkedIn
Chapter 45 is out: A2A for External Agents.

MCP already covers tools we expose. Agent-to-agent is for a peer in another process. Hermes Agent documents that split and cancels a remote task with tasks/cancel. This chapter is the boundary in memory. `call` returns `InProcess` for a `local://` URL and does not acquire a permit. Other calls take one of 8 permits, scrub `Bearer` and `sk-`, and record the URL. A cancel flag set before send does not append to that list. `wrap_inbound` keeps a leading `/shutdown` inside the frame.

There is no socket. The agent card name is `harness` and the skill is `research`.

Cancel of a waiting thread measured 48.33 µs. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: seeing inside. Read Chapter 45: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
