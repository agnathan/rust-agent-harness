# Self-improvement

How one agent gets better with use, and how a team keeps one shared library. The book teaches the single-agent loop in Part II (chapters 18–25; chapter 21 is the server-side learned-tools row) and the shared library in Chapter 44. This page is the design those chapters implement.

The shape follows Hermes Agent's closed learning loop: skills, memory, session search, a background review, and a curator. Two agents in this process still collaborate through the [team patterns](team-patterns.md). Learning writes into stores. It does not replace the supervisor.

Reflection, skill writes the model did not ask for, memory writes, index rebuilds, and FTS updates run as background tasks. The hot path awaits the model and the tools the model called. Spawning the reviewer, enqueueing a proposal, and swapping an `Arc` index are the costs that stay on the turn. The budgets are in [performance](performance.md).

## What goes where

| Store | Holds | When it enters context |
|---|---|---|
| Skills | Procedures for a kind of task | A short index in the system prompt. The full text loads on demand. |
| Memory | Facts true in every session | A snapshot frozen into the system prompt at session start. |
| Session search | What happened in past sessions | On demand, through the `session_search` tool. |

Skills come first. A procedure, a pitfall, or a preference for a kind of task belongs in the skill for that task. Memory is the narrow exception: a declarative fact that is true in every session ("User prefers concise responses"), not an instruction ("Always respond concisely"). Anything that will be stale within a week stays in the session, where recall can find it.

## Crate layout

`harness-learning` depends on `harness-core` (`Provider`, `Tool`, `Conversation`) and `harness-store` (SQLite). Modules:

- `skills` — store, index, tools
- `memory` — bounded files and the `memory` tool
- `recall` — FTS session search
- `reflect` — the post-turn reviewer
- `curate` — lifecycle of agent-written skills

Skill and memory tools implement the same MCP-shaped `Tool` trait as every other tool: name, description, input schema, content results.

## Skills

A skill is a folder `<category>/<name>/` with `SKILL.md` (YAML frontmatter plus a markdown body) and optional files under `references/`, `templates/`, `scripts/`, and `assets/`. The frontmatter carries `name`, `description`, `version`, `author`, `license`, and `platforms`. Harness-specific fields live under `metadata.harness` so an agentskills.io client can ignore them. The on-disk layout matches agentskills.io, so `anthropics/skills` and `openai/skills` import as read-only hub skills after the scanner runs.

Limits, matching Hermes: name ≤ 64 characters, the description that appears in the index ≤ 60, the body ≤ 100_000 characters, a supporting file ≤ 1 MiB.

```rust
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: Option<String>,
    pub platforms: Vec<String>,
    pub tags: Vec<String>,
    pub origin: Origin, // Bundled | Hub | Project | Agent { created_by } | Team { team_id }
    pub pinned: bool,
    pub extra: serde_yaml::Mapping,
}

#[async_trait]
pub trait SkillStore: Send + Sync {
    async fn list(&self) -> Result<Vec<SkillMeta>>;
    async fn view(&self, name: &str, file: Option<&RelPath>) -> Result<String>;
    async fn create(&self, category: &str, raw_skill_md: &str, actor: &Actor) -> Result<SkillMeta>;
    async fn patch(&self, name: &str, file: Option<&RelPath>, find: &str, replace: &str, actor: &Actor) -> Result<()>;
    async fn delete(&self, name: &str, actor: &Actor) -> Result<()>;
    async fn write_file(&self, name: &str, file: &RelPath, bytes: &[u8], actor: &Actor) -> Result<()>;
    fn generation(&self) -> u64;
}
```

`FsSkillStore` is that directory tree. Writes are atomic (temp file, then rename) and locked per skill. `RelPath` refuses anything outside the four supporting directories. `LayeredSkillStore` reads in precedence order — project, then team, then local, then bundled — and is writable in only one layer. A same-named skill in a higher layer hides the lower one. `list` is sorted, so the rendered index is byte-stable.

Every write and every import runs a `RegexSet` scan for injection and exfiltration. Usage (views, uses, patches, last activity, state, pinned) is a SQLite row, not a file the model edits.

### Tools and guards

| Tool | Effect |
|---|---|
| `skills_list` | Names, descriptions, categories. Optional category filter. |
| `skill_view` | The whole `SKILL.md`, or one supporting file. No offset or limit: the model reads a skill whole. |
| `skill_create` | A new skill from name, category, and full content. |
| `skill_patch` | Find and replace. Preferred over a full rewrite. |
| `skill_delete` | Remove a skill. Refuses a pinned skill. Agent-origin only, by default. |
| `skill_write_file` | A supporting file inside an allowed subdirectory. |

`Actor` is who is writing: user, agent, reviewer, librarian, or a team member. Policy is per actor. The reviewer's `ToolCtx` records the content hash of each skill and file viewed in that run, and `patch` requires a matching hash (read-before-write). An optional `Approver` holds a write for a person.

A `/skill-name` command injects the skill as a user message. It does not edit the system prompt.

### An index that stays cache-stable

`SkillIndex::render` builds one `<available_skills>` block: category, then name, one line each, fixed formatting. The block is computed once at session start, stored on the session with its hash, and left untouched for the rest of the session. Loaded bodies live in `skill_view` results or in a `/skill` user message.

