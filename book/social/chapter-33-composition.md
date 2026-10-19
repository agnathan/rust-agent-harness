# Social posts: Chapter 33, "Composing Patterns"

## X thread
1/
Chapter 33 is up: Composing Patterns.

The outer loop is a supervisor. An inner fan-out uses that same pool of 8. A handoff runs only when ownership should move. The critic loops. 🧵

2/
Hermes Agent keeps delegation flat by default. tools/delegate_tool_config.py sets MAX_DEPTH to 1 and the concurrent-children default to 10. hermes_cli/goals.py is the loop I pulled in: 20 turns, a 30 second judge timeout, a quality gate with 3 retries, and a wait that must name a session, a pid, or a number of seconds.

3/
A nested request that would pass the bound is refused before any task is scheduled. A wait with no target continues. A failing gate does not ask the judge. The shell is not spawned. FakeModel is a queue of strings. No key.

4/
Eight nested children measured 270.8 µs. The demo prints "fan n0 | n1; owner supervisor; done".

[personal note]

5/
Next is managing context: compression and a scan of the files that would otherwise enter the prompt.

Read Chapter 33: [link]

#rustlang #AIagents

## LinkedIn
Chapter 33 is out: Composing Patterns.

The patterns from the last five chapters only help if you know which one you are in. Hermes Agent keeps a child delegation flat by default (depth 1, and a concurrent-children default of 10) and runs a goal loop with a judge, a wait target, and a quality gate. I kept the wait-target order and the rule that a failing gate does not ask the judge. Nested fan-out here is one pool of eight. A request that would pass that bound is refused before anything is scheduled. A handoff runs only when ownership should move.

Eight nested children measured 270.8 µs. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: managing context, so the prompt the model sees is the one we meant to send. Read Chapter 33: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
