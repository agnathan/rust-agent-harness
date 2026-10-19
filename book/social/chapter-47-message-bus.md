# Social posts: Chapter 47, "The Message Bus"

## X thread
1/
Chapter 47 is up: The Message Bus.

One envelope, many subscribers, a channel of 64. A full subscriber is Slow. The 21st bot publish is Cooldown and is not sent. 🧵

2/
Hermes Agent claims unseen events under a lock, then fires the hook outside that lock. gateway/bot_loop_guard.py stops at 20 bot messages. agent/redact.py masks a bearer residue before egress.

3/
This bus is sync_channel and try_send. Ordinary publish does not count toward the cooldown. Bearer and sk- are masked in submit. Fake replies. No key.

4/
One publish to 100 agents measured 45.75 µs. The demo prints "bus: ping ok".

[personal note]

5/
Next is shared team skills: a proposal queue, and a librarian who is the only writer.

Read Chapter 47: [link]

#rustlang #AIagents

## LinkedIn
Chapter 47 is out: The Message Bus.

Peers need a way to hand a small envelope to each other without waiting on the slowest reader. Hermes Agent notifies subscribers from a claimed cursor and keeps the bot-loop guard at 20 messages. This chapter is an in-process bus. Each subscriber gets a `sync_channel` of 64. `publish` uses `try_send`. A full channel returns `Slow` and does not grow. `publish_bot` counts deliveries and the 21st returns `Cooldown` without sending. There is no clock in that count.

`submit` masks `Bearer` and `sk-` before the body is queued. A different topic delivers zero.

One publish of `ping` to 100 agents measured 45.75 µs. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: shared team skills. Read Chapter 47: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
