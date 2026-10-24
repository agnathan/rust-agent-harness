# Chapter 48: Shared Team Skills

Chapter 27 can curate one agent's skills. A team that learns together needs a shared library, and it needs exactly one writer. If every worker patches the team store, two patches will tear a file, and a dangerous install will depend on which worker remembered the scanner. This chapter adds the library, the proposal, and the librarian. The crate is `steps/ch48-team-skills`, a copy of chapter 47 plus `team_skills`. The package is `ch48-team-skills`.

`Library` has layers: team, agent-local, and bundled. `load` reads them. `render_team` writes the index the workers see. `Index` is stored in an `Arc`, and `swap_index` replaces that `Arc`. Workers do not wait on the swap. `skill_propose` enqueues a `Proposal` on the bus from chapter 47 and returns. `PROPOSAL_CAP` bounds that queue. `librarian_accept` is the single writer. It calls `allow_install`, which calls chapter 25's `scan_text` and `should_allow_install`. Community plus `force` does not override a dangerous body. A local check for `rm -rf /` stays, because that scanner does not name the command. `Notice` is what running agents hear on their next turn. Agents spawned after the swap get the new index. `TEAM.md` is the team fact file, and it is not this chapter's worker-writable skill body.

This is the team half of the closed loop. Chapter 24 showed one agent a stable index. Chapter 25 let that agent write, behind a scan. Chapter 26 reflected. Chapter 27 curated. Here the workers propose and the librarian applies, which is the same "one writer" rule scaled to a team. A child still does not write the team store. What it learned comes back in the result, and the parent's reflection, or the librarian, decides what to keep. The index swap is how a running agent hears about a new skill without a rewritten system prompt in the middle of its turn. Chapter 2's cache rule still holds.

Rust, at the swap: `Arc` is the index, and swapping it is one atomic store the readers clone. The proposal queue is a bounded channel, the same shape as the bus. The scanner call is an ordinary function call into `skill_write`, which is on this crate because the harness was copied forward. `Result` is the reject, with the dangerous verdict intact.

`cargo run -p ch48-team-skills --bin ch48-team-skills` runs the librarian accept. The exercise is `my-harness/tests/ch48.rs`: `skill_propose` enqueues, and workers do not wait.

**In this chapter**

- 48.1 `PROPOSAL_CAP`, `Layer`, `Trust`, `Skill`, and `Index`
- 48.2 `Proposal`, `Propose`, `Reject`, and `Notice`
- 48.3 `Library`, `load`, `notices`, and `render_team`
- 48.4 `skill_propose`
- 48.5 `allow_install`, `is_dangerous`, and chapter 25's scanner
- 48.6 `swap_index`
- 48.7 `FakeModel`, `librarian_accept`, and `demo`
- 48.8 The manifest, the install policy, and this heading

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

`skill_propose` pushes a `Proposal` onto a `VecDeque` until `PROPOSAL_CAP`. `PROPOSAL_CAP` is 64. The next call returns `Full` and does not push. The function does not scan the body and does not take the library lock. Enqueue does not wait on the librarian.

`allow_install` returns false for an empty body. It returns false when `is_dangerous` is true and trust is `Community`, even if `force` is true. Then it calls `crate::skill_write::scan_text` and `crate::skill_write::should_allow_install`. `Builtin` maps to `skill_write::Trust::Trusted`. `Community` maps to `skill_write::Trust::Community`. Only `Decision::Allow` becomes true. `skill_write` does not flag `rm -rf /`. That extra check stays in `is_dangerous`.

`librarian_accept` is the only writer. An empty body is `Reject::Empty`. A failed `allow_install` is `Reject::Guard`. Both return before `FakeModel`. When the guard passes, `model.complete` runs, then the skill list is cloned, then `swap_index` replaces the `Arc`. The notice is pushed after the swap. The kind is `skills.updated`.

`swap_index` is `mem::replace` of an `Arc<Index>` under a short mutex. The caller builds `next` before taking the lock. `load` clones the `Arc` and drops the mutex. A worker who loaded the old index still holds version 1 after the swap.

`render_team` writes `"# TEAM\n"` and then one line per skill, `- name (bundled|team)\n`. `demo` proposes `cite`, the librarian accepts it, and the line is `skills: v2 3`. The 3 is `team_md.lines().count()` after echo and cite are both in the list.

