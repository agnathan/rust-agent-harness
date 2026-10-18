# Social posts: Chapter 0

## X thread
1/
I'm learning Rust by building something real: a high-performance harness for *teams* of AI agents.

I'm using Nous Research's open-source Hermes Agent as my reference architecture. 🧵

2/
An agent is just a loop: call the model → run the tools it asks for → append the results → repeat.

The "harness" is everything that makes that loop trustworthy: budgets, retries, streaming, parallel tools, cancellation, memory, persistence.

3/
Hermes Agent (github.com/NousResearch/hermes-agent) is a big, mature Python agent:
• one core behind a CLI, TUI, chat gateways and a desktop app
• self-registering tools
• subagents via delegate_task
• SQLite sessions, memory files, skills, cron

4/
Reading it with "how would I run many agents at once?" in mind, I noticed:
• each running agent occupies a thread
• Python can't forcibly stop a thread, so cancelling a stuck subagent is cooperative
• lots of care goes into keeping shared state on the right thread

5/
Rust + tokio flips those around: thousands of cheap tasks, cancellation by simply dropping a future, and a compiler that won't let two tasks race on the same data.

6/
So I built the course as six parts, chapters 0 through 48. Each chapter has failing tests I make pass, and the same harness grows from messages to a team.

7/
I'll post each chapter as I go, including the parts where the borrow checker wins.

Chapter 0 is up: [link]

#rustlang #AIagents

## LinkedIn
I'm starting a new learning project: learning Rust by building a multi-agent AI harness from scratch.

My reference is Hermes Agent, Nous Research's open-source agent. It runs one agent core behind a terminal UI, messaging platforms and a desktop app. It has a self-registering tool system, subagent delegation with shared budgets and depth limits, SQLite-backed sessions, and context compression designed so it never invalidates the model provider's prompt cache.

Studying its architecture, I kept asking: what would it take to run *many* agents at once, cooperating? That question leads straight to Rust's strengths: lightweight async tasks, cancellation that actually stops work, and compile-time guarantees against data races.

So I put together a test-driven path in six parts, chapters 0 through 48. It starts with Rust fundamentals expressed as agent types (messages, tools, errors), then builds a real model client, the agent loop, an explicit turn state machine, and keeps growing that same harness through streaming, actors, supervisors, and a message bus.

I'll share each chapter as I write it. Chapter 0 explains the why and gives a tour of Hermes's architecture: [link]

[personal note: one sentence on why this matters to you professionally]

#Rust #AIAgents #SoftwareArchitecture #LearningInPublic

## Blog teaser
**Learning Rust by building a multi-agent harness: Chapter 0**

An AI agent is a ten-line loop. Making it reliable (budgets, retries, streaming, parallel tools, cancellation, memory) is the real work, and running *teams* of agents makes all of it harder. In this first chapter I explain why I'm learning Rust by building exactly that, take a tour of Nous Research's open-source Hermes Agent as my reference architecture, and lay out the six-part plan, chapters 0 through 48. [Read Chapter 0 →]
