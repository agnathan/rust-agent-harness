# Social posts: Chapter 32, "The Critic"

## X thread
1/
Chapter 32 is up: The Critic.

The critic takes one permit from the supervisor pool, locks the notes, and does not push. accepted is true only for Verdict::Done. A missing verdict word is Reject. 🧵

2/
Hermes Agent's judge lives in hermes_cli/goals.py. DEFAULT_MAX_TURNS is 20. DEFAULT_JUDGE_TIMEOUT is 30.0. _JUDGE_RESPONSE_SNIPPET_CHARS is 4000. JUDGE_SYSTEM_PROMPT is the prompt. DEFAULT_GATE_TIMEOUT_SECONDS is 300. DEFAULT_GATE_MAX_RETRIES is 3. The parser is _parse_judge_response.

3/
This parser scans the reply for the word verdict. as_tool and final_gate both call judge. The 30 second timeout is not slept. The continuation loop, the wait target, and the quality-gate shell commands wait until chapter 33.

4/
The mean for judge_one_permit is MEAN_32_judge_one_permit, mean.point_estimate from run BENCH_RUN_URL. The demo prints "accepted: done". No key.

[personal note]

5/
Next is composition: fan-out under the same pool, a handoff when ownership moves, and the loop around this critic.

Read Chapter 32: [link]

#rustlang #AIagents

## LinkedIn
Chapter 32 is out: The Critic.

A finished reply still needs a verdict. Hermes Agent caps a goal at 20 turns, times the judge out at 30.0 seconds, and keeps a 4,000 character snippet. I stored 20 and 30, cut the snippet at 4,000 bytes on a character boundary, and scanned the reply for the word verdict. Done is the only accept. The critic takes one permit, locks the shared notes, and does not push. It does not sleep the 30 seconds.

The mean for judge_one_permit is MEAN_32_judge_one_permit.

[personal note]

Next: composing the supervisor, the fan-out, the handoff, and this critic. Read Chapter 32: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