## The module

`steps/ch48-team-skills/crates/harness-core/src/team_skills.rs` is the whole new file.

```rust
//! Book chapter 48. Workers propose. The librarian is the only writer.
//! Readers hold an `Arc` of the index and do not wait on the scan.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

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

pub fn skill_propose(queue: &mut VecDeque<Proposal>, proposal: Proposal) -> Propose {
    if queue.len() >= PROPOSAL_CAP {
        return Propose::Full;
    }
    queue.push_back(proposal);
    Propose::Queued
}

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

/// Swap the index pointer. The caller builds `next` before taking the lock.
pub fn swap_index(library: &Library, next: Arc<Index>) -> Arc<Index> {
    let mut slot = library.index.lock().expect("index");
    std::mem::replace(&mut *slot, next)
}

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

The module comment is the contract. Workers propose. The librarian is the only writer. Readers hold an `Arc` and do not wait on the scan. `skill_propose` never sees the `Library`. `librarian_accept` never sees the `VecDeque`. The demo is the one function that uses both, and it pops the queue before it asks the librarian. The next sections walk each item in the order the file defines it.

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

`PROPOSAL_CAP` is 64, the same number as `BUS_CAP` and as `INBOX_BOUND`. It is not an alias of either. It bounds the proposal deque, not a channel. `skill_propose` is the only reader of the constant.

`Layer` is where a skill sits in the rendered list. `Bundled` spells `bundled`. `Team` spells `team`. The variant is not trust. A community proposal that the librarian accepts is stored as `Layer::Team`. The `Trust` on the proposal is not copied onto the `Skill`.

`Trust` has two variants. `skill_write::Trust` has four: `Builtin`, `Trusted`, `Community`, and `AgentCreated`. This chapter's enum is the smaller one. The mapping is in `allow_install`, not in a `From` impl. There is no `AgentCreated` proposal. There is no `Trusted` proposal. A builtin proposal becomes `Trusted` only at the call into chapter 25.

`Skill` owns its name and body. `layer` is `Copy`. `Index` owns the vec, a `u64` version, and the rendered string. All three fields are public. A worker who has an `Arc<Index>` can read `version` and `team_md` without a method and without the mutex.

`bundled` always sets `version` to 1. It does not look at the layers you passed. A vec that already contains `Layer::Team` still comes back as version 1, and `render_team` will still label those rows `team`. The tests pass a bundled echo, or an empty vec. `demo` passes echo. The rendered string is stored at construction so a later `load` does not have to rebuild it to print the list. `librarian_accept` builds the next index with `Index { ... }` directly, not with `bundled`, because the next version is `current.version + 1` and `bundled` would force it back to 1.

### Rust: `Copy` on the two enums

`Layer` and `Trust` are `Copy`. Matching them does not move the proposal. `allow_install` takes `&Proposal` and compares `proposal.trust == Trust::Community`. The proposal is still the caller's after the function returns. `Skill` is not `Copy`, because of the strings. Cloning a skill list clones every name and body. That clone happens in `librarian_accept`, outside the mutex, after `load` has returned.

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

`Proposal` is what a worker enqueues. `force` is a `bool`. `trust` is the small enum. There is no layer field. The librarian decides the layer, and it always pushes `Layer::Team`.

`Propose` is the queue result. `Queued` means the proposal was moved onto the deque. `Full` means it was not. Neither variant carries the proposal. On the `Full` path the proposal argument drops when the function returns. A caller who still needs the text has to clone it before the call. The bench clones inside the timed closure and keeps the original outside.

`Reject` is the librarian's error. `Empty` is a blank body. `Guard` is `allow_install` returning false. There is no `Full` on this enum. A full queue is the worker's problem, before the librarian is asked. `demo` uses `expect("accept")`, so either reject would panic the demo rather than print a different line.

`Notice` has a `'static` kind and a version. The only kind this file writes is `"skills.updated"`. `notices` returns a cloned `Vec`, so a reader does not hold the notices mutex. `PartialEq` is why a test could compare a whole notice. The queue test compares `kind` only.

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

