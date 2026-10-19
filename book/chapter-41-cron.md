# Chapter 41: Cron

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 40 routes a live call. A cron job is not a live call. It is due, or it is not, and two processes must not both run it. This chapter claims a job for one owner. A crash may release that claim once. The second crash leaves it held. The prompt is scanned before the claim. A job that would restart or stop the gateway is blocked. Scanning a thousand due jobs does not tick the watchdog and does not chain the next job.

The step crate is `steps/ch41-cron`. The book number is 39. The package is `ch41-cron`.

| Section | Step | What it adds |
|---|---|---|
| 41 | `steps/ch41-cron` | one claim, one crash retry, a prompt scan, the lifecycle guard |

> **How Hermes does it**
>
> I read `cron/jobs.py` (`claim_job_for_fire`, `advance_next_run`), `tools/cronjob_prompt_scan.py` (`_CRON_THREAT_PATTERNS`, `_CRON_EXFIL_COMMAND_PATTERNS`, `_scan_cron_prompt`), and `cron/lifecycle_guard.py` (`GatewayLifecycleBlocked`, `_GATEWAY_LIFECYCLE_PATTERN`, `check_gateway_lifecycle`) on Hermes `main` at `e31c674f`.
>
> `claim_job_for_fire` is a file-lock claim. One caller wins. A younger claim blocks the others until the TTL. `advance_next_run` moves a recurring job's `next_run_at` before the run, so a crash misses that slot instead of firing it again. The strict prompt scan matches injection, a hidden instruction, `cat` of a secret file, `authorized_keys`, sudoers, `rm -rf /`, and curl exfil of a secret variable. `check_gateway_lifecycle` blocks `hermes gateway restart`, `stop`, and `uninstall`. `start` is allowed. `systemctl restart`, `stop`, and `start` of a hermes-gateway unit are blocked. A path like `/docs/hermes gateway restart-notes.md` is not a command.
>
> **Adopted.** One winner for the claim. The strict patterns, case insensitive, including one curl-exfil shape. `hermes gateway start` is allowed. `systemctl` against the gateway unit is not. A path prefix does not count as the command. The scan does not run the watchdog.
>
> **Adapted.** The claim is an `Option<u32>`, not a file lock. The book budget is one crash retry, then the claim stays held. Hermes recurring jobs skip the slot instead. `unattended_denies` is the cron call site for the chapter 11 deny, and the only shape it adds here is `rm -rf /`. `FakeModel` runs after a clean claim. The bench scans 1,000 due jobs and does not compile a regex inside the sample.
>
> **Deferred.** The jobs file, the claim TTL, invisible-unicode stripping, the GitHub `api.github.com` curl exemption, and `launchctl`. The full dangerous-command list stays in chapter 11.

[personal note: a job that was due, and a second process that did not get to run it]

## What this chapter decides

`claim` sets the owner when the job is unclaimed, not done, and not blocked. A second `claim` returns false.

`crash` clears the owner and sets the retry count to 1. A later `crash` returns false and leaves the owner in place. The job is not due while a claim is held.

`scan_due` returns the indexes whose `due` is at or before `now` and that are not claimed, done, or blocked. It does not increment `watchdog_runs` or `chained`. `watchdog` and `chain` are separate methods. `chain` pushes the first job id onto an inbox.

`scan_prompt` returns the first pattern id, or `None`. `run_job` scans, then checks the lifecycle regexes, then `unattended_denies`, and only then claims. A blocked job is not due.

`check_lifecycle` returns `hermes_gateway`, `systemctl`, or `kill_gateway`. `hermes gateway start` and a path that merely contains the words are `Ok`.

`demo` claims `daily` with the prompt `summarize the inbox` and returns `cron: claimed`. No key.

## Rust: the due scan is a filter

`scan_due` walks the vec once and collects indexes. The bench builds 1,000 jobs before the sample. Each prompt is the word `summarize`. Each `due` is 0. The timed call is `scan_due(0)`, and the result vec of 1,000 indexes is inside the sample. The regexes are `OnceLock` and are not touched by this function.

## Tests

Six tests. Owner 1 claims, owner 2 loses, a crash lets owner 2 claim, and a second crash does not release it. Of three jobs, only the due unclaimed one is indexed, and the watchdog and chain counters stay 0 until those methods are called. A thousand due jobs return a thousand indexes. `ignore previous instructions` and a curl of `$API_KEY` match. `rm -rf /` blocks before any claim. `hermes gateway restart` and `systemctl stop hermes-gateway` and `pkill hermes gateway` are blocked. `hermes gateway start` and `/docs/hermes gateway restart-notes.md` are not. `echo hi` is not an unattended deny. The demo is `cron: claimed`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch41-cron
```

```text
     Summary [   0.027s] 6 tests run: 6 passed, 0 skipped
```

```bash
cargo run -p ch41-cron
```

```text
cron: claimed
```

The binary claims one clean job. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `scan_thousand` | 1.607 µs | scan 1,000 due jobs under 10 ms | indexes of 1,000 unclaimed jobs |

`scan_thousand` builds the store and the 1,000 jobs before the bench. The timed closure is `b.iter`. Inside: `scan_due` of timestamp `0` passed through `black_box`, then `black_box` of the index vec. The walk and the `Vec` allocation of 1,000 indexes are inside the sample. Building the jobs is excluded. `scan_prompt`, `check_lifecycle`, `watchdog`, `chain`, and `FakeModel` are not called. No sleep runs.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38989567446](https://github.com/agnathan/rust-agent-harness/actions/runs/38989567446), section `39`. The flags were `--warm-up-time 1 --measurement-time 2 --sample-size 10`.

### Milestone demo

The `cargo run` above. The line is `cron: claimed`.

## Pitfalls

The bench is the index walk. A number that includes the prompt regex is the threat scan. That scan is the test that takes the longer fraction of the nextest second, and it is still not this sample.

One crash retry is the budget in this book. Calling `crash` twice does not mean two retries. The second call keeps the claim.

`scan_due` on a store whose jobs are all claimed returns an empty vec. That is a correct scan, and it is not the bench.

A prose sentence that says `hermes gateway restart` as a command still matches. The path test is the one that must stay quiet, because the character before `hermes` is `/`.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Prompt scan (M1).** The user prompt is matched before the claim. The ids include `prompt_injection`, `deception_hide`, `sys_prompt_override`, `disregard_rules`, `read_secrets`, `ssh_backdoor`, `sudoers_mod`, `destructive_root_rm`, and `exfil_curl_url`.

**Lifecycle (M2).** `hermes gateway restart`, `stop`, and `uninstall` are blocked. `start` is not. `systemctl` restart, stop, or start of the gateway unit is blocked. `pkill` of hermes and gateway is blocked. A blocked job is not claimed.

**Unattended deny (A4).** Chapter 11 wrote the approval pipeline. This chapter's cron path denies `rm -rf /` before a run. The rest of that list is not copied here.

**Left for later.** The jobs file, invisible unicode, and the GitHub curl exemption. E1 and E2 are still absent.

## Chapter summary

- **Claim.** One owner. The other loses.
- **Crash.** One release. The next crash holds.
- **Scan.** The prompt, then the lifecycle, then the claim.
- **Off the path.** The watchdog and the chained inbox push are separate calls.
- **Model.** `FakeModel` answers `ok`. The demo needs no key.
- **Time.** One thousand due jobs measured **1.607 µs**.

[personal note: a due list that was an index, and a job the scanner refused to claim]

## What's next

Chapter 42 is the editor. ACP is another client of the session inbox. A cancelled turn and a finished turn are different stop reasons, and the protocol bytes stay on their own stream.
