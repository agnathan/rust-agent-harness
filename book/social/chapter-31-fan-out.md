# Social posts: Chapter 31, "Fan-out and Gather"

## X thread
1/
Chapter 31 is up: Fan-out and Gather.

The board accepts 100 tasks. The 101st schedule returns FanError::Full. gather runs them on the supervisor pool and joins replies in task_index order. tool_trace stays off that string. 🧵

2/
Hermes Agent runs children in parallel in tools/delegate_tool_dispatch.py (_run_children_parallel). _get_max_async_children sets the background cap to delegation.max_concurrent_children. A leftover delegation.max_async_children key is ignored with a one-time warning. At capacity a new async dispatch is rejected, not queued.

3/
QUEUE_CAP here is 100, not the concurrent-children number. The pool that runs the tasks is still WORKER_BOUND of 8. Heartbeats are not this chapter. They stay in chapter 36.

4/
The mean for schedule_100 is 4.12 µs, mean.point_estimate from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589). The demo prints three lines: a, b, c. No key.

5/
Next is the critic: one permit from the same pool, accept only on Verdict::Done, and a missing verdict word is Reject.

Read Chapter 31: [link]

#rustlang #AIagents

## LinkedIn
Chapter 31 is out: Fan-out and Gather.

Parallel children need a stop. Hermes Agent runs them together, and a new async dispatch at the concurrency cap is rejected rather than queued. The background cap equals delegation.max_concurrent_children. I queued up to 100 and then returned FanError::Full, and gather still takes permits from the pool of 8. Replies come back sorted by task_index. The tool trace is not in the summary.

The mean for schedule_100 is 4.12 µs.

Next: a critic that takes one permit and does not write the answer. Read Chapter 31: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
