# Chapter 24: Using Skills

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 22's harness can remember a fact and search a session. It cannot yet carry a procedure for a kind of task: the steps, the pitfalls, the files you always open. Stuffing that procedure into `MEMORY.md` would mix instructions with facts, and stuffing it into the system prompt would bust the cache every time a skill changed. This chapter adds skills as something you read. Writing them is chapter 25. The crate is `steps/ch24-skills`, a copy of chapter 22 — chapter 23 is the server and is not on this line — plus a `skills` module. The package is `ch24-skills`.

A skill is a folder, category then name, with `SKILL.md` at the top: YAML frontmatter and a markdown body. The caps match Hermes: the name at most 64 characters, the description that appears in the index at most 60, the body at most 100,000 characters. `load_tree` walks categories, then skill folders. `parse_skill` reads the keys this chapter needs. `SkillSession` stores the rendered index in an `Arc`. `render_index` writes one `<available_skills>` block, category then name, fixed formatting. The block is computed once and left untouched for the rest of the session, so loading 500 skills does not change the index bytes when one is viewed. `skills_list` filters by category. `skill_view` reads one file, the whole of it, with no offset. `change_note` compares names so a later message can say what appeared or disappeared without rewriting the system prompt.

That index is the harness's half of self-improvement. The agent can see which procedures exist. It loads the one it needs, as a tool result or as a user message, which is where chapter 2 said new text has to go. The system prompt stays put. Chapter 25 adds create, patch, and delete, with a scanner. Chapter 26's reviewer is allowed to call `skill_view` and, once writes exist, to patch. Chapter 34 leaves a `[SKILL_PRUNED]` marker if compression drops a loaded skill, and the agent reloads with `skill_view` if it still needs it. Chapter 48 shares a team library on top of this shape. None of those work if viewing a skill reshuffles the prompt prefix.

Rust, beside the loader: the directory walk is `std::fs`, and the frontmatter parse is a small pull of a few keys rather than a general YAML lesson. `Arc<str>` or an `Arc` of the rendered block is how five hundred skills become one cheap clone. Byte-stability is an assertion on the bytes, which is the test you want for a cache.

`cargo run --locked -p ch24-skills --bin ch24-skills` writes one skill and views it. The exercise is `my-harness/tests/ch24.rs`: loading 500 skills does not change the index bytes when one is viewed.

**In this chapter**

- 24.1 The caps and the two types
- 24.2 `ScriptedModel` replies with a stored line
- 24.3 `SkillSession` stores the index in an `Arc`
- 24.4 `load_tree` walks categories, then skill folders
- 24.5 `parse_skill` reads three keys
- 24.6 `render_index` writes the block
- 24.7 `skills_list` filters by category
- 24.8 `skill_view` reads one file
- 24.9 `change_note` compares names
- 24.10 `demo` writes one skill and views it

| Section | What it walks |
|---|---|
| 24.1 | `MAX_NAME_LENGTH`, `MAX_DESCRIPTION_LENGTH`, `MAX_BODY_CHARS`, `SkillMeta`, `SkillError` |
| 24.2 | `ScriptedModel` |
| 24.3 | `SkillSession` and the `Arc` |
| 24.4 | `load_tree`, `list_dirs`, `file_name` |
| 24.5 | `parse_skill`, `split_frontmatter` |
| 24.6 | `render_index` |
| 24.7 | `skills_list` |
| 24.8 | `skill_view`, `find_dir`, `support_path` |
| 24.9 | `change_note`, `index_names` |
| 24.10 | `demo` |

> **How Hermes does it**
>
> I read `agent/prompt_builder.py` (`_render_skills_index`), `agent/skills_index_delta.py` (`stage_skills_index_note`, `_render`, `_NOTE_PREFIX`), `tools/skills_tool.py` (`skills_list`, `skill_view`), and `tools/skill_manager_tool.py` (`MAX_NAME_LENGTH`, `MAX_DESCRIPTION_LENGTH`, `MAX_SKILL_CONTENT_CHARS`) on Hermes `main` at `ab3fe9dffb0a`.
>
> `_render_skills_index` renders `<available_skills>`. `stage_skills_index_note` and `_render` are the later note. `_NOTE_PREFIX` opens that note. A skill added after the prompt is a note. The index bytes stay. `skills_list` and `skill_view` are the two tools. `MAX_NAME_LENGTH` is 64. `MAX_DESCRIPTION_LENGTH` is 1024. `MAX_SKILL_CONTENT_CHARS` is 100_000.
>
> **Adopted.** The caps. The index. The note.
>
> **Adapted.** Loading is a directory walk in this process.
>
> **Deferred.** The live hub. Prompt-cache byte identity with the parent model request. That stays a note in chapter 26.

> **Guardrails.** This chapter reads. A name over 64 characters, a description over 1,024, or a body over 100,000 characters does not enter the index. `skill_view` refuses a relative path that contains `..`, an absolute path, and a file whose first component is outside `references`, `templates`, `scripts`, and `assets`. The stored block is one `Arc`. The scan and the write gate are chapter 25.

[personal note: a skill you loaded late, and the step it would have saved]

## What this chapter decides

A skill on disk is `<category>/<name>/SKILL.md`. The frontmatter keys this parser reads are `name`, `description`, and `requires_mcp`. An empty `name` leaves the directory name in place.

`load_tree` walks category directories, then skill directories. Both listings are sorted. A second folder with the same frontmatter name is skipped. The slice comes back sorted by category, then by name.

