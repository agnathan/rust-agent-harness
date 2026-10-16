# Chapter 11: Approvals and Guardrails

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 10 will not start a tool. It will not refuse one either. A turn can sit in `Dispatching` and the loop from chapter 9 will run whatever name it repaired. This chapter is the check that runs first: a command is either allowed, denied, or handed to one human prompt. The prompt is the only place concurrent calls wait, and that wait does not hold a tool-concurrency permit.

The step crate is `steps/ch09-approvals`. The directory name stays `ch09-approvals` so it matches the CI filter. The book number is 11. The package is `ch09-approvals`.

| Section | Step | What it adds |
|---|---|---|
| 11 | `steps/ch09-approvals` | `Risk`, the approval pipeline, one prompt, a fake model |

> **How Hermes does it**
>
> I read these files on Hermes `main` at `cf71b1aa`: `tools/approval.py` (`check_all_command_guards`, `_floor_block`, `_STOP_COMMAND`, `_STOP_ACTION`, the denial-breaker addendum), `tools/approval_floors.py` (`_hardline_block_result`), `tools/approval_detection.py` (`HARDLINE_PATTERNS`, `detect_hardline_command`, `detect_dangerous_command`), and `tools/approval_context.py` (`_get_approval_timeout`, `_binary_approval_mode`).
>
> The floor runs before YOLO, `approvals.mode: "off"`, and cron approve-mode. `_floor_block` is hardline, then a runtime self-delete, then `sudo -S`, then the user's `approvals.deny` list. After the floor, YOLO or mode `off` approves, then the permanent allowlist, then an unattended context, then `detect_dangerous_command`, then smart mode, then the human. `detect_hardline_command` says the hardline table is never bypassable, even in YOLO. The timeout default is 300 seconds and a miss denies. Cron, `-q`, and unattended mode default to deny. A denial tells the model the user has not consented and must not retry or rephrase. Three consecutive denials append a circuit-breaker sentence.
>
> **Adopted.** That order. Hardline text that names `--yolo` and `approvals.mode=off`. The stop sentence from `_STOP_COMMAND`. The breaker threshold of 3. Timeout copy that includes "Silence is not consent." Unattended deny as the default. A fake model that stops the script on the first denial, so the next call is not issued.
>
> **Adapted.** The pattern table is a short set of the commands I read in `HARDLINE_PATTERNS` (root `rm -rf`, `mkfs`, `dd` onto a raw disk, a fork bomb, `kill -1`, shutdown). Hermes's `DANGEROUS_PATTERNS` list is much longer; this chapter treats recursive `rm`, a write under `/etc` or `.ssh` or `.env`, a `curl` upload, and an interpreter `-c`/`-e` as the dangerous class. Quote masking is a character scan, not Hermes's quote-and-heredoc rewriter. A safe command with an empty deny list returns before that scan allocates. The smart step is `fake_smart_review`, not an auxiliary model. The human wait is one `tokio::sync::Mutex`.
>
> **Deferred.** Deleting the running interpreter (`agent/runtime_self_protection.py`, B6), the gateway lifecycle block (B4), and the self-repo git guard (B5) stay out. Persisting "always" to a file is A7 and is not this chapter. A plugin that escalates to the human gate is chapter 12 (A8). The whole-script `execute_code` gate is chapter 13 (A2).

[personal note: a command you expected the allowlist to pass, and which floor still refused it]

## What this chapter decides

`Risk` is `Safe`, `Sensitive`, or `Destructive`. It is a label. The pipeline is the decision.

`check` walks one command:

| Step | When it fires | Result |
|---|---|---|
| Fast skip | Empty deny list, no shell metacharacters, none of the command words | `Allow` |
| Hardline | Root delete, `mkfs`, raw `dd`, fork bomb, `kill -1`, shutdown | `Deny`, and YOLO does not help |
| `sudo -S` | The command pipes a password to sudo | `Deny` |
| Deny list | A configured substring matches the masked command | `Deny`, before YOLO |
| YOLO or mode off | The floor did not fire | `Allow` |
| Allowlist | The normalized command is listed | `Allow` |
| Safe | Nothing dangerous left | `Allow` |
| Unattended | Cron, single-query, or unattended, and the mode is deny | `Deny` |
| Smart | `fake_smart_review` returns deny or approve | `Deny` or `Allow` |
| Ask | Manual mode, or smart escalates | `Ask` |

`run_script` is the fake model. It is a list of `ScriptCall` values. The first `Deny` ends the list. The demo is `rm -rf /` followed by `echo hello`. The second call never runs.

A denial message always contains the stop sentence. The third denial in a session also contains `CIRCUIT BREAKER`. An approval resets the tally. Hardline denials count too.

The human path is `run_call`. It acquires a permit from a semaphore of 8, which is the same bound chapter 3 uses. If the verdict is `Ask`, it drops that permit, then locks the prompt. The test records how many permits were free at the start of the wait, and the peak number of callers inside the lock. The peak is 1. A silent prompt waits for `Policy.timeout` and denies. The default timeout is 300 seconds. The test uses 30 milliseconds so the suite does not sit there.

## Rust: a mutex you can hold across `.await`

