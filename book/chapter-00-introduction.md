# Chapter 0: Why I'm learning Rust by building a multi-agent harness

I'm learning Rust by building the thing I actually want: a continuously improving harness, one that can run a team of agents and can get better at a kind of task because it keeps what it learned. The loop is ten lines. You send the messages so far and the tools the model is allowed to see. If the model answers with text, the turn is over. If it asks for a tool, you run the tool, append the result, and ask again. The harness is most of the work. A tutorial that introduces `struct` and then stops would teach the syntax and leave the design untested. A real harness has to keep a conversation, call a tool, refuse a bad call, remember a fact, and hand work to another agent. Each of those problems forces a language feature at the moment it is needed, which is the pace this book uses.

What "keeps what it learned" means here is the set of stores the later chapters actually build. Sessions land on disk, memory is a file the agent can read back, a skill is a procedure it can write and reuse, reflection looks at a finished turn. Curation decides what is worth keeping. That is the closed loop Hermes Agent already runs. It is the loop this book rebuilds in Rust, one chapter at a time, with tests. Every chapter's demo runs offline. The exercises are in `my-harness/tests/`, and the finished reference for each step is in `steps/`.

By the last chapter you can point at four things, and then at one run that uses them together. One agent that talks, calls tools, and stops. Guardrails around that agent, which are approvals, a sandbox, a budget, and a recovery path when a call fails. The learning loop just named, so a later run can start from a memory and a skill instead of a blank prompt. And a team, plus a separate server that compares three ways of offering a large tool catalog. Chapter 53 is where that learning loop is actually run: the same task fails, the harness keeps a skill, and the second attempt passes. The model in that run is the fake one, and the chapter says what that does and doesn't prove. The long inventory, turn table through the closing loop, lives in the outline at [`book/book-outline.md`](book-outline.md) rather than in one sentence here.

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

The one that changed how I structure a prompt is the prefix rule. Hermes will not change the beginning of a conversation mid-session, because providers cache that prefix. A changed prefix makes every later call slower and more expensive. Context compression is the single allowed exception. I treat that as a constraint on every later chapter that wants to add a fact: the fact has to land somewhere other than the front of the prompt.

Two other choices are why the harness stays smaller than the product around it. Many front ends, a terminal, a messaging gateway, a desktop app, protocol adapters, all call the same `AIAgent` core. So a new door doesn't become a new agent. Tools register themselves: each tool file adds a JSON schema, a handler, and an availability check to `tools/registry.py`. A toolset decides which of those the model actually sees. Around that sit the pieces the later chapters borrow one at a time.

The turn loop lives in `agent/conversation_loop.py`, with one module per phase under `agent/turn_*.py`. Each provider is a profile plugin. `delegate_task` spawns a child with a fresh context, a restricted toolset, and a shared budget, and only a summary comes back. Sessions sit in SQLite. There's a cron scheduler and a kanban board for several processes. The other rule I keep is a small core: new capabilities arrive as plugins and skills, rather than as more branches in the loop.

## Why rebuild ideas from it in Rust?

I started because I wanted many agents in one process, cooperating. Hermes's own shape shows where a thread-per-agent runtime fights that. The agent loop is synchronous. The messaging gateway is asynchronous, but it hands each conversation turn to a worker thread, child agents run in thread pools. Parallel tool calls run in another pool. Python's global interpreter lock means only one of those threads runs Python at a time. Python also can't forcibly stop a thread, so cancelling a stuck subagent depends on flags, heartbeats, and timeouts. And because one process serves several user profiles across many threads, a lot of the rules exist to make sure the right settings and credentials are on the right thread.

Rust's async runtime, tokio, can run thousands of lightweight tasks on a handful of threads. Dropping a future cancels it. The compiler won't let two tasks mutate the same data without synchronisation. Those three properties line up with what a multi-agent harness needs, which is the reason to rebuild the ideas here instead of wrapping the Python.

## The plan

The book is six parts and chapters 0 through 53. The outline is [`book/book-outline.md`](book-outline.md). Where a chapter has a reader exercise, it is `my-harness/tests/chNN.rs` and it fails until that chapter's code is in place. Chapters 3 through 10 keep their tests in the step crates, and the finished reference for every chapter is under `steps/`.

