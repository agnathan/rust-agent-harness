# Social posts: Chapter 33, "Composing Patterns"

## X thread
1/
Chapter 33 is up: Composing Patterns.

nested_fanout refuses when held plus width passes the pool limit, before schedule. Width 9 on a pool of 8 is ExceedsBound. max_seen stays 0. Stay does not call handoff. 🧵

2/
Hermes Agent at 9e01b44. hermes_cli/goals.py: DEFAULT_MAX_TURNS is 20. DEFAULT_JUDGE_TIMEOUT is 30.0. DEFAULT_GATE_TIMEOUT_SECONDS is 300. DEFAULT_GATE_MAX_RETRIES is 3. _parse_judge_response prefers a session, then a pid, then seconds.

3/
tools/delegate_tool_config.py: MAX_DEPTH is 1. The default concurrent-children cap is 10. This pool's limit is WORKER_BOUND, 8. A red gate does not call as_tool. Four failures pause. The 30 and the 300 are not slept.

4/
The bench id is 33/nested_eight. The mean is 271.87 µs, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The demo prints "fan n0 | n1; owner supervisor; done". No key.

5/
Next is context: scan the project file, keep a head and a tail, wrap untrusted tool output, and share the snapshot with Arc::clone.

Read Chapter 33: [link]

#rustlang #AIagents

## LinkedIn
Chapter 33 is out: Composing Patterns.

A fan-out under a supervisor still has to fit the pool. Hermes Agent caps a goal at 20 turns, times the judge at 30.0 seconds, times a quality gate at 300 seconds, and allows 3 gate retries. The judge's wait target prefers a session, then a pid, then seconds. I stored those numbers, refused held plus width past the pool limit before schedule, and ran scripted gate exits before as_tool. The pool the tests build has limit 8. Hermes's default concurrent-children cap in tools/delegate_tool_config.py is 10. MAX_DEPTH there is 1. A red gate does not call the judge. Nothing in the loop sleeps 30 or 300.

The bench id is 33/nested_eight. The mean is 271.87 µs, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573).

Next: context files, the head and the tail, the untrusted wrap, and a snapshot Arc the hot path only clones. Read Chapter 33: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
