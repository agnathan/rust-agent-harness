# Social posts: Chapter 28, "Handoffs"

## X thread
1/
Chapter 28 is up: Handoffs.

Ownership moves to the next agent after scratch messages are removed. A turn can do that four times. The fifth is refused, and the owner stays. 🧵

2/
Hermes Agent's handoff.request in tui_gateway/methods_session.py is a platform transfer. It writes pending, and a watcher re-binds the session. It does not count handoffs, and it does not filter a transcript.

3/
MAX_HANDOFFS is 4. That number is this harness. It is not a Hermes constant. The filter drops role scratch and keeps order. An empty result does not move the owner and does not burn a count.

4/
Filtering 100 messages measured 2.740 µs. The demo is "draft sees 2". No key.

[personal note]

5/
Next is fan-out and gather: a queue of 100, the same bound of eight, and replies joined in order.

Read Chapter 28: [link]

#rustlang #AIagents

## LinkedIn
Chapter 28 is out: Handoffs.

Giving the next agent the whole transcript hands over the scratch too. Hermes Agent's handoff.request is a platform transfer: it queues pending work and a watcher re-binds the session. It does not cap how many times that happens, and it does not filter messages. I added both. Scratch is dropped before the owner changes, and the fifth handoff in a turn is refused.

Filtering 100 messages measured 2.740 µs. Four is the harness cap, not a number I found in Hermes.

[personal note: one takeaway from writing this chapter]

Next: scheduling a hundred subagents without an unbounded queue. Read Chapter 28: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