1. **Part I, one agent.** Messages, a provider, tools, an MCP client, then the agent loop and the turn table that loop drives. Approvals, hooks, a sandbox, and failure handling sit on that same harness, and the built-in tools, the other providers, and streaming follow. A separate MCP gateway starts in chapter 5 and grows beside the harness (chapters 7, 8, and 15).
2. **Part II, learning.** Sessions, memory, recall, skills, reflection, and curation. That work stays off the hot path.
3. **Part III, team patterns.** A pipeline, a supervisor, handoffs, fan-out, a critic, and composing those patterns. The default team is a supervisor that calls specialists as tools.
4. **Part IV, making it robust.** Context, actors with a bounded inbox, supervision and restart, and a server chapter that balances upstream MCP servers.
5. **Part V, running as a service.** Profiles, a chat gateway into the session inbox, browser and vision tools, the MCP gateway as a service, cron, and an editor protocol.
6. **Part VI, loosely coupled teams.** A blackboard, a kanban board, a message bus, shared skills, A2A for agents outside this process, tracing, a load and token capstone, and evals. The last chapter runs one task twice so the learning loop has to change the score.

Rust shows up when the harness needs it: ownership when the conversation is a value, a trait when a tool is, a mutex when a human prompt must be held across `.await`. I'm borrowing the ideas that carry the design, and leaving the years of integrations Hermes has grown around them: stable prompts, summary-only results from subagents, restricted toolsets for children, and classifying an error before deciding how to recover. Then I want to see what changes when the language's guarantees do some of that work.

## Where the loop actually closes

The opening named four things and then one run. It is worth saying, before the parts, what you will be able to point at when each of those arrives, because a promise that only shows up in the last chapter is easy to lose in the middle.

Sessions are chapter 20. One turn appends to a file. The secret scrub happens on the way in, so a key in the transcript stays out of the lesson. Memory is chapter 21. `add` changes `MEMORY.md` or `USER.md`, and the system prompt keeps the `Arc` that `open` already handed out. I expected a write to show up in the prompt on the same turn. The file moves. The prefix stays, and that is the prefix rule from Hermes applied to a fact you just stored.

A skill is chapters 24 and 25. The index is rendered once into an `Arc`, and the full text loads when the model asks. A write goes through a scan, and through an approval hold when the gate is on. Reflection is chapter 26. The user turn leaves it alone. A counter hits ten, `finalize` returns a reviewer fork, and the person has already moved on. Curation is chapter 27. A pass can archive a skill, and it writes a snapshot of the tree first, so another copy survives.

The team starts at chapter 28. Four stages share one `Arc`. A supervisor, a handoff, a fan-out, and a critic follow, and the critic is someone other than the agent that did the work. Chapter 48 is the team library: workers propose, and one librarian applies. Evals are chapter 52. The saved tasks from chapter 9 run again on that later tree, and a score is a name and a bool. Chapter 53 is the run that uses the stores together. The same task fails, a skill is kept, and the second attempt passes. The model in that run is the fake one. The chapter says what the fake model proves and what it leaves open. If you only remember one sentence from this introduction, make it that one: the harness gets better at a kind of task because it keeps what it learned, and the last chapter is where a score moves for that reason.

## The two programs, at the seams

You will feel the book change temperature when the server chapters arrive, and again when the harness resumes. That is one book, not two drafts. The server chapters are 5, 7, 8, 15, 23, 37, 42, and 51. They are shorter at the start, because the first server is a list and a gate, and they stay a separate program the whole way. They bind no socket. The harness chapters copy the previous harness forward and add one module.

The seams are where a reader can lose the thread, so here is the thread. After chapter 8's session view, chapter 9 is the agent loop on the harness, and it searches with the client from chapter 6. That loop leaves the gateway's revealed set on the other program. After chapter 15's limits, chapter 16 is web search on a copy of chapter 14, and the server returns at chapter 23. After chapter 37's pool, chapter 38 is profiles on the harness, and the server returns at chapter 42. After chapter 42's service, chapter 43 is cron on the harness. After chapter 51's three offer shapes, chapter 52 is evals on the harness, and chapter 53 leaves the server directory where chapter 51 put it.

When a chapter says the gateway stays the other program, it is pointing at this split. Learning, the memory file, the skill, the reflection fork, the curator, the team, lives in the harness. The catalog, the offer, the load, lives in the server. A continuously improving harness still needs both. A procedure that calls a tool the catalog will not offer has nowhere to land, and carrying every schema would bury the process that is trying to remember.

## What "I expected" means

The early chapters talk in the first person, because that is the honest position of someone building the thing in front of them. Later chapters keep that voice, more quietly. When I write that I expected something, I mean a reading the file then contradicted. The expectation is the wrong guess. The code is the correction. There is no mood in that sentence, and no invented weekend.

A few of those corrections are worth knowing in advance, so the later sentence does not feel like a new author. I expected `describe` on a session to refuse a name that session had not revealed. It writes the card, or the word `unknown`, and the refusal waits for `call`. I expected an empty reply to be just another stop. Chapter 14 gives it a name, and the first empty changes the retry budget. I expected a memory write to change the prompt. Chapter 21 changes the file and leaves the snapshot. I expected a pipeline's critique to read the draft. Chapter 28 preloads the replies, and the notes still share one `Arc`. I expected the actor demo to fill the inbox to 64. It steps after each message, and the cap is the unit test. I expected cron to have a clock. Chapter 43 has a claim, a scan, and an in-memory store a test can crash. I expected the reviewer in the last chapter to be the code that writes the skill file. `skill_manage` returns a string. `Writer::create` writes the file.