`render_index` writes one string. The same slice and the same connected-server list write the same bytes. `SkillSession::open` stores that string in an `Arc<String>`. `index` clones the pointer.

`skills_list` returns every meta, or the metas in one category. `skill_view` returns the whole `SKILL.md`, or one supporting file under the four directory names. `change_note` compares names. An added name or a removed name returns the note. The `Arc` from `open` is still the first render.

`ScriptedModel` replies with the string it was built with. The demo writes one skill, completes once, and views the name in that reply. No key.

## 24.1 The caps and the two types

The module starts with three caps and the list of supporting directories. The caps are the Hermes numbers. The directory list is what `support_path` allows.

```rust
pub const MAX_NAME_LENGTH: usize = 64;
pub const MAX_DESCRIPTION_LENGTH: usize = 1024;
pub const MAX_BODY_CHARS: usize = 100_000;

const SUPPORT: &[&str] = &["references", "templates", "scripts", "assets"];
```

`MAX_NAME_LENGTH` and `MAX_DESCRIPTION_LENGTH` are public because the long-name test imports the name cap and builds a string one character past it. `MAX_BODY_CHARS` is public too. `SUPPORT` is private. Callers learn the four names when `skill_view` returns `SkillError::Path`.

`usize` is the length type. The comparisons later use `chars().count()`, which counts Unicode scalar values. A name of 64 `n` characters is legal. A name of 65 is `SkillError::Name`. The test builds that 65-character name with `"n".repeat(MAX_NAME_LENGTH + 1)`.

The note strings sit next to the caps. They are the text `change_note` assembles.

```rust
const NOTE_PREFIX: &str =
    "[System: The skills available in this conversation changed after its system prompt was built.";
const ADDED_HEAD: &str = "Now available (load with skill_view(name) when relevant, like the skills listed in the system prompt):";
const REMOVED_HEAD: &str = "No longer available:";
```

`NOTE_PREFIX` has no closing bracket. `change_note` appends `]` after the joined parts. The prefix is one line. The added head is the next line when something was added. The removed head shares its line with the comma-separated names.

`SkillMeta` is the row the index is built from. It is not the file.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    pub category: String,
    pub requires_mcp: Option<String>,
}
```

Every field is owned. `load_tree` returns `Vec<SkillMeta>`, and the caller can drop the files' text. `description` is a `String`, which may be empty. `requires_mcp` is `Option<String>`. `None` means the frontmatter had no non-empty `requires_mcp` key. `Some` is the server name after trim. `Clone` is what `skills_list` uses when it copies a row out of the slice. `PartialEq` and `Eq` are what the duplicate-name check uses, through `got.name == meta.name`. The struct derive makes the whole row comparable. The check compares the name field only.

`SkillError` is the error of every public function in this file. It is an enum, not a trait. Each variant is a different failure.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    Name(String),
    Description(String),
    Body(usize),
    Missing(String),
    Path(String),
    Io(String),
}
```

`Name` holds the rejected name. `Description` holds the rejected description. `Body` holds the character count that was too large, not the text. `Missing` holds the name `find_dir` could not locate. `Path` holds the relative path `support_path` refused, or a directory name that was not valid Unicode. `Io` holds `err.to_string()` from `std::io::Error`. The original `io::Error` is not stored. `Debug` is how later chapters print it: `WriteError::Skill(format!("{err:?}"))`. There is no `Display` impl in this file. There is no `thiserror` derive. Callers match on the variant, or they format the `Debug` text.

> **Rust sidebar: `Result` is the return, `Option` is a field**
>
> - `load_tree`, `skill_view`, `parse_skill`, and `demo` return `Result<T, SkillError>`. The `?` operator in those functions returns the `Err` early. A successful path is the value inside `Ok`.
> - `requires_mcp` is `Option<String>` because a skill may have no server. That is not an error. `change_note` returns `Option<String>` for the same reason: no name change is `None`, and a note is `Some`.
> - `SkillError` does not implement `std::error::Error` here. `map_err` still works. It takes the `io::Error` and builds `SkillError::Io`. The `?` on a `Result<T, SkillError>` needs the error type to already be `SkillError`. That is why the `map_err` sits before the `?`.

## 24.2 `ScriptedModel` replies with a stored line

The demo needs a model that does not call a network. This struct is that model. It is not the `Provider` trait from the copied harness. It lives in `skills` and only this chapter's demo uses it.

```rust
#[derive(Debug)]
pub struct ScriptedModel {
    reply: String,
    calls: u32,
}

impl ScriptedModel {
    pub fn new(reply: impl Into<String>) -> Self {
        Self {
            reply: reply.into(),
            calls: 0,
        }
    }

    pub fn calls(&self) -> u32 {
        self.calls
    }

    pub fn complete(&mut self, _prompt: &str) -> String {
        self.calls += 1;
        self.reply.clone()
    }
}
```

`new` takes `impl Into<String>`. A `&str` and a `String` both work. The demo passes the literal `"skill_view rename"`. `into()` stores an owned copy. `calls` starts at 0.

`complete` takes `&mut self` because it writes `calls`. The prompt is `_prompt`. The underscore is the compiler's permission to ignore it. The function does not look at the index. It adds one to `calls` and clones `reply`. The clone is so the model can answer again with the same text. The caller owns the returned `String`. The model still owns `reply`.

`calls` is a `u32` on the struct. `complete` needs `&mut self` because it writes that field. The return type is `String`. A scripted reply has no `Err` arm. The demo's `?` sits on `open` and on `skill_view`.

