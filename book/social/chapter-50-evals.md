# Social posts: Chapter 50, "Evals"

## X thread
1/
Chapter 50 is up: Evals. The last chapter.

The saved names from chapter 9 run again on a fake model. One red-team case per layer, L1 through L9 and X. There is still no fail threshold. 🧵

2/
Hermes Agent appends a trajectory as JSONL and keeps a separate file for failures. The guardrail checklist is one case per layer. This crate prints the bools and leaves FAIL_THRESHOLD as None.

3/
One hundred mock trajectories share a pool of 8. The ninth permit is refused. A fast run can show a peak of 1 and still be inside that bound. No key.

4/
One trajectory append measured 81.3 ns. The saved set measured 51.1 ns. The demo ends with "trajectories: 100 peak: 1" and "threshold: None".

[personal note]

5/
That is the end of the book. The harness, the gateway, and the team are in the step crates. The numbers stay measurements, not a gate.

Read Chapter 50: [link]

#rustlang #AIagents

## LinkedIn
Chapter 50 is out: Evals. It is the last chapter of the book.

Chapter 9 saved three tasks and four stress names. This chapter runs those names again with `FakeModel`: echo-hello, direct, eight-reads, and the stress cases that recover, hit the iteration cap, repair an unknown tool, and stall. Beside them is one red-team case per guardrail layer, L1 through L9, plus the cross-cutting case X. `FAIL_THRESHOLD` is `None`. The process prints pass or fail and does not exit on a score.

A trajectory line is one JSON object pushed onto a vec. One hundred mock trajectories take permits from a pool of 8. Filling that pool makes the ninth permit fail. The peak observed in the demo can be 1, because the mock returns before the next thread is scheduled. The test checks that the peak stays between 1 and 8 and that the ninth permit is refused.

One append measured 81.3 ns. The saved set measured 51.1 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

This is the last chapter. Read Chapter 50: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
