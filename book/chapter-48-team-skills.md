# Chapter 48: Shared Team Skills

The workers can already talk. A skill they want to keep is a different kind of message.

On the bus from chapter 47, a peer can publish without knowing the subscriber. A skill proposal is a different path: `skill_propose` pushes a `Proposal` onto a `VecDeque` capped at 64, and it doesn't call `publish`. The librarian, the function `librarian_accept`, is the only writer. Workers don't patch the team store. This queue is the team half of a continuously improving harness. A skill becomes shared only when one librarian accepts the proposal, and the workers still leave the index alone.

The crate is `steps/ch48-team-skills`, a copy of chapter 47 plus `team_skills`, and the package is `ch48-team-skills`. A `Library` holds an `Arc` index that `swap_index` replaces, and the rendered rows are bundled or team. Workers return from `skill_propose` without waiting. The librarian calls chapter 25's scanner, and community plus `force` doesn't install a dangerous body.

The index is an `Arc`, and swapping it is one store the readers clone. The proposal queue is that `VecDeque`, rather than a channel. The scanner call is an ordinary function call into `skill_write`, which is on this crate because the harness was copied forward. `Result` is the reject, with the dangerous verdict intact.

Running `cargo run -p ch48-team-skills --bin ch48-team-skills` runs the librarian accept and prints `skills: v2 3`. The exercise is `my-harness/tests/ch48.rs`, where `skill_propose` enqueues and workers don't wait.

| Section | What it walks |
|---|---|
| 48.1 | `PROPOSAL_CAP`, `Layer`, `Trust`, `Skill`, `Index` |
| 48.2 | `Proposal`, `Propose`, `Reject`, `Notice` |
| 48.3 | `Library`, `load`, `notices`, `render_team` |
| 48.4 | `skill_propose` |
| 48.5 | `allow_install`, `is_dangerous`, and chapter 25's scanner |
| 48.6 | `swap_index` |
| 48.7 | `FakeModel`, `librarian_accept`, and `demo` |
| 48.8 | the manifest, the install policy, and this heading |

> **How Hermes does it**
>
> I read `tools/skills_sync.py` (the module docstring and `MANIFEST_FILE`), `tools/skills_guard.py` at `should_allow_install` (line 818), and `tools/write_approval.py` at `stage_write` (line 74) on Hermes `main` at `9e01b44`. I checked those lines. I also searched that tree for the string `TEAM.md`. I did not find it. I did not find `bootstrap_pipeline.py`. I did not re-open a file that writes that heading.
>
> The `skills_sync.py` docstring says the sync copies repo `skills/` into `~/.hermes/skills/` and tracks each synced skill's origin hash in `.bundled_manifest`. The file constant is `MANIFEST_FILE = SKILLS_DIR / ".bundled_manifest"`. The docstring says new skills are copied and recorded. Existing skills update only when the bundled copy changed and the user copy still matches the origin hash. A user-customized copy is skipped. User-deleted skills are not re-added. Upstream-removed skills leave the manifest once no active or archived copy remains. That path is a manifest and a disk copy. It is not the string this chapter renders.
>
> `should_allow_install` returns `(allowed, reason)` from verdict plus trust. `force` overrides every block except a dangerous verdict on community or trusted sources. `allowed` is `None` when policy says ask. The blocked string is built as `Blocked ({trust} source + {verdict} verdict, {n} findings). ` and then, when the hard block holds, `"--force does not override a dangerous verdict."` The sentence ends with that clause.
>
> `stage_write` persists a pending write and returns its record. The module docstring names memory writes and skill writes. When the approval gate is on, the write is staged under `pending/` and not committed inline. `team_skills.rs` does not call `stage_write`. The install decision in this chapter is `allow_install`, which calls chapter 25's `should_allow_install`. I am not giving `stage_write` a job in this file.
>
> This harness renders the heading. `render_team` starts with `"# TEAM\n"`. Hermes's sync path is the manifest, not this string.
>
> **Adopted.** A community skill that is dangerous is refused, including when `force` is true. Workers do not write the index. Readers hold an `Arc`. A swap publishes a notice whose kind is `skills.updated`.
>
> **Adapted.** The proposal queue is a `VecDeque` with cap 64. The librarian runs `FakeModel` before the pointer swap. `allow_install` also calls chapter 25's `scan_text`. `team_skills::Trust::Builtin` is passed to that scanner as `skill_write::Trust::Trusted`. The queue is not the chapter 47 bus. `team_skills.rs` does not call `publish`.
>
> **Deferred.** The manifest hash, the disk copy, and the user-deleted skip. A git history of the library is not this file. `stage_write` is not this file.

