# Social posts: Chapter 25, "Writing Skills"

## X thread
1/
Chapter 25 is up: Writing Skills.

A skill write is scanned before it lands. The approval gate is off until you turn it on. A reviewer has to load the file before it can patch it. A write the model did not ask for is a queue entry. 🧵

2/
Hermes Agent keeps that policy in tools/skills_guard.py (INSTALL_POLICY, should_allow_install). force cannot override a dangerous verdict on a community or trusted skill. The gate is tools/write_approval.py, and skills.write_approval is off unless configured.

3/
A reviewer that has not called view is refused. The guard is tools/skill_manager_guards.py (_background_review_read_before_write_guard). A pin blocks delete. Origin bundled, hub, or team blocks delete.

4/
Scanning one skill measured MEAN_25_scan_one_skill. An unsolicited enqueue measured MEAN_25_enqueue_unsolicited (BENCH_RUN_URL). Those figures are the means. The demo scan is safe. No key.

[personal note]

5/
Next is the reflection loop: a reviewer fork after the turn, with memory only when the memory nudge fired.

Read Chapter 25: [link]

#rustlang #AIagents

## LinkedIn
Chapter 25 is out: Writing Skills.

Loading a skill and writing one are different jobs. Hermes Agent's INSTALL_POLICY has four rows, and should_allow_install refuses to let force override a dangerous verdict on a community or trusted skill. The write-approval gate stays off unless skills.write_approval is set. I kept that table and that force rule. The pattern list here is five rules, a subset of THREAT_PATTERNS. A reviewer has to view the current file before a patch.

Scanning one skill measured MEAN_25_scan_one_skill. An unsolicited enqueue measured MEAN_25_enqueue_unsolicited (BENCH_RUN_URL). Those figures are the means. A write the model did not ask for is queued.

[personal note: one takeaway from writing this chapter]

Next: a reviewer fork after the turn, cancelled with a flag when the user returns. Read Chapter 25: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