`new` wraps the index in an `Arc` and a `Mutex`, and starts notices as an empty vec. The mutexes are separate. A reader of notices does not take the index lock. A reader of the index does not take the notices lock. `librarian_accept` takes them one after the other, not nested. `swap_index` releases the index lock when `slot` drops, and only then does the function lock notices.

`load` locks, clones the `Arc`, and unlocks when the guard drops at the end of the statement. `Arc::clone` bumps the strong count. It does not clone the `Index`. The `Skill` vec stays shared until a writer builds a new `Arc`. `expect("index")` is the panic if the mutex is poisoned. This file has no `catch_unwind`. A poison panics the caller.

`notices` clones the vec under the lock and returns the clone. The lock does not escape. An empty library has an empty vec. After one accept, the vec has one element.

`render_team` starts with the exact string `"# TEAM\n"`. The match has two arms, so a new `Layer` would fail to compile until this function learned a spelling. The format is `- {name} ({layer})\n`. `{layer}` is the string from the match, not the `Debug` of the enum. `Bundled` does not render as `Bundled`. For echo bundled and cite team, the string is:

```text
# TEAM
- echo (bundled)
- cite (team)
```

`lines()` does not yield a trailing empty line after the final newline. `lines().count()` on that string is 3. That is the demo's second number. An index with only echo is two lines: the heading and `- echo (bundled)`. `Index::bundled` of an empty vec is one line, `# TEAM`, because the loop pushes nothing. The bench's prebuilt string is `"# TEAM\n- cite (team)\n"`. `lines().count()` on that string is 2. `benchmarks/descriptions.json` calls the same value a three-line `TEAM.md` in the `what` field, and the `inputs` field quotes the two-line string with its trailing newline. The bench source is the string. The demo's 3 is echo plus cite plus the heading.

I did not find a Hermes file that writes `# TEAM`. The heading in this crate is `String::from("# TEAM\n")`.

### Rust: the lock does not leave `load`

`index.lock()` returns a `MutexGuard`. `Arc::clone` only needs a reference to the `Arc` inside it. The guard is a temporary. It drops at the semicolon. The returned `Arc` is not a guard. A worker can read `held.version` for as long as they want, and `librarian_accept` can lock the same mutex again. Holding `held` does not block the swap. That is the point of cloning the `Arc` instead of returning the guard.

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

The queue is the caller's. This function does not look at a `Library`, a `FakeModel`, or `allow_install`. A dangerous body queues. An empty body queues. `Full` is the only refusal, and it is about length. `push_back` adds at the tail. `demo` reads with `pop_front`. A queue of proposals stays in arrival order.

At length 64 the next call returns `Full` and does not push. The 65th proposal is dropped with the function argument. `len` stays 64. The test `the_queue_holds_64_and_the_librarian_swaps_an_arc` loops `0..PROPOSAL_CAP` with names `s0` through `s63`, body `"ok"`, `force: false`, `Trust::Builtin`, and expects `Queued` each time. The next call passes `cite()` and expects `Full`. `cite()` is the helper at the top of the test file: name `cite`, body `Name the file you read.`, `force` false, `Trust::Community`. That proposal is not on the deque after `Full`.

The bench starts from `VecDeque::new`, so the result is `Queued`. The `Full` path is not taken. `librarian_accept` is not called. A worker who only runs `skill_propose` does not wait on the model, because the model is not in this function. The milestone `chapter_48_propose_waits()` wants `false`. Enqueue does not wait.

### Rust: `VecDeque` and a moved proposal

`Proposal` is not `Copy`. `skill_propose` takes it by value. `push_back` moves it onto the deque. The `Full` return happens before the move onto the deque, so the value drops as the parameter goes out of scope. `with_capacity` is not called. `VecDeque::new` grows as it is pushed. The cap is a length check, not an allocation limit. The buffer may be larger than 64. The guard is `len`, the same shape as chapter 44's inbox.

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

The doc comment is the rule. Dangerous community text is refused, including when `force` is set. The pattern scan is chapter 25. `rm -rf /` is this chapter's extra, because that scanner does not name the command. I read `rules()` in `skill_write.rs`. The ids are `env_exfil_curl`, `prompt_injection_ignore`, `disregard_rules`, `ssh_dir_access`, and `role_hijack`. None of the patterns is `rm -rf /`.

