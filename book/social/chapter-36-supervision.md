# Social posts: Chapter 36, "Supervision and Restart"

## X thread
1/
Chapter 36 is up: Supervision and Restart.

is_stale is implemented. Idle goes stale at 15 cycles, 450 seconds. Inside a tool the cap is 40 cycles, 1200 seconds. The interval is 30 seconds. Progress returns false. The function does not sleep and does not start a thread. 🧵

2/
Hermes Agent at 9e01b44, tools/delegate_tool.py. _HEARTBEAT_INTERVAL = 30. _HEARTBEAT_STALE_CYCLES_IDLE = 15. _HEARTBEAT_STALE_CYCLES_IN_TOOL = 40. The comment says progress is iteration, current_tool, or last_activity_ts. An in-flight model wait refreshes that timestamp, so a slow model is not idle.

3/
tools/delegate_tool_child_run.py picks stale_cycles: the in-tool constant when current_tool is set, otherwise the idle constant. The restart breaker is separate. gateway/restart_loop_guard.py uses 3, a 60 second window, and a 300 second gap. An I/O failure fails open. One scripted failure retries once.

4/
The bench is restart_bookkeeping: one record on an empty supervisor. The mean is 29.8 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The demo prints "restarted: ok". No key.

[personal note]

5/
Next is chapter 37, the server track: balancing and federation. Least outstanding across a per-tool pool. Discovery connects at most 4. No socket.

Read Chapter 36: [link]

#rustlang #AIagents

## LinkedIn
Chapter 36 is out: Supervision and Restart.

A failed child needs two different decisions. Hermes Agent heartbeats a delegated child every 30 seconds. Idle is stale at 15 cycles, 450 seconds. A child inside a tool gets 40 cycles, 1200 seconds. The comment in tools/delegate_tool.py says progress is iteration, current_tool, or last_activity_ts. I implemented that check as is_stale. It divides elapsed seconds by 30 and compares. It does not sleep and it does not start a thread. The restart breaker still trips at three chained boots, with a 300 second gap, and an I/O failure stays open. One scripted failure retries once. The bench records one boot and does not call the child.

The mean for restart_bookkeeping is 29.8 ns.

[personal note]

Next: balancing on the server track. A tool gets a pool, the pick is least outstanding, and discovery connects at most four. Read Chapter 36: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