None of those are surprises I am saving for a twist. They are the places the design is more specific than the slogan. The slogan is the continuously improving harness. The specifics are which function runs, which snapshot stays put, and which score is allowed to move.

## How to read this book

If you're new to Rust, read the chapters in order and do the exercises. The compiler is strict, and the early chapters are where that strictness shows up against a small program.

If you already know Rust and you're here for the agent architecture, skim to Chapter 9 (the loop), Chapter 10 (the turn state machine), and Chapters 29, 35, and 36 (the supervisor, actors, and supervision). Each has a "How Hermes does it" section with file paths, so you can read the Python next to the Rust.

The step crates, the milestone stubs, the sidebars, and the two boxes are in [How to read this book](preface-how-to-read-this-book.md). The measurements are in [How the numbers were measured](appendix-benchmark-method.md).

The exercises live in `my-harness/tests/chNN.rs`. The file compiles, then panics on `todo!` until the function you write makes the assertion pass. The finished version of that function is in `steps/`, and the preface tells you not to paste it over. Chapters 3 through 10 keep their tests inside the step crates. Later chapters add the milestone file. If a chapter has no exercise, it still has the step crate, the demo command, and the tests nextest runs. The point of the exercise is the one behavior the chapter exists to pin down, not a second copy of the whole module.

Read the opening of a chapter before the code. The first paragraphs are the payoff: a number, a refusal, or a score. The middle is the walk through the file. The end says what the next chapter will ask of this one. When the next chapter is on the other program, the handoff says so. When it returns to the harness, it says that too. If you skip those last lines, the server chapters feel like a different book. They are the catalog this harness calls.

One more note on length. The early server chapters are the list, the reload, and the session view. They are smaller than the loop chapter that follows them, and the later harness chapters are longer because the tree they copy is longer. The voice is the same. A short chapter is still a person reading a file and saying what the file did.

If you already know Rust, the sidebars will feel slow and the design is the point. Chapter 1 is still worth the hour. It is where `Option`, `Result`, and `Arc` get used for a real reply and a real cost, and later chapters point back there instead of teaching them again. If you are new to Rust, the compiler will be the strict reviewer the early chapters keep promising. Read the error, then the sidebar next to the code that provoked it. The feature shows up because the harness needed it, which is a better reason than a tour of the language in alphabetical order.

The reference project is large, and this book is a selection. Hermes has years of integrations this text leaves alone: messaging platforms, a desktop app, a full cron expression, a SQLite session store. What it borrows is named in each chapter's Hermes box, against a commit, with the files that were read. Adopted, adapted, and deferred are the three labels, and they stay in that shape so you can see the choice. Guardrails boxes name catalog ids. They are the catalog, not a second implementation. When a chapter says a guard is absent, it says so, and it names the chapter that has it or it admits that no chapter owns it. That honesty is part of the voice. A continuously improving harness that hides a missing check inside a cheerful sentence is the thing the book is trying not to become.

The last chapter's close is smaller than this opening, on purpose. By then you will have seen the stores, the team, and the other program. The score moves once. The limits of that movement are stated in the same section, including the fake model and the skill body that was a constant. If the opening feels like a promise, the close is the size of the promise kept. Read both, and notice that the close refuses to replay the opening scene just to feel finished.

Bring a question to the first chapter. You will send one prompt, read one reply, and keep a cost. The types around that reply are the part that has to be right, which is the sentence chapter 1 opens on. The HTTP call is real. It is also the smaller problem. From there the book adds a conversation, a tool, a client, and then the other program. The loop comes after both exist. The score comes last.

The measurements stay in the chapters that produced them. A mean in a table is `mean.point_estimate` from a named run, or from the machine the appendix describes. This introduction has no new number. If a later sentence and a table disagree, the table and the run id are the ones to trust, and the sentence should be fixed. That rule is how the book stays one voice instead of a pile of drafts that each remembered a different benchmark.

You can stop at the end of a part and still have a program that runs. Part I is one agent that can talk, call a tool, and stop. Part II is the same agent with a session, a memory file, and a skill. Part III is a team. The later parts make that team harder to break and willing to talk to something outside the process. Chapter 53 adds no new part. It is the run that asks the earlier parts to share one task. If a middle chapter feels like it forgot the slogan, check the handoff. The slogan is waiting in the stores, and the last chapter is where a score is allowed to notice them.

Let's start with `cargo new`.
