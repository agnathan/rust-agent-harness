# Social posts: Chapter 15, "Limits and Breakers"

## X thread
1/
Chapter 15 is up: Limits and Breakers.

This one is the server. Each tool accepts 32 calls and runs 8. A retry runs only when the tool is marked idempotent. Three failures open a breaker, and the next call never reaches the backend. 🧵

2/
Hermes Agent caps a turn at 50 web_search calls and 50 delegate_task spawns (agent/tool_guardrails.py). A tasks array counts as its length. list, steer, and stop still run after the cap. The block happens before the call.

3/
429 and 503 do not retry. A transport error on an idempotent tool does, once. A hedge starts a second call only for an idempotent tool. The 30-second client reconnect cooldown is not this breaker.

4/
The open-breaker check measured 31.97 ns. Taking one execute permit and dropping it measured 57.73 ns. The demo is a fake script. No socket. No key.

5/
Next is providers: an Anthropic body, a Bedrock body, an OpenRouter route, and a key rotation that keeps the same client.

Read Chapter 15: [link]

#rustlang #AIagents

## LinkedIn
Chapter 15 is out: Limits and Breakers.

The failure chapter decides retry or fallback in the harness. The gateway still needs its own bound. I gave each tool 32 accept permits and 8 execute permits. A timeout drops both. A second attempt runs only when the tool is marked idempotent, and a 429 or 503 does not retry at all. Three failures open a breaker.

Hermes Agent already stops a turn at 50 web searches and 50 subagent spawns. Those caps landed here, including the batch count and the control actions that still run. The open-breaker check measured 31.97 ns.

Next: native provider adapters and key rotation. Read Chapter 15: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
