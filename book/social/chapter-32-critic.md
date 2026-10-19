# Social posts: Chapter 32, "The Critic"

## X thread
1/
Chapter 32 is up: The Critic.

The critic reads a finished reply. It takes one permit from the supervisor pool. verdict: done accepts. A missing verdict rejects. The critic does not write the answer. 🧵

2/
Hermes Agent's goal judge is hermes_cli/goals.py. Verdicts are done, blocked, continue, and wait. _parse_judge_response fails open: an empty reply or an unknown verdict becomes continue. The snippet cap is 4,000 characters. The turn cap is 20. The timeout is 30 seconds.

3/
This gate has to accept before the answer leaves, so a missing verdict is Reject. The 20 turns and the 30 seconds are named. This crate does not loop and does not sleep. wait parses and does not wait.

4/
One judgment measured 196.6 ns. The demo prints "accepted: done". No key.

[personal note]

5/
Next is composing the patterns: fan-out under the supervisor, and a handoff only when ownership should move.

Read Chapter 32: [link]

#rustlang #AIagents

## LinkedIn
Chapter 32 is out: The Critic.

Someone has to say the work is finished, and that someone should not be the worker who just did it. Hermes Agent's goal judge returns done, blocked, continue, or wait, and it fails open: a reply it cannot parse becomes continue. I kept the four names and the 4,000-character snippet. A missing verdict rejects, because this gate has to accept before the answer leaves.

The critic takes one permit from the same pool of eight and reads the shared context. It does not write the notes. One judgment measured 196.6 ns.

[personal note: one takeaway from writing this chapter]

Next: composing these patterns in one supervisor. Read Chapter 32: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
