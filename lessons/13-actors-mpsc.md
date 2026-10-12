# Lesson 13: One actor per agent with an mpsc inbox

**Goal:** Turn an agent into an independent task that you talk to only by sending messages.

## Concepts
- **The actor pattern.** One task owns the state and nobody else touches it, so you need no locks. Other code holds a cloneable *handle* containing a `Sender`.
- **Bounded `mpsc::channel(capacity)`.** A full inbox makes senders wait (`send().await`) or fail fast (`try_send`). That's **backpressure**: a slow agent can't be flooded with work.
- **Request/reply.** Put a `oneshot::Sender` inside the message; the actor answers on it.
- **Shutdown.** The actor ends on a `Stop` message, or when every handle has been dropped (`recv()` returns `None`).
- **Closures.** `FnMut(&str, &[String]) -> String + Send + 'static` is the "brain". Later it becomes the real turn loop.

## How Hermes does it
- The gateway queues messages that arrive while an agent is busy (`gateway/platforms/base.py` `_pending_messages`).
- `/stop` and `/steer` go through `agent/interrupt_control.py` (`interrupt()`, `steer()`).
- Steer text is delivered as a user message after the next tool result (`prompt_builder.steer_user_row`). Our actor applies steers to the next Ask, which is the same idea.

## Tasks
1. `spawn_agent`: create the channel, `tokio::spawn(async move { while let Some(msg) = rx.recv().await { match msg {...} } count })`.
2. `ask`: create a oneshot channel, send, then await the reply. Map both possible errors to `SendError::Gone`.
3. `try_steer`: match `TrySendError::Full` to `Busy` and `Closed` to `Gone`.

## Milestone
`cargo test --test l13_actor` is green, including the backpressure test.

## Exercises
- Replace the closure brain with your lesson 07/11 loop running against the `ScriptedModel`.
- Add `AgentMsg::Status(oneshot::Sender<AgentStatus>)`.
- Run 1,000 agents that each answer 100 asks, and time it.
