# Chapter 0: Why I'm learning Rust by building a multi-agent harness

*Draft. Bracketed notes like [personal note] are where my own story goes.*

I'm learning Rust by building the thing I actually want: a harness that can run teams of agents, and that can get better at a kind of task the longer it runs. A tutorial that introduces `struct` and then stops would teach the syntax and leave the design untested. A harness has to keep a conversation, call tools, refuse a bad call, remember a fact, and hand work to another agent. Each of those problems forces a language feature at the moment it is needed, which is the pace this book uses.

An agent, stripped of the product language, is a loop. You send the messages so far and a list of tools to a model. If the model answers with text, the turn is over. If it asks for a tool, you run the tool, append the result, and ask again. The loop is short. The harness is everything that makes the loop trustworthy: which provider you called, which tools the model was allowed to see, how long it may run, what happens when a call fails, where the session is stored, and how a second agent receives only the part of the work it should see. Hermes Agent, the open-source system this book learns from, already has a version of that harness, including the closed loop in which an agent writes skills, keeps memory, searches its own sessions, and curates what it learned. This book rebuilds the ideas that matter in Rust, one chapter at a time, with tests.

The book grows two programs, and it keeps them apart on purpose. The harness decides what enters a prompt. The MCP gateway is a separate server the harness talks to as a client. The server holds a large catalog and, later, limits, a learned offer, pools, and a service front. Merging them would make every agent carry every tool. Chapters 5, 7, 8, 15, 23, 37, 42, and 51 are that server. They bind no socket. The other chapters copy the previous harness forward and add one module.

By the last chapter the harness has a turn table, approvals, a sandbox, sessions, memory, recall, skills, a reflection fork, curation, a supervisor and the other team patterns, a bounded inbox, a chat gateway, browser and vision tools behind traits, cron, an editor inbox, a blackboard, a kanban board, a bus, a team skill library, an external-agent call, tracing, and an eval suite that re-runs the early saved tasks. The server side ends in a token comparison of three ways to offer tools. You will be able to run each chapter's demo offline. The reader's exercises live in `my-harness/tests/`, and the finished reference lives in `steps/`.

Rust shows up in sidebars when the design needs it: a struct for a message, an enum for an event, a trait for a tool, ownership for a conversation you do not want aliased by accident, `async` for overlapping reads, and `Arc` for a snapshot many tasks share. Nothing here assumes you have written Rust before. If you already have, the sidebars are short and the design is the point.

**In this chapter**

- What is an "agent harness"?
- The reference: Hermes Agent
- Why rebuild ideas from it in Rust?
- The plan
- How to read this book

[personal note: what got me interested in AI agents and in Rust in the first place, and what I do day to day]

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

The book is six parts and chapters 0 through 52. The outline is [`book/book-outline.md`](book-outline.md). Each chapter has a milestone test in `my-harness/tests/chNN.rs` that fails until the code for that chapter is in place, and a finished reference under `steps/`.

1. **Part I, one agent.** Messages, a provider, tools, an MCP client, then the agent loop and the turn table that loop drives. Approvals, hooks, a sandbox, and failure handling sit on that same harness. A separate MCP gateway starts in chapter 5 and grows beside the harness (chapters 7, 8, and 15).
2. **Part II, learning.** Sessions, memory, recall, skills, reflection, and curation. That work stays off the hot path.
3. **Part III, team patterns.** A pipeline, a supervisor, handoffs, fan-out, a critic, and composing those patterns. The default team is a supervisor that calls specialists as tools.
4. **Part IV, making it robust.** Context, actors with a bounded inbox, supervision and restart, and a server chapter that balances upstream MCP servers.
5. **Part V, running as a service.** Profiles, a chat gateway into the session inbox, the MCP gateway as a service, cron, and an editor protocol.
6. **Part VI, loosely coupled teams.** A blackboard, a kanban board, a message bus, shared skills, A2A for agents outside this process, tracing, a load and token capstone, and evals.

Rust shows up when the harness needs it: ownership when the conversation is a value, a trait when a tool is, a mutex when a human prompt must be held across `.await`. I'm not trying to port Hermes. It has years of integrations I don't need. I'm borrowing its ideas: stable prompts, summary-only results from subagents, restricted toolsets for children, and classifying an error before deciding how to recover. Then I'm seeing what changes when the language's guarantees do some of the work.

## How to read this book

If you're new to Rust like me, read the chapters in order and do the exercises. The compiler is strict, and early on that's frustrating. I'll be honest about the moments it beat me.

[personal note: first impressions of Rust after the setup chapter]

If you already know Rust and are here for the agent architecture, skim to Chapter 9 (the loop), Chapter 10 (the turn state machine), and Chapters 27 and 33–34 (the supervisor, actors, and supervision). Each has a "How Hermes does it" section with file paths, so you can read the Python next to the Rust.

Let's start with `cargo new`.
