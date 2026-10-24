# Social posts: Chapter 28, "The Sequential Pipeline"

## X thread
1/
Chapter 28 is up: The Sequential Pipeline.

Outline, then draft, then critique, then revise. Each stage is a scripted reply. The next one receives the same Arc. hand_off returns that Arc. 🧵

2/
Hermes Agent's _append_moa_context in agent/turn_request_assembly.py runs MoA reference models and appends aggregated context to the last user message. It calls agent.moa_loop.aggregate_moa_context. That file has no outline, draft, critique, revise pipeline.

3/
Four stages share one Arc<Context>. The message vec stays one allocation. A separate bench clones 100 messages so the copy has a mean. The pipeline does not call that clone.

4/
The mean for handing off the Arc is 9.15 ns, and the mean for copying the transcript is 6.12 µs, both mean.point_estimate from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589). The demo ends on "revise: the revision". No key.

5/
Next is the supervisor: specialists on one pool, eight at a time. A child at depth 1 cannot call the blocked tools.

Read Chapter 28: [link]

#rustlang #AIagents

## LinkedIn
Chapter 28 is out: The Sequential Pipeline.

A later stage should read what the earlier stage attached, on the same context. Hermes Agent's _append_moa_context appends aggregated reference-model context to the last user message, via aggregate_moa_context. I used that idea for four scripted stages, outline through revise, and gave every stage the same Arc. hand_off returns that Arc.

The mean for handing off the Arc is 9.15 ns, and the mean for copying the transcript is 6.12 µs.

Next: a supervisor that runs specialists under a bound of 8. Read Chapter 28: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
