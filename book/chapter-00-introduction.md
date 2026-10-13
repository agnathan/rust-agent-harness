# Chapter 0: Why I'm learning Rust by building a multi-agent harness

*Draft. Bracketed notes like [personal note] are where my own story goes.*

I'm learning Rust. Instead of working through the language one feature at a time and building toy examples, I'm going to learn it by building something I actually want: a fast, reliable harness for running *teams* of AI agents that work together.

[personal note: what got me interested in AI agents and in Rust in the first place, and what I do day to day]

This book is my learning log. Each chapter takes one problem that every agent system has to solve, looks at how an existing open-source agent solves it, and then builds a Rust version from scratch, with tests, one small step at a time. I'm writing it as I go, which means you'll see where I struggled as well as where things clicked.

## What is an "agent harness"?

Strip away the hype and an AI agent is a loop:

1. Send the conversation so far, plus a list of available tools, to a language model.
2. If the model answers with text, you're done.
3. If the model asks to call tools ("read this file", "search the web", "run this command"), run them, append the results to the conversation, and go back to step 1.

The *harness* is everything around that loop that turns it from a demo into something you can trust:
- talking to different model providers
- defining and running tools safely
- enforcing budgets so the agent can't loop forever
- recovering from rate limits and network errors
- compressing the conversation when it gets too long
- saving sessions
- letting a human interrupt or redirect the agent
- for multi-agent systems, letting agents hand work to each other, run in parallel, and be cancelled cleanly

The loop is ten lines. The harness is most of the work.

## The reference: Hermes Agent

The agent I'm learning from is **Hermes Agent** by Nous Research, an open-source (MIT) project at github.com/NousResearch/hermes-agent. Its README calls it a "self-improving AI agent": it keeps its own memory files, writes reusable "skills" from experience, refines them as it uses them, and can search its own past conversations.

Hermes is a big, mature system, and it's worth knowing what's inside before borrowing ideas from it:
- **Many front ends, one core.** The same agent runs behind a terminal UI, a messaging gateway (Telegram, Discord, Slack, WhatsApp, Signal and more), a desktop app, and protocol adapters. They all call the same `AIAgent` core.
- **The loop, split into phases.** The turn loop lives in `agent/conversation_loop.py`, and each phase of a turn (preparing the request, calling the model, handling errors, running tools, compressing context, deciding when to stop) has its own module under `agent/turn_*.py`.
- **Tools that register themselves.** Each tool file adds itself to a registry (`tools/registry.py`) with a JSON schema, a handler, and an availability check. "Toolsets" control which tools the model actually sees.
- **Any model provider.** Each provider is described by a profile plugin, with adapters for APIs that don't follow OpenAI's format.
- **Subagents.** A `delegate_task` tool spawns child agents with a fresh context, a restricted set of tools, a shared iteration budget, and limits on concurrency and nesting depth. Only a summary comes back to the parent.
- **Memory and persistence.** Sessions are stored in SQLite with full-text search. Memory lives in plain markdown files plus optional memory-provider plugins. Long conversations are compressed by summarising older turns with a helper model.
- **Automation.** There's a built-in cron scheduler, and a kanban-style task board that lets multiple agent processes collaborate.

It also follows a couple of strict design rules that I found instructive. The biggest: **never change the beginning of a conversation mid-session**, because model providers cache that prefix, and changing it makes every later call slower and more expensive. Context compression is the single allowed exception. The second: **keep the core small** and add new capabilities as plugins and skills rather than core features.

Hermes is written almost entirely in Python, and it's large: thousands of modules and hundreds of thousands of lines including tests.

## Why rebuild ideas from it in Rust?

Not because Python is "bad". Hermes works, and much of its complexity comes from real-world problems it has already solved. But reading its code with a specific question in mind ("what would it take to run *many* agents at once, cooperating?") points to a few places where the language works against the design:

- **Every running agent occupies a thread.** Hermes's agent loop is synchronous. The messaging gateway is asynchronous, but it hands each conversation turn to a worker thread. Child agents run in thread pools. Parallel tool calls run in another pool. Python's global interpreter lock means only one of those threads runs Python code at any moment.
- **Cancelling work is cooperative.** Python can't forcibly stop a thread, so stopping a stuck subagent depends on flags, heartbeats, and timeouts, and some code exists just to deal with work that won't stop.
- **Shared state needs constant care.** Because one process can serve several user profiles across many threads, a lot of Hermes's rules are about making sure the right settings and credentials are used on the right thread.

Rust's async runtime (tokio) can run thousands of lightweight tasks on a handful of threads. Dropping a future cancels it immediately and completely. The compiler won't let two tasks mutate the same data without synchronisation. Those properties line up closely with what a multi-agent harness needs.

[personal note: other reasons Rust appeals to me, e.g. performance, reliability, wanting to learn a systems language, career goals]

## The plan

The course behind this book has sixteen lessons. Each has a test suite that fails until I've written the code correctly. Roughly:

1. **Rust fundamentals through agent types.** Messages and roles as structs and enums, ownership through conversation history, error handling through command parsing, JSON through the OpenAI wire format, and traits through a `Tool` interface and registry.
2. **A working agent.** A synchronous loop against a scripted fake model, then a real OpenAI-compatible client that also works with local models.
3. **Async and streaming.** tokio, concurrent work, and parsing tokens as they stream in.
4. **Making the turn explicit.** Hermes's turn logic, rebuilt as an explicit state machine: one `TurnState` enum and a transition table I can test for every edge case.
5. **Parallel tools.** A planner that decides which tool calls can safely run at the same time, the same distinction Hermes makes between read-only tools and tools that write to files.
6. **Agents as actors.** Each agent becomes an independent task with a bounded inbox, so a busy agent pushes back instead of being flooded with work.
7. **Teams.** Cancellation trees (cancel a parent and every descendant stops), a shared budget, a supervisor that delegates to child agents, and a message bus that supports orchestrator–worker setups, peer messaging, and a shared task board.
8. **Making it real.** Saving sessions to SQLite, tracing, and benchmarks.

I'm not trying to port Hermes. It has years of integrations I don't need. I'm borrowing its *ideas*: stable prompts, summary-only results from subagents, restricted toolsets for children, and classifying an error before deciding how to recover. Then I'm seeing what changes when the language's guarantees do some of the work.

## How to read this book

If you're new to Rust like me, read the chapters in order and do the exercises. The compiler is strict, and early on that's frustrating. I'll be honest about the moments it beat me.

[personal note: first impressions of Rust after the setup chapter]

If you already know Rust and are here for the agent architecture, skim to Chapter 9 (the loop), Chapter 10 (the turn state machine), and Chapters 27 and 33–34 (the supervisor, actors, and supervision). Each has a "How Hermes does it" section with file paths, so you can read the Python next to the Rust.

Let's start with `cargo new`.