## 24.3 `SkillSession` stores the index in an `Arc`

The session is the object that freezes the index. It has one field.

```rust
pub struct SkillSession {
    index: Arc<String>,
}

impl SkillSession {
    pub fn open(root: &Path, connected: &[String]) -> Result<Self, SkillError> {
        let skills = load_tree(root)?;
        Ok(Self {
            index: Arc::new(render_index(&skills, connected)),
        })
    }

    pub fn index(&self) -> Arc<String> {
        Arc::clone(&self.index)
    }
}
```

`open` borrows the root and the connected list. It does not store either borrow. `load_tree` returns an owned `Vec<SkillMeta>`. `render_index` borrows that vec and returns an owned `String`. `Arc::new` moves that `String` onto the heap and returns a pointer. The vec drops at the end of `open`. The files are not held open. The session holds the rendered text.

`index` takes `&self`. It does not re-read the directory. `Arc::clone` copies the pointer and bumps the strong count. It does not copy the characters. The test in `a_new_skill_is_a_note_and_the_arc_stays` keeps that pointer:

```rust
let kept: Arc<String> = session.index();
```

The type ascription is there so the reader sees the pointer, not a `String`. After another skill is written to disk, `session.index().as_str()` still equals `kept.as_str()`. `as_str` borrows the characters inside the `Arc`. The comparison is byte equality of the text. Both pointers still refer to the allocation `open` made.

`connected` is `&[String]`, a slice of owned server names. `open` only needs it for the duration of `render_index`. An empty slice is `&[]`, which is what the demo passes. A missing server then marks the row. Passing `&["github".to_string()]` clears that mark in a new render. It does not clear the mark inside an `Arc` that was already built with `&[]`.

> **Rust sidebar: `Arc` is a shared pointer, and the borrow ends**
>
> - `Arc<String>` is an owned handle to one heap `String`. Cloning the `Arc` clones the handle. Dropping one handle decrements the count. The `String` is freed when the last handle drops.
> - `open` takes `&Path` and `&[String]`. Those borrows end when `open` returns. The `SkillSession` does not contain a reference to the caller's path, so the session can outlive the local that was passed in, as long as the `Path` data was not required later. `skill_view` takes the path again. The session does not remember it.
> - `index` returns `Arc<String>` by value. The caller owns that handle. The session keeps its own handle. Two owners, one allocation.
> - A later `load_tree` builds a new `Vec`. It does not write the field `index`. Nothing in this file replaces the `Arc`. A new session is a new `open`.

## 24.4 `load_tree` walks categories, then skill folders

`load_tree` is the directory walk. It is public. The bench calls it. `open` calls it. The note test calls it again after a file appears.

```rust
pub fn load_tree(root: &Path) -> Result<Vec<SkillMeta>, SkillError> {
    let mut found = Vec::new();
    let categories = list_dirs(root)?;
    for category in categories {
        let cat_name = file_name(&category)?;
        for skill_dir in list_dirs(&category)? {
            let path = skill_dir.join("SKILL.md");
            if !path.is_file() {
                continue;
            }
            let text = fs::read_to_string(&path).map_err(|err| SkillError::Io(err.to_string()))?;
            if text.chars().count() > MAX_BODY_CHARS {
                return Err(SkillError::Body(text.chars().count()));
            }
            let meta = parse_skill(&text, &cat_name, &file_name(&skill_dir)?)?;
            if found.iter().any(|got: &SkillMeta| got.name == meta.name) {
                continue;
            }
            found.push(meta);
        }
    }
    found.sort_by(|a, b| (&a.category, &a.name).cmp(&(&b.category, &b.name)));
    Ok(found)
}
```

`root` is borrowed. `found` is owned. The function returns the vec or the first hard error.

`list_dirs(root)` yields category directories, sorted by path. A missing root is an empty vec from `list_dirs`, so `load_tree` returns `Ok` with no skills. A file sitting directly in `root` is ignored, because `list_dirs` keeps directories only.

For each category, `file_name` is the directory's last component as a `String`. That string is the category stored on every skill inside it. `files/rename/SKILL.md` gets category `files`.

The inner loop lists skill directories. `skill_dir.join("SKILL.md")` builds the file path. `is_file` is false for a missing file and for a directory named `SKILL.md`. Those entries `continue`. A folder of notes with no `SKILL.md` is not a skill.

`fs::read_to_string` reads the whole file as UTF-8. A non-UTF-8 file is `SkillError::Io` with the `io::Error` display text. `map_err` is required because `read_to_string` returns `std::io::Error`, and this function returns `SkillError`.

The body cap runs before parse. `text.chars().count()` is the number of scalar values. `MAX_BODY_CHARS` is 100_000. A longer file returns `Err(SkillError::Body(count))` and drops `found`. Skills already pushed are discarded with the error. One huge file fails the whole tree. The check is on the file that was just read, including frontmatter. It is not a check of the body after the `---` split.

`parse_skill` can return `Name` or `Description`. The `?` returns that error too. One bad name fails the tree. The long-name test writes a legal file first, then overwrites `SKILL.md` with a 65-character name, and expects `load_tree` to be `Err`. The directory name `bad` is not the name that fails. The frontmatter name is.

The duplicate check walks `found` so far. `any` stops on the first equal name. The closure's `|got: &SkillMeta|` writes the type so the comparison is field to field. A duplicate `continue`s. The first name in walk order stays. Walk order is sorted paths, because `list_dirs` sorts. The later folder is the one that disappears.

