# Social posts: Chapter 29, "The Supervisor"

## X thread
1/
Chapter 29 is up: The Supervisor.

Specialists share one pool. WORKER_BOUND is 8. A child at depth 1 cannot call delegate_task, clarify, memory, send_message, or cronjob_manage. Depth above 1 cannot call any tool. 🧵

2/
Hermes Agent lists those five names as DELEGATE_BLOCKED_TOOLS in tools/delegate_tool_toolsets.py. MAX_DEPTH is 1 and _DEFAULT_MAX_CONCURRENT_CHILDREN is 10 in tools/delegate_tool_config.py. delegation.max_concurrent_children is the config key. _get_worktree_isolation defaults to false.

3/
This pool uses 8, not 10. HERMES_MAX_CONCURRENT_CHILDREN keeps the 10 visible. worktree returns None unless you opt in, and it does not create a git worktree. enqueue_reflection pushes a string and does not start chapter 26's reviewer. max_seen stays 0 for that call.

4/
The mean for schedule_eight is MEAN_29_schedule_eight, and the mean for enqueue_reflection is MEAN_29_enqueue_reflection, both mean.point_estimate from run BENCH_RUN_URL. The demo prints "three steps | the draft". No key.

[personal note]

5/
Next is a handoff: filter scratch out of the transcript, then a cap of four transfers. Platform re-bind stays chapter 39.

Read Chapter 29: [link]

#rustlang #AIagents

## LinkedIn
Chapter 29 is out: The Supervisor. It closes M5 and M6.

Hermes Agent blocks delegate_task, clarify, memory, send_message, and cronjob_manage on a child, keeps MAX_DEPTH at 1, and defaults concurrent children to 10. I kept the block list, the depth, and the opt-in worktree flag. The pool allows 8. worktree does not create a git worktree. A reflection note is a string on a queue of 64, and it does not spawn the reviewer.

The mean for schedule_eight is MEAN_29_schedule_eight, and the mean for enqueue_reflection is MEAN_29_enqueue_reflection.

[personal note]

Next: handing the conversation to another owner after scratch is filtered out. Read Chapter 29: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
