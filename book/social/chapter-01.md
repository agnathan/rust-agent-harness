# Social posts: Chapter 1, "First Contact"

## X thread
1/
Chapter 1 of my "learning Rust by building a multi-agent harness" series is up: First Contact.

One prompt in, one reply out. Four small steps, each with tests. 🧵

2/
Why start with a single model call? Because the agent loop is just that call, repeated.

Hermes Agent (Nous Research) does this in agent/conversation_loop.py: call the model → run tools → repeat until it answers.

3/
I borrowed Hermes's core ideas:
• OpenAI-style messages as the internal format
• providers behind one interface (Hermes: providers/base.py)

In Rust that's a `Provider` trait with an async `complete()`.

4/
The HTTP client is OpenAI-compatible, and OpenRouter is the default (`https://openrouter.ai/api/v1`).

`--model` takes an OpenRouter id: `openai/gpt-4o-mini`, `anthropic/claude-3.5-sonnet`, `meta-llama/...`.

`--base-url` still points the same client at OpenAI, Ollama, or a local fake. The key it reads follows the host.

5/
`harness ask` prints the reply, then the model that actually answered and the token counts, when the response includes them.

`OPENROUTER_API_KEY` for OpenRouter. `--mock` needs no key and no network.

6/
Section 1.4 adds one request timeout. It does not retry. That waits for a later chapter.

Tests talk to a local wiremock server. I did not paste a live API transcript.

7/
Next, Chapter 2: conversations and a chat REPL.

Read Chapter 1: [link]

#rustlang #AIagents

## LinkedIn
Chapter 1 of my "learning Rust by building a multi-agent harness" series is out: First Contact.

Every agent framework comes down to one repeated act: send messages to a model, get a message back. Nous Research's open-source Hermes Agent, my reference architecture, organises everything around that loop (`agent/conversation_loop.py`). It uses OpenAI-style messages internally and puts each model provider behind a common profile (`providers/base.py`).

So I started there, in four steps:
• messages serialized to that JSON shape
• a `Provider` trait and an offline mock, so tests need no key
• one pooled HTTP client whose default server is OpenRouter, with `--model` for ids such as `openai/gpt-4o-mini` or `anthropic/claude-3.5-sonnet`, and `--base-url` for OpenAI, Ollama, and other compatible servers
• a single request timeout, plus the model id and token counts the response actually carried

I'm new to Rust, so the chapter explains each language feature in a short sidebar right when it's needed. The runs in the chapter are the mock and a local fake server.

Next up: conversations and a chat REPL. Read Chapter 1: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
