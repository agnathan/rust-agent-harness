# Chapter 0: Why I'm learning Rust by building a multi-agent harness

I'm learning Rust by building the thing I actually want: a continuously improving harness, one that can run a team of agents and can get better at a kind of task because it keeps what it learned. The loop is ten lines. You send the messages so far and the tools the model is allowed to see. If the model answers with text, the turn is over. If it asks for a tool, you run the tool, append the result, and ask again. The harness is most of the work. A tutorial that introduces `struct` and then stops would teach the syntax and leave the design untested, because a real harness has to keep a conversation, call a tool, refuse a bad call, remember a fact, and hand work to another agent. Each of those problems forces a language feature at the moment it is needed, which is the pace this book uses.

What "keeps what it learned" means here is the set of stores the later chapters actually build. Sessions land on disk, memory is a file the agent can read back, a skill is a procedure it can write and reuse, reflection looks at a finished turn, and curation decides what is worth keeping. That is the closed loop Hermes Agent already runs, and it is the loop this book rebuilds in Rust, one chapter at a time, with tests. Every chapter's demo runs offline. The exercises are in `my-harness/tests/`, and the finished reference for each step is in `steps/`.

By the last chapter you can point at four things. One agent that talks, calls tools, and stops. Guardrails around that agent, which are approvals, a sandbox, a budget, and a recovery path when a call fails. The learning loop just named, so a later run can start from a memory and a skill instead of a blank prompt. And a team, plus a separate server that compares three ways of offering a large tool catalog. The long inventory, turn table through eval suite, lives in the outline at [`book/book-outline.md`](book-outline.md) rather than in one sentence here.

The book grows two programs, and it keeps them apart on purpose. The harness decides what enters a prompt. The MCP gateway is a separate server the harness talks to as a client. The server holds a large catalog and, later, limits, a learned offer, pools, and a service front. Merging them would make every agent carry every tool. Chapters 5, 7, 8, 15, 23, 37, 42, and 51 are that server. They bind no socket. The other chapters copy the previous harness forward and add one module.

Rust shows up in sidebars when the design needs it: a struct for a message, an enum for an event, a trait for a tool, ownership for a conversation you don't want aliased by accident, `async` for overlapping reads, and `Arc` for a snapshot many tasks share. Nothing here assumes you've written Rust before. If you already have, the sidebars are short and the design is the point.

**In this chapter**

- What is an "agent harness"?
- The reference: Hermes Agent
- Why rebuild ideas from it in Rust?
- The plan
- How to read this book

## What is an "agent harness"?

An agent, stripped of the product language, is that short loop from the opening. The *harness* is everything around it that turns the loop from a demo into something you can leave running: talking to more than one model provider, defining and running tools, enforcing a budget so the agent can't loop forever, recovering from rate limits and network errors, compressing the conversation when it gets too long, saving the session, letting a human interrupt, and, once there is more than one agent, handing work across, running it in parallel, and cancelling it cleanly.

## The reference: Hermes Agent

The agent I'm learning from is **Hermes Agent** by Nous Research, an open-source (MIT) project at github.com/NousResearch/hermes-agent. Its README calls it a "self-improving AI agent": it keeps its own memory files, writes reusable "skills" from experience, refines them as it uses them, and can search its own past conversations. It's a large Python system, thousands of modules, and it's worth knowing which of its choices this book is actually borrowing.

The one that changed how I structure a prompt is the prefix rule. Hermes will not change the beginning of a conversation mid-session, because providers cache that prefix, and a changed prefix makes every later call slower and more expensive. Context compression is the single allowed exception. I treat that as a constraint on every later chapter that wants to add a fact: the fact has to land somewhere other than the front of the prompt.

