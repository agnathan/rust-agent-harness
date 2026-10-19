# Social posts: Chapter 39, "The Gateway"

## X thread
1/
Chapter 39 is up: The Gateway.

This is the chat gateway, not the MCP server. Allowlist, then pairing, then deny. A code is 8 characters and lasts an hour. The message is a push onto an inbox of 64. 🧵

2/
Hermes Agent: gateway/pairing.py uses an alphabet with no I, O, 0, or 1. Five failed approvals lock the platform for an hour. gateway/bot_loop_guard.py allows 20 bot messages in 300 seconds, then a 600 second cooldown. agent/estop.py fails safe when a stat errors.

3/
A busy session rejects handoff.request with code 4009. That re-bind was deferred from chapter 30. The code in this crate is stored in a Vec so a test can approve it. Hermes stores a salted hash.

4/
One enqueue measured 174.5 ns. The demo prints "gateway: queued".

[personal note]

5/
Next is the MCP gateway service: replicas, a drain, and the client checks around sampling and malware advisories.

Read Chapter 39: [link]

#rustlang #AIagents

## LinkedIn
Chapter 39 is out: The Gateway.

The chat gateway decides who may talk to the agent. Hermes Agent default-denies after the allowlist and pairing checks. A pairing code is 8 characters from an alphabet that drops I, O, 0, and 1, lives one hour, and five wrong attempts lock the platform for an hour. Two bots answering each other hit a sliding window: 20 messages in 300 seconds, then 600 seconds of cooldown. hermes pause is a sentinel file, and a stat error is treated as paused.

This chapter keeps those decisions in memory. A platform message, including a mid-turn steer, is a push onto an inbox of 64. Text chunks from a stream fold into one string. A running session refuses a platform re-bind with error 4009.

One enqueue measured 174.5 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: the MCP gateway service. Read Chapter 39: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
