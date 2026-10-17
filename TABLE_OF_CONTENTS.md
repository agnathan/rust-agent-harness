# Table of contents

The chapter list for browsing on GitHub. Grouping, titles, and the *(server)* marks match [book/SUMMARY.md](book/SUMMARY.md). The longer plan is [book/book-outline.md](book/book-outline.md).

Written chapters link to `book/chapter-….md`. A chapter that has no page yet is marked (not yet written). Each line also links the step crates under `steps/` that exist for that chapter. Server-track chapters are marked *(server)*. The process crate is [`mcp-gateway/`](mcp-gateway/).

## Part I: One agent

- [Introduction: Why I'm learning Rust by building agents](book/chapter-00-introduction.md)
- [Chapter 1: First Contact](book/chapter-01-first-contact.md) — [`steps/ch01-1-messages`](steps/ch01-1-messages), [`steps/ch01-2-provider`](steps/ch01-2-provider), [`steps/ch01-3-cli`](steps/ch01-3-cli), [`steps/ch01-4-errors`](steps/ch01-4-errors)
- [Chapter 2: Conversations](book/chapter-02-conversations.md) — [`steps/ch02-conversations`](steps/ch02-conversations)
- [Chapter 3: Tools](book/chapter-03-tools.md) — [`steps/ch03-1-registry`](steps/ch03-1-registry), [`steps/ch03-2-toolsets`](steps/ch03-2-toolsets), [`steps/ch03-3-dispatch`](steps/ch03-3-dispatch)
- [Chapter 4: MCP Client](book/chapter-04-mcp-client.md) — [`steps/ch04-1-stdio`](steps/ch04-1-stdio), [`steps/ch04-2-http`](steps/ch04-2-http), [`steps/ch04-3-registry`](steps/ch04-3-registry), [`steps/ch04-4-cache`](steps/ch04-4-cache), [`steps/ch04-fixture`](steps/ch04-fixture)
- [Chapter 5: A Minimal MCP Server](book/chapter-05-minimal-mcp-server.md) *(server)* — [`steps/gw-minimal-server`](steps/gw-minimal-server)
- [Chapter 6: Thousands of Tools](book/chapter-05-thousands-of-tools.md) — [`steps/ch05-1-schema-cost`](steps/ch05-1-schema-cost), [`steps/ch05-2-tool-index`](steps/ch05-2-tool-index), [`steps/ch05-3-search-tool`](steps/ch05-3-search-tool), [`steps/ch05-4-routing`](steps/ch05-4-routing), [`steps/ch05-5-lazy-list`](steps/ch05-5-lazy-list)
- [Chapter 7: Dynamic Catalog](book/chapter-07-dynamic-catalog.md) *(server)* — [`steps/gw-catalog`](steps/gw-catalog)
- [Chapter 8: Session Toolsets](book/chapter-08-session-toolsets.md) *(server)* — [`steps/gw-session-toolsets`](steps/gw-session-toolsets)
- [Chapter 9: The Agent Loop](book/chapter-06-agent-loop.md) — [`steps/ch06-1-live`](steps/ch06-1-live), [`steps/ch06-2-cancel`](steps/ch06-2-cancel), [`steps/ch06-3-validate`](steps/ch06-3-validate), [`steps/ch06-4-stall`](steps/ch06-4-stall), [`steps/ch06-5-budget`](steps/ch06-5-budget), [`steps/ch06-6-truncate`](steps/ch06-6-truncate), [`steps/ch06-7-steer`](steps/ch06-7-steer), [`steps/ch06-8-canonical`](steps/ch06-8-canonical), [`steps/ch06-9-outcome`](steps/ch06-9-outcome), [`steps/ch06-10-evals`](steps/ch06-10-evals)
- [Chapter 10: The Turn State Machine](book/chapter-10-turn-state.md) — [`steps/ch07-turn-state`](steps/ch07-turn-state)
- [Chapter 11: Approvals and Guardrails](book/chapter-11-approvals.md) — [`steps/ch09-approvals`](steps/ch09-approvals)
- [Chapter 12: Hooks and Plugins](book/chapter-12-hooks.md) — [`steps/ch10-hooks`](steps/ch10-hooks)
- [Chapter 13: Sandboxed Execution](book/chapter-13-sandbox.md) — [`steps/ch11-sandbox`](steps/ch11-sandbox)
- [Chapter 14: When Things Fail](book/chapter-14-when-things-fail.md) — [`steps/ch12-when-things-fail`](steps/ch12-when-things-fail)
- [Chapter 15: Limits and Breakers](book/chapter-15-limits.md) *(server)* — [`steps/gw-limits`](steps/gw-limits)
- [Chapter 16: Providers](book/chapter-16-providers.md) — [`steps/ch13-providers`](steps/ch13-providers)
- [Chapter 17: Streaming](book/chapter-17-streaming.md) — [`steps/ch14-streaming`](steps/ch14-streaming)

## Part II: Learning

- [Chapter 18: Sessions on Disk](book/chapter-18-sessions.md) — [`steps/ch15-sessions`](steps/ch15-sessions)
- [Chapter 19: Memory Files](book/chapter-19-memory.md) — [`steps/ch16-memory`](steps/ch16-memory)
- [Chapter 20: Recall](book/chapter-20-recall.md) — [`steps/ch17-recall`](steps/ch17-recall)
- [Chapter 21: Learned Tools](book/chapter-21-learned-tools.md) *(server)* — [`steps/gw-learned-tools`](steps/gw-learned-tools)
- [Chapter 22: Using Skills](book/chapter-22-using-skills.md) — [`steps/ch18-skills`](steps/ch18-skills)
- [Chapter 23: Writing Skills](book/chapter-23-writing-skills.md) — [`steps/ch19-skill-writing`](steps/ch19-skill-writing)
- [Chapter 24: The Reflection Loop](book/chapter-24-reflection.md) — [`steps/ch20-reflection`](steps/ch20-reflection)
- [Chapter 25: Curation](book/chapter-25-curation.md) — [`steps/ch21-curation`](steps/ch21-curation)

## Part III: The team patterns

- [Chapter 26: The Sequential Pipeline](book/chapter-26-pipeline.md) — [`steps/ch22-pipeline`](steps/ch22-pipeline)
- [Chapter 27: The Supervisor](book/chapter-27-supervisor.md) — [`steps/ch23-supervisor`](steps/ch23-supervisor)
- [Chapter 28: Handoffs](book/chapter-28-handoffs.md) — [`steps/ch24-handoffs`](steps/ch24-handoffs)
- [Chapter 29: Fan-out and Gather](book/chapter-29-fan-out.md) — [`steps/ch25-fan-out`](steps/ch25-fan-out)
- [Chapter 30: The Critic](book/chapter-30-critic.md) — [`steps/ch26-critic`](steps/ch26-critic)
- [Chapter 31: Composing Patterns](book/chapter-31-composition.md) — [`steps/ch27-composition`](steps/ch27-composition)

## Part IV: Making it robust

- [Chapter 32: Managing Context](book/chapter-32-context.md) — [`steps/ch28-context`](steps/ch28-context)
- [Chapter 33: Agents as Actors](book/chapter-33-actors.md) — [`steps/ch29-actors`](steps/ch29-actors)
- [Chapter 34: Supervision and Restart](book/chapter-34-supervision.md) — [`steps/ch30-supervision`](steps/ch30-supervision)
- [Chapter 35: Balancing and Federation](book/chapter-35-balancing.md) *(server)* — [`steps/gw-balancing`](steps/gw-balancing)

## Part V: Running as a service

- [Chapter 36: Profiles](book/chapter-36-profiles.md) — [`steps/ch31-profiles`](steps/ch31-profiles)
- [Chapter 37: The Gateway](book/chapter-37-gateway.md) — [`steps/ch32-gateway`](steps/ch32-gateway)
- [Chapter 38: MCP Gateway Service](book/chapter-38-service.md) *(server)* — [`steps/gw-service`](steps/gw-service)
- [Chapter 39: Cron](book/chapter-39-cron.md) — [`steps/ch33-cron`](steps/ch33-cron)
- [Chapter 40: Editor Integration](book/chapter-40-acp.md) — [`steps/ch34-acp`](steps/ch34-acp)

## Part VI: Loosely coupled teams

- [Chapter 41: The Blackboard](book/chapter-41-blackboard.md) — [`steps/ch35-blackboard`](steps/ch35-blackboard)
- [Chapter 42: The Kanban Board](book/chapter-42-kanban.md) — [`steps/ch36-kanban`](steps/ch36-kanban)
- [Chapter 43: The Message Bus](book/chapter-43-message-bus.md) — [`steps/ch37-message-bus`](steps/ch37-message-bus)
- [Chapter 44: Shared Team Skills](book/chapter-44-team-skills.md) — [`steps/ch38-team-skills`](steps/ch38-team-skills)
- [Chapter 45: A2A for External Agents](book/chapter-45-a2a.md) — [`steps/ch39-a2a`](steps/ch39-a2a)
- [Chapter 46: Seeing Inside](book/chapter-46-tracing.md) — [`steps/ch40-tracing`](steps/ch40-tracing)
- [Chapter 47: Load and Token Capstone](book/chapter-47-capstone.md) *(server)* — [`steps/gw-capstone`](steps/gw-capstone)
- Chapter 48: Evals (not yet written)
