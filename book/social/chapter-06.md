# Social posts: Chapter 6, "The Agent Loop"

## X thread
1/
Chapter 6 of my "learning Rust by building a multi-agent harness" series is up: The Agent Loop.

The model returns text or tool calls. The dispatcher runs them. The results become messages. Then it asks again, unless the gate says stop. 🧵

2/
Hermes Agent (Nous Research) does this in agent/conversation_loop.py. The once-per-turn setup is build_turn_context in agent/turn_context.py. The loop checks an interrupt flag and an iteration budget before the next model call. Several tool calls share a thread pool capped at 8 workers. A single call, and anything interactive like clarify, stays sequential.

3/
My gate is one function, check_gate, in front of every model call. Cancel, or a budget of iterations, tokens, time, or cost, and that call does not start.

The check measured 2.711 ns. One mock iteration with no tool call measured 263.9 ns. Eight reads that each slept 1 ms, in one turn, measured 2.392 ms.

4/
A tool can set the cancel flag while it is still running. The next model request does not start. The test counts provider calls and expects 1.

The mock does not invent usage. A token cap only trips when the scripted reply carries a Usage. 0.25 USD is the cost the earlier chapters already use.

5/
Three saved tasks run on the mock: echo a hello through the MCP fixture, answer with no tool, and fan eight reads out under the bound of 8. The score fails if more than 8 run at once. Later chapters re-run this set. No live model call.

[personal note]

6/
Next, the turn state machine: Finished, Cancelled, and Budget become states with a tested transition table.

Read Chapter 6: [link]

#rustlang #AIagents

## LinkedIn
Chapter 6 of my "learning Rust by building a multi-agent harness" series is out: The Agent Loop.

The earlier chapters can talk to a model, store a conversation, run tools, and call an MCP server. They do not yet take a turn. This chapter is that turn. The model returns text or tool calls, the same dispatcher from chapter 3 runs native tools and MCP tools, and the results go back on the transcript.

Nous Research's Hermes Agent runs the same shape in agent/conversation_loop.py. A prologue in agent/turn_context.py builds the turn once. The loop then checks an interrupt and an iteration budget before each model call. Concurrent tool calls use a thread pool capped at 8, and the results are restored in the original order. A single call, and an interactive tool, stay on the calling thread.

I put one function, check_gate, in front of every model call. It measured 2.711 ns. A cancelled tool does not start the next request. Token, cost, and time budgets stop the same way. One mock iteration with no tool call measured 263.9 ns. Eight 1 ms reads in one turn measured 2.392 ms, on the bound of 8.

The mock is scripted and does not invent usage. Three saved tasks pass offline: an MCP echo, a direct answer, and eight overlapping reads. The demo does not call OpenRouter.

[personal note: one takeaway from writing this chapter]

Next up: the turn state machine. Read Chapter 6: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