Empty is first. `trim` makes a body of spaces empty. The function returns false and does not call `is_dangerous` or the scanner. `force` is not consulted. `librarian_accept` also checks `trim` and returns `Reject::Empty` before it calls `allow_install`. A blank proposal fails twice if you called both. `skill_propose` does not check it, so a worker can enqueue a blank body. The librarian is who rejects it.

`is_dangerous` lowercases with `to_ascii_lowercase` and looks for two substrings. `RM -RF /` matches. `Ignore Previous Instructions` matches this function, because the check is on the lowered string. The chapter 25 injection regex is case-sensitive. It can miss a capitalized phrase that this function already caught. For `Trust::Community`, that does not matter past the next line: if `is_dangerous` is true, `allow_install` returns false immediately. `proposal.force` is not read. The test body is `then rm -rf /` with `force: true` and `Trust::Community`. `allow_install` is false. The scanner is not asked. If this early return were gone, `scan_text` would see no finding, the verdict would be `Safe`, and `policy` for community plus safe is `Allow`. The local check is what refuses the command. The comment in the file says so.

For `Trust::Builtin`, the early return does not fire. `is_dangerous` can be true and the function still falls through. That is the builtin test. The body is `document rm -rf / as a thing not to run`, `force` false, `Trust::Builtin`. `allow_install` is true. The scanner does not flag the command, so the verdict is `Safe`. The trust mapped into chapter 25 is `Trusted`, not `Builtin`. `policy` for `Trusted` and `Safe` is `Allow`.

The mapping is the whole `match`:

```rust
let trust = match proposal.trust {
    Trust::Builtin => crate::skill_write::Trust::Trusted,
    Trust::Community => crate::skill_write::Trust::Community,
};
```

`skill_write::Trust::Builtin` would allow every verdict, including `Dangerous`. This chapter does not pass that variant. A builtin proposal is judged as trusted. A dangerous verdict on a trusted source is a hard block inside `should_allow_install`. A dangerous builtin can still be allowed when the scanner does not call it dangerous. `rm -rf /` is that case. `ignore previous instructions` is not. The scanner rule is:

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

Community plus `Dangerous` is `Block`. `hard_block` is true for community and for trusted. `force && !hard_block` does not run. The function returns `Block`. `allow_install` wants `Decision::Allow`, so it returns false. Community plus `force` does not override `Dangerous`. Community plus a non-dangerous block can still be overridden: `Caution` is `Block` from `policy`, and `hard_block` is false, so `force` returns `Allow`. The local `is_dangerous` check is stricter than that for its two phrases, because it returns false before `force` is visible.

`Decision::Ask` is also not `Allow`. An agent-created dangerous skill would ask, and `allow_install` would return false. This chapter's `Trust` cannot express `AgentCreated`, so that arm is not reachable from `allow_install`. It is still what chapter 25 does.

Hermes's function is the one I read at line 818, and it returns a reason string this crate does not build:

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

The blocked string ends `--force does not override a dangerous verdict.` when `hard_block` is true. Otherwise it ends `Use --force to override.` `skill_write::block_reason` in this crate builds the same dangerous tail. `allow_install` does not call `block_reason`. It only keeps the bool. `librarian_accept` turns a false into `Reject::Guard` and does not store the sentence.

### Rust: `matches!` on `Decision::Allow`

`matches!` is true only for `Allow`. `Block` and `Ask` are false. The function's return type is `bool`, so the caller cannot tell an empty body from a community `rm -rf /` from a scanner block. `librarian_accept` can tell empty from the rest, because it checks `trim` itself and returns `Reject::Empty` first. Every other false is `Guard`.

## 48.6 `swap_index`

```rust
/// Swap the index pointer. The caller builds `next` before taking the lock.
pub fn swap_index(library: &Library, next: Arc<Index>) -> Arc<Index> {
    let mut slot = library.index.lock().expect("index");
    std::mem::replace(&mut *slot, next)
}
```

The doc comment says the caller builds `next` before taking the lock. `librarian_accept` does. It clones the skill vec, pushes the new skill, renders the heading, and allocates the `Arc` before `swap_index`. The timed bench does too. Its setup closure builds the `Library` and the next `Arc`. The timed call is only the replace.

`lock` poisons with `expect("index")`. `mem::replace` writes `next` into the mutex and returns the previous `Arc`. The guard drops at the end of the function, which is immediately after the replace. The critical section is the pointer swap. It is not the scan, not the model, and not `render_team`.

