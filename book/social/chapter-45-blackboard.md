# Social posts: Chapter 45, "The Blackboard"

## X thread
1/
Chapter 45 is up: The Blackboard.

One board of facts and waiters. tick claims under the lock and drops the guard before the model. A paused board claims nothing. post refuses four injection phrases and does not store them. 🧵

2/
I looked at Hermes main at 9e01b44. hermes_cli/kanban_db_dispatch.py line 2029 fires the tick observer after the dispatch lock is released. Line 2348 repeats that. agent/estop.py is_engaged fails safe on a stat error. check_paused returns true when engaged.

3/
tools/threat_patterns.py line 27 matches ignore, a bounded filler, previous|all|above|prior, then instructions. This board keeps the four phrases as exact substrings. No filler. Eight wakes, then the fact stays.

4/
The demo prints "blackboard: researcher ok". The bench id is 45/claim_with_waiters. The old ids 43/claim_with_waiters and 41/claim_with_waiters are aliases. The mean is MEAN_PENDING_45, mean.point_estimate to be filled from this PR's Benchmarks run.

[personal note]

5/
Next is chapter 46, the kanban board: one lock file, one winner.

Read Chapter 45: [link]

#rustlang #AIagents

## LinkedIn
Chapter 45 is out: The Blackboard.

The harness is the chapter 44 tree copied forward, plus a blackboard module. Facts sit in a HashMap behind a Mutex. tick locks, claims one waiter whose key is already posted, and returns. answer try-locks before FakeModel, and panics if the lock is still held. A paused board returns Claim::Paused and leaves the fact where it is. post refuses ignore previous instructions, ignore all instructions, ignore above instructions, and ignore prior instructions. Eight wakes is the cap. The next fact stays on the board.

Hermes Agent at 9e01b44 does not ship a blackboard type. hermes_cli/kanban_db_dispatch.py releases the dispatch lock before the tick observer, at line 2029, and line 2348 says the same thing. agent/estop.py is_engaged is true if any sentinel exists and fails safe on a stat error. check_paused returns true when engaged. tools/threat_patterns.py bounds the filler between ignore and instructions. This chapter's check is the four phrases with no filler words.

The demo prints "blackboard: researcher ok". The bench id is 45/claim_with_waiters. Aliases: 43/claim_with_waiters and 41/claim_with_waiters. The mean is MEAN_PENDING_45, the mean.point_estimate to be filled from this PR's Benchmarks run.

[personal note]

Next: chapter 46, the kanban board, one lock file, one winner. Read Chapter 45: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
