# Social posts: Chapter 27, "Curation"

## X thread
1/
Chapter 27 is up: Curation.

A skill goes stale at 14 days and is archived at 30. A pin, a cron mark, a bundled skill, and a hub skill stay. The request path only enqueues the pass. 🧵

2/
Hermes Agent does this in agent/curator.py (apply_automatic_transitions). Stale is 14, archive is 30, the interval is 24*7 hours, idle is 2 hours, and DEFAULT_CONSOLIDATE is false. The snapshot is agent/curator_backup.py.

3/
Enqueue pushes the word curate. It does not scan or move a directory. The pass snapshots first, then marks or archives. The scan and the write-approval gate are chapter 25's, and they run inside the pass, not inside enqueue.

4/
Enqueue measured MEAN_27_enqueue_curation (BENCH_RUN_URL). That figure is the mean. The demo marks one skill stale. No key.

[personal note]

5/
Next is a pipeline of four stages that share one Arc. The next stage does not copy the transcript.

Read Chapter 27: [link]

#rustlang #AIagents

## LinkedIn
Chapter 27 is out: Curation.

Unused skills should leave the working set, and that decision should stay off the turn the user is waiting on. Hermes Agent's apply_automatic_transitions uses 14 days, 30 days, a 24*7 hour interval, and 2 idle hours. DEFAULT_CONSOLIDATE is false. I kept those day counts and took a snapshot before the move. Consolidate stays off. There is no clock and no scheduler thread. The caller passes the age.

The request path only enqueues the pass. That enqueue measured MEAN_27_enqueue_curation (BENCH_RUN_URL). That figure is the mean. The scan and the approval gate run when the pass runs.

[personal note: one takeaway from writing this chapter]

Next: four stages in a row, sharing one context. Read Chapter 27: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
