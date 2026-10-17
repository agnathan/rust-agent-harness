# Social posts: Chapter 26, "The Sequential Pipeline"

## X thread
1/
Chapter 26 is up: The Sequential Pipeline.

Outline, then draft, then critique, then revise. Each stage is a value. The next one receives the same Arc. The transcript is not copied. 🧵

2/
Hermes Agent does not have that four-stage writer. The closest pass is mixture-of-agents in agent/turn_request_assembly.py: reference models run, and an aggregator appends their context to the last user message.

3/
hand_off returns the same Arc. Arc::ptr_eq is the test. A separate bench clones 100 messages so the copy has a number. The pipeline does not call it.

4/
Handing off the Arc measured 10.78 ns. Copying 100 messages measured 7.089 µs. The demo ends on "revise: the revision". No key.

[personal note]

5/
Next is the supervisor: specialists as tools, eight at a time, one shared context. A child cannot delegate again.

Read Chapter 26: [link]

#rustlang #AIagents

## LinkedIn
Chapter 26 is out: The Sequential Pipeline.

A draft that cannot see the outline is a second conversation. Hermes Agent's mixture-of-agents path appends reference-model context onto the message the main model sees. I used that idea for four named stages, outline through revise, and gave every stage the same Arc. The message vec stays one allocation.

Handing off that Arc measured 10.78 ns. Cloning the 100 messages measured 7.089 µs, and the pipeline does not do that clone.

[personal note: one takeaway from writing this chapter]

Next: a supervisor that runs specialists in parallel under a bound. Read Chapter 26: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