When `SkillStore::generation` changes mid-session, the next user message carries a one-time note: `[skills changed: +foo (desc), -bar]`. The system-prompt index is rebuilt only at the next compaction. If compaction drops a loaded skill, it leaves a `[SKILL_PRUNED name]` marker and the agent reloads with `skill_view` if it still needs the skill. If the index is over its token budget, categories collapse to names. Entries are never dropped. Agent-written skills are the model's project memory.

A skill that needs an MCP server says so in frontmatter (`metadata.harness.requires_mcp`). The index marks it `[needs: github]` when that server is not connected, and loading it can offer to connect.

## Memory

`MEMORY.md` is the agent's notes. `USER.md` is the user profile. Entries are separated by `\n§\n`. Default budgets are 2,200 characters for memory and 1,375 for the user profile. One `memory` tool supports `add`, `replace` (the old text must match exactly one entry, and the new content replaces that whole entry), `remove`, and a batch. The target is `memory` or `user`.

Going over the budget returns how many characters to free, so the model consolidates instead of dropping the write. Writes are atomic, file-locked, and scanned with the same strict patterns as skills, because a poisoned entry would persist across sessions. A hand edit that breaks the delimiter format is a drift error, not a silent rewrite.

`snapshot()` is taken at session start and rendered into the system prompt. A write during the session takes effect next session. That keeps the prompt cache stable.

`MemoryProvider` is the extension point for an external backend: `prefetch`, `sync_turn`, `on_pre_compress`, `on_delegation`. Chapter 12 defines the trait so a backend can be swapped. Chapter 19 is the file backend. Chapter 32 flushes memory before it compresses the conversation, so a fact that would otherwise be cut still has a chance to land in the file.

## Recall

Sessions live in SQLite (`rusqlite` with the bundled FTS5, WAL mode):

- `sessions(id, source, parent_id, started_at, title)`
- `messages(id, session_id, role, content, ts)`
- `messages_fts`, trigram tokenizer, kept in sync by triggers

`session_search` returns real messages. It does not call a model. The mode comes from the arguments:

| Arguments | Mode | Returns |
|---|---|---|
| `query` | Discover | BM25 hits, one row per session lineage. The top hit includes the matching text. |
| `session_id` + `around` | Scroll | A window of messages around one id. |
| `session_id` | Read | The session, or its head and tail. Compaction summaries stay out of that view. |
| none | Browse | Recent sessions. |

Subagent sessions are hidden. Automation sessions (cron, workers) are demoted so their repeated vocabulary does not push the user's own sessions out of the results. The guidance is: when the user refers to a past conversation, search before asking them to repeat it.

## Reflection

Counters live on the agent. A memory nudge fires every 10 user turns. A skill nudge fires every 10 tool iterations. Both defaults are configurable. Crossing either threshold after a turn finalizes runs `ReflectionHook`, a `TurnHook::after_turn`.

The hook spawns a reviewer as a supervised child with its own cancellation token. A new user turn cancels that child. The reviewer uses the parent's model and the same system-prompt prefix, so it can reuse the provider's prompt cache. Its tools are only the skill and memory tools, with `Actor::Reviewer`. Its input is the conversation trimmed to a token budget. It writes straight to the stores. The parent transcript and its prompt cache stay untouched.

The review prompt is active: most sessions produce at least one skill update. Act on a correction of style or workflow, a non-trivial fix, or a loaded skill that turned out to be wrong. Prefer, in order: patch a skill loaded this session, patch an existing umbrella skill, add a support file, then create a new class-level skill. A new skill is named for the class of task, not for a pull request, an error string, or today's job.

What a lesson looks like: the procedure first, each pitfall as a general rule plus one clause of why, no dates or quoted user text, a fix in place rather than a second copy. Leave out environment failures, "tool X is broken" (those harden into refusals), transient errors, one-off narratives, and attempts that never worked.

The hook emits `ReviewReport` as a tracing event and, once the bus exists, on `learning.updated`. An approval mode can stage the writes instead of applying them.

## Curation

`Curator` is a scheduled job. It moves agent-created skills from active to stale, then to archived. Archiving is a move into an archive directory. The curator never deletes. Pinned skills are exempt. Bundled, hub, and team skills are not candidates, including for the optional LLM pass that merges skills into umbrella skills. That pass is off unless configured. Each run starts with a tar.gz snapshot (`tar` and `flate2`) so a bad consolidation can be restored.

## Team librarian

Chapter 44 adds one more layer after the message bus exists. Each agent sees:

- a team store, shared
- an agent-local store, private
- a bundled store, read-only

Workers do not write the team store. They call `skill_propose`, which publishes a `SkillProposal` on `skills.proposals`. A librarian agent (or the supervisor's reflection hook) is the single writer: it deduplicates against existing umbrella skills and applies with `Actor::Librarian`. Applied changes publish `skills.updated { generation, diff }`. Running agents hear about it as a one-time note on their next turn. Agents spawned after that get the new index.

A child inherits a read-only snapshot of the parent's index and may load skills. It does not write them. What it learned comes back in the delegation result (`on_delegation`), and the parent's reflection decides what to keep.

The team library is versioned (a git-backed directory or a SQLite history) so a bad write can roll back. Each skill records who created it, which role, and which sessions contributed. `USER.md` stays per human. `MEMORY.md` stays per agent role. Facts about the team live in `TEAM.md`, writable only by the supervisor.

The same library is what the MCP gateway serves once skills exist (the server starts in chapter 5; the service chapter is 38), and what the research team in Chapter 46 accumulates across runs. Chapter 47 is the load and token capstone, a different chapter.
