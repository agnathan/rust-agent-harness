# Social posts: Chapter 30, "Handoffs"

## X thread
1/
Chapter 30 is up: Handoffs.

A handoff is a recorded transfer, not a model call. The next owner sees the transcript with role scratch removed. The fifth transfer is refused, and the owner stays. 🧵

2/
Hermes Agent's handoff.request in tui_gateway/methods_session.py only writes pending. The gateway watcher claims and re-binds. If the session is running, the error is 4009. platform is required.

3/
This board filters before it counts. MAX_HANDOFFS is 4, and that cap is this harness, not a Hermes constant. An empty filtered transcript is HandoffError::Empty and does not move the owner. Platform re-bind stays chapter 39.

4/
The mean for filter_100 is 2.13 µs, mean.point_estimate from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589). The demo prints "draft sees 2". No key.

5/
Next is fan-out: a queue of 100, gather on the pool of 8, replies in task_index order. Heartbeats stay chapter 36.

Read Chapter 30: [link]

#rustlang #AIagents

## LinkedIn
Chapter 30 is out: Handoffs.

The next agent should receive a filtered transcript. Hermes Agent's handoff.request writes a pending transfer for a watcher to claim and re-bind. A running session is error 4009, and platform is required. I kept the idea that a handoff is a recorded transfer, not a model call. This board drops role scratch first, then allows four transfers. The fifth is Limit, and the owner string stays.

The mean for filter_100 is 2.13 µs.

Next: a queue of 100 tasks that refuses the 101st. Read Chapter 30: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
