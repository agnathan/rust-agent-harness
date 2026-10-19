# Social posts: Chapter 52, "Evals"

## X thread
1/
Chapter 52 is up: Evals. The last chapter.

The saved names from chapter 9 run again on this copy of the loop. One red-team case per layer, L1 through L9 and X. There is still no fail threshold. 🧵

2/
I looked for a Hermes saved-eval runner at 9e01b44 and did not find one that scores these tasks. The cases call the guards I did read: estop fails safe, threat patterns, redact_for_egress, and a skill install where force does not override a dangerous verdict.

3/
unknown-tool-repair asks for eco and the loop's repair_name turns it into echo. One hundred trajectories stay inside the worker bound of 8. The ninth permit is refused. The peak is not printed. The line says within_bound: true.

4/
The demo ends with "trajectories: 100 within_bound: true" and "threshold: None". The bench ids are 52/append_event and 52/saved_set. The old ids 50/append_event, 48/append_event, 50/saved_set, and 48/saved_set are aliases. The means are 82.2 ns and 21.26 ms, mean.point_estimate from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). saved_set includes the 20 millisecond sleep inside eight-reads.

[personal note]

5/
That is the end of the book. The harness is the chapter 50 tree plus the suite. The server line ended at chapter 51. The numbers stay measurements.

Read Chapter 52: [link]

#rustlang #AIagents

## LinkedIn
Chapter 52 is out: Evals. It is the last chapter of the book.

Chapter 9 saved three tasks and four stress names. This chapter runs those names again on this copy of the loop: echo-hello, direct, eight-reads, and the stress cases that recover, hit the iteration cap, repair an unknown tool, and stall. unknown-tool-repair asks for the tool name eco, and repair_name inside the loop turns it into echo. Beside them is one red-team case per layer from the guardrail checklist, L1 Entry/Gateway through L9 Scheduling/delegation, plus X Cross-cutting. Each case calls the real module. FAIL_THRESHOLD is None.

I looked through Hermes at 9e01b44 for a saved-eval runner to quote and did not find one. The guards the cases call are there: agent/estop.py fails safe on a stat error, tools/threat_patterns.py, agent/redact.py redact_for_egress, and tools/skills_guard.py, where force does not override a dangerous verdict.

A trajectory line is one JSON object pushed onto a vec. One hundred tasks take permits from a pool of 8. Filling that pool makes the ninth permit fail. The demo prints within_bound: true and does not print the peak, which can sit anywhere from 1 to 8. The bench ids are 52/append_event and 52/saved_set. Aliases: 50/append_event, 48/append_event, 50/saved_set, and 48/saved_set. The means are 82.2 ns and 21.26 ms, mean.point_estimate from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). saved_set sleeps 20 milliseconds inside eight-reads. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

This is the last chapter. The harness is the chapter 50 tree plus the suite. The server line ends at chapter 51. Read Chapter 52: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
