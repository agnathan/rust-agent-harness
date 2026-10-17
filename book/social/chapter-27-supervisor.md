# Social posts: Chapter 27, "The Supervisor"

## X thread
1/
Chapter 27 is up: The Supervisor.

Specialists are tools. Eight run at once, on one shared Arc. A child cannot call delegate_task, clarify, memory, send_message, or cronjob_manage. Depth stops at 1. 🧵

2/
Hermes Agent lists those five names in tools/delegate_tool_toolsets.py (DELEGATE_BLOCKED_TOOLS). MAX_DEPTH is 1 in tools/delegate_tool_config.py. delegation.max_concurrent_children defaults to 10.

3/
The harness bound stays 8. That is the measurement, not the Hermes default. The code names 10 so the default stays visible, and the pool does not use it. A worktree is opt-in (tools/subagent_worktree.py) and this chapter does not create one.

4/
Eight specialists that each slept 40 ms finished in 0.044 s in nextest, with max in flight at 8. Scheduling eight with no sleep measured 98.54 µs. The demo prints "three steps | the draft". No key.

[personal note]

5/
Next is a handoff: filter the transcript, then a cap on how many times ownership can move.

Read Chapter 27: [link]

#rustlang #AIagents

## LinkedIn
Chapter 27 is out: The Supervisor.

Parallel specialists are how a manager spends a turn, and a child that can delegate again is a tree you did not mean to grow. Hermes Agent blocks delegate_task, clarify, memory, send_message, and cronjob_manage on the child, keeps depth at 1, and defaults concurrent children to 10. I kept the block list and the depth. The pool allows 8, which is the number this book measures.

Eight specialists that each slept 40 ms finished in 0.044 s, and the peak in flight was 8. A git worktree stays opt-in, and this crate does not create one.

[personal note: one takeaway from writing this chapter]

Next: handing a conversation to another agent, after the scratch is filtered out. Read Chapter 27: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