`sort_by` runs after the walk. The key is the pair `(&category, &name)`. `cmp` on a tuple compares the first element, then the second. Categories sort, and names sort inside a category. The references are there so the sort borrows the strings instead of cloning them into the key. The sort is total for these fields because `String` is `Ord`.

`list_dirs` is the listing both levels use.

```rust
fn list_dirs(root: &Path) -> Result<Vec<PathBuf>, SkillError> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut dirs = Vec::new();
    let entries = fs::read_dir(root).map_err(|err| SkillError::Io(err.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|err| SkillError::Io(err.to_string()))?;
        if entry.path().is_dir() {
            dirs.push(entry.path());
        }
    }
    dirs.sort();
    Ok(dirs)
}
```

A missing directory is `Ok(Vec::new())`. An existing directory that cannot be read is `Io`. Each `DirEntry` can also fail, and that is `Io` too. `entry.path()` is an owned `PathBuf`. `is_dir` follows the metadata. Files are left out. `dirs.sort()` sorts `PathBuf` in the platform's path order. The tests use names `alpha`, `zeta`, `mail`, and `files`, and the rendered index has `alpha` before `zeta`. Category order is the sort of the category paths. `files` sorts before `mail`.

`file_name` turns the last component into a `String`.

```rust
fn file_name(path: &Path) -> Result<String, SkillError> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .ok_or_else(|| SkillError::Path(path.display().to_string()))
}
```

`file_name()` on `Path` returns `Option<&OsStr>`. `None` means the path ends in `..` or is empty. `to_str()` returns `Option<&str>`. `None` means the name is not Unicode. `and_then` chains those two options. `map(str::to_string)` owns the `&str`. `ok_or_else` turns a remaining `None` into `SkillError::Path` with `path.display()`. The closure is `ok_or_else` so the display string is built only on the error path. `ok_or` would build it every time.

> **Rust sidebar: the vec owns the metas, the loop owns the paths**
>
> - `for category in categories` moves each `PathBuf` out of the vec. `file_name(&category)` borrows it. `list_dirs(&category)` borrows it. At the next iteration the path drops.
> - `found.push(meta)` moves the `SkillMeta`. After the push, the local `meta` is gone. The duplicate check borrows `found` with `iter` before the push. The borrow ends at the semicolon of `any`. The push is a later statement. The compiler allows that because the shared borrow does not span the mutation.
> - `sort_by` takes `&mut found` for the duration of the sort. The closure receives `&SkillMeta` for `a` and `b`. The `cmp` result is `Ordering`.

## 24.5 `parse_skill` reads three keys

The parser is private. `load_tree` and `find_dir` are the callers.

```rust
fn parse_skill(text: &str, category: &str, dir_name: &str) -> Result<SkillMeta, SkillError> {
    let (front, _body) = split_frontmatter(text);
    let mut name = dir_name.to_string();
    let mut description = String::new();
    let mut requires_mcp = None;
    for line in front.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim().trim_matches('"').to_string();
        match key.trim() {
            "name" if !value.is_empty() => name = value,
            "description" => description = value,
            "requires_mcp" if !value.is_empty() => requires_mcp = Some(value),
            _ => {}
        }
    }
    if name.chars().count() > MAX_NAME_LENGTH {
        return Err(SkillError::Name(name));
    }
    if description.chars().count() > MAX_DESCRIPTION_LENGTH {
        return Err(SkillError::Description(description));
    }
    Ok(SkillMeta {
        name,
        description,
        category: category.to_string(),
        requires_mcp,
    })
}
```

`split_frontmatter` returns two borrowed slices of `text`. `_body` is unused. The index is built from the frontmatter, not from the instructions. The instructions stay in the file until `skill_view`.

`name` starts as the directory name, owned. A frontmatter `name` replaces it when the value is non-empty after trim. An empty `name:` leaves the directory name. `description` starts empty. A `description` line replaces it, including a line whose value trims to empty. `requires_mcp` stays `None` until a non-empty value.

`split_once(':')` splits on the first colon. `let Some(...) else { continue }` is the pattern form of matching an `Option`. A line with no colon is skipped. `key.trim()` drops surrounding whitespace. The match arms use a guard: `"name" if !value.is_empty()`. The guard is a condition on that pattern. An empty name fails the guard and falls through to `_`.

`trim_matches('"')` strips quote characters from both ends. A value `"rename"` becomes `rename`. It strips every leading and trailing `"`, not a matched pair only. Inner quotes stay.

The length checks run after the loop, on the final strings. The directory name is checked too, because it is the fallback. A 65-character directory name with no frontmatter `name` is `SkillError::Name`. The error holds the string, so a caller can see what was rejected. This function does not truncate.

`category.to_string()` copies the category onto the meta. `category` itself is borrowed from the caller and must not be stored as a reference. The meta has to own it, because the meta outlives `load_tree`'s loop variable.

```rust
fn split_frontmatter(text: &str) -> (&str, &str) {
    let rest = text.strip_prefix("---\n").unwrap_or(text);
    if let Some((front, body)) = rest.split_once("\n---\n") {
        if text.starts_with("---\n") {
            return (front, body);
        }
    }
    ("", text)
}
```

The return is two slices into `text`. No allocation. `strip_prefix("---\n")` returns `Option<&str>`. `unwrap_or(text)` keeps the original text when the prefix is absent. `split_once("\n---\n")` finds the closing marker. Both have to succeed, and the original text has to start with `---\n`, before the function returns the split. A file that contains `\n---\n` but does not start with `---\n` returns `("", text)`. The whole file is then the body, and the frontmatter loop sees an empty string. The directory name stands.

