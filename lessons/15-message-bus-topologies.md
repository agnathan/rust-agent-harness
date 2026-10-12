# Lesson 15: A message bus and team topologies

**Goal:** Let agents address each other by name and by topic, and build teams on top of that.

## Concepts
- **Router.**
  - Direct messages: a `HashMap<name, mpsc::Sender>`.
  - Topics: `tokio::sync::broadcast`, where every subscriber gets a copy.
- **Envelope.** `from`, `to`, `corr_id` (to match replies to requests) and `body`.
- **`std::sync::Mutex` in async code.** It's fine for quick map lookups, but **never hold the guard across an `.await`**. Clone the `Sender`, drop the lock, then await.
- **Topologies:**
  - **Orchestrator–worker:** one agent sends tasks out and collects replies.
  - **Peer messaging:** any agent can message any other.
  - **Blackboard/kanban:** agents claim tasks from a shared board.

## How Hermes does it
Hermes has several separate mechanisms rather than one bus:
- `delegate_task` (parent → child, synchronous).
- The async-delegation completion queue (child → parent).
- `tools/send_message_tool.py` (agent → chat platform).
- The A2A JSON-RPC adapter (`plugins/platforms/a2a/adapter.py`).
- The kanban board: SQLite plus a dispatcher that polls every 60 s (`hermes_cli/kanban_db*.py`, `tools/kanban_tools.py`, `cron/AGENTS.md`).

Your Router unifies all of these (deep dive §B3–B4).

## Tasks
1. `Router::register` and `subscribe`.
   - Hint for `subscribe`: `entry(..).or_insert_with(|| broadcast::channel(64).0).subscribe()`.
2. `Router::send`: clone the sender inside a short block, then await outside it.
3. The `Blackboard` methods. Hold the lock for the whole of `claim`, so the claim is atomic.
4. `orchestrate`: register the orchestrator's inbox, spawn the workers, send task `i` to `workers[i % n]`, collect `n` replies and sort them by `corr_id`.

## Milestone
`cargo test --test l15_bus` is green.

## Exercises
- Build a peer `ask(router, to, body, timeout)` helper with automatic `corr_id`s.
- Build a blackboard team: three workers loop on `claim`/`complete` until the board is empty. Then publish a `board.updated` topic so they wake up only when work arrives (no polling).
- Add a "debate" topology: two agents alternate on a topic, and a judge reads the result.
