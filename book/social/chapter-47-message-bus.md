# Social posts: Chapter 47, "The Message Bus"

## X thread
1/
Chapter 47 is up: The Message Bus.

One publish, many peers. The channel holds 64. A full subscriber is Slow. The 21st bot publish is Cooldown and is not sent. Every body that deliver queues goes through the chapter 20 egress scrub. 🧵

2/
I read Hermes at 9e01b44. hermes_cli/kanban_db_notify.py claim_unseen_events_for_sub claims inside BEGIN IMMEDIATE and advances last_event_id in that transaction. gateway/bot_loop_guard.py defaults to 20 events, a 300 second window, and a 600 second cooldown.

3/
agent/redact.py redact_for_egress is the one scrub for text leaving the process. It runs redact_sensitive_text, then a Bearer residue of 20 or more opaque characters. An exception returns "[redaction-unavailable]" and never the raw text. This crate calls that chapter 20 function. bus.rs does not define a second scrubber.

4/
The demo prints "bus: ping ok". The bench id is 47/publish_to_agents. The old ids 45/publish_to_agents and 43/publish_to_agents are aliases. The mean is 47.98 µs, mean.point_estimate from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). The Criterion time: line is the slope.

5/
Next is chapter 48, shared team skills. Workers enqueue. The librarian is the only writer.

Read Chapter 47: [link]

#rustlang #AIagents

## LinkedIn
Chapter 47 is out: The Message Bus.

The harness is the chapter 46 tree copied forward, plus a bus module. subscribe opens a sync_channel of 64. publish fans out on a topic. A full channel returns Slow with the subscriber's name. A missing name on send_to is Disconnected. A different topic is Delivered(0). publish_bot counts delivered bot publishes. The 21st is Cooldown and is not sent. There is no 300 second window and no 600 second clock. The counter is in this process.

deliver calls crate::redact::redact_for_egress before try_send. That is E2, the chapter 20 function. There is no local redact_for_egress in bus.rs. An Authorization Bearer header is consumed before the residue sweep, so it stays an Authorization mask and does not become "Bearer [redacted]". A bare Bearer plus 32 opaque characters does.

Hermes Agent at 9e01b44 claims kanban notify events inside BEGIN IMMEDIATE and advances the subscriber cursor in that transaction. BotLoopGuardSettings admits 20 bot messages, then cools down. admit returns tripped when the window fills and cooldown while that deadline still holds. redact_for_egress is the one scrub for text leaving the process, and it fails closed.

The demo prints "bus: ping ok". The bench id is 47/publish_to_agents. Aliases: 45/publish_to_agents and 43/publish_to_agents. The mean is 47.98 µs, mean.point_estimate from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

Next: chapter 48, shared team skills. Workers enqueue. The librarian is the only writer. Read Chapter 47: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