The demo file is:

```text
---
name: rename
description: Rename a file
---
Rename a file by its path.
```

`strip_prefix` removes the opening line. `split_once` splits at the blank line before the instruction. `front` is `name: rename\ndescription: Rename a file`. `body` is `Rename a file by its path.\n`. `parse_skill` ignores the body. `skill_view` returns the whole file, body included.

## 24.6 `render_index` writes the block

This is the string the `Arc` holds. It is public. The tests call it twice and compare the bytes.

```rust
pub fn render_index(skills: &[SkillMeta], connected: &[String]) -> String {
    let mut lines = vec![
        "## Skills".to_string(),
        "Before replying, scan the skills below. If a skill matches or is even partially relevant to your task, you MUST load it with skill_view(name) and follow its instructions. Err on the side of loading — it is always better to have context you don't need than to miss critical steps, pitfalls, or established workflows.".to_string(),
        "<available_skills>".to_string(),
    ];
    let mut category = String::new();
    for skill in skills {
        if skill.category != category {
            category = skill.category.clone();
            lines.push(format!("  {category}:"));
        }
        let mut row = if skill.description.is_empty() {
            format!("    - {}", skill.name)
        } else {
            format!("    - {}: {}", skill.name, skill.description)
        };
        if let Some(server) = &skill.requires_mcp {
            if !connected.iter().any(|got| got == server) {
                row.push_str(&format!(" [needs: {server}]"));
            }
        }
        lines.push(row);
    }
    lines.push("</available_skills>".to_string());
    lines.push(
        "Only proceed without loading a skill if genuinely none are relevant to the task."
            .to_string(),
    );
    lines.join("\n")
}
```

The input is borrowed. The output is owned. Nothing in the function writes a file.

The first three lines are fixed. The heading is `## Skills`. The second line is the instruction to load a skill with `skill_view(name)` when it is even partly relevant. The third line is the opening tag `<available_skills>`. That tag is the block `_render_skills_index` renders.

`category` starts as an empty `String`. The first skill's category always differs from `""`, so the first skill always emits a category line. The line is two spaces, the category, and a colon: `  files:`. A later skill in the same category does not emit the header again. The function does not sort. If the slice alternates categories, the header repeats. `load_tree` sorts before anyone renders. `render_index` trusts that order.

A row with an empty description is `    - {name}`. A row with a description is `    - {name}: {description}`. Four spaces, a dash, a space. `index_names` depends on that prefix.

`requires_mcp` is matched as `if let Some(server) = &skill.requires_mcp`. The `&` borrows the inner `String`. `connected.iter().any(...)` looks for an equal `String`. Equality is the full server name. `github` does not match `GitHub`. When no element equals `server`, the row gains ` [needs: {server}]`, with a leading space. When the server is connected, the suffix is absent. The test renders once with `&[]` and expects `[needs: github]`, then renders with `&["github".to_string()]` and expects the substring `[needs:` to be absent.

The closing tag and the last sentence are fixed. `lines.join("\n")` inserts a newline between lines and does not add one after the last line. Two calls with the same inputs return equal `String`s. The test `the_index_is_sorted_and_byte_stable` asserts `once == twice`. It also asserts `once.find("alpha") < once.find("zeta")`, which is the sort from `load_tree` showing up in the text.

The 500-skill bench pays for the walk and this render inside the timer.

## 24.7 `skills_list` filters by category

Listing does not touch the disk. It filters the slice `load_tree` already returned.

```rust
pub fn skills_list(skills: &[SkillMeta], category: Option<&str>) -> Vec<SkillMeta> {
    skills
        .iter()
        .filter(|skill| category.is_none_or(|want| skill.category == want))
        .cloned()
        .collect()
}
```

`category` is `Option<&str>`. `None` means every skill. `Some("files")` means the `files` category. `is_none_or` is the method on `Option`: true when the option is `None`, or when the closure returns true for the value inside `Some`. Here the closure asks `skill.category == want`. A `None` filter keeps the row. A `Some` filter keeps the row when the category equals the borrowed string.

`iter` yields `&SkillMeta`. `cloned` turns that into an owned `SkillMeta` by `Clone`. `collect` builds the `Vec`. The order is the input order. A category filter does not re-sort.

The test writes `zeta` and `alpha` under `files`, and `send` under `mail`. `skills_list(&skills, Some("files"))` has length 2. `skills_list(&skills, Some("mail"))` has length 1. `Some("files")` is a `&str` literal coerced to the option. There is no test for `None` in this file. The closure says `None` keeps every row.

The returned vec is a snapshot. Changing it does not change the slice the caller still holds. Rendering uses the original slice, not the filtered vec, unless the caller passes the filtered vec to `render_index`.

> **Rust sidebar: `Option<&str>` does not own the category**
>
> - `Some("files")` points at a string literal. The `'static` lifetime is long enough for the call. A `Some(local.as_str())` would borrow `local` for the call and no longer.
> - The function returns `Vec<SkillMeta>`, owned rows. The borrow of `skills` ends when the function returns. The caller can drop the original vec and keep the list.
> - `filter` is lazy. `cloned` is lazy. `collect` is the step that runs them. Nothing is allocated for a row the filter rejects, beyond the iterator walking past it.

## 24.8 `skill_view` reads one file

Viewing is the load the index asked for. It takes the root again, because the session did not store it.