> **Guardrails.** `force` does not install a dangerous community skill. An empty body is rejected. Workers load an `Arc` and do not take the write lock across the model. The full box is [Guardrails in this chapter](#guardrails-in-this-chapter).

## What this chapter decides

Calling `skill_propose` pushes a `Proposal` onto a `VecDeque` until `PROPOSAL_CAP`, which is 64. The next call returns `Full` and leaves the deque alone. The function skips the body scan and skips the library lock, so enqueue returns before the librarian runs. `skill_propose` never sees the `Library`, and `librarian_accept` never sees the `VecDeque`. The demo is the one function that uses both, and it pops the queue before it asks the librarian.

An install check in `allow_install` returns false for an empty body, and it returns false when `is_dangerous` is true and trust is `Community`, even with `force` set. It then calls `crate::skill_write::scan_text` and `crate::skill_write::should_allow_install`. `Builtin` maps to `skill_write::Trust::Trusted`, and `Community` maps to `skill_write::Trust::Community`. Only `Decision::Allow` becomes true. The chapter 25 scanner leaves `rm -rf /` unmarked, so that extra check stays in `is_dangerous`.

The only writer is `librarian_accept`. An empty body is `Reject::Empty`, and a failed `allow_install` is `Reject::Guard`. Both return before `FakeModel`. When the guard passes, `model.complete` runs, then the skill list is cloned, then `swap_index` replaces the `Arc`. The notice follows the swap, and the kind is `skills.updated`.

A swap is `mem::replace` of an `Arc<Index>` under a short mutex, and the caller builds `next` before taking the lock. Loading clones the `Arc` and drops the mutex, so a worker who loaded the old index still holds version 1 after the swap.

Rendering in `render_team` writes `"# TEAM\n"` and then one line per skill, `- name (bundled|team)\n`. The demo proposes `cite`, the librarian accepts it, and the line is `skills: v2 3`. The 3 is `team_md.lines().count()` after echo and cite are both in the list.

## How the loop closes

Chapter 24 showed one agent a stable index, chapter 25 let that agent write behind a scan, chapter 26 reflected, and chapter 27 curated one agent's skills, so each of those steps still belonged to one agent. Chapter 47 gave peers a bus, which means a worker can publish an outcome without knowing the subscriber and without waiting on a slow librarian.

Here workers propose and the librarian applies, the same one-writer rule at team scale. A proposal carries a name, a body, a force flag, and a trust. `skill_propose` pushes that `Proposal` onto a `VecDeque` capped at 64 and returns as soon as the value is queued, and the call never reaches `publish`, because the bus is how peers talk and a kept skill is a different kind of message. The function `librarian_accept` is the only writer. Workers patch no team store, so two patches can't tear the list, and a dangerous install can't depend on which worker remembered the scanner.

A child still doesn't write the team store. What it learned comes back in the result, and the parent's reflection, or the librarian, decides what to keep. The index swap is how a running agent hears about a new skill without a rewritten system prompt mid-turn. `load` returns an `Arc`, and `swap_index` replaces the pointer under a short mutex after the next index has already been built, so a worker who already holds an arc keeps the version it loaded. Chapter 2's cache rule still holds.

A continuously improving harness gets a new team skill only after the scanner and the single writer, so the loop can keep a procedure without letting every worker teach the others directly.

## 48.1 `PROPOSAL_CAP`, `Layer`, `Trust`, `Skill`, and `Index`

```rust
pub const PROPOSAL_CAP: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Bundled,
    Team,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trust {
    Builtin,
    Community,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub body: String,
    pub layer: Layer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
    pub skills: Vec<Skill>,
    pub version: u64,
    pub team_md: String,
}

impl Index {
    pub fn bundled(skills: Vec<Skill>) -> Self {
        let team_md = render_team(&skills);
        Self {
            skills,
            version: 1,
            team_md,
        }
    }
}
```

The proposal bound is `PROPOSAL_CAP`, 64, the same number as `BUS_CAP` and as `INBOX_BOUND`. It isn't an alias of either. It bounds the proposal deque, rather than a channel, and `skill_propose` is the only reader of the constant.

A layer is where a skill sits in the rendered list. `Bundled` spells `bundled`, and `Team` spells `team`. The variant is separate from trust. A community proposal that the librarian accepts is stored as `Layer::Team`, and the `Trust` on the proposal stays off the `Skill`.

Trust here has two variants, while `skill_write::Trust` has four: `Builtin`, `Trusted`, `Community`, and `AgentCreated`. This chapter's enum is the smaller one, and the mapping lives in `allow_install`, rather than in a `From` impl. A proposal can't be `AgentCreated` or `Trusted` in this enum. A builtin proposal becomes `Trusted` only at the call into chapter 25.

A skill owns its name and body, and `layer` is `Copy`. An index owns the vec, a `u64` version, and the rendered string, and all three fields are public, so a worker who has an `Arc<Index>` can read `version` and `team_md` without a method and without the mutex.

The bundled constructor always sets `version` to 1 and doesn't look at the layers passed in. A vec that already contains `Layer::Team` still comes back as version 1, while `render_team` still labels those rows `team`. The tests pass a bundled echo, or an empty vec, and `demo` passes echo. The rendered string is stored at construction so a later `load` can print the list without rebuilding it. `librarian_accept` builds the next index with a struct literal because the next version is `current.version + 1`, and `bundled` would force it back to 1. The constructor should have taken the version.

> **Rust sidebar: `Copy` on the two enums**
>
> `Layer` and `Trust` are `Copy`, so matching them leaves the proposal where it is. `allow_install` takes `&Proposal` and compares `proposal.trust == Trust::Community`, and the proposal is still the caller's after the function returns. `Skill` isn't `Copy`, because of the strings. Cloning a skill list clones every name and body, and that clone happens in `librarian_accept`, outside the mutex, after `load` has returned.

## 48.2 `Proposal`, `Propose`, `Reject`, and `Notice`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub name: String,
    pub body: String,
    pub force: bool,
    pub trust: Trust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propose {
    Queued,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reject {
    Guard,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    pub kind: &'static str,
    pub version: u64,
}
```

A proposal is what a worker enqueues. `force` is a `bool`, and `trust` is the small enum. The proposal carries no layer field, because the librarian decides the layer and always pushes `Layer::Team`.

The queue result is `Propose`. `Queued` means the proposal moved onto the deque, and `Full` means it stayed off. Neither variant carries the proposal, so on the `Full` path the proposal argument drops when the function returns. A caller who still needs the text has to clone it before the call. The bench clones inside the timed closure and keeps the original outside.

The librarian's error is `Reject`. `Empty` is a blank body, and `Guard` is `allow_install` returning false. `Reject` has no `Full` variant, because a full queue is the worker's problem, before the librarian is asked. The demo uses `expect("accept")`, so either reject would panic the demo rather than print a different line.

A notice has a `'static` kind and a version. The only kind this file writes is `"skills.updated"`. Calling `notices` returns a cloned `Vec`, so a reader releases the notices mutex before it looks at the list. `PartialEq` is why a test could compare a whole notice, and the queue test compares `kind` only.

## 48.3 `Library`, `load`, `notices`, and `render_team`

```rust
pub struct Library {
    index: Mutex<Arc<Index>>,
    notices: Mutex<Vec<Notice>>,
}

impl Library {
    pub fn new(index: Index) -> Self {
        Self {
            index: Mutex::new(Arc::new(index)),
            notices: Mutex::new(Vec::new()),
        }
    }

    pub fn load(&self) -> Arc<Index> {
        Arc::clone(&self.index.lock().expect("index"))
    }

    pub fn notices(&self) -> Vec<Notice> {
        self.notices.lock().expect("notices").clone()
    }
}

pub fn render_team(skills: &[Skill]) -> String {
    let mut text = String::from("# TEAM\n");
    for skill in skills {
        let layer = match skill.layer {
            Layer::Bundled => "bundled",
            Layer::Team => "team",
        };
        text.push_str(&format!("- {} ({layer})\n", skill.name));
    }
    text
}
```

Construction wraps the index in an `Arc` and a `Mutex`, and starts notices as an empty vec. The mutexes are separate, so a notices reader takes only the notices lock, and an index reader takes only the index lock. `librarian_accept` takes them one after the other, rather than nested. `swap_index` releases the index lock when `slot` drops, and only then does the function lock notices.

Loading locks, clones the `Arc`, and unlocks when the guard drops at the end of the statement. `Arc::clone` bumps the strong count and leaves the `Index` shared until a writer builds a new `Arc`. `expect("index")` is the panic if the mutex is poisoned. This file has no `catch_unwind`, so a poison panics the caller.

The notices getter clones the vec under the lock and returns the clone, and the lock ends before the caller sees the vec. An empty library has an empty vec. After one accept, the vec has one element.

Rendering starts with the exact string `"# TEAM\n"`. The match has two arms, so a new `Layer` would fail to compile until this function learned a spelling. The format is `- {name} ({layer})\n`, and `{layer}` is the string from the match, rather than the `Debug` of the enum. The spelling on the line is `bundled`, from the match arm, rather than the `Debug` form `Bundled`. For echo bundled and cite team, the string is:

```text
# TEAM
- echo (bundled)
- cite (team)
```

Counting with `lines()` stops on the final newline and yields no trailing empty line after it. `lines().count()` on that string is 3, which is the demo's second number. An index with only echo is two lines: the heading and `- echo (bundled)`. `Index::bundled` of an empty vec is one line, `# TEAM`, because the loop pushes nothing. The bench's prebuilt string is `"# TEAM\n- cite (team)\n"`. `lines().count()` on that string is 2. `benchmarks/descriptions.json` calls the same value a three-line `TEAM.md` in the `what` field, and the `inputs` field quotes the two-line string with its trailing newline. The bench source is the string. The demo's 3 is echo plus cite plus the heading.

I didn't find a Hermes file that writes `# TEAM`. The heading in this crate is `String::from("# TEAM\n")`.

> **Rust sidebar: the lock ends inside `load`**
>
> `index.lock()` returns a `MutexGuard`, and `Arc::clone` only needs a reference to the `Arc` inside it. The guard is a temporary, and it drops at the semicolon, so the returned `Arc` is the index with the guard already gone. A worker can read `held.version` for as long as they want, and `librarian_accept` can lock the same mutex again. Holding `held` leaves the swap free, which is the point of cloning the `Arc` instead of returning the guard.

## 48.4 `skill_propose`

```rust
pub fn skill_propose(queue: &mut VecDeque<Proposal>, proposal: Proposal) -> Propose {
    if queue.len() >= PROPOSAL_CAP {
        return Propose::Full;
    }
    queue.push_back(proposal);
    Propose::Queued
}
```

The queue is the caller's. This function sees neither a `Library`, nor a `FakeModel`, nor `allow_install`, so a dangerous body still queues and an empty body still queues. `Full` is the only refusal, and it is about length. `push_back` adds at the tail, `demo` reads with `pop_front`, and a queue of proposals stays in arrival order.

At length 64 the next call returns `Full` and leaves the deque alone. The 65th proposal drops with the function argument, and `len` stays 64. The test `the_queue_holds_64_and_the_librarian_swaps_an_arc` loops `0..PROPOSAL_CAP` with names `s0` through `s63`, body `"ok"`, `force: false`, `Trust::Builtin`, and expects `Queued` each time. The next call passes `cite()` and expects `Full`. `cite()` is the helper at the top of the test file: name `cite`, body `Name the file you read.`, `force` false, `Trust::Community`. That proposal stays off the deque after `Full`.

The bench starts from `VecDeque::new`, so the result is `Queued`, the `Full` path stays cold, and `librarian_accept` stays out of the sample. A worker who only runs `skill_propose` returns before any model runs, because the model lives in `librarian_accept`. The milestone `chapter_48_propose_waits()` wants `false`, which is that return.

> **Rust sidebar: `VecDeque` and a moved proposal**
>
> `Proposal` isn't `Copy`. `skill_propose` takes it by value, and `push_back` moves it onto the deque. The `Full` return happens before the move onto the deque, so the value drops as the parameter goes out of scope. The constructor in the bench is `VecDeque::new`, which grows as it is pushed, and `with_capacity` stays unused. The cap is a length check, rather than an allocation limit, so the buffer may be larger than 64. The guard is `len`, the same shape as chapter 44's inbox.

## 48.5 `allow_install`, `is_dangerous`, and chapter 25's scanner

```rust
/// Dangerous community text is refused, including when `force` is set.
/// The pattern scan is chapter 25's `skill_write` scanner. `rm -rf /` is
/// this chapter's extra, because that scanner does not name the command.
pub fn allow_install(proposal: &Proposal) -> bool {
    if proposal.body.trim().is_empty() {
        return false;
    }
    let dangerous = is_dangerous(&proposal.body);
    if dangerous && proposal.trust == Trust::Community {
        return false;
    }
    let scan = crate::skill_write::scan_text(&proposal.body);
    let trust = match proposal.trust {
        Trust::Builtin => crate::skill_write::Trust::Trusted,
        Trust::Community => crate::skill_write::Trust::Community,
    };
    matches!(
        crate::skill_write::should_allow_install(&scan, trust, proposal.force),
        crate::skill_write::Decision::Allow
    )
}

pub fn is_dangerous(body: &str) -> bool {
    let lower = body.to_ascii_lowercase();
    lower.contains("rm -rf /") || lower.contains("ignore previous instructions")
}
```

The doc comment is the rule. Dangerous community text is refused, including when `force` is set. The pattern scan is chapter 25, and `rm -rf /` is this chapter's extra, because that scanner leaves the command unnamed. The ids in `rules()` are `env_exfil_curl`, `prompt_injection_ignore`, `disregard_rules`, `ssh_dir_access`, and `role_hijack`. None of the patterns is `rm -rf /`.

Empty is first. `trim` makes a body of spaces empty, and the function returns false before it reaches `is_dangerous` or the scanner, with `force` left unread. `librarian_accept` also checks `trim` and returns `Reject::Empty` before it calls `allow_install`, so a blank proposal fails twice if you called both. A worker can still enqueue a blank body, because `skill_propose` only checks length, and the librarian is who rejects it.

The local check lowercases with `to_ascii_lowercase` and looks for two substrings. `RM -RF /` matches, and `Ignore Previous Instructions` matches this function, because the check is on the lowered string. The chapter 25 injection regex is case-sensitive, so it can miss a capitalized phrase that this function already caught. For `Trust::Community`, that difference stops at the next line: if `is_dangerous` is true, `allow_install` returns false immediately, and `proposal.force` stays unread. The test body is `then rm -rf /` with `force: true` and `Trust::Community`. `allow_install` is false, and the scanner stays out of it. If this early return were gone, `scan_text` would see no finding, the verdict would be `Safe`, and `policy` for community plus safe is `Allow`. The local check is what refuses the command, and the comment in the file says so.

For `Trust::Builtin`, the community clause stays quiet, so `is_dangerous` can be true and the function still falls through. That is the builtin test. The body is `document rm -rf / as a thing not to run`, `force` false, `Trust::Builtin`. `allow_install` is true. The scanner leaves the command unmarked, so the verdict is `Safe`, and the trust mapped into chapter 25 is `Trusted`, rather than `Builtin`. `policy` for `Trusted` and `Safe` is `Allow`.

The mapping is the whole `match`:

```rust
let trust = match proposal.trust {
    Trust::Builtin => crate::skill_write::Trust::Trusted,
    Trust::Community => crate::skill_write::Trust::Community,
};
```

Passing `skill_write::Trust::Builtin` would allow every verdict, including `Dangerous`. This chapter passes that variant nowhere, and a builtin proposal is judged as trusted. A dangerous verdict on a trusted source is a hard block inside `should_allow_install`. A dangerous builtin can still be allowed when the scanner calls the body safe, and `rm -rf /` is that case. A body of `ignore previous instructions` takes the other path, because the scanner marks it critical. The scanner rule is:

```rust
(
    "prompt_injection_ignore",
    "critical",
    r"ignore\s+(?:\w+\s+)*(previous|all|above|prior)\s+instructions",
),
```

A critical finding makes `scan_text` return `Verdict::Dangerous`. `should_allow_install` then does this:

```rust
pub fn should_allow_install(result: &ScanResult, trust: Trust, force: bool) -> Decision {
    let decision = policy(trust, result.verdict);
    let hard_block =
        result.verdict == Verdict::Dangerous && matches!(trust, Trust::Community | Trust::Trusted);
    if decision == Decision::Allow {
        return Decision::Allow;
    }
    if force && !hard_block {
        return Decision::Allow;
    }
    decision
}
```

And `policy`:

```rust
fn policy(trust: Trust, verdict: Verdict) -> Decision {
    match (trust, verdict) {
        (Trust::Builtin, _) => Decision::Allow,
        (Trust::Trusted, Verdict::Dangerous) => Decision::Block,
        (Trust::Trusted, _) => Decision::Allow,
        (Trust::Community, Verdict::Safe) => Decision::Allow,
        (Trust::Community, _) => Decision::Block,
        (Trust::AgentCreated, Verdict::Dangerous) => Decision::Ask,
        (Trust::AgentCreated, _) => Decision::Allow,
    }
}
```

Community plus `Dangerous` is `Block`. `hard_block` is true for community and for trusted, so the force override stays closed and the function returns `Block`. `allow_install` wants `Decision::Allow`, so it returns false, and community plus `force` doesn't install a dangerous body. Community plus a block that isn't dangerous can still be overridden: `Caution` is `Block` from `policy`, and `hard_block` is false, so `force` returns `Allow`. The local `is_dangerous` check is stricter than that for its two phrases, because it returns false before `force` is visible.

An ask decision fails the `Allow` match, since `Decision::Ask` is its own variant. An agent-created dangerous skill would ask, and `allow_install` would return false. This chapter's `Trust` can't express `AgentCreated`, so that arm stays unreachable from `allow_install`, and it is still what chapter 25 does.

Hermes's function at line 818 returns a reason string this crate never builds:

```python
def should_allow_install(result: ScanResult, force: bool = False) -> tuple[bool, str]:
    """``(allowed, reason)`` from verdict + trust; *force* overrides every block except a dangerous verdict on
    community/trusted sources. ``allowed`` is None when policy says "ask"."""
    decision = INSTALL_POLICY.get(result.trust_level, INSTALL_POLICY["community"])[VERDICT_INDEX.get(result.verdict, 2)]
    n = len(result.findings)
    hard_block = result.verdict == "dangerous" and result.trust_level in ("community", "trusted")
    if decision == "allow":
        return True, f"Allowed ({result.trust_level} source, {result.verdict} verdict)"
    if force and not hard_block:
        return True, f"Force-installed despite {result.verdict} verdict ({n} findings)"
    if decision == "ask":
        return None, f"Requires confirmation ({result.trust_level} source + {result.verdict} verdict, {n} findings)"
    blocked = f"Blocked ({result.trust_level} source + {result.verdict} verdict, {n} findings). "
    return False, blocked + ("--force does not override a dangerous verdict." if hard_block else "Use --force to override.")
```

The blocked string ends `--force does not override a dangerous verdict.` when `hard_block` is true. Otherwise it ends `Use --force to override.` `skill_write::block_reason` in this crate builds the same dangerous tail. `allow_install` leaves `block_reason` uncalled and keeps only the bool. `librarian_accept` turns a false into `Reject::Guard` and stores the sentence nowhere.

> **Rust sidebar: `matches!` on `Decision::Allow`**
>
> `matches!` is true only for `Allow`, so `Block` and `Ask` are false. The function's return type is `bool`, which means the caller can't tell an empty body from a community `rm -rf /` from a scanner block. `librarian_accept` can tell empty from the rest, because it checks `trim` itself and returns `Reject::Empty` first. Every other false is `Guard`.

## 48.6 `swap_index`

```rust
/// Swap the index pointer. The caller builds `next` before taking the lock.
pub fn swap_index(library: &Library, next: Arc<Index>) -> Arc<Index> {
    let mut slot = library.index.lock().expect("index");
    std::mem::replace(&mut *slot, next)
}
```

The doc comment says the caller builds `next` before taking the lock, and `librarian_accept` does. It clones the skill vec, pushes the new skill, renders the heading, and allocates the `Arc` before `swap_index`. The timed bench does too. Its setup closure builds the `Library` and the next `Arc`, and the timed call is only the replace.

The lock poisons with `expect("index")`. `mem::replace` writes `next` into the mutex and returns the previous `Arc`, and the guard drops at the end of the function, which is immediately after the replace. The critical section is the pointer swap. It leaves out the scan, the model, and `render_team`.

The returned `Arc` is the old index. Callers who ignore it drop one strong count, and `librarian_accept` ignores it. The test `a_worker_reads_without_the_librarian` keeps it. That test loads `seen`, builds a new `Arc` at version 9 with the same skills and the same `team_md`, and swaps. `previous.version` is 1, `seen.version` is still 1, and `library.load().version` is 9. There is no `FakeModel` and no `librarian_accept` in that test. The worker's `Arc` and the library's `Arc` are different allocations. The other test asserts `Arc::as_ptr` of the held index differs from `Arc::as_ptr` of the index loaded after accept.

> **Rust sidebar: `mem::replace` and `Arc`**
>
> `*slot` is the `Arc<Index>` inside the mutex. `replace` needs a new value to put down, because the old `Arc` moves out. `std::mem::swap` would need a second `Arc` already stored in a local that ends up holding the old one, and `replace` returns it instead. `Arc::new` happened before the lock. Workers who cloned the old `Arc` keep a strong count, so the old `Index` stays alive for them after the mutex points at the new one. Dropping the last old `Arc` drops the old vec, and this file doesn't wait for that drop.

## 48.7 `FakeModel`, `librarian_accept`, and `demo`

```rust
pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

/// The only writer. The model runs before the pointer swap.
pub fn librarian_accept(
    library: &Library,
    proposal: &Proposal,
    model: &FakeModel,
) -> Result<u64, Reject> {
    if proposal.body.trim().is_empty() {
        return Err(Reject::Empty);
    }
    if !allow_install(proposal) {
        return Err(Reject::Guard);
    }
    let _note = model.complete(&proposal.name);
    let current = library.load();
    let mut skills = current.skills.clone();
    skills.push(Skill {
        name: proposal.name.clone(),
        body: proposal.body.clone(),
        layer: Layer::Team,
    });
    let version = current.version + 1;
    let next = Arc::new(Index {
        team_md: render_team(&skills),
        skills,
        version,
    });
    swap_index(library, next);
    library.notices.lock().expect("notices").push(Notice {
        kind: "skills.updated",
        version,
    });
    Ok(version)
}

pub fn demo() -> String {
    let library = Library::new(Index::bundled(vec![Skill {
        name: "echo".into(),
        body: "Say the line back.".into(),
        layer: Layer::Bundled,
    }]));
    let mut queue = VecDeque::new();
    skill_propose(
        &mut queue,
        Proposal {
            name: "cite".into(),
            body: "Name the file you read.".into(),
            force: false,
            trust: Trust::Community,
        },
    );
    let proposal = queue.pop_front().expect("proposal");
    let version =
        librarian_accept(&library, &proposal, &FakeModel { reply: "ok" }).expect("accept");
    let index = library.load();
    format!("skills: v{version} {}", index.team_md.lines().count())
}
```

The stand-in model ignores the prompt, and `let _ = prompt` keeps the argument from looking unused. The bus module has its own `FakeModel`, while this one is `harness_core::team_skills::FakeModel`. The reply in `demo` is `ok`. `complete` is called with `&proposal.name`, which is `cite`, rather than the body, and the returned note is dropped. The model leaves the version alone, and it leaves allow-or-refuse alone, because those decisions have already returned when they were going to refuse.

The order inside `librarian_accept` is fixed: empty body, then `allow_install`, then the model, then `load`, then clone and push, then render, then `swap_index`, then the notice, then `Ok(version)`. The model runs before the swap, and the model runs with the index mutex free. `load` takes the mutex only long enough to clone the `Arc`, the clone of the skill vec happens after that guard is gone, and `swap_index` takes the mutex again for the replace. A worker who is inside `load` waits for that short guard, rather than for `FakeModel`.

The pushed skill uses the proposal's name and body and hardcodes `Layer::Team`. A builtin proposal that passes the guard becomes a team-layer row, and the rendered line says `team`. Version is the loaded version plus one. `Index::bundled` started at 1, so the first accept returns 2, and the notice uses that same version. If `swap_index` panicked, the notice would stay unpushed. The function panics on a poisoned mutex and otherwise returns the version. The old `Arc` returned by `swap_index` drops here, and workers who called `load` earlier still hold theirs.

The demo builds a library with one bundled skill, echo, body `Say the line back.` `Index::bundled` renders two lines and stores version 1. The queue starts empty. `skill_propose` of cite, body `Name the file you read.`, `force` false, `Trust::Community`, returns `Queued`, and `demo` ignores the `Propose` value. On an empty queue the result is `Queued`. `pop_front` takes cite back off the queue. `expect("proposal")` would panic if the queue were empty, and this queue has the one item. `librarian_accept` runs. The body has text, `is_dangerous` is false for that sentence, and `scan_text` leaves it unmarked. Community plus `Safe` is `Allow`. The model returns `ok` and the note is dropped. The new index is version 2. The rendered text has the heading, echo as bundled, and cite as team. `lines().count()` is 3. The format string is `skills: v{version} {}`, with `v` literal, `{version}` the `u64` from `librarian_accept`, the space literal, and the second placeholder the line count rather than the rendered markdown. The returned line is `skills: v2 3`. A change to echo's body leaves the count alone, and a change that dropped the heading would move it. The library stays in scope until the format has read the count out of `team_md`, then `demo` drops the library, and the printed line is the only thing `main` keeps.

The binary source prints it:

```rust
use harness_core::team_skills::demo;

fn main() {
    println!("{}", demo());
}
```

Running the crate needs `cargo run` with `--bin ch48-team-skills`. The package also builds `ch48_mcp_fixture`, `sandbox-worker`, and `ch48_kanban_claim`. The example binary prints eval lines, and no key is read. The result of `skill_propose` is ignored. A queue already at 64 would make `pop_front` return `None` only if this propose had returned `Full` on an empty queue, which this demo never does. This queue had one item, and the `Full` path is the test, rather than `demo`.

## 48.8 The manifest, the install policy, and this heading

The docstring at the top of `tools/skills_sync.py` is the manifest sync:

```python
"""Skills Sync -- manifest-based seeding and updating of bundled skills. Copies repo skills/ into
~/.hermes/skills/, tracking each synced skill's origin hash in .bundled_manifest (v2 "name:hash"
lines; v1 plain names auto-migrate). NEW skills are copied and recorded; EXISTING skills update
only when bundled changed AND the user copy still matches the origin hash (else user-customized
-> SKIP); user-DELETED skills are not re-added; upstream-REMOVED ones leave the manifest once no active or
archived copy remains (the entry is that copy's only built-in provenance record)."""
```

The manifest path `MANIFEST_FILE` is `SKILLS_DIR / ".bundled_manifest"`. The hash is the origin hash in that file. An update requires both a bundled change and a user copy that still matches. A deleted skill stays deleted on that path. This chapter keeps the library in memory, with no origin hash, no directory copy, and no memory of a deletion. `librarian_accept` will push the same name again if you ask it again, so two rows named `cite` would both render, because the function searches no existing name. That dedupe stays deferred, along with the manifest hash, the disk copy, and the user-deleted skip.

The Hermes `should_allow_install` is the policy in section 48.5. The hard-block refusal ends `--force does not override a dangerous verdict.` `allow_install` returns false for that case and leaves the sentence unformatted.

Staging in `stage_write`, at line 74 of `tools/write_approval.py`, writes a json record under `pending/` and returns it. On a disk error it logs and still returns a record. The docstring calls that the safe failure, because nothing was silently committed. `librarian_accept` leaves `stage_write` alone. A refusal is `Reject`, an accept is an in-memory `Arc` swap, and `team_skills.rs` has no pending directory.

I searched the Hermes tree at `9e01b44` for `TEAM.md` and didn't find that string. I looked for `optional-skills/creative/kanban-video-orchestrator/scripts/bootstrap_pipeline.py`. That file isn't in this checkout. I didn't re-open a file that writes the heading. This harness renders the heading. Hermes's sync path is the manifest, rather than this string.

The bus module is still in the crate, and `publish` still scrubs through `crate::redact::redact_for_egress`. `team_skills.rs` calls neither `publish`, nor `submit`, nor `send_to`. A proposal is a struct on a `VecDeque`, and the bench `skill_propose` subscribes nobody.

The file `tests/skills.rs` in this package is the team-skills file. `tests/writing.rs` is still the chapter 25 copy, and `tests/bus.rs` is still the chapter 47 copy. `ch48-harness-cli` depends on `ch48-team-skills` by path. `ch48-team-skills` lists no dependency on `ch47-message-bus`. `pub mod bus` and `pub mod team_skills` are both public, and the skills tests leave `bus` uncalled.

## Tests

The new tests are `steps/ch48-team-skills/crates/harness-core/tests/skills.rs`. The package also builds the copied harness tests, and the nextest line below is the package run.

The queue test `the_queue_holds_64_and_the_librarian_swaps_an_arc` fills the queue, refuses the next proposal, and leaves the worker's old `Arc` at version 1 after accept.

The guard test `force_does_not_install_a_dangerous_community_skill` expects `Reject::Guard` for `rm -rf /` with `force` set, and the model stays uncalled.

The builtin test `a_builtin_dangerous_body_is_still_the_librarians_write` allows a builtin body that mentions `rm -rf /`, and the local check leaves that body for the librarian's write.

The reader test `a_worker_reads_without_the_librarian` swaps the index, and the worker keeps the arc it already loaded.

The demo test `demo_prints_the_new_version` expects `skills: v2 3`.

The helper `cite` is shared by the fill test and the accept inside the first test. Body `Name the file you read.` is the demo's body. The bench uses a shorter body, `Name the file.`, and skips this helper.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch48-team-skills
```

```text
    Starting 249 tests across 53 binaries
     Summary [   0.415s] 249 tests run: 249 passed, 0 skipped
```

```bash
cargo run -p ch48-team-skills --bin ch48-team-skills
```

```text
skills: v2 3
```

The binary proposes one skill, accepts it, and prints the line, and no key is read.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `skill_propose` | 80.1 ns | enqueue one proposal under 1 ms | length check and `push_back` on an empty deque |
| `index_swap` | 200.3 ns | pointer replace under 1 ms | mutex and `mem::replace` of a prebuilt `Arc` |

```rust
fn skill_propose_enqueue(c: &mut Criterion) {
    let proposal = Proposal {
        name: "cite".into(),
        body: "Name the file.".into(),
        force: false,
        trust: Trust::Community,
    };
    c.bench_function("skill_propose", |b| {
        b.iter_batched(
            VecDeque::new,
            |mut queue| {
                let queued = skill_propose(black_box(&mut queue), proposal.clone());
                black_box((queued, queue));
            },
            BatchSize::SmallInput,
        );
    });
}
```

The criterion name is `skill_propose`. The Rust function around it is `skill_propose_enqueue`. `VecDeque::new` is the setup function item, and the clone of `cite` sits inside the sample.

```rust
fn index_swap(c: &mut Criterion) {
    c.bench_function("index_swap", |b| {
        b.iter_batched(
            || {
                let library = Library::new(Index::bundled(vec![Skill {
                    name: "echo".into(),
                    body: "Say it back.".into(),
                    layer: harness_core::team_skills::Layer::Bundled,
                }]));
                let next = Arc::new(Index {
                    skills: vec![Skill {
                        name: "cite".into(),
                        body: "Name the file.".into(),
                        layer: harness_core::team_skills::Layer::Team,
                    }],
                    version: 2,
                    team_md: "# TEAM\n- cite (team)\n".into(),
                });
                (library, next)
            },
            |(library, next)| {
                let previous = swap_index(black_box(&library), next);
                black_box(previous);
            },
            BatchSize::SmallInput,
        );
    });
}
```

Echo's body in this setup is `Say it back.`, the test's body, rather than `demo`'s `Say the line back.` Cite's body is `Name the file.`, the propose bench's body, rather than `Name the file you read.` The layer on the next skill is `Team`, version is 2, and `team_md` is the literal the descriptions `inputs` field quotes. `Index::bundled` in the setup does render echo's heading, and that render sits outside the timer. The timed `swap_index` leaves `render_team` uncalled.

The file `benchmarks/descriptions.json` says `skill_propose` is a `Proposal` named cite, body `Name the file.`, `force` false, trust `Community`, and the deque starts empty.

The means are `mean.point_estimate` from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), section `48`. Bench ids: `48/skill_propose`, `48/index_swap`, `46/skill_propose`, `44/skill_propose`, `46/index_swap`, `44/index_swap`. Command: `cargo bench -p ch48-team-skills --bench skills --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

The propose bench is one push onto an empty deque, and `allow_install` stays out of it. A number that includes `FakeModel` is the librarian, rather than this sample. The swap bench is the pointer replace, and a number that includes `render_team` or the skill-vec clone is the setup, which `iter_batched` leaves outside the timer.

Mapping `Builtin` to `skill_write::Trust::Builtin` would allow a scanner-dangerous body, and the match maps it to `Trusted` instead. A builtin `rm -rf /` is still allowed, because the scanner leaves that command unmarked and the local check forces the refuse only for `Community`. A builtin `ignore previous instructions` stays refused, because the scanner marks it critical and trusted plus dangerous is a hard block.

The demo drops the `Propose` value and `expect`s the accept, so a guard failure panics the binary. The dangerous-community test is the one that checks `Reject::Guard`.

Workers who keep the `Arc` from `load` keep seeing the old version in that `Arc`. They see a later accept on the next `load`. Reading `held.version` after an accept and expecting 2 is the bug the first test is written to catch.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Skills guard (H1).** The catalog row is H1. The name is skills guard. Hermes scans a community skill and `force` cannot override a dangerous verdict on a community skill. `should_allow_install` at line 818 applies the same hard block to community and trusted sources. The blocked string ends `--force does not override a dangerous verdict.` This chapter refuses `Trust::Community` when `is_dangerous` finds `rm -rf /` or `ignore previous instructions`, and it refuses before `force` is read. It then calls chapter 25's `scan_text` and `should_allow_install`. `Decision::Allow` is the only yes. A dangerous community verdict from that scanner is a hard block. `force: true` does not install it.

**Empty body.** `allow_install` returns false. `librarian_accept` returns `Reject::Empty`. Neither path pushes a skill or a notice.

**Readers and the lock.** `load` clones an `Arc` and drops the mutex before the caller reads the list. `librarian_accept` runs `FakeModel::complete` before `swap_index`. The model does not run while the index mutex is held. Workers do not write. `skill_propose` does not take the library lock at all.

**Left for later.** The `.bundled_manifest` hash, the disk copy into `~/.hermes/skills/`, and the skip of a skill the user deleted. `stage_write` stays in Hermes. This file swaps memory.

## What you implement

The milestone is `my-harness/tests/ch48.rs`.

```rust
use my_harness::chapter_48_propose_waits;

#[test]
fn skill_propose_enqueues_without_waiting() {
    assert!(!chapter_48_propose_waits());
}
```

The stub panics on `todo!("chapter 48")`.

```rust
pub fn chapter_48_propose_waits() -> bool {
    todo!("chapter 48")
}
```

That panic is `todo!("chapter 48")`. The test wants `chapter_48_propose_waits()` to be false, because enqueue returns before the librarian runs. In the step crate that is `skill_propose` on a `VecDeque`: a length check, and `push_back` when the length is under 64. `FakeModel` stays uncalled, `swap_index` stays uncalled, and the stub stays `todo!`.

## Chapter summary

- **Queue.** `skill_propose` holds 64 proposals, the next one comes back `Full` and stays off the deque, and the push returns without waiting on the librarian.
- **Guard.** An empty body is rejected, and community text that contains `rm -rf /` or `ignore previous instructions` is rejected even when `force` is true, while the scanner leaves `rm -rf /` to the local check.
- **Writer.** `librarian_accept` runs `FakeModel` before `swap_index`, and the notice kind is `skills.updated`.
- **Readers.** `load` returns an `Arc`, and the old arc keeps its version after the swap.
- **Heading.** `render_team` writes `# TEAM` and one bullet per skill, and the demo line is `skills: v2 3`.
- **Time.** `skill_propose` mean 80.1 ns and `index_swap` mean 200.3 ns, each `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

## What's next

A skill can stay inside the team, and the next chapter reaches an agent that lives outside this process.
