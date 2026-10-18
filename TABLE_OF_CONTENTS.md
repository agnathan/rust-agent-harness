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
- [Chapter 16: Providers](book/chapter-16-providers.md) — [`steps/ch16-providers`](steps/ch16-providers)
- [Chapter 17: Streaming](book/chapter-17-streaming.md) — [`steps/ch17-streaming`](steps/ch17-streaming)

## Part II: Learning

- [Chapter 18: Sessions on Disk](book/chapter-18-sessions.md) — [`steps/ch18-sessions`](steps/ch18-sessions)
- [Chapter 19: Memory Files](book/chapter-19-memory.md) — [`steps/ch19-memory`](steps/ch19-memory)
- [Chapter 20: Recall](book/chapter-20-recall.md) — [`steps/ch20-recall`](steps/ch20-recall)
- [Chapter 21: Learned Tools](book/chapter-21-learned-tools.md) *(server)* — [`steps/ch21-learned-tools`](steps/ch21-learned-tools)
- [Chapter 22: Using Skills](book/chapter-22-using-skills.md) — [`steps/ch22-skills`](steps/ch22-skills)
- [Chapter 23: Writing Skills](book/chapter-23-writing-skills.md) — [`steps/ch23-skill-writing`](steps/ch23-skill-writing)
- [Chapter 24: The Reflection Loop](book/chapter-24-reflection.md) — [`steps/ch24-reflection`](steps/ch24-reflection)
- [Chapter 25: Curation](book/chapter-25-curation.md) — [`steps/ch25-curation`](steps/ch25-curation)

## Part III: The team patterns

- [Chapter 26: The Sequential Pipeline](book/chapter-26-pipeline.md) — [`steps/ch26-pipeline`](steps/ch26-pipeline)
- [Chapter 27: The Supervisor](book/chapter-27-supervisor.md) — [`steps/ch27-supervisor`](steps/ch27-supervisor)
- [Chapter 28: Handoffs](book/chapter-28-handoffs.md) — [`steps/ch28-handoffs`](steps/ch28-handoffs)
- [Chapter 29: Fan-out and Gather](book/chapter-29-fan-out.md) — [`steps/ch29-fan-out`](steps/ch29-fan-out)
- [Chapter 30: The Critic](book/chapter-30-critic.md) — [`steps/ch30-critic`](steps/ch30-critic)
- [Chapter 31: Composing Patterns](book/chapter-31-composition.md) — [`steps/ch31-composition`](steps/ch31-composition)

## Part IV: Making it robust

- [Chapter 32: Managing Context](book/chapter-32-context.md) — [`steps/ch32-context`](steps/ch32-context)
- [Chapter 33: Agents as Actors](book/chapter-33-actors.md) — [`steps/ch33-actors`](steps/ch33-actors)
- [Chapter 34: Supervision and Restart](book/chapter-34-supervision.md) — [`steps/ch34-supervision`](steps/ch34-supervision)
- [Chapter 35: Balancing and Federation](book/chapter-35-balancing.md) *(server)* — [`steps/ch35-balancing`](steps/ch35-balancing)

## Part V: Running as a service

- [Chapter 36: Profiles](book/chapter-36-profiles.md) — [`steps/ch36-profiles`](steps/ch36-profiles)
- [Chapter 37: The Gateway](book/chapter-37-gateway.md) — [`steps/ch37-gateway`](steps/ch37-gateway)
- [Chapter 38: MCP Gateway Service](book/chapter-38-service.md) *(server)* — [`steps/ch38-service`](steps/ch38-service)
- [Chapter 39: Cron](book/chapter-39-cron.md) — [`steps/ch39-cron`](steps/ch39-cron)
- [Chapter 40: Editor Integration](book/chapter-40-acp.md) — [`steps/ch40-acp`](steps/ch40-acp)

## Part VI: Loosely coupled teams

- [Chapter 41: The Blackboard](book/chapter-41-blackboard.md) — [`steps/ch41-blackboard`](steps/ch41-blackboard)
- [Chapter 42: The Kanban Board](book/chapter-42-kanban.md) — [`steps/ch42-kanban`](steps/ch42-kanban)
- [Chapter 43: The Message Bus](book/chapter-43-message-bus.md) — [`steps/ch43-message-bus`](steps/ch43-message-bus)
- [Chapter 44: Shared Team Skills](book/chapter-44-team-skills.md) — [`steps/ch44-team-skills`](steps/ch44-team-skills)
- [Chapter 45: A2A for External Agents](book/chapter-45-a2a.md) — [`steps/ch45-a2a`](steps/ch45-a2a)
- [Chapter 46: Seeing Inside](book/chapter-46-tracing.md) — [`steps/ch46-tracing`](steps/ch46-tracing)
- [Chapter 47: Load and Token Capstone](book/chapter-47-capstone.md) *(server)* — [`steps/ch47-capstone`](steps/ch47-capstone)
- [Chapter 48: Evals](book/chapter-48-evals.md) — [`steps/ch48-evals`](steps/ch48-evals)