```rust
pub fn skill_view(root: &Path, name: &str, file: Option<&str>) -> Result<String, SkillError> {
    let dir = find_dir(root, name)?;
    let path = match file {
        None => dir.join("SKILL.md"),
        Some(rel) => support_path(&dir, rel)?,
    };
    fs::read_to_string(&path).map_err(|err| SkillError::Io(err.to_string()))
}
```

`file: None` is `SKILL.md` inside the skill directory. `file: Some(rel)` goes through `support_path`. The match is exhaustive. Both arms produce a `PathBuf`. The `?` on `support_path` returns `Path` errors before any read.

`read_to_string` returns the whole file. There is no offset and no character cap on this path. The cap ran in `load_tree` when the index was built. A file that grows after `open` can be viewed at its new size. The index `Arc` still has the old description.

The test writes `rename`, writes `references/notes.md` with the contents `a note`, and checks three calls. `skill_view(..., None)` contains `name: rename` and `Do rename.` `skill_view(..., Some("references/notes.md"))` equals `a note`. `Some("../SKILL.md")` is `Err`. `Some("secret.txt")` is `Err`.

`find_dir` is the search by frontmatter name, not by directory name.

```rust
fn find_dir(root: &Path, name: &str) -> Result<PathBuf, SkillError> {
    for category in list_dirs(root)? {
        for skill_dir in list_dirs(&category)? {
            let path = skill_dir.join("SKILL.md");
            if !path.is_file() {
                continue;
            }
            let text = fs::read_to_string(&path).map_err(|err| SkillError::Io(err.to_string()))?;
            let meta = parse_skill(&text, &file_name(&category)?, &file_name(&skill_dir)?)?;
            if meta.name == name {
                return Ok(skill_dir);
            }
        }
    }
    Err(SkillError::Missing(name.to_string()))
}
```

The walk matches `load_tree`: sorted categories, sorted skill directories, skip a folder with no `SKILL.md`. `parse_skill` runs again. A file that fails the caps fails the view of every name, because the `?` returns before the function reaches a later folder. The first directory whose parsed name equals `name` is the one returned. `return Ok(skill_dir)` moves that path out. Later duplicates are not considered. That agrees with `load_tree`, which kept the first name and skipped the rest.

No match is `SkillError::Missing` with an owned copy of `name`. The borrowed `name` can die with the caller. The error has to own its string.

`support_path` is the path check.

```rust
fn support_path(dir: &Path, rel: &str) -> Result<PathBuf, SkillError> {
    if rel.contains("..") || Path::new(rel).is_absolute() {
        return Err(SkillError::Path(rel.to_string()));
    }
    let path = Path::new(rel);
    let first = path
        .components()
        .next()
        .and_then(|part| part.as_os_str().to_str())
        .unwrap_or("");
    if !SUPPORT.contains(&first) {
        return Err(SkillError::Path(rel.to_string()));
    }
    Ok(dir.join(path))
}
```

Two refusals happen before the component check. `rel.contains("..")` is a substring test. `../SKILL.md` contains `..`. `references/../SKILL.md` contains `..`. A name like `foo..bar` also contains those two characters, so it is refused even when it is not a parent-directory component. `Path::new(rel).is_absolute()` refuses an absolute path. On Unix that is a path starting with `/`.

`components().next()` is the first component. `as_os_str().to_str()` is `Option<&str>`. `unwrap_or("")` turns a missing or non-Unicode component into an empty string, which is not in `SUPPORT`, so the function returns `Path`. `SUPPORT.contains(&first)` asks whether the first component is one of the four names. `secret.txt` has first component `secret.txt`. That is not in the list. `references/notes.md` has first component `references`. `dir.join(path)` appends the relative path under the skill directory. The function does not check that the file exists. `skill_view` finds that out when the read fails, and the error is `Io`, not `Missing`.

`SUPPORT` is `&[&str]`. `contains` wants `&&str`, and `&first` is `&&str` because `first` is `&str`. The four names are `references`, `templates`, `scripts`, and `assets`.

## 24.9 `change_note` compares names

The note is how a skill that appeared after `open` is announced. The function does not write the session.

```rust
pub fn change_note(previous: &str, skills: &[SkillMeta], connected: &[String]) -> Option<String> {
    let old = index_names(previous);
    let rendered = render_index(skills, connected);
    let new = index_names(&rendered);
    let added: Vec<&str> = new
        .iter()
        .filter(|name| !old.iter().any(|got| got == *name))
        .map(String::as_str)
        .collect();
    let removed: Vec<&str> = old
        .iter()
        .filter(|name| !new.iter().any(|got| got == *name))
        .map(String::as_str)
        .collect();
    if added.is_empty() && removed.is_empty() {
        return None;
    }
    let mut parts = vec![NOTE_PREFIX.to_string()];
    if !added.is_empty() {
        parts.push(ADDED_HEAD.to_string());
        for name in added {
            parts.push(format!("    - {name}"));
        }
    }
    if !removed.is_empty() {
        parts.push(format!("{REMOVED_HEAD} {}", removed.join(", ")));
    }
    Some(parts.join("\n") + "]")
}
```

`previous` is the stored index text. The test passes `kept.as_str()`. `skills` is a fresh `load_tree`. `connected` is whatever the caller wants the new render to assume. The session's `Arc` is not an argument. The function cannot modify it.

`index_names` pulls the skill names out of a rendered block. `render_index` builds the block the new tree would have produced. The new names come from that temporary string. `rendered` is dropped at the end of the function. Only the note, or `None`, escapes.

