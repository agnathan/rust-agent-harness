# Social posts: Chapter 14, "When Things Fail"

## X thread
1/
Chapter 14 is up: When Things Fail.

An empty reply, a cut-off reply, a repetition loop, a timeout, and a crashed session are not the same failure. This chapter names them and picks retry, fallback, or stop. 🧵

2/
Hermes Agent retries an empty completion 3 times, or once if that attempt costs more than $0.25 (agent/empty_response_guard.py). Two proven empties skip straight to the fallback. agent/repetition_guard.py stops a loop of at least 400 characters when a 60-character window covers half the output.

3/
The timeout drops the tool permit. Three retries hold one permit at a time. A fake session that crashes once reconnects and then answers.

4/
The empty-budget check measured 0.9040 ns. The repetition scan on 800 characters measured 2.551 µs. The demo is two fake models. No key.

[personal note]

5/
Next is the server side: per-tool limits, retries only for idempotent tools, and a breaker. The web-search cap of 50 lands there.

Read Chapter 14: [link]

#rustlang #AIagents

## LinkedIn
Chapter 14 is out: When Things Fail.

The agent loop already stops for a budget or a bad tool name. What it deferred was an empty reply and a model that repeats itself. Hermes Agent retries an empty completion up to three times, drops that to one when a single attempt costs more than $0.25, and treats two consecutive zero-output empties as deterministic. A repetition that covers at least half of a long reply is not continued.

I wrote those rules against a fake model. A timeout cancels the in-flight call and gives the concurrency permit back. The empty-budget check measured 0.9040 ns.

[personal note: one takeaway from writing this chapter]

Next: limits and circuit breakers on the gateway. Read Chapter 14: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
