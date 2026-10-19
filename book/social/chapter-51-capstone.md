# Social posts: Chapter 51, "Load and Token Capstone"

## X thread
1/
Chapter 51 is up: Load and Token Capstone.

Thirty-two tool cards. Every card, the first eight, or the one named tool_07. Success is the tool offered and the model reply matching. No socket. 🧵

2/
Hermes Agent counts schema characters with 4 per token in tools/tool_search_catalog.py. A deadline that the wide arms miss still leaves the dynamic arm inside the limit. The call bound stays 8.

3/
Integer division makes an 18-byte schema 4 tokens. The arms are 128, 32, and 4. One hundred requests run as 13 waves. Fake replies. No key. oha is not run.

4/
The dynamic arm measured 245.5 ns. The load of 100 measured 30.05 µs. The demo prints "capstone: every=128 static=32 dynamic=4 success=1/1/1 load=100/100 p99_ns=1160". That p99 is the debug run, not the Criterion mean.

[personal note]

5/
Next is evals: the saved task names, one red-team case per layer, and no fail threshold.

Read Chapter 51: [link]

#rustlang #AIagents

## LinkedIn
Chapter 51 is out: Load and Token Capstone.

The server track ends by comparing what a completed task costs in schema tokens. Hermes Agent divides schema characters by 4. This chapter builds 32 cards, `tool_00` through `tool_31`, and scores three arms: every card, a static set of the first eight, and the dynamic card whose name is the one the task needs. An arm succeeds only when that card is offered and, if a deadline is set, the token count is inside it. `tool_07` succeeds on all three. `tool_31` fails the static arm. A missing name fails all three.

The load runs 100 of those dispatches in memory, eight at a time, so 100 requests are 13 waves. Nothing listens. oha is not invoked. The debug demo's `p99_ns` is a nearest-rank sample from that process, separate from the Criterion mean of the whole batch.

The dynamic arm measured 245.5 ns. The load of 100 measured 30.05 µs. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: evals. Read Chapter 51: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