`std::sync::Mutex` cannot be held across an `.await`. The guard is not `Send`, and the runtime may move the task between threads while it waits. `tokio::sync::Mutex` is the lock that is allowed to stay held while the task is paused. This chapter uses that one for the prompt, and a `tokio::sync::Semaphore` for the tool permits. The permit is an owned guard. Dropping it, which is what `drop(permit)` does before the wait, returns the slot. The wait then cannot pin a tool worker.

`Risk` is an enum so a match in `check` has to name every level. A string `"high"` would let a typo through until a command hit it.

## Tests

Fourteen tests. Hardline still denies under YOLO and mode off. `echo "rm -rf /"` is allowed. `bash -c 'rm -rf /'` is not. `sudo -S` is a floor under YOLO. A user deny rule matches before YOLO. An allowlist entry skips the human. Cron denies `rm -rf ./build` and still allows `echo hello`. The default timeout is 300 seconds and the unattended modes are deny. Smart mode denies a `curl` upload and escalates `rm`. The third denial adds the breaker. The script stops. Two asks share one prompt and both saw a free permit. A silent prompt denies in well under two seconds and the permit is back.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch09-approvals
```

```text
Starting 14 tests across 3 binaries
     Summary [   0.036s] 14 tests run: 14 passed, 0 skipped
```

```bash
cargo run -p ch09-approvals
```

```text
verdict: deny
stopped: 1
```

The binary runs the two-call script. No key. `stopped: 1` is the fake model refusing to try `echo hello` after the hardline denial.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `skip_decision` | CI_MEAN_11_SKIP | under 1 µs | `check` of `echo hello` |

`skip_decision` calls `skip_decision("echo hello")`, which is `check` on `Policy::manual()` with a fresh session. The string is passed through `black_box`. The timed closure is `b.iter`. Building the policy is inside the call, not a separate setup step. There is no runtime, no sleep, and no prompt. The fast path scans the bytes and returns `Allow`. It does not allocate a normalized copy.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, CI_RUN_11, section `11`.

### Milestone demo

The `cargo run` above. The verdict is `deny`.

## Pitfalls

The fast path only runs when the deny list is empty. A deny rule of `echo` would be skipped if the fast path ignored the list. It does not. It falls through to the scan.

Quote masking deletes the inside of quotes before the hardline check. That is what lets `echo "rm -rf /"` through. `bash -c 'rm -rf /'` is a second variant: the payload is scanned raw. Forgetting that second variant is how a quoted shell one-liner escapes the floor.

`run_call` drops the permit only on `Ask`. An `Allow` or a `Deny` holds the permit until the function returns. That is the tool body, which this function does not run. A caller that then does its own long wait must not keep the permit either. The test that joins two asks is the one that checks the drop.

The breaker reads the tally after this denial has been counted. Threshold 3 means the third message carries the sentence, not the fourth.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L5.

**A1, the pipeline.** `check` is the single entry. The order is the floor, then YOLO or mode off, then the allowlist, then unattended mode, then smart, then ask.

**B1, hardline.** `rm -rf /`, `mkfs`, raw `dd`, the fork bomb, `kill -1`, and shutdown are denied with `hardline: true`. YOLO, mode off, and cron approve-mode do not move that branch. The message says so.

**B2, dangerous commands.** Recursive `rm`, a sensitive path, a `curl` upload, and inline interpreter code are the dangerous class. They are normalized (`sudo`, `nohup`, `env NAME=value`) before the match. They are not hardline. An interactive session asks. Cron denies them.

**B3, floors that are not the hardline table.** `sudo -S` and `approvals.deny` run in the floor, before YOLO. A deny-list hit is not marked hardline. It still cannot be YOLO'd past.

**A4.** `Presence::Cron`, `SingleQuery`, and `Unattended` read `cron_mode`, `single_query_mode`, and `unattended_mode`. All three default to deny. A safe command still runs. A dangerous one does not wait for a human who is not there.

**A5.** `DEFAULT_TIMEOUT` is 300 seconds. The silent prompt denies. The message includes the stop sentence and "Silence is not consent."

**A6.** The denial text is `_STOP_COMMAND`: the user has not consented, and the model must not retry or rephrase. `run_script` drops the rest of the calls. The breaker sentence is appended at `DENIAL_BREAKER_THRESHOLD`, which is 3.

**Left for later.** A8, a plugin escalating into this prompt, is chapter 12. A2, the `execute_code` script gate, is chapter 13. A9, memory and skill writes, is chapter 23. Secret redaction (E1, E2) is still absent. This chapter does not log the command.

## Chapter summary

- **Pipeline.** Hardline and the other floors first. Then YOLO, the allowlist, unattended deny, smart, and one ask.
- **Prompt.** Two dangerous calls share one mutex. The tool permit is back in the semaphore before either wait starts.
- **Fake model.** `rm -rf /` then `echo hello` produces one verdict, `deny`.
- **Time.** The skip of `echo hello` measured **CI_MEAN_11_SKIP**. The target is under 1 µs.

[personal note: the first command you put on the allowlist, and whether the floor still caught a cousin of it]

## What's next

Chapter 12 is hooks. A `pre_tool_call` hook can block, rewrite arguments, or escalate into the prompt this chapter just added. The availability check stays in the registry from chapter 3.
