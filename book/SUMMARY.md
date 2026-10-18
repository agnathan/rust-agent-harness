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
- [Chapter 16: Providers](chapter-16-providers.md)
- [Chapter 17: Streaming](chapter-17-streaming.md)

<!-- Planned, not yet written: Built-in Tools I (Web Search and Extract) and Built-in Tools II (Files, Terminal, Todo, and Clarify), inserted after chapter 15. Browser Tools and Vision, inserted later in Part V. Spec: docs/harness-architecture.md. -->

# Part II: Learning

- [Chapter 18: Sessions on Disk](chapter-18-sessions.md)
- [Chapter 19: Memory Files](chapter-19-memory.md)
- [Chapter 20: Recall](chapter-20-recall.md)
- [Chapter 21: Learned Tools](chapter-21-learned-tools.md) *(server)*
- [Chapter 22: Using Skills](chapter-22-using-skills.md)
- [Chapter 23: Writing Skills](chapter-23-writing-skills.md)
- [Chapter 24: The Reflection Loop](chapter-24-reflection.md)
- [Chapter 25: Curation](chapter-25-curation.md)

# Part III: The team patterns

- [Chapter 26: The Sequential Pipeline](chapter-26-pipeline.md)
- [Chapter 27: The Supervisor](chapter-27-supervisor.md)
- [Chapter 28: Handoffs](chapter-28-handoffs.md)
- [Chapter 29: Fan-out and Gather](chapter-29-fan-out.md)
- [Chapter 30: The Critic](chapter-30-critic.md)
- [Chapter 31: Composing Patterns](chapter-31-composition.md)

# Part IV: Making it robust

- [Chapter 32: Managing Context](chapter-32-context.md)
- [Chapter 33: Agents as Actors](chapter-33-actors.md)
- [Chapter 34: Supervision and Restart](chapter-34-supervision.md)
- [Chapter 35: Balancing and Federation](chapter-35-balancing.md) *(server)*

# Part V: Running as a service

- [Chapter 36: Profiles](chapter-36-profiles.md)
- [Chapter 37: The Gateway](chapter-37-gateway.md)
- [Chapter 38: MCP Gateway Service](chapter-38-service.md) *(server)*
- [Chapter 39: Cron](chapter-39-cron.md)
- [Chapter 40: Editor Integration](chapter-40-acp.md)

# Part VI: Loosely coupled teams

- [Chapter 41: The Blackboard](chapter-41-blackboard.md)
- [Chapter 42: The Kanban Board](chapter-42-kanban.md)
- [Chapter 43: The Message Bus](chapter-43-message-bus.md)
- [Chapter 44: Shared Team Skills](chapter-44-team-skills.md)
- [Chapter 45: A2A for External Agents](chapter-45-a2a.md)
- [Chapter 46: Seeing Inside](chapter-46-tracing.md)
- [Chapter 47: Load and Token Capstone](chapter-47-capstone.md) *(server)*
- [Chapter 48: Evals](chapter-48-evals.md)
