# Table of contents

The chapter list for browsing on GitHub. Grouping, titles, and the *(server)* marks match [book/SUMMARY.md](book/SUMMARY.md). The longer plan is [book/book-outline.md](book/book-outline.md).

Written chapters link to `book/chapter-….md`. A chapter that has no page yet is marked (not yet written). Each line also links the step crates under `steps/` that exist for that chapter. Server-track chapters are marked *(server)*. The process crate is [`mcp-gateway/`](mcp-gateway/).

## Part I: One agent

- [Introduction: Why I'm learning Rust by building agents](book/chapter-00-introduction.md)
- [Chapter 1: First Contact](book/chapter-01-first-contact.md) — [`steps/ch01-1-messages`](steps/ch01-1-messages), [`steps/ch01-2-provider`](steps/ch01-2-provider), [`steps/ch01-3-cli`](steps/ch01-3-cli), [`steps/ch01-4-errors`](steps/ch01-4-errors)
- [Chapter 2: Conversations](book/chapter-02-conversations.md) — [`steps/ch02-conversations`](steps/ch02-conversations)
- [Chapter 3: Tools](book/chapter-03-tools.md) — [`steps/ch03-1-registry`](steps/ch03-1-registry), [`steps/ch03-2-toolsets`](steps/ch03-2-toolsets), [`steps/ch03-3-dispatch`](steps/ch03-3-dispatch)
- [Chapter 4: MCP Client](book/chapter-04-mcp-client.md) — [`steps/ch04-1-stdio`](steps/ch04-1-stdio), [`steps/ch04-2-http`](steps/ch04-2-http), [`steps/ch04-3-registry`](steps/ch04-3-registry), [`steps/ch04-4-cache`](steps/ch04-4-cache), [`steps/ch04-fixture`](steps/ch04-fixture)
- [Chapter 5: A Minimal MCP Server](book/chapter-05-minimal-mcp-server.md) *(server)* — [`steps/ch05-minimal-server`](steps/ch05-minimal-server)
- [Chapter 6: Thousands of Tools](book/chapter-06-thousands-of-tools.md) — [`steps/ch06-1-schema-cost`](steps/ch06-1-schema-cost), [`steps/ch06-2-tool-index`](steps/ch06-2-tool-index), [`steps/ch06-3-search-tool`](steps/ch06-3-search-tool), [`steps/ch06-4-routing`](steps/ch06-4-routing), [`steps/ch06-5-lazy-list`](steps/ch06-5-lazy-list)
- [Chapter 7: Dynamic Catalog](book/chapter-07-dynamic-catalog.md) *(server)* — [`steps/ch07-catalog`](steps/ch07-catalog)
- [Chapter 8: Session Toolsets](book/chapter-08-session-toolsets.md) *(server)* — [`steps/ch08-session-toolsets`](steps/ch08-session-toolsets)
- [Chapter 9: The Agent Loop](book/chapter-09-agent-loop.md) — [`steps/ch09-1-live`](steps/ch09-1-live), [`steps/ch09-2-cancel`](steps/ch09-2-cancel), [`steps/ch09-3-validate`](steps/ch09-3-validate), [`steps/ch09-4-stall`](steps/ch09-4-stall), [`steps/ch09-5-budget`](steps/ch09-5-budget), [`steps/ch09-6-truncate`](steps/ch09-6-truncate), [`steps/ch09-7-steer`](steps/ch09-7-steer), [`steps/ch09-8-canonical`](steps/ch09-8-canonical), [`steps/ch09-9-outcome`](steps/ch09-9-outcome), [`steps/ch09-10-evals`](steps/ch09-10-evals)
- [Chapter 10: The Turn State Machine](book/chapter-10-turn-state.md) — [`steps/ch10-turn-state`](steps/ch10-turn-state)
- [Chapter 11: Approvals and Guardrails](book/chapter-11-approvals.md) — [`steps/ch11-approvals`](steps/ch11-approvals)
- [Chapter 12: Hooks and Plugins](book/chapter-12-hooks.md) — [`steps/ch12-hooks`](steps/ch12-hooks)
- [Chapter 13: Sandboxed Execution](book/chapter-13-sandbox.md) — [`steps/ch13-sandbox`](steps/ch13-sandbox)
- [Chapter 14: When Things Fail](book/chapter-14-when-things-fail.md) — [`steps/ch14-when-things-fail`](steps/ch14-when-things-fail)
- [Chapter 15: Limits and Breakers](book/chapter-15-limits.md) *(server)* — [`steps/ch15-limits`](steps/ch15-limits)
- [Chapter 16: Built-in Tools I: Web Search and Extract](book/chapter-16-web-tools.md) — [`steps/ch16-web-tools`](steps/ch16-web-tools)
- [Chapter 17: Built-in Tools II: Files, Terminal, Todo, and Clarify](book/chapter-17-local-tools.md) — [`steps/ch17-local-tools`](steps/ch17-local-tools)
- [Chapter 18: Providers](book/chapter-18-providers.md) — [`steps/ch18-providers`](steps/ch18-providers)
- [Chapter 19: Streaming](book/chapter-19-streaming.md) — [`steps/ch19-streaming`](steps/ch19-streaming)

## Part II: Learning

- [Chapter 20: Sessions on Disk](book/chapter-20-sessions.md) — [`steps/ch20-sessions`](steps/ch20-sessions)
- [Chapter 21: Memory Files](book/chapter-21-memory.md) — [`steps/ch21-memory`](steps/ch21-memory)
- [Chapter 22: Recall](book/chapter-22-recall.md) — [`steps/ch22-recall`](steps/ch22-recall)
- [Chapter 23: Learned Tools](book/chapter-23-learned-tools.md) *(server)* — [`steps/ch23-learned-tools`](steps/ch23-learned-tools)
- [Chapter 24: Using Skills](book/chapter-24-using-skills.md) — [`steps/ch24-skills`](steps/ch24-skills)
- [Chapter 25: Writing Skills](book/chapter-25-writing-skills.md) — [`steps/ch25-skill-writing`](steps/ch25-skill-writing)
- [Chapter 26: The Reflection Loop](book/chapter-26-reflection.md) — [`steps/ch26-reflection`](steps/ch26-reflection)
- [Chapter 27: Curation](book/chapter-27-curation.md) — [`steps/ch27-curation`](steps/ch27-curation)

## Part III: The team patterns

- [Chapter 28: The Sequential Pipeline](book/chapter-28-pipeline.md) — [`steps/ch28-pipeline`](steps/ch28-pipeline)
- [Chapter 29: The Supervisor](book/chapter-29-supervisor.md) — [`steps/ch29-supervisor`](steps/ch29-supervisor)
- [Chapter 30: Handoffs](book/chapter-30-handoffs.md) — [`steps/ch30-handoffs`](steps/ch30-handoffs)
- [Chapter 31: Fan-out and Gather](book/chapter-31-fan-out.md) — [`steps/ch31-fan-out`](steps/ch31-fan-out)
- [Chapter 32: The Critic](book/chapter-32-critic.md) — [`steps/ch32-critic`](steps/ch32-critic)
- [Chapter 33: Composing Patterns](book/chapter-33-composition.md) — [`steps/ch33-composition`](steps/ch33-composition)

## Part IV: Making it robust

- [Chapter 34: Managing Context](book/chapter-34-context.md) — [`steps/ch34-context`](steps/ch34-context)
- [Chapter 35: Agents as Actors](book/chapter-35-actors.md) — [`steps/ch35-actors`](steps/ch35-actors)
- [Chapter 36: Supervision and Restart](book/chapter-36-supervision.md) — [`steps/ch36-supervision`](steps/ch36-supervision)
- [Chapter 37: Balancing and Federation](book/chapter-37-balancing.md) *(server)* — [`steps/ch37-balancing`](steps/ch37-balancing)

## Part V: Running as a service

- [Chapter 38: Profiles](book/chapter-38-profiles.md) — [`steps/ch38-profiles`](steps/ch38-profiles)
- [Chapter 39: The Gateway](book/chapter-39-gateway.md) — [`steps/ch39-gateway`](steps/ch39-gateway)
- [Chapter 40: MCP Gateway Service](book/chapter-40-service.md) *(server)* — [`steps/ch40-service`](steps/ch40-service)
- [Chapter 41: Cron](book/chapter-41-cron.md) — [`steps/ch41-cron`](steps/ch41-cron)
Planned, not yet written. No page and no step crate. Browser Tools, then Vision, will be inserted here in Part V, after the gateway and before Editor Integration. The spec is [docs/harness-architecture.md](docs/harness-architecture.md).

- [Chapter 42: Editor Integration](book/chapter-42-acp.md) — [`steps/ch42-acp`](steps/ch42-acp)

## Part VI: Loosely coupled teams

- [Chapter 43: The Blackboard](book/chapter-43-blackboard.md) — [`steps/ch43-blackboard`](steps/ch43-blackboard)
- [Chapter 44: The Kanban Board](book/chapter-44-kanban.md) — [`steps/ch44-kanban`](steps/ch44-kanban)
- [Chapter 45: The Message Bus](book/chapter-45-message-bus.md) — [`steps/ch45-message-bus`](steps/ch45-message-bus)
- [Chapter 46: Shared Team Skills](book/chapter-46-team-skills.md) — [`steps/ch46-team-skills`](steps/ch46-team-skills)
- [Chapter 47: A2A for External Agents](book/chapter-47-a2a.md) — [`steps/ch47-a2a`](steps/ch47-a2a)
- [Chapter 48: Seeing Inside](book/chapter-48-tracing.md) — [`steps/ch48-tracing`](steps/ch48-tracing)
- [Chapter 49: Load and Token Capstone](book/chapter-49-capstone.md) *(server)* — [`steps/ch49-capstone`](steps/ch49-capstone)
- [Chapter 50: Evals](book/chapter-50-evals.md) — [`steps/ch50-evals`](steps/ch50-evals)
