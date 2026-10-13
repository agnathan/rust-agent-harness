# Social posts: Chapter 9, "The Agent Loop"

## X thread
1/
Chapter 9 of my "learning Rust by building a multi-agent harness" series is up: The Agent Loop.

Ten sections. The model returns text or tool calls. The dispatcher runs them. The results become messages. Then it asks again, unless the turn has a reason to stop. 🧵

2/
Hermes Agent (Nous Research) does this in agent/conversation_loop.py. Validation is agent/turn_tool_validation.py. Identical calls and short cycles are agent/tool_guardrails.py. The iteration counter is agent/iteration_budget.py. Soft stop versus hard stop is agent/interrupt_control.py. A stop that can keep the turn going is agent/turn_stop_gates.py, which I left for later.

3/
I adopted the parts that belong in the loop now: repair a close tool name, retry broken JSON, return tool errors so the roles still alternate, warn on the newest tool result, canonicalize arguments on the copy that is sent, and race each model call and each tool call with a CancellationToken.

Empty-response retries, length continuation, and the repetition guard wait for When Things Fail. Preflight compression waits for Managing Context.

4/
The mock is the default. `--live` is optional and CI does not pass it. With the key unset, the process prints `missing OPENROUTER_API_KEY: pass --api-key, or run with --mock` and opens no socket.

5/
On the finished loop, one scripted iteration with no tool call measured 608.0 ns. The same bench before those checks measured 248.5 ns. check_gate measured 11.92 ns. Eight reads that each slept 1 ms measured 2.421 ms, on the bound of 8.

Repair was 188.8 ns. A stall check was 100.0 ns. The budget notice was 58.20 ns. Truncation was 3.174 µs. A steer append was 47.12 ns. Canonical arguments were 771.0 ns.

6/
Seven mock evals pass: echo, a direct answer, eight overlapping reads, a tool that fails once and then recovers, the iteration cap, an unknown name that repairs to echo, and a stall. No live model call.

[personal note]

7/
Next, chapter 10, the turn state machine: TurnOutcome is already one variant per exit. That chapter is the transition table around it.

Read Chapter 9: [link]

#rustlang #AIagents

## LinkedIn
Chapter 9 of my "learning Rust by building a multi-agent harness" series is out: The Agent Loop.

The earlier chapters can talk to a model, store a conversation, run tools, and call an MCP server. They do not yet take a turn. This chapter is that turn, split into ten sections so each check has its own demo, tests, and bench.

Nous Research's Hermes Agent runs the same shape in agent/conversation_loop.py. I read that file, plus agent/turn_tool_validation.py, agent/tool_guardrails.py, agent/iteration_budget.py, agent/interrupt_control.py, and agent/turn_stop_gates.py, and took the ideas that belong in the loop now. A close unknown tool name is repaired. Broken JSON is retried, then returned as a tool result so the roles still alternate. An identical call, and a short A, B cycle, can halt the turn. A budget warning and a time-budget wrap-up are appended to the newest tool result. Each model call and each tool call is raced with a CancellationToken. Soft stop and hard stop are different: hard stop drops a pending steer.

Empty-response retries, length continuation, and the repetition guard are deferred to When Things Fail. Preflight compression is deferred to Managing Context. The stop-gate nudges in turn_stop_gates.py wait until this harness has a verifier and a board.

The mock is the default. An optional `--live` flag calls OpenRouter with echo and note. CI does not pass it. One finished mock iteration measured 608.0 ns. check_gate measured 11.92 ns. Eight 1 ms reads in one turn measured 2.421 ms. Seven saved and stress tasks pass on the mock.

[personal note: one takeaway from writing this chapter]

Next up: chapter 10, the turn state machine. Read Chapter 9: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