The returned `Arc` is the old index. Callers who ignore it drop one strong count. `librarian_accept` ignores it. The test `a_worker_reads_without_the_librarian` keeps it. That test loads `seen`, builds a new `Arc` at version 9 with the same skills and the same `team_md`, and swaps. `previous.version` is 1. `seen.version` is still 1. `library.load().version` is 9. No `FakeModel`. No `librarian_accept`. The worker's `Arc` and the library's `Arc` are different allocations. The other test asserts `Arc::as_ptr` of the held index differs from `Arc::as_ptr` of the index loaded after accept.

### Rust: `mem::replace` and `Arc`

`*slot` is the `Arc<Index>` inside the mutex. `replace` needs a new value to put down, because the old `Arc` moves out. `std::mem::swap` would need a second `Arc` already stored in a local that ends up holding the old one. `replace` returns it. `Arc::new` happened before the lock. Workers who cloned the old `Arc` keep a strong count, so the old `Index` stays alive for them after the mutex points at the new one. Dropping the last old `Arc` drops the old vec. This file does not wait for that drop.

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

`FakeModel` ignores the prompt. `let _ = prompt` keeps the argument from looking unused. The bus module has its own `FakeModel`. This one is `harness_core::team_skills::FakeModel`. The reply in `demo` is `ok`. `complete` is called with `&proposal.name`, which is `cite`, not the body. The returned note is dropped. The model does not choose the version and it does not choose allow or refuse. Those decisions have already returned if they were going to refuse.

The order inside `librarian_accept` is fixed. Empty body, then `allow_install`, then the model, then `load`, then clone and push, then render, then `swap_index`, then the notice, then `Ok(version)`. The model runs before the swap. The model does not run under the index mutex. `load` takes the mutex only long enough to clone the `Arc`. The clone of the skill vec happens after that guard is gone. `swap_index` takes it again for the replace. A worker who is inside `load` waits for that short guard, not for `FakeModel`.

The pushed skill uses the proposal's name and body and hardcodes `Layer::Team`. A builtin proposal that passes the guard becomes a team-layer row. The rendered line says `team`, not `bundled`. Version is the loaded version plus one. `Index::bundled` started at 1, so the first accept returns 2. The notice uses that same version. If `swap_index` panicked, the notice would not be pushed. The function does not panic on a healthy mutex. The old `Arc` returned by `swap_index` is dropped here. Workers who called `load` earlier still hold theirs.

`demo` builds a library with one bundled skill, echo, body `Say the line back.` `Index::bundled` renders two lines and stores version 1. The queue starts empty. `skill_propose` of cite, body `Name the file you read.`, `force` false, `Trust::Community`, returns `Queued`. `demo` does not check the `Propose` value. On an empty queue it is `Queued`. `pop_front` takes cite back off the queue. `expect("proposal")` would panic if the queue were empty. It is not. `librarian_accept` runs. The body is not empty. `is_dangerous` is false for that sentence. `scan_text` does not flag it. Community plus `Safe` is `Allow`. The model returns `ok` and the note is dropped. The new index is version 2. The rendered text has the heading, echo as bundled, and cite as team. `lines().count()` is 3. The format string is `skills: v{version} {}`. The returned line is `skills: v2 3`.

`chapter_demo.rs` prints it:

```rust
use harness_core::team_skills::demo;

fn main() {
    println!("{}", demo());
}
```

`cargo run` needs `--bin ch48-team-skills`. The package also builds `ch48_mcp_fixture`, `sandbox-worker`, and `ch48_kanban_claim`. The example binary prints eval lines. No key is read. `skill_propose`'s result is ignored. A queue that was already at 64 would `pop_front` a `None` only if this propose had returned `Full` and the queue had been empty, which is not this demo. This queue had one item. The `Full` path is the test, not `demo`.

### Rust: the format of the demo line

`format!("skills: v{version} {}", index.team_md.lines().count())` is the whole return value. `v` is literal. `{version}` is the `u64` from `librarian_accept`. The space is literal. The second placeholder is the line count, not the rendered markdown. The test compares the full string. A change to echo's body does not change the count. A change that dropped the heading would. The library stays in scope until the format has cloned the count out of `team_md`. Then `demo` drops the library. The printed line is the only thing `main` keeps.

