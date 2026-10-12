# Team patterns

How this harness puts more than one agent on a job. The book teaches each pattern in Part II. This page is the rule for which one to reach for.

The default team is a **supervisor** that calls **agents as tools** and shares one **context object**. Other patterns are for a specific shape of work. They compose. They do not replace the default.

Inside one process, agents talk through these patterns. **A2A is only for an agent that lives somewhere else** (another service, another organization). MCP is the tool boundary, and it is first-class from Part I: our tools are MCP-shaped, we call MCP servers, and we expose our own tools and agents as an MCP server. Two agents in this process still collaborate through the patterns below. MCP does not replace the supervisor.

## Default: supervisor and agents-as-tools

A supervisor is an agent with a job: decide what to do next, call a specialist, read what came back, and either call another specialist or answer.

Each specialist is a tool. The supervisor's tool list is the team. Calling the tool runs that agent and returns its result. The supervisor writes the final answer. Specialists do not talk to each other, and they do not see the supervisor's whole transcript.

This is Chapter 10. The specialists it calls may be native tools or tools from an MCP server. An agent used as a tool can later be published as an MCP tool so a client outside this process can call that same specialist. Use the supervisor unless a later pattern fits better.

## Shared context object

The team shares one explicit object: the task, the constraints, and the facts collected so far. It is not the message list.

The supervisor owns it. A specialist receives the slice it needs, and its reply is written back onto the object by the supervisor. That is the same idea as context filtering on a handoff, applied to every tool call.

Do not smuggle the parent conversation into a specialist to "give it more context." If a fact matters, put it on the object.

## Sequential pipeline

Some work is a fixed order: outline, then draft, then critique, then revise. Each stage is an agent value (instructions, tools, and the context it is allowed to see) run once, and its output is the next stage's input.

No one in the pipeline chooses the next stage. The sequence is the program. This is Chapter 9. Use it when the steps are known before the run starts.

## Handoffs

A handoff transfers ownership. The current agent is a pointer. A handoff is a tool that moves that pointer to another agent and continues the turn there. The caller does not come back to synthesize.

Use a handoff to route ("this is a billing question") or to hand a job off for good. Bound it with a max-handoff limit so agents cannot pass the same task around forever. Filter the context before the next agent sees it: the new agent gets the shared object and the messages it needs, not the previous agent's private scratchpad.

Handoffs are Chapter 11. They are not the default team. A supervisor that must combine two specialists' answers should call them as tools, not hand off. Those tools can be MCP servers the same way a native tool is.

## Fan-out and gather

When several specialists can work at the same time, the supervisor (or a small aggregator) starts them together, waits, and folds the results. tokio runs the tasks. A worker limit caps how many run at once.

Each worker gets its own slice of the shared context. The aggregator is the only one that sees every reply. This is Chapter 12. Fan-out is a way the supervisor works, not a second kind of team.

## Critic

A critic is an evaluator. It can be a tool the supervisor calls ("is this draft good enough, and what is missing?") or a final gate that must accept the answer before it leaves the team.

The critic does not do the work it judges. Its result is a verdict plus notes, written back onto the shared context. This is Chapter 13. A pipeline's critique stage is one critic. A supervisor may call the same critic after any specialist.

## Composition

Patterns nest. The shape in Chapter 14 is a supervisor that fans out for parallel specialists and uses a handoff only when ownership should actually move. A pipeline stage can itself be a small supervisor. A critic can sit at the end of either.

Composition still has one default: if you are unsure, the outer loop is a supervisor and the inner agents are tools.

## Blackboard

A blackboard is shared state that any agent can read, plus conditions that wake an agent when the state matches. A thin scheduler notices the condition and runs that agent. Agents do not call each other.

Use it when the next step depends on what is on the board, not on a manager's plan: a task no one owns yet, a fact that unblocks a waiting specialist. This is Chapter 20, in the loosely coupled part of the book. It is a poor default for a short question with a known team.

## Message bus

Peers publish and subscribe, or send one another a direct message. There is no manager in the middle. This is Chapter 21. Reach for it when agents are long-lived and the work arrives as events. For one user request with a few specialists, the supervisor is simpler and easier to bound.

## Hard limits

Every pattern has a ceiling, and hitting it stops the run with a clear error:

- an iteration budget on the agent loop
- a max-handoff count
- a worker limit on fan-out
- a turn or token budget on the supervisor
- a depth limit when patterns nest

A limit is part of the pattern, not an afterthought. An unbounded team is a bug.

## MCP

Tools start out MCP-shaped (Chapter 3): a name, a description, an input schema, and content results. Chapter 4 is the client, built with `rmcp`. It connects over stdio and streamable HTTP, lists tools, and calls them. Native tools and MCP tools share one registry, so the agent loop (Chapter 5) does not care which side a tool came from.

Chapter 6 is the server. It exposes this harness's tools, and later a whole agent, so another client (Claude Desktop, Cursor) can call them. Chapter 7 treats an MCP server crash, timeout, or dropped connection as a classified failure: retry, reconnect, or fall back.

The supervisor (Chapter 10) and the handoff chapters call MCP tool servers through that same registry. An agent-as-a-tool can be the thing the server exposes. The capstone research team (Chapter 23) gets its tools from MCP servers.

## What talks to the outside

| Boundary | Use |
|---|---|
| A tool in another process, or this harness's tools offered to another client | MCP |
| An agent in another process or organization | A2A (Chapter 22) |
| An agent in this process | supervisor, pipeline, handoff, fan-out, critic, blackboard, or bus |

Do not put A2A between two agents in this harness. Do not use MCP as the way those two agents decide who works next. MCP carries tools (and, once Chapter 6 exists, an agent we choose to publish). The team pattern still decides the call.