Two other choices are why the harness stays smaller than the product around it. Many front ends, a terminal, a messaging gateway, a desktop app, protocol adapters, all call the same `AIAgent` core, so a new door doesn't become a new agent. Tools register themselves: each tool file adds a JSON schema, a handler, and an availability check to `tools/registry.py`, and a toolset decides which of those the model actually sees. Around that sit the pieces the later chapters borrow one at a time. The turn loop lives in `agent/conversation_loop.py`, with one module per phase under `agent/turn_*.py`. Each provider is a profile plugin. `delegate_task` spawns a child with a fresh context, a restricted toolset, and a shared budget, and only a summary comes back. Sessions sit in SQLite. There's a cron scheduler and a kanban board for several processes. The other rule I keep is a small core: new capabilities arrive as plugins and skills, rather than as more branches in the loop.

## Why rebuild ideas from it in Rust?

I started because I wanted many agents in one process, cooperating, and Hermes's own shape shows where a thread-per-agent runtime fights that. The agent loop is synchronous. The messaging gateway is asynchronous, but it hands each conversation turn to a worker thread, child agents run in thread pools, and parallel tool calls run in another pool. Python's global interpreter lock means only one of those threads runs Python at a time. Python also can't forcibly stop a thread, so cancelling a stuck subagent depends on flags, heartbeats, and timeouts. And because one process serves several user profiles across many threads, a lot of the rules exist to make sure the right settings and credentials are on the right thread.

Rust's async runtime, tokio, can run thousands of lightweight tasks on a handful of threads. Dropping a future cancels it. The compiler won't let two tasks mutate the same data without synchronisation. Those three properties line up with what a multi-agent harness needs, which is the reason to rebuild the ideas here instead of wrapping the Python.

## The plan

The book is six parts and chapters 0 through 52. The outline is [`book/book-outline.md`](book-outline.md). Where a chapter has a reader exercise, it is `my-harness/tests/chNN.rs` and it fails until that chapter's code is in place. Chapters 3 through 10 keep their tests in the step crates, and the finished reference for every chapter is under `steps/`.

1. **Part I, one agent.** Messages, a provider, tools, an MCP client, then the agent loop and the turn table that loop drives. Approvals, hooks, a sandbox, and failure handling sit on that same harness, and the built-in tools, the other providers, and streaming follow. A separate MCP gateway starts in chapter 5 and grows beside the harness (chapters 7, 8, and 15).
2. **Part II, learning.** Sessions, memory, recall, skills, reflection, and curation. That work stays off the hot path.
3. **Part III, team patterns.** A pipeline, a supervisor, handoffs, fan-out, a critic, and composing those patterns. The default team is a supervisor that calls specialists as tools.
4. **Part IV, making it robust.** Context, actors with a bounded inbox, supervision and restart, and a server chapter that balances upstream MCP servers.
5. **Part V, running as a service.** Profiles, a chat gateway into the session inbox, browser and vision tools, the MCP gateway as a service, cron, and an editor protocol.
6. **Part VI, loosely coupled teams.** A blackboard, a kanban board, a message bus, shared skills, A2A for agents outside this process, tracing, a load and token capstone, and evals.

Rust shows up when the harness needs it: ownership when the conversation is a value, a trait when a tool is, a mutex when a human prompt must be held across `.await`. I'm borrowing the ideas that carry the design, and leaving the years of integrations Hermes has grown around them: stable prompts, summary-only results from subagents, restricted toolsets for children, and classifying an error before deciding how to recover. Then I want to see what changes when the language's guarantees do some of that work.

## How to read this book

If you're new to Rust, read the chapters in order and do the exercises. The compiler is strict, and the early chapters are where that strictness shows up against a small program.

If you already know Rust and you're here for the agent architecture, skim to Chapter 9 (the loop), Chapter 10 (the turn state machine), and Chapters 29, 35, and 36 (the supervisor, actors, and supervision). Each has a "How Hermes does it" section with file paths, so you can read the Python next to the Rust.

The step crates, the milestone stubs, the sidebars, and the two boxes are in [How to read this book](preface-how-to-read-this-book.md). The measurements are in [How the numbers were measured](appendix-benchmark-method.md).

Let's start with `cargo new`.
