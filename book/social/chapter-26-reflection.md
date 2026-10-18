# Social posts: Chapter 26, "The Reflection Loop"

## X thread
1/
Chapter 26 is up: The Reflection Loop.

Every 10 user turns, and every 10 tool iterations, finalize returns a reviewer fork. The reviewer can use skill tools. It gets memory only when the memory nudge fired. Cancel is a flag. 🧵

2/
Hermes Agent sets those intervals in agent/agent_init.py. The fork's own intervals are 0 in agent/background_review.py, so the reviewer does not nudge itself. A skill-nudge review never gets the memory tool. memory.nudge_interval is the config key.

3/
The whitelist adds read_file and search_files. Write tools stay off the slice. Hermes waits up to 2 seconds for cancel. This chapter sets the flag and joins. It does not sleep that 2 seconds. run_loop does not call the fork.

4/
Building the fork measured MEAN_26_spawn_reviewer. Cancel and join measured MEAN_26_cancel_review (BENCH_RUN_URL). Those figures are the means. The demo note is "prefer short notes". No key.

[personal note]

5/
Next is curation: stale at 14 days, archived at 30, and the request path only enqueues the pass.

Read Chapter 26: [link]

#rustlang #AIagents

## LinkedIn
Chapter 26 is out: The Reflection Loop.

A finished turn can teach the next one, and that work stays outside the turn the user is waiting on. Hermes Agent nudges a background review on memory.nudge_interval and the skill interval, then forks a reviewer whose own nudge intervals are 0. Memory is on that fork only when the memory nudge fired. I did the same: Loop::finalize returns the fork, terminal is refused, and cancel is a flag. The 2 second wait is a constant this crate does not sleep. run_loop does not call finalize.

Building the fork measured MEAN_26_spawn_reviewer. Cancel and join measured MEAN_26_cancel_review (BENCH_RUN_URL). Those figures are the means.

[personal note: one takeaway from writing this chapter]

Next: a curator that archives unused skills off the request path. Read Chapter 26: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