## 48.8 The manifest, the install policy, and this heading

The docstring at the top of `tools/skills_sync.py` is the sync I read:

```python
"""Skills Sync -- manifest-based seeding and updating of bundled skills. Copies repo skills/ into
~/.hermes/skills/, tracking each synced skill's origin hash in .bundled_manifest (v2 "name:hash"
lines; v1 plain names auto-migrate). NEW skills are copied and recorded; EXISTING skills update
only when bundled changed AND the user copy still matches the origin hash (else user-customized
-> SKIP); user-DELETED skills are not re-added; upstream-REMOVED ones leave the manifest once no active or
archived copy remains (the entry is that copy's only built-in provenance record)."""
```

`MANIFEST_FILE` is `SKILLS_DIR / ".bundled_manifest"`. The hash is the origin hash in that file. An update requires both a bundled change and a user copy that still matches. A deleted skill is not put back. This chapter does not hash a file. It does not copy a directory. It does not remember a deletion. `librarian_accept` will push the same name again if you ask it again. Two rows named `cite` would both render. The function does not search for an existing name. That dedupe is not here. The deferred pieces are the manifest hash, the disk copy, and the user-deleted skip.

`should_allow_install` in Hermes is the policy in section 48.5. The hard-block refusal ends `--force does not override a dangerous verdict.` `allow_install` returns false for that case and does not format the sentence.

`stage_write` at line 74 of `tools/write_approval.py` writes a json record under `pending/` and returns it. On a disk error it logs and still returns a record. The docstring calls that the safe failure, because nothing was silently committed. `librarian_accept` does not call it. A refusal is `Reject`. An accept is an in-memory `Arc` swap. There is no pending directory in `team_skills.rs`.

I searched the Hermes tree at `9e01b44` for `TEAM.md` and did not find that string. I looked for `optional-skills/creative/kanban-video-orchestrator/scripts/bootstrap_pipeline.py`. That file is not in this checkout. I did not re-open a file that writes the heading. This harness renders the heading. Hermes's sync path is the manifest, not this string.

`bus.rs` is still in the crate. `publish` still scrubs through `crate::redact::redact_for_egress`. `team_skills.rs` does not call `publish`, `submit`, or `send_to`. A proposal is a struct on a `VecDeque`. The bench `skill_propose` does not subscribe anyone.

`tests/skills.rs` in this package is the team-skills file. `tests/writing.rs` is still the chapter 25 copy. `tests/bus.rs` is still the chapter 47 copy. `ch48-harness-cli` depends on `ch48-team-skills` by path. `ch48-team-skills` does not depend on `ch47-message-bus`. `pub mod bus` and `pub mod team_skills` are both public. The skills tests do not call `bus`.

## Tests

The new tests are `steps/ch48-team-skills/crates/harness-core/tests/skills.rs`. The package also builds the copied harness tests. The nextest line below is the package run.

`the_queue_holds_64_and_the_librarian_swaps_an_arc` fills `PROPOSAL_CAP` builtin proposals with body `"ok"` and expects `Queued`. The next `cite()` expects `Full`. It then builds a library with bundled echo, body `Say it back.`, and loads `held`. `librarian_accept` of `cite()` with `FakeModel { reply: "ok" }` returns 2. `held.version` is still 1. `held.skills.len()` is 1. The loaded current index has version 2. `current.team_md` contains `cite (team)`. `library.notices()[0].kind` is `skills.updated`. `Arc::as_ptr(&held)` is not `Arc::as_ptr(&current)`. The worker's pointer and the library's pointer differ. The old arc was not edited in place.

`force_does_not_install_a_dangerous_community_skill` builds name `bad`, body `then rm -rf /`, `force` true, `Trust::Community`. `allow_install` is false. `librarian_accept` on an empty bundled index equals `Err(Reject::Guard)`. `notices()` is empty. `load().version` is 1. The model was not reached, because `Reject::Guard` returns before `complete`. The version did not become 2. The notice was not pushed.

`a_builtin_dangerous_body_is_still_the_librarians_write` builds name `bundled-rm`, body `document rm -rf / as a thing not to run`, `force` false, `Trust::Builtin`. `allow_install` is true. The test does not call `librarian_accept`. The name of the test is the policy outcome: the local dangerous check does not refuse a builtin, and the scanner does not flag that command, so the librarian is allowed to write it. The function that would write it is `librarian_accept`, which this test does not call.

