# Social posts: Chapter 11, "Approvals and Guardrails"

## X thread
1/
Chapter 11 is up: Approvals and Guardrails.

The loop will run a tool once the name is valid. This chapter decides the command first. Hardline, then a deny list, then the allowlist, then one human prompt. 🧵

2/
Hermes Agent runs that order in tools/approval.py (check_all_command_guards and _floor_block). detect_hardline_command in tools/approval_detection.py is never bypassed, including YOLO. The timeout default in tools/approval_context.py is 300 seconds, and a miss denies.

3/
A denial tells the model the user has not consented and must not retry. The fake model stops the script, so the next call is not issued. Three denials append the circuit breaker. Cron denies a dangerous command and still allows echo.

4/
Two asks share one mutex. The tool permit is dropped before the wait. A skip of echo hello measured 251.1 ns.

[personal note]

5/
Hooks are next. A pre_tool_call hook can block, rewrite the arguments, or escalate into this prompt.

Read Chapter 11: [link]

#rustlang #AIagents

## LinkedIn
Chapter 11 is out: Approvals and Guardrails.

The agent loop already stops for a bad tool name. What it did not have was a single check that refuses a destructive command before the tool runs. Hermes Agent puts hardline patterns, the user's deny list, an allowlist, an auxiliary review, and a human prompt in that order, and the hardline floor still fires when YOLO is on. I wrote that pipeline, with a fake model and no API key.

Two calls that need a human share one prompt. The wait gives the tool-concurrency permit back first. A skip of a safe command measured 251.1 ns.

[personal note: one takeaway from writing this chapter]

Next: hooks that can veto a call or send it to this prompt. Read Chapter 11: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
