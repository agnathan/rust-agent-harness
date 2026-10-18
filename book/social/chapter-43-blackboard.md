# Social posts: Chapter 43, "The Blackboard"

## X thread
1/
Chapter 43 is up: The Blackboard.

A fact lives behind one mutex. The scheduler claims a waiter and drops the guard before the model runs. A paused board refuses a new claim. 🧵

2/
Hermes Agent wakes work from a dispatcher and runs the hook after the lock. agent/estop.py fails safe and skips new work. tools/threat_patterns.py names the prompt-injection phrases this board refuses at post.

3/
One hundred waiters, the matching key last. Fake replies. No key. Eight wakes, then idle.

4/
One claim measured 4.97 µs. The demo prints "blackboard: researcher ok".

[personal note]

5/
Next is the kanban board: the card on disk, and two processes that cannot both win the claim.

Read Chapter 43: [link]

#rustlang #AIagents

## LinkedIn
Chapter 43 is out: The Blackboard.

Shared state for a team is a board of facts and waiters. Hermes Agent does not hold its dispatcher lock across the work that the claim starts, and an emergency stop skips new work without cancelling a claim already taken. This chapter does that in one process. `tick` locks, claims one waiter whose key is already posted, and returns. `FakeModel` runs only after that return. A test takes the mutex again before the model, so a lock held across the call fails the test.

An injection phrase is refused at `post` and is not stored. A waiter stops after eight claims. The fact stays for someone else.

One claim with 100 waiters measured 4.97 µs. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: the kanban board. Read Chapter 43: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
