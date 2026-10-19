# Social posts: Chapter 36, "Supervision and Restart"

## X thread
1/
Chapter 36 is up: Supervision and Restart.

A failed child runs once more. Three chained restarts skip auto-resume. The measurement is the decision, not the child's work. 🧵

2/
Hermes Agent records this in gateway/restart_loop_guard.py. Three restarts, a 60 second window, and a 300 second gap. The gap that links two boots is 300 seconds. A read error fails open: the breaker does not trip.

3/
The log here is in memory. now is an argument, so the test does not sleep. Cancel and a zero budget never start the child. FakeModel fails once and then returns ok. No key.

4/
One record measured 27.75 ns. The demo prints "restarted: ok".

[personal note]

5/
Next is the server: a pool of backends for one tool, and a pick that stays local.

Read Chapter 36: [link]

#rustlang #AIagents

## LinkedIn
Chapter 36 is out: Supervision and Restart.

A failed child should get one more try, and a process that keeps dying should stop being resumed. Hermes Agent chains restart-interrupted boots and skips auto-resume at three, as long as the gap between them stays inside 300 seconds. A failure to read that log does not trip the breaker. I kept those rules. Cancel and an empty budget return before the child runs. The bench records one boot on an empty supervisor and does not call the child.

That record measured 27.75 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: pick a backend from a pool, on the server. Read Chapter 36: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