`a_worker_reads_without_the_librarian` loads the bundled echo index, swaps in a version-9 `Arc` with the same skills and the same `team_md`, and checks three versions. The returned previous arc is 1. The worker's `seen` arc is 1. The library now loads 9. No model runs. The swap is the write.

`demo_prints_the_new_version` expects `demo()` to equal `skills: v2 3`.

The helper `cite` is shared by the fill test and the accept inside the first test. Body `Name the file you read.` is the demo's body. The bench uses a shorter body, `Name the file.`, and does not call this helper.

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

The binary proposes one skill, accepts it, and prints the line. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `skill_propose` | 80.1 ns | enqueue one proposal under 1 ms | length check and `push_back` on an empty deque |
| `index_swap` | 200.3 ns | pointer replace under 1 ms | mutex and `mem::replace` of a prebuilt `Arc` |

The bench ids are `48/skill_propose` and `48/index_swap`. The old ids `46/skill_propose` and `44/skill_propose` are aliases of the first. The old ids `46/index_swap` and `44/index_swap` are aliases of the second. The aliases live in `benchmarks/descriptions.json`. The function names did not change when the ids moved.

`descriptions.json` says `skill_propose` is a `Proposal` named cite, body `Name the file.`, `force` false, trust `Community`. The deque starts empty. The result is `Queued`. The timing note says the `Proposal` is built before the bench. `iter_batched`. Setup is `VecDeque::new` and is not timed. The timed routine clones the proposal, calls `skill_propose` with the queue and that clone passed through `black_box`, then `black_box` of the result and the queue. The length check and `push_back` are inside the sample. `librarian_accept`, `allow_install`, `swap_index`, and `FakeModel` are not called. The `Full` path is not taken. The inputs line says the body is 14 characters. `Name the file.` is those 14, including the period.

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

The criterion name is `skill_propose`. The Rust function around it is `skill_propose_enqueue`. `VecDeque::new` is the setup function item. The clone of `cite` is inside the sample.

`descriptions.json` says `index_swap` replaces a version-1 index that holds the bundled skill echo with a version-2 `Arc` that holds the team skill cite. The `what` field calls the rendered text a three-line `TEAM.md`. The `inputs` field says the string is `# TEAM\n- cite (team)\n`. The bench source stores that string. The timing note says setup builds `Library::new` of the bundled echo skill and an `Arc` of the version-2 index, and is not timed. The timed routine calls `swap_index` with that library and `Arc` passed through `black_box`, then `black_box` of the previous `Arc`. The mutex and `mem::replace` are inside the sample. Cloning the skill vec, rendering `TEAM.md`, and `FakeModel` are not called.

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

Echo's body in this setup is `Say it back.`, the test's body, not `demo`'s `Say the line back.` Cite's body is `Name the file.`, the propose bench's body, not `Name the file you read.` The layer on the next skill is `Team`. Version is 2. `team_md` is the literal the descriptions `inputs` field quotes. `Index::bundled` in the setup does render echo's heading, and that render is outside the timer. The timed `swap_index` does not call `render_team`.

The mean for `skill_propose` is 80.1 ns. The mean for `index_swap` is 200.3 ns. Each is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196), section `48`. The ids are `48/skill_propose` and `48/index_swap`. `.github/scripts/run_bench_section.sh` runs `cargo bench -p ch48-team-skills --bench skills --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10` for that section. The same flags are in `steps/ch48-team-skills/README.md`. The Criterion `time:` line is the slope, not the mean.

### Milestone demo

The `cargo run` above. The line is `skills: v2 3`.

## Pitfalls

The propose bench is one push onto an empty deque. `allow_install` is not in it. A number that includes `FakeModel` is the librarian, not this sample. The swap bench is the pointer replace. A number that includes `render_team` or the skill-vec clone is the setup, which `iter_batched` leaves outside the timer.

`skill_propose` will queue a body that `allow_install` will later refuse. The cap is 64 proposals, not 64 safe proposals. `Full` drops the proposal. The 65th text is gone unless the caller cloned it first.

