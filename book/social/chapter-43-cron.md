# Social posts: Chapter 43, "Cron"

## X thread
1/
Chapter 43 is up: Cron.

A job has one owner. A crash may release that claim once. The second crash leaves it held. The prompt is scanned before the claim. 🧵

2/
Hermes Agent at 9e01b44 scans the user prompt in tools/cronjob_prompt_scan.py. _scan_cron_prompt is what tools/cronjob_tools.py calls on create and on update. The strict ids include prompt_injection and exfil_curl_url. FIRE_CLAIM_TTL_SECONDS in cron/constants.py is 300. This claim is an Option, and the retry budget is one.

3/
M2 stays here. cron/lifecycle_guard.py check_gateway_lifecycle raises GatewayLifecycleBlocked. create_job in cron/jobs.py calls it. hermes gateway restart is blocked. hermes gateway start is allowed. I did not move that check to the chapter 39 gateway.

4/
The demo prints "cron: claimed". The bench id is 43/scan_thousand. The old ids 41/scan_thousand and 39/scan_thousand are aliases. The mean is @@MEAN:43/scan_thousand@@ (`mean.point_estimate`). Scanning a thousand due jobs does not tick the watchdog.

[personal note]

5/
Next is chapter 44, the editor: one ACP update becomes one inbox submit.

Read Chapter 43: [link]

#rustlang #AIagents

## LinkedIn
Chapter 43 is out: Cron.

The harness is the chapter 41 tree copied forward, plus a cron module. There is no path dependency. A job is claimed by one owner. crash releases that claim once. The next crash keeps the new owner. run_job scans the prompt, then the lifecycle text, then the unattended root-rm shape, and only then claims.

Hermes Agent at 9e01b44 does the prompt scan in tools/cronjob_prompt_scan.py, and tools/cronjob_tools.py calls _scan_cron_prompt on create and update. The lifecycle guard is cron/lifecycle_guard.py check_gateway_lifecycle, reached from cron/jobs.py create_job. hermes gateway restart, stop, and uninstall are blocked. start is allowed. systemctl against the gateway unit is blocked. That cron check stays in this chapter. Chapter 39 keeps the shell form of the same idea.

The demo prints "cron: claimed". The bench id is 43/scan_thousand. Aliases: 41/scan_thousand and 39/scan_thousand.

[personal note]

Next: chapter 44, the editor. Read Chapter 43: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
