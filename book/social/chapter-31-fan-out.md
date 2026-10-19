# Social posts: Chapter 31, "Fan-out and Gather"

## X thread
1/
Chapter 31 is up: Fan-out and Gather.

A board takes 100 tasks and refuses the 101st. Gather runs them eight at a time and joins the replies in task_index order. The parent does not see the child tool trace. 🧵

2/
Hermes Agent sorts that batch in tools/delegate_tool_dispatch.py (_run_children_parallel). The parent sees the summary, not the child's tool calls (tools/delegate_tool.py). At the concurrency cap, a new async dispatch is rejected rather than queued.

3/
This queue holds 100, then returns Full. The worker bound stays 8, from the supervisor pool. Those are two different caps.

4/
Scheduling 100 tasks onto a fresh board measured 4.285 µs. The demo prints a, then b, then c, from indexes 2, 0, 1. No key.

[personal note]

5/
Next is the critic: one permit from the same pool, a verdict, and a missing verdict is a reject.

Read Chapter 31: [link]

#rustlang #AIagents

## LinkedIn
Chapter 31 is out: Fan-out and Gather.

A hundred subagents need a queue with an end. Hermes Agent runs children in parallel, sorts the results by task_index, and shows the parent the summary rather than the child's tool calls. When the concurrency cap is hit, a new async dispatch is rejected instead of queued. I queued up to 100 and then refused, and gather still takes permits from the pool of eight.

Scheduling those 100 tasks measured 4.285 µs. The tool trace stayed on the task. It did not appear in the summary.

[personal note: one takeaway from writing this chapter]

Next: a critic that accepts or rejects, and does not do the work itself. Read Chapter 31: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