`force: true` does not install `then rm -rf /` from a community proposal, and it does not install a scanner `Dangerous` verdict on a community or trusted source. A non-dangerous community block can still become `Allow` under `force`, because `hard_block` is only the dangerous verdict. The two local phrases return before that branch when trust is `Community`.

An empty body is `false` from `allow_install` and `Reject::Empty` from `librarian_accept`. Spaces are empty. `skill_propose` does not check.

`Builtin` mapped to `skill_write::Trust::Builtin` would allow a scanner-dangerous body. The match maps it to `Trusted`. A builtin `rm -rf /` is still allowed, because the scanner does not flag that command and the local check only forces the refuse for `Community`. A builtin `ignore previous instructions` is not allowed, because the scanner marks it critical and trusted plus dangerous is a hard block.

`swap_index` does not render and does not scan. Calling it with an `Arc` you built by hand, the way the worker test does, publishes no `skills.updated` notice. The notice is `librarian_accept`'s push, after a successful swap. A direct swap leaves `notices()` empty.

`librarian_accept` does not replace a skill with the same name. It appends. The rendered list can name `cite` twice. The manifest's "user copy still matches" rule is not applied.

`demo` drops the `Propose` value and `expect`s the accept. A guard failure panics the binary. The dangerous-community test is the one that checks `Reject::Guard`.

Workers who keep the `Arc` from `load` do not see later accepts in that `Arc`. They see them on the next `load`. Reading `held.version` after an accept and expecting 2 is the bug the first test is written to catch.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md).

**Skills guard (H1).** The catalog row is H1. The name is skills guard. Hermes scans a community skill and `force` cannot override a dangerous verdict on a community skill. `should_allow_install` at line 818 applies the same hard block to community and trusted sources. The blocked string ends `--force does not override a dangerous verdict.` This chapter refuses `Trust::Community` when `is_dangerous` finds `rm -rf /` or `ignore previous instructions`, and it refuses before `force` is read. It then calls chapter 25's `scan_text` and `should_allow_install`. `Decision::Allow` is the only yes. A dangerous community verdict from that scanner is a hard block. `force: true` does not install it.

**Empty body.** `allow_install` returns false. `librarian_accept` returns `Reject::Empty`. Neither path pushes a skill or a notice.

**Readers and the lock.** `load` clones an `Arc` and drops the mutex before the caller reads the list. `librarian_accept` runs `FakeModel::complete` before `swap_index`. The model does not run while the index mutex is held. Workers do not write. `skill_propose` does not take the library lock at all.

**Left for later.** The `.bundled_manifest` hash, the disk copy into `~/.hermes/skills/`, and the skip of a skill the user deleted. `stage_write` stays in Hermes. This file swaps memory.

## What you implement

`my-harness/tests/ch48.rs` is the milestone.

```rust
use my_harness::chapter_48_propose_waits;

#[test]
fn skill_propose_enqueues_without_waiting() {
    assert!(!chapter_48_propose_waits());
}
```

The stub panics.

```rust
pub fn chapter_48_propose_waits() -> bool {
    todo!("chapter 48")
}
```

`todo!("chapter 48")` is the panic. The test wants `chapter_48_propose_waits()` to be false. Enqueue does not wait. In the step crate that is `skill_propose` on a `VecDeque`: a length check, and `push_back` when the length is under 64. `FakeModel` is not called. `swap_index` is not called. The stub stays `todo!`. Do not paste `team_skills.rs` into `my-harness/`.

## Chapter summary

- **Queue.** `skill_propose` holds 64. The next one is `Full` and is not pushed. The push does not wait.
- **Guard.** An empty body is rejected. Community plus `rm -rf /` or `ignore previous instructions` is rejected even when `force` is true. The scanner does not know `rm -rf /`. That check stays here.
- **Writer.** `librarian_accept` runs `FakeModel` before `swap_index`. The notice kind is `skills.updated`.
- **Readers.** `load` returns an `Arc`. The old arc keeps its version.
- **Heading.** `render_team` writes `# TEAM` and one bullet per skill. The demo line is `skills: v2 3`.
- **Time.** `skill_propose` mean 80.1 ns. `index_swap` mean 200.3 ns. Each is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). The Criterion `time:` line is the slope.

## What's next

Chapter 49 is A2A for agents outside this process.