`added` keeps names present in `new` and absent from `old`. `removed` keeps names present in `old` and absent from `new`. The filter closure borrows each name. `map(String::as_str)` turns `&String` into `&str` so the vec does not need its own allocation per name. The `&str` borrows `new` or `old`. Those vecs of `String` have to stay alive until `parts` is built. They do. They are locals in this function.

Both empty means `None`. A second render of the same tree is `None`. A description edit that keeps the name is `None`, because `index_names` stores the name and drops the description. A `[needs: …]` suffix is stripped before the comparison, so connecting a server does not by itself add or remove a name.

When something changed, `parts` starts with `NOTE_PREFIX`. Added names follow, first the head, then one line `    - {name}` per name. Removed names are a single line: `No longer available:` plus the names joined by `, `. The join of `parts` uses newlines. The trailing `+ "]"` closes the bracket the prefix opened. `+` on a `String` and a `&str` consumes the joined string and returns a new one. `Some` wraps it.

The test writes `rename`, opens the session, writes `sync`, reloads, and expects the note to contain `sync` and to start with `[System: The skills available`. It then calls `change_note` again with another `load_tree` and expects `Some`. The names are still different from the stored index, so the note is still `Some`. The `Arc` text is unchanged.

```rust
fn index_names(index: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in index.lines() {
        let Some(rest) = line.strip_prefix("    - ") else {
            continue;
        };
        let name = rest.split(':').next().unwrap_or(rest).trim();
        let name = name.split(" [needs:").next().unwrap_or(name).trim();
        if !name.is_empty() {
            names.push(name.to_string());
        }
    }
    names
}
```

Only lines that start with four spaces, a dash, and a space are names. Category lines start with two spaces. The heading, the tags, and the prose are skipped.

`split(':').next()` takes the text before the first colon. A row `    - alpha: First` yields `alpha`. A row with an empty description, `    - alpha`, has no colon. `split` still yields one element, the whole rest, and `next` returns it. `unwrap_or(rest)` covers the empty-iterator case, which `split` does not produce. `trim` drops surrounding whitespace.

The second split cuts at ` [needs:`. A row `    - alpha: First [needs: github]` has already lost the description at the colon split, so the name is `alpha` before the needs split runs. A row whose description is empty and whose needs suffix was attached would be `    - alpha [needs: github]`. There is no colon. The needs split then takes `alpha`. The order of the two splits matters for a name that itself contained a colon. The first colon wins, and the rest is discarded. This parser does not accept a colon inside a name.

Empty names are not pushed. The vec is owned `String`s so the caller can drop the index text after the call. `change_note` does not drop `previous` while `old` is alive. `previous` is a borrow from the caller, and `old` borrows it only inside `index_names`. `index_names` returns owned strings. After it returns, `old` does not borrow `previous`.

## 24.10 `demo` writes one skill and views it

The binary calls `demo`. The test calls `demo` too.

```rust
pub fn demo(root: &Path) -> Result<(String, String), SkillError> {
    fs::create_dir_all(root.join("files/rename")).map_err(|err| SkillError::Io(err.to_string()))?;
    let raw = "---\nname: rename\ndescription: Rename a file\n---\nRename a file by its path.\n";
    fs::write(root.join("files/rename/SKILL.md"), raw)
        .map_err(|err| SkillError::Io(err.to_string()))?;
    let session = SkillSession::open(root, &[])?;
    let mut model = ScriptedModel::new("skill_view rename");
    let call = model.complete(session.index().as_str());
    let name = call.trim_start_matches("skill_view ").trim();
    let body = skill_view(root, name, None)?;
    let line = body.lines().last().unwrap_or("").to_string();
    Ok((name.to_string(), line))
}
```

`create_dir_all` makes `files/rename` under the root. `fs::write` writes the raw skill. The description in the file is `Rename a file`. The last line of the file is `Rename a file by its path.`

`SkillSession::open(root, &[])` loads that one skill and stores the index. `session.index().as_str()` borrows the index for the `complete` call. The temporary `Arc` from `index()` lives until the end of the statement. `complete` ignores the text and returns `skill_view rename`.

`trim_start_matches("skill_view ")` strips that prefix once, from the start. `trim` drops leftover whitespace. `name` is `rename`, a `&str` borrowed from `call`. `skill_view` looks up that name and returns the file. `lines().last()` is the last line. `unwrap_or("")` covers an empty file. `to_string` owns the line so it can be returned beside `name.to_string()`. The tuple is `(name, last line)`.

The return type is `Result<(String, String), SkillError>`. Both strings are owned. `call` can drop. The binary prints them:

```rust
let (name, line) = demo(&root).expect("skills");
println!("skill: {name}");
println!("body: {line}");
```

`expect` turns a `SkillError` into a panic. The demo root is `std::env::temp_dir().join("ch24-skills-demo")`. `main` deletes it before and after. The test uses its own temp directory and asserts `name == "rename"` and `line == "Rename a file by its path."`

No API key is read.

## Tests

Four tests, in `steps/ch24-skills/crates/harness-core/tests/skills.rs`. The package run is larger. This crate is the copied harness, so the inherited tests still run. The nextest line for the package is 127 passed.

`the_index_is_sorted_and_byte_stable` writes `files/zeta`, `files/alpha` with `requires_mcp: github`, and `mail/send`. `render_index` twice is byte-equal. `alpha` occurs before `zeta`. The disconnected render contains `[needs: github]`. The render with `github` connected does not contain `[needs:`. `skills_list` of `files` has length 2. `skills_list` of `mail` has length 1.

