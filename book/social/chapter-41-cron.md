# Social posts: Chapter 41, "Cron"

## X thread
1/
Chapter 41 is up: Cron.

One owner claims a due job. A crash may release that claim once. The prompt is scanned before the claim. A job that restarts the gateway does not run. 🧵

2/
Hermes Agent claims a fire under a file lock in cron/jobs.py. Recurring jobs advance next_run_at before the run, so a crash skips the slot. tools/cronjob_prompt_scan.py blocks injection and curl exfil. cron/lifecycle_guard.py blocks hermes gateway restart and stop. start is allowed.

3/
This crate's retry budget is one. The second crash keeps the claim. The due scan does not tick the watchdog. Fake replies. No key.

4/
Scanning 1,000 due jobs measured 1.607 µs. The demo prints "cron: claimed".

[personal note]

5/
Next is the editor. ACP is another client of the session inbox.

Read Chapter 41: [link]

#rustlang #AIagents

## LinkedIn
Chapter 41 is out: Cron.

A scheduled job has to run once. Hermes Agent takes a file-lock claim so one machine wins, and it advances a recurring job's next run time before the work so a crash does not fire that slot again. The user prompt is scanned for injection, secret-file reads, and curl exfiltration. A job that would restart or stop the gateway, or stop it through systemctl, is rejected. hermes gateway start is left alone.

The claim here is in memory. The book budget is one crash retry, and the second crash leaves the claim held. Scanning the due list does not run the watchdog and does not enqueue the next job. Those are separate calls.

Scanning 1,000 due jobs measured 1.607 µs. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: editor integration. Read Chapter 41: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
