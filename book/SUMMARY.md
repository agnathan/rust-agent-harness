# Summary

[Book outline](book-outline.md)

Server-track chapters are marked *(server)*. They are interleaved with the harness. Chapters 5, 7, 8, 15, 21, 35, 38, and 47 are written. The process crate is `mcp-gateway/`. The harness from chapter 10 on is one growing crate; the plan is [`docs/harness-architecture.md`](../docs/harness-architecture.md).

# Part I: One agent

- [Introduction: Why I'm learning Rust by building agents](chapter-00-introduction.md)
- [Chapter 1: First Contact](chapter-01-first-contact.md)
- [Chapter 2: Conversations](chapter-02-conversations.md)
- [Chapter 3: Tools](chapter-03-tools.md)
- [Chapter 4: MCP Client](chapter-04-mcp-client.md)
- [Chapter 5: A Minimal MCP Server](chapter-05-minimal-mcp-server.md) *(server)*
- [Chapter 6: Thousands of Tools](chapter-06-thousands-of-tools.md)
- [Chapter 7: Dynamic Catalog](chapter-07-dynamic-catalog.md) *(server)*
- [Chapter 8: Session Toolsets](chapter-08-session-toolsets.md) *(server)*
- [Chapter 9: The Agent Loop](chapter-09-agent-loop.md)
- [Chapter 10: The Turn State Machine](chapter-10-turn-state.md)
- [Chapter 11: Approvals and Guardrails](chapter-11-approvals.md)
- [Chapter 12: Hooks and Plugins](chapter-12-hooks.md)
- [Chapter 13: Sandboxed Execution](chapter-13-sandbox.md)
- [Chapter 14: When Things Fail](chapter-14-when-things-fail.md)
- [Chapter 15: Limits and Breakers](chapter-15-limits.md) *(server)*
- [Chapter 16: Built-in Tools I: Web Search and Extract](chapter-16-web-tools.md)
- [Chapter 17: Built-in Tools II: Files, Terminal, Todo, and Clarify](chapter-17-local-tools.md)
- [Chapter 18: Providers](chapter-18-providers.md)
- [Chapter 19: Streaming](chapter-19-streaming.md)

<!-- Planned, not yet written, in Part V between the gateway and Editor Integration: Browser Tools and Vision. Spec: docs/harness-architecture.md. -->

# Part II: Learning

- [Chapter 20: Sessions on Disk](chapter-20-sessions.md)
- [Chapter 21: Memory Files](chapter-21-memory.md)
- [Chapter 22: Recall](chapter-22-recall.md)
- [Chapter 23: Learned Tools](chapter-23-learned-tools.md) *(server)*
- [Chapter 24: Using Skills](chapter-24-using-skills.md)
- [Chapter 25: Writing Skills](chapter-25-writing-skills.md)
- [Chapter 26: The Reflection Loop](chapter-26-reflection.md)
- [Chapter 27: Curation](chapter-27-curation.md)

# Part III: The team patterns

- [Chapter 28: The Sequential Pipeline](chapter-28-pipeline.md)
- [Chapter 29: The Supervisor](chapter-29-supervisor.md)
- [Chapter 30: Handoffs](chapter-30-handoffs.md)
- [Chapter 31: Fan-out and Gather](chapter-31-fan-out.md)
- [Chapter 32: The Critic](chapter-32-critic.md)
- [Chapter 33: Composing Patterns](chapter-33-composition.md)

# Part IV: Making it robust

- [Chapter 34: Managing Context](chapter-34-context.md)
- [Chapter 35: Agents as Actors](chapter-35-actors.md)
- [Chapter 36: Supervision and Restart](chapter-36-supervision.md)
- [Chapter 37: Balancing and Federation](chapter-37-balancing.md) *(server)*

# Part V: Running as a service

- [Chapter 38: Profiles](chapter-38-profiles.md)
- [Chapter 39: The Gateway](chapter-39-gateway.md)
- [Chapter 40: MCP Gateway Service](chapter-40-service.md) *(server)*
- [Chapter 41: Cron](chapter-41-cron.md)
- [Chapter 42: Editor Integration](chapter-42-acp.md)

# Part VI: Loosely coupled teams

- [Chapter 43: The Blackboard](chapter-43-blackboard.md)
- [Chapter 44: The Kanban Board](chapter-44-kanban.md)
- [Chapter 45: The Message Bus](chapter-45-message-bus.md)
- [Chapter 46: Shared Team Skills](chapter-46-team-skills.md)
- [Chapter 47: A2A for External Agents](chapter-47-a2a.md)
- [Chapter 48: Seeing Inside](chapter-48-tracing.md)
- [Chapter 49: Load and Token Capstone](chapter-49-capstone.md) *(server)*
- [Chapter 50: Evals](chapter-50-evals.md)