`skill_view_returns_the_whole_file_and_refuses_a_path_escape` writes `rename` and `references/notes.md`. The full view contains the frontmatter name and `Do rename.` The supporting view equals `a note`. `../SKILL.md` is an error. `secret.txt` is an error.

`a_new_skill_is_a_note_and_the_arc_stays` opens a session on `rename`, keeps the `Arc`, writes `sync`, and expects a note that contains `sync` and starts with `[System: The skills available`. `session.index().as_str()` still equals the kept text. A second `change_note` against the stored text is still `Some`.

`a_long_name_is_refused_and_the_demo_is_keyless` writes a frontmatter name of `MAX_NAME_LENGTH + 1` characters and expects `load_tree` to fail. `demo` on a nested directory returns `rename` and `Rename a file by its path.`

Each test builds a temp directory from the process id, the clock, and an atomic counter, and deletes it on `Drop`.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch24-skills
```

```text
Starting 127 tests across 31 binaries
     Summary [   0.291s] 127 tests run: 127 passed, 0 skipped
```

```bash
cargo run --locked -p ch24-skills --bin ch24-skills
```

```text
skill: rename
body: Rename a file by its path.
```

The binary writes one skill under a temp directory, renders the index, and views the file the scripted reply names. No key.

| Benchmark | Mean | What it is |
|---|---|---|
| `load_render_500` | 4.15 ms | load 500 skills and render the index |

`load_render_500` writes 500 `SKILL.md` files before the bench. Category is `cat` plus `n % 10`. Name is `skill` plus the index zero-padded to three digits, from `skill000` through `skill499`. The description is `Skill {n}`. The body is `Body {n}.` The timed closure is `b.iter`. Inside it: `load_tree` and `render_index`, the root and the slice and the connected list passed through `black_box`. The writes sit outside the timer. No model is called. The bench deletes the directory after the samples.

The Mean cell is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `24`. The id is `24/load_render_500`. The old id `22/load_render_500` is an alias in `benchmarks/descriptions.json`. I did not rewrite gh-pages history. Criterion also prints a `time:` line. That line is the slope. It is not the mean.

### Milestone demo

The `cargo run` above. The skill is `rename`. The body line is `Rename a file by its path.`

## Pitfalls

`change_note` compares names. A description that changes, and a `requires_mcp` server that becomes connected, leave the name list alone. The function returns `None`. The `Arc` still has the first render. A new `open` is how the block itself changes.

A second folder with the same frontmatter name is skipped by `load_tree` and ignored by `find_dir` after the first hit. The index and the view agree on the first directory in sorted path order. The skipped file is still on disk.

One file over the body cap, or one name over 64 characters, fails `load_tree` for the whole root. Skills that parsed earlier in the walk are dropped with the `Err`.

`support_path` treats `..` as a substring. A supporting file whose name contains those two characters is refused. The check is not a component-only check.

`skill_view` reads the whole file. The 100_000 character cap is on `load_tree`. A file edited after `open` can come back larger than the cap. The index does not move.

`render_index` does not sort. It emits a category header whenever the category changes. Pass it the vec `load_tree` returned.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Skills sit on L6 and L8 once they can be written. This chapter only reads.

**Path.** `support_path` refuses `..`, an absolute path, and a first component outside `references`, `templates`, `scripts`, and `assets`. That is the read side of the path check.

**Size.** `MAX_NAME_LENGTH` is 64. `MAX_DESCRIPTION_LENGTH` is 1024. `MAX_BODY_CHARS` is 100_000, the same number as `MAX_SKILL_CONTENT_CHARS`. A file over the body cap returns `SkillError::Body` and does not enter the vec.

**Index.** `SkillSession` stores one `Arc<String>`. `change_note` returns a separate `String`. The bytes inside the `Arc` stay. That is the note from `stage_skills_index_note`, with the index left where `_render_skills_index` put it.

**Left for later.** The skills-guard scan (H1) and the write-approval gate (A9) are chapter 25. The live hub is deferred. Prompt-cache byte identity with the parent model request stays a note in chapter 26. Secret redaction (E1, E2) is still absent.

## Chapter summary

- **Layout.** `<category>/<name>/SKILL.md`. Frontmatter `name`, `description`, and optional `requires_mcp`.
- **Caps.** Name 64. Description 1,024. Body 100,000 characters.
- **Index.** `render_index` writes `<available_skills>`. `SkillSession` stores an `Arc`. The same inputs render the same bytes.
- **Tools.** `skills_list` filters by category. `skill_view` returns one whole file, or one supporting file under the four directories.
- **Note.** Added and removed names become the note. The `Arc` stays.
- **Time.** Loading and rendering 500 skills measured **4.15 ms**. That is the mean.

[personal note: a procedure that belonged in a skill instead of in the prompt]

## What you implement

`my-harness/tests/ch24.rs` is the milestone. The test is `loading_skills_does_not_change_the_index_bytes_when_one_is_viewed`. It builds an index string and asserts `chapter_24_index_changes_when_viewed(index, index)` is false. The same bytes in and the same bytes out means viewing a skill did not change the index.

The function in `my-harness` is a stub. It ends in `todo!("chapter 24")` and panics until the assertion can pass. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 25 writes skills. Create, patch, supporting files, and delete go through a scan and, when the gate is on, an approval hold. A reviewer has to read a file before it can patch it. A write the model did not ask for is a queue entry.
