# Social posts: Chapter 1, "First Contact"

## X thread
1/
Chapter 1 of my "learning Rust by building a multi-agent harness" series is up: First Contact.

The goal: one prompt in, one reply out. Done properly. 🧵

2/
Why start with a single model call? Because the agent loop is just that call, repeated.

Hermes Agent (Nous Research) does exactly this in agent/conversation_loop.py: call the model → run tools → repeat until it answers.

3/
I borrowed Hermes's core ideas:
• OpenAI-style messages as the internal format
• providers behind one interface (Hermes: providers/base.py)

In Rust that's a `Provider` trait with a single async `complete()` method.

4/
Two implementations:
• `OpenAiCompatible` (reqwest + serde): works with OpenAI, OpenRouter, Ollama, LM Studio, vLLM… via 3 settings
• `MockProvider`: canned replies, records what it was sent, so tests need no network, no keys, no cost

5/
On top of that, a tiny clap CLI:

harness ask "Explain a turn loop in one sentence."

Config via OPENAI_API_KEY / OPENAI_BASE_URL / HARNESS_MODEL, or --mock for fully offline runs.

6/
About 200 lines of Rust. New-to-me concepts: cargo workspaces, enums + serde, traits, Result and `?`, and just enough async/await to make an HTTP call.

`cargo test` → 3 passed.

7/
Next, Chapter 2: conversations and a chat REPL. That's where Rust's ownership rules start to bite.

Read Chapter 1: [link]

#rustlang #AIagents

## LinkedIn
Chapter 1 of my "learning Rust by building a multi-agent harness" series is out: First Contact.

Every agent framework comes down to one repeated act: send messages to a model, get a message back. Nous Research's open-source Hermes Agent, my reference architecture, organises everything around that loop (`agent/conversation_loop.py`). It uses OpenAI-style messages internally and puts each model provider behind a common profile (`providers/base.py`).

So I started there. In about 200 lines of Rust:
• a two-crate workspace (core library plus a thin CLI)
• a `Provider` trait with an OpenAI-compatible implementation (reqwest + serde + tokio) that works with OpenAI, OpenRouter, Ollama and other compatible servers
• a mock provider, so tests run offline and give the same result every time
• a `harness ask "..."` command configured through environment variables

I'm new to Rust, so the chapter explains each language feature (cargo, structs and serde, Result and `?`, traits, async/await) in a short sidebar right when it's needed.

[personal note: one takeaway from writing this chapter]

Next up: conversations and a chat REPL. Read Chapter 1: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
