# Social posts: Chapter 27, "Curation"

## X thread
1/
Chapter 27 is up: Curation.

A skill goes stale at 14 days and is archived at 30. A pin, a cron job, a bundled skill, and a hub skill stay. The request path only enqueues the pass. 🧵

2/
Hermes Agent does this in agent/curator.py (apply_automatic_transitions). Stale is 14, archive is 30, consolidate defaults to false, prune_builtins defaults to false. The snapshot lives in agent/curator_backup.py and keeps 2 tarballs.

3/
Hermes snapshots before consolidation, not before a prune that only moves directories. This chapter snapshots before the move too, and it leaves .git and .curator_backups out of the tarball.

4/
An archive uses the write-approval gate. An install uses the skills-guard scan. Neither runs from enqueue. Enqueue measured 27.80 ns. The demo marks one skill stale. No key.

[personal note]

5/
Next is a pipeline of four agents that share one Arc. The next stage does not copy the transcript.

Read Chapter 27: [link]

#rustlang #AIagents

## LinkedIn
Chapter 27 is out: Curation.

Unused skills should leave the working set, and that decision should not run while a user is waiting. Hermes Agent marks a skill stale after 14 days and archives it after 30, skips anything pinned or referenced by cron, and leaves bundled and hub skills alone unless you opt in. I did the same, then took a snapshot before the move. Hermes takes that tarball before consolidation. A prune that moves directories needed one too.

The request path only enqueues the pass. That enqueue measured 27.80 ns. The scan and the approval gate run when the pass runs.

[personal note: one takeaway from writing this chapter]

Next: four agents in a row, sharing one context. Read Chapter 27: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
