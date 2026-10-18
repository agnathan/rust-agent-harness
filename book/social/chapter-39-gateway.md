# Social posts: Chapter 39, "The Gateway"

## X thread
1/
Chapter 39 is up: The Gateway.

This is the chat gateway. Allowlist, then pairing, then deny. A code is 8 characters and lasts an hour. The message is a push onto an inbox of 64. A supervised restart is blocked. 🧵

2/
Hermes Agent: gateway/pairing.py uses an alphabet with no I, O, 0, or 1. CODE_TTL_SECONDS is 3600. Five failed approvals lock the platform for 3600 seconds. gateway/bot_loop_guard.py allows 20 bot messages in 300 seconds, then a 600 second cooldown. agent/estop.py fails safe when a stat errors.

3/
tools/terminal_tool_guards.py gateway_lifecycle_block returns immediately when the process is not supervised. A supervised hermes gateway restart, stop, or uninstall is blocked. echo stays allowed. Chapter 43 keeps the cron lifecycle check. send_out calls redact_for_egress.

4/
One enqueue measured 152.0 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The demo prints "gateway: queued". submit of a turn does not call FakeModel.

[personal note]

5/
Next is the browser: a page opens only after the secret-URL check and the SSRF check. The snapshot is capped. The text is untrusted.

Read Chapter 39: [link]

#rustlang #AIagents

## LinkedIn
Chapter 39 is out: The Gateway.

The chat gateway decides who may talk to the agent. Hermes Agent default-denies after the allowlist and pairing checks. A pairing code is 8 characters from an alphabet that drops I, O, 0, and 1, lives 3600 seconds, and five wrong attempts lock the platform for 3600 seconds. Two bots answering each other hit a sliding window: 20 messages in 300 seconds, then 600 seconds of cooldown. hermes pause is a sentinel file, and a stat error is treated as paused.

A shell command that would restart, stop, or uninstall the gateway is refused while the process is supervised. tools/terminal_tool_guards.py gateway_lifecycle_block is that check. An unsupervised process is left alone. Outbound text goes through redact_for_egress from chapter 20. A platform message, including a mid-turn steer, is a push onto an inbox of 64. submit does not call the model.

One enqueue measured 152.0 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The demo prints "gateway: queued".

[personal note: one takeaway from writing this chapter]

Next: browser tools. A metadata URL is not opened. Read Chapter 39: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
