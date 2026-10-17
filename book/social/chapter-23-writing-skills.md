# Social posts: Chapter 23, "Writing Skills"

## X thread
1/
Chapter 23 is up: Writing Skills.

A skill write is scanned before it lands. The approval gate is off until you turn it on. A reviewer has to load the file before it can patch it. A write the model did not ask for is a queue entry. 🧵

2/
Hermes Agent keeps that policy in tools/skills_guard.py (INSTALL_POLICY, should_allow_install). force cannot override a dangerous verdict on a community or trusted skill. The gate is tools/write_approval.py evaluate_gate, and skills.write_approval defaults to False.

3/
A reviewer that has not called skill_view is refused. The wording is the one in tools/skill_manager_guards.py (_background_review_read_before_write_guard). A pin blocks delete. It does not block a patch.

4/
Scanning one skill of about 1 KB measured 242.5 ns. Enqueueing an unsolicited write measured 17.96 ns. The demo scan is safe. No key.

[personal note]

5/
Next is the reflection loop: a reviewer forked after the turn, with only skill and memory tools, cancelled when the user comes back.

Read Chapter 23: [link]

#rustlang #AIagents

## LinkedIn
Chapter 23 is out: Writing Skills.

Loading a skill and writing one are different jobs. Hermes Agent scans a community skill before install, and it refuses to let --force override a dangerous verdict. The write-approval gate stays off unless skills.write_approval is set. I did the same: the gate starts off, a dangerous community skill stays blocked, and a reviewer has to view the current file before a patch.

Scanning one skill measured 242.5 ns. A write the model did not ask for is queued, not written.

[personal note: one takeaway from writing this chapter]

Next: a reviewer that runs after the turn and can be cancelled when the user returns. Read Chapter 23: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
