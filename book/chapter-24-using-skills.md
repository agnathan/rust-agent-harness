# Chapter 24: Using Skills

The procedure for renaming a file is a folder, not a paragraph in the system prompt. The demo writes this `SKILL.md` under `files/rename`:

```text
---
name: rename
description: Rename a file
---
Rename a file by its path.
```

`render_index` turns that file into one stable row, `    - rename: Rename a file`, inside a single `<available_skills>` block. `SkillSession` stores the block in an `Arc` and leaves those bytes alone for the rest of the session, so viewing the skill, or loading 500 of them, doesn't move the prefix the provider has cached. That frozen index is how a continuously improving harness can show the procedures it has learned without paying for a new system prompt every time one of them is opened.

Chapter 22's harness can remember a fact and search a session, but it can't yet carry the steps, the pitfalls, and the files you always open. Putting that procedure in `MEMORY.md` would mix instructions with facts, and putting it in the system prompt would bust the cache every time a skill changed. This chapter adds skills as something you read, and writing them is chapter 25. Chapter 23 was the server, and this line copies chapter 22, not that crate, and the step is `steps/ch24-skills`, package `ch24-skills`, plus a `skills` module.

The caps match Hermes: the name at most 64 characters, the description that appears in the index at most 1,024, the body at most 100,000. `load_tree` walks categories, then skill folders, and `parse_skill` reads the few keys this chapter needs. `skills_list` filters by category, and `skill_view` reads one file, the whole of it, with no offset. `change_note` compares names, so a later message can say what appeared or disappeared while the system prompt stays put. The directory walk is `std::fs`, and the frontmatter parse pulls those keys rather than teaching YAML in general. An `Arc` of the rendered block is how five hundred skills become one cheap clone, and byte-stability is the assertion you want for a cache.

The agent can see which procedures exist, then load the one it needs as a tool result or as a user message, which is where chapter 2 said new text has to go. Chapter 26's reviewer may call `skill_view` and, once writes exist, patch. Chapter 34 leaves a `[SKILL_PRUNED]` marker if compression drops a loaded skill, and the agent reloads with `skill_view` if it still needs it. Chapter 48 shares a team library on top of this shape, and none of those hold if viewing a skill reshuffles the prompt prefix.

`cargo run --locked -p ch24-skills --bin ch24-skills` writes that one skill and views it, and `my-harness/tests/ch24.rs` checks that loading 500 skills leaves the index bytes unchanged when one is viewed.

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

## What this chapter decides

A skill on disk is `<category>/<name>/SKILL.md`, and the frontmatter keys this parser reads are `name`, `description`, and `requires_mcp`. An empty `name` leaves the directory name in place.

`load_tree` walks category directories, then skill directories, and both listings are sorted. A second folder with the same frontmatter name is skipped, and the slice comes back sorted by category, then by name.

`render_index` writes one string, and the same slice and the same connected-server list write the same bytes. `SkillSession::open` stores that string in an `Arc<String>`, and `index` clones the pointer.

`skills_list` returns every meta, or the metas in one category, and `skill_view` returns the whole `SKILL.md`, or one supporting file under the four directory names. `change_note` compares names, and an added name or a removed name returns the note, and the `Arc` from `open` is still the first render.

`ScriptedModel` replies with the string it was built with, and the demo writes one skill, completes once, and views the name in that reply. No key.

## 24.1 The caps and the two types

The module starts with three caps and the list of supporting directories, and the caps are the Hermes numbers; the directory list is what `support_path` allows.

```rust
pub const MAX_NAME_LENGTH: usize = 64;
pub const MAX_DESCRIPTION_LENGTH: usize = 1024;
pub const MAX_BODY_CHARS: usize = 100_000;

const SUPPORT: &[&str] = &["references", "templates", "scripts", "assets"];
```

`MAX_NAME_LENGTH` and `MAX_DESCRIPTION_LENGTH` are public because the long-name test imports the name cap and builds a string one character past it. `MAX_BODY_CHARS` is public too, and `SUPPORT` is private. Callers learn the four names when `skill_view` returns `SkillError::Path`.

`usize` is the length type, and the comparisons later use `chars().count()`, which counts Unicode scalar values. A name of 64 `n` characters is legal, and a name of 65 is `SkillError::Name`. The test builds that 65-character name with `"n".repeat(MAX_NAME_LENGTH + 1)`.

The note strings sit next to the caps, and they are the text `change_note` assembles.

```rust
const NOTE_PREFIX: &str =
    "[System: The skills available in this conversation changed after its system prompt was built.";
const ADDED_HEAD: &str = "Now available (load with skill_view(name) when relevant, like the skills listed in the system prompt):";
const REMOVED_HEAD: &str = "No longer available:";
```

`NOTE_PREFIX` has no closing bracket, and `change_note` appends `]` after the joined parts. The prefix is one line, and the added head is the next line when something was added. The removed head shares its line with the comma-separated names.

`SkillMeta` is the row the index is built from, but it isn't the file.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    pub category: String,
    pub requires_mcp: Option<String>,
}
```

Every field is owned, and `load_tree` returns `Vec<SkillMeta>`, and the caller can drop the files' text; `description` is a `String`, which may be empty. `requires_mcp` is `Option<String>`, and `None` means the frontmatter had no non-empty `requires_mcp` key, and `Some` is the server name after trim. `Clone` is what `skills_list` uses when it copies a row out of the slice, and `PartialEq` and `Eq` are what the duplicate-name check uses, through `got.name == meta.name`. The struct derive makes the whole row comparable, and the check compares the name field only.

`SkillError` is the error of every public function in this file, and it is an enum, not a trait, and each variant is a different failure.

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

`Name` holds the rejected name, and `Description` holds the rejected description. `Body` holds the character count that was too large, not the text, and `Missing` holds the name `find_dir` could not locate. `Path` holds the relative path `support_path` refused, or a directory name that was not valid Unicode. `Io` holds `err.to_string()` from `std::io::Error`, but the original `io::Error` isn't stored. `Debug` is how later chapters print it: `WriteError::Skill(format!("{err:?}"))`, and there is no `Display` impl in this file. There is no `thiserror` derive, and callers match on the variant, or they format the `Debug` text.

> **Rust sidebar: `Result` is the return, `Option` is a field**
>
> - `load_tree`, `skill_view`, `parse_skill`, and `demo` return `Result<T, SkillError>`. The `?` operator in those functions returns the `Err` early. A successful path is the value inside `Ok`.
> - `requires_mcp` is `Option<String>` because a skill may have no server. That is not an error. `change_note` returns `Option<String>` for the same reason: no name change is `None`, and a note is `Some`.
> - `SkillError` does not implement `std::error::Error` here. `map_err` still works. It takes the `io::Error` and builds `SkillError::Io`. The `?` on a `Result<T, SkillError>` needs the error type to already be `SkillError`. That is why the `map_err` sits before the `?`.

## 24.2 `ScriptedModel` replies with a stored line

The demo in 24.10 needs a reply that never touches the network, and this struct is that stand-in. It lives in `skills`, it isn't the `Provider` trait from the copied harness, and nothing else in the chapter calls it.

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

`new` takes `impl Into<String>`, so the demo can pass the literal `"skill_view rename"` and `into()` stores an owned copy, with `calls` starting at 0. `complete` takes `&mut self` because it increments that counter, ignores `_prompt`, and clones `reply` so the same text can be returned again while the model keeps its own copy. A scripted reply has no `Err` arm, and the demo's `?` sits on `open` and on `skill_view`.

## 24.3 `SkillSession` stores the index in an `Arc`

The session is the object that freezes the index, and it has one field.

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

`open` borrows the root and the connected list, but it doesn't store either borrow. `load_tree` returns an owned `Vec<SkillMeta>`, and `render_index` borrows that vec and returns an owned `String`. `Arc::new` moves that `String` onto the heap and returns a pointer, and the vec drops at the end of `open`. The files are not held open, and the session holds the rendered text.

`index` takes `&self`; it doesn't re-read the directory, and `Arc::clone` copies the pointer and bumps the strong count. It doesn't copy the characters, and the test in `a_new_skill_is_a_note_and_the_arc_stays` keeps that pointer:

```rust
let kept: Arc<String> = session.index();
```

The type ascription is there so the reader sees the pointer, not a `String`, and after another skill is written to disk, `session.index().as_str()` still equals `kept.as_str()`. `as_str` borrows the characters inside the `Arc`, and the comparison is byte equality of the text. Both pointers still refer to the allocation `open` made.

`connected` is `&[String]`, a slice of owned server names, and `open` only needs it for the duration of `render_index`. An empty slice is `&[]`, which is what the demo passes, and a missing server then marks the row. Passing `&["github".to_string()]` clears that mark in a new render, but it doesn't clear the mark inside an `Arc` that was already built with `&[]`.

> **Rust sidebar: `Arc` is a shared pointer, and the borrow ends**
>
> - `Arc<String>` is an owned handle to one heap `String`. Cloning the `Arc` clones the handle. Dropping one handle decrements the count. The `String` is freed when the last handle drops.
> - `open` takes `&Path` and `&[String]`. Those borrows end when `open` returns. The `SkillSession` does not contain a reference to the caller's path, so the session can outlive the local that was passed in, as long as the `Path` data was not required later. `skill_view` takes the path again. The session does not remember it.
> - `index` returns `Arc<String>` by value. The caller owns that handle. The session keeps its own handle. Two owners, one allocation.
> - A later `load_tree` builds a new `Vec`. It does not write the field `index`. Nothing in this file replaces the `Arc`. A new session is a new `open`.

## 24.4 `load_tree` walks categories, then skill folders

`load_tree` is the directory walk; it is public. The bench calls it. `open` calls it. The note test calls it again after a file appears.

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

`root` is borrowed, and `found` is owned. The function returns the vec or the first hard error.

`list_dirs(root)` yields category directories, sorted by path, and a missing root is an empty vec from `list_dirs`, so `load_tree` returns `Ok` with no skills. A file sitting directly in `root` is ignored, because `list_dirs` keeps directories only.

For each category, `file_name` is the directory's last component as a `String`, and that string is the category stored on every skill inside it. `files/rename/SKILL.md` gets category `files`.

The inner loop lists skill directories, and `skill_dir.join("SKILL.md")` builds the file path. `is_file` is false for a missing file and for a directory named `SKILL.md`; those entries `continue`. A folder of notes with no `SKILL.md` isn't a skill.

`fs::read_to_string` reads the whole file as UTF-8, and a non-UTF-8 file is `SkillError::Io` with the `io::Error` display text. `map_err` is required because `read_to_string` returns `std::io::Error`, and this function returns `SkillError`.

The body cap runs before parse, and `text.chars().count()` is the number of scalar values, and `MAX_BODY_CHARS` is 100_000. A longer file returns `Err(SkillError::Body(count))` and drops `found`, and skills already pushed are discarded with the error. One huge file fails the whole tree, and the check is on the file that was just read, including frontmatter. It isn't a check of the body after the `---` split.

`parse_skill` can return `Name` or `Description`, and the `?` returns that error too. One bad name fails the tree. The long-name test writes a legal file first, then overwrites `SKILL.md` with a 65-character name, and expects `load_tree` to be `Err`. The directory name `bad` isn't the name that fails, and the frontmatter name is.

The duplicate check walks `found` so far, and `any` stops on the first equal name. The closure's `|got: &SkillMeta|` writes the type so the comparison is field to field; a duplicate `continue`s. The first name in walk order stays, and walk order is sorted paths, because `list_dirs` sorts. The later folder is the one that disappears.

`sort_by` runs after the walk, and the key is the pair `(&category, &name)`. `cmp` on a tuple compares the first element, then the second, and categories sort, and names sort inside a category. The references are there so the sort borrows the strings instead of cloning them into the key. The sort is total for these fields because `String` is `Ord`.

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

A missing directory is `Ok(Vec::new())`, but an existing directory that cannot be read is `Io`. Each `DirEntry` can also fail, and that is `Io` too, and `entry.path()` is an owned `PathBuf`. `is_dir` follows the metadata, and files are left out. `dirs.sort()` sorts `PathBuf` in the platform's path order, and the tests use names `alpha`, `zeta`, `mail`, and `files`, and the rendered index has `alpha` before `zeta`. Category order is the sort of the category paths, and `files` sorts before `mail`.

`file_name` turns the last component into a `String`.

```rust
fn file_name(path: &Path) -> Result<String, SkillError> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .ok_or_else(|| SkillError::Path(path.display().to_string()))
}
```

`file_name()` on `Path` returns `Option<&OsStr>`, and `None` means the path ends in `..` or is empty. `to_str()` returns `Option<&str>`. `None` means the name isn't Unicode, and `and_then` chains those two options. `map(str::to_string)` owns the `&str`, and `ok_or_else` turns a remaining `None` into `SkillError::Path` with `path.display()`. The closure is `ok_or_else` so the display string is built only on the error path. `ok_or` would build it every time.

> **Rust sidebar: the vec owns the metas, the loop owns the paths**
>
> - `for category in categories` moves each `PathBuf` out of the vec. `file_name(&category)` borrows it. `list_dirs(&category)` borrows it. At the next iteration the path drops.
> - `found.push(meta)` moves the `SkillMeta`. After the push, the local `meta` is gone. The duplicate check borrows `found` with `iter` before the push. The borrow ends at the semicolon of `any`. The push is a later statement. The compiler allows that because the shared borrow does not span the mutation.
> - `sort_by` takes `&mut found` for the duration of the sort. The closure receives `&SkillMeta` for `a` and `b`. The `cmp` result is `Ordering`.

## 24.5 `parse_skill` reads three keys

The parser is private, and `load_tree` and `find_dir` are the callers.

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

`split_frontmatter` returns two borrowed slices of `text`. `_body` is unused. The index is built from the frontmatter, not from the instructions, and the instructions stay in the file until `skill_view`.

`name` starts as the directory name, owned, and a frontmatter `name` replaces it when the value is non-empty after trim. An empty `name:` leaves the directory name. `description` starts empty. A `description` line replaces it, including a line whose value trims to empty, and `requires_mcp` stays `None` until a non-empty value.

`split_once(':')` splits on the first colon, and `let Some(...) else { continue }` is the pattern form of matching an `Option`. A line with no colon is skipped, and `key.trim()` drops surrounding whitespace. The match arms use a guard: `"name" if !value.is_empty()`, and the guard is a condition on that pattern. An empty name fails the guard and falls through to `_`.

`trim_matches('"')` strips quote characters from both ends, and a value `"rename"` becomes `rename`. It strips every leading and trailing `"`, not a matched pair only. Inner quotes stay.

The length checks run after the loop, on the final strings, and the directory name is checked too, because it is the fallback. A 65-character directory name with no frontmatter `name` is `SkillError::Name`, and the error holds the string, so a caller can see what was rejected. This function doesn't truncate.

`category.to_string()` copies the category onto the meta, and `category` itself is borrowed from the caller and must not be stored as a reference. The meta has to own it, because the meta outlives `load_tree`'s loop variable.

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

The return is two slices into `text`. No allocation. `strip_prefix("---\n")` returns `Option<&str>`. `unwrap_or(text)` keeps the original text when the prefix is absent, and `split_once("\n---\n")` finds the closing marker. Both have to succeed, and the original text has to start with `---\n`, before the function returns the split. A file that contains `\n---\n` but doesn't start with `---\n` returns `("", text)`, and the whole file is then the body, and the frontmatter loop sees an empty string. The directory name stands.

The demo file is:

```text
---
name: rename
description: Rename a file
---
Rename a file by its path.
```

`strip_prefix` removes the opening line, and `split_once` splits at the blank line before the instruction. `front` is `name: rename\ndescription: Rename a file`, and `body` is `Rename a file by its path.\n`. `parse_skill` ignores the body, and `skill_view` returns the whole file, body included.

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

The input is borrowed, and the output is owned. Nothing in the function writes a file.

The first three lines are fixed, and the heading is `## Skills`. The second line is the instruction to load a skill with `skill_view(name)` when it is even partly relevant. The third line is the opening tag `<available_skills>`, and that tag is the block `_render_skills_index` renders.

`category` starts as an empty `String`, and the first skill's category always differs from `""`, so the first skill always emits a category line. The line is two spaces, the category, and a colon: `  files:`, but a later skill in the same category doesn't emit the header again. The function doesn't sort, and if the slice alternates categories, the header repeats. `load_tree` sorts before anyone renders, and `render_index` trusts that order.

A row with an empty description is `    - {name}`, and a row with a description is `    - {name}: {description}`. Four spaces, a dash, a space, and `index_names` depends on that prefix.

`requires_mcp` is matched as `if let Some(server) = &skill.requires_mcp`, and the `&` borrows the inner `String`. `connected.iter().any(...)` looks for an equal `String`, and equality is the full server name. `github` doesn't match `GitHub`, and when no element equals `server`, the row gains ` [needs: {server}]`, with a leading space. When the server is connected, the suffix is absent. The test renders once with `&[]` and expects `[needs: github]`, then renders with `&["github".to_string()]` and expects the substring `[needs:` to be absent.

The closing tag and the last sentence are fixed, but `lines.join("\n")` inserts a newline between lines and doesn't add one after the last line. Two calls with the same inputs return equal `String`s, and the test `the_index_is_sorted_and_byte_stable` asserts `once == twice`. It also asserts `once.find("alpha") < once.find("zeta")`, which is the sort from `load_tree` showing up in the text.

The 500-skill bench pays for the walk and this render inside the timer.

## 24.7 `skills_list` filters by category

Listing doesn't touch the disk, and it filters the slice `load_tree` already returned.

```rust
pub fn skills_list(skills: &[SkillMeta], category: Option<&str>) -> Vec<SkillMeta> {
    skills
        .iter()
        .filter(|skill| category.is_none_or(|want| skill.category == want))
        .cloned()
        .collect()
}
```

`category` is `Option<&str>`. `None` means every skill, and `Some("files")` means the `files` category. `is_none_or` is the method on `Option`: true when the option is `None`, or when the closure returns true for the value inside `Some`. Here the closure asks `skill.category == want`, and a `None` filter keeps the row. A `Some` filter keeps the row when the category equals the borrowed string.

`iter` yields `&SkillMeta`. `cloned` turns that into an owned `SkillMeta` by `Clone`, and `collect` builds the `Vec`. The order is the input order, but a category filter doesn't re-sort.

The test writes `zeta` and `alpha` under `files`, and `send` under `mail`, and `skills_list(&skills, Some("files"))` has length 2. `skills_list(&skills, Some("mail"))` has length 1, and `Some("files")` is a `&str` literal coerced to the option. There is no test for `None` in this file, and the closure says `None` keeps every row.

The returned vec is a snapshot, but changing it doesn't change the slice the caller still holds. Rendering uses the original slice, not the filtered vec, unless the caller passes the filtered vec to `render_index`.

> **Rust sidebar: `Option<&str>` does not own the category**
>
> - `Some("files")` points at a string literal. The `'static` lifetime is long enough for the call. A `Some(local.as_str())` would borrow `local` for the call and no longer.
> - The function returns `Vec<SkillMeta>`, owned rows. The borrow of `skills` ends when the function returns. The caller can drop the original vec and keep the list.
> - `filter` is lazy. `cloned` is lazy. `collect` is the step that runs them. Nothing is allocated for a row the filter rejects, beyond the iterator walking past it.

## 24.8 `skill_view` reads one file

Viewing is the load the index asked for, and it takes the root again, because the session didn't store it.

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

`file: None` is `SKILL.md` inside the skill directory, and `file: Some(rel)` goes through `support_path`. The match is exhaustive, and both arms produce a `PathBuf`. The `?` on `support_path` returns `Path` errors before any read.

`read_to_string` returns the whole file, and there is no offset and no character cap on this path. The cap ran in `load_tree` when the index was built, and a file that grows after `open` can be viewed at its new size. The index `Arc` still has the old description.

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

The walk matches `load_tree`: sorted categories, sorted skill directories, skip a folder with no `SKILL.md`. `parse_skill` runs again. A file that fails the caps fails the view of every name, because the `?` returns before the function reaches a later folder. The first directory whose parsed name equals `name` is the one returned, and `return Ok(skill_dir)` moves that path out. Later duplicates are not considered, and that agrees with `load_tree`, which kept the first name and skipped the rest.

No match is `SkillError::Missing` with an owned copy of `name`, and the borrowed `name` can die with the caller. The error has to own its string.

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

Two refusals happen before the component check, and `rel.contains("..")` is a substring test. `../SKILL.md` contains `..`. `references/../SKILL.md` contains `..`. A name like `foo..bar` also contains those two characters, so it is refused even when it isn't a parent-directory component. `Path::new(rel).is_absolute()` refuses an absolute path, and on Unix that is a path starting with `/`.

`components().next()` is the first component. `as_os_str().to_str()` is `Option<&str>`. `unwrap_or("")` turns a missing or non-Unicode component into an empty string, which isn't in `SUPPORT`, so the function returns `Path`. `SUPPORT.contains(&first)` asks whether the first component is one of the four names, and `secret.txt` has first component `secret.txt`. That isn't in the list, and `references/notes.md` has first component `references`. `dir.join(path)` appends the relative path under the skill directory, but the function doesn't check that the file exists. `skill_view` finds that out when the read fails, and the error is `Io`, not `Missing`.

`SUPPORT` is `&[&str]`. `contains` wants `&&str`, and `&first` is `&&str` because `first` is `&str`, and the four names are `references`, `templates`, `scripts`, and `assets`.

## 24.9 `change_note` compares names

The note is how a skill that appeared after `open` is announced, but the function doesn't write the session.

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

The one idea is a note, not a rewrite, and `previous` is the stored index text, the test passes `kept.as_str()`, and `skills` is a fresh `load_tree`. The session's `Arc` isn't an argument, so this function can't modify it. `index_names` pulls names out of a rendered block, `render_index` builds the block the new tree would have produced, and only the note, or `None`, escapes when `rendered` drops.

`added` keeps names present in the new list and absent from the old one, and `removed` does the reverse. `map(String::as_str)` borrows those names from the local vecs, which stay alive until `parts` is built. Both lists empty means `None`, so a second render of the same tree, or a description edit that keeps the name, produces no note. A `[needs: …]` suffix is stripped before the comparison, so connecting a server doesn't by itself add or remove a name.

When something changed, `parts` starts with `NOTE_PREFIX`, added names follow as `    - {name}` lines, and removed names are one line, `No longer available:` plus the names joined by `, `. The trailing `+ "]"` closes the bracket the prefix opened. The test writes `rename`, opens the session, writes `sync`, reloads, and expects the note to contain `sync` and to start with `[System: The skills available`. A second call is still `Some`, because the names still differ from the stored index, and the `Arc` text is unchanged.

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

Only lines that start with four spaces, a dash, and a space are names, and category lines start with two spaces, and the heading, the tags, and the prose are skipped. `split(':').next()` takes the text before the first colon, so `    - alpha: First` yields `alpha`, and a row with no colon yields the whole rest after `trim`. The second split cuts at ` [needs:`, which only matters when the needs suffix is still attached to the name. The first colon wins, so a colon inside a name would be cut off with the description. Empty names are dropped, and the vec owns its `String`s, and once `index_names` returns, `old` no longer borrows `previous`.

## 24.10 `demo` writes one skill and views it

The binary calls `demo`, and the test calls `demo` too.

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

`create_dir_all` makes `files/rename` under the root, and `fs::write` writes the raw skill. The description in the file is `Rename a file`, and the last line of the file is `Rename a file by its path.`

`SkillSession::open(root, &[])` loads that one skill and stores the index, and `session.index().as_str()` borrows the index for the `complete` call. The temporary `Arc` from `index()` lives until the end of the statement, and `complete` ignores the text and returns `skill_view rename`.

`trim_start_matches("skill_view ")` strips that prefix once, from the start, and `trim` drops leftover whitespace. `name` is `rename`, a `&str` borrowed from `call`, and `skill_view` looks up that name and returns the file. `lines().last()` is the last line, and `unwrap_or("")` covers an empty file. `to_string` owns the line so it can be returned beside `name.to_string()`, and the tuple is `(name, last line)`.

The return type is `Result<(String, String), SkillError>`, and both strings are owned. `call` can drop. The binary prints them:

```rust
let (name, line) = demo(&root).expect("skills");
println!("skill: {name}");
println!("body: {line}");
```

`expect` turns a `SkillError` into a panic, and the demo root is `std::env::temp_dir().join("ch24-skills-demo")`. `main` deletes it before and after. The test uses its own temp directory and asserts `name == "rename"` and `line == "Rename a file by its path."`

No API key is read.

## Tests

Four tests, in `steps/ch24-skills/crates/harness-core/tests/skills.rs`, and the package run is larger. This crate is the copied harness, so the inherited tests still run, and the nextest line for the package is 127 passed.

`the_index_is_sorted_and_byte_stable` writes `files/zeta`, `files/alpha` with `requires_mcp: github`, and `mail/send`, and `render_index` twice is byte-equal. `alpha` occurs before `zeta`, and the disconnected render contains `[needs: github]`. The render with `github` connected doesn't contain `[needs:`, and `skills_list` of `files` has length 2. `skills_list` of `mail` has length 1.

`skill_view_returns_the_whole_file_and_refuses_a_path_escape` writes `rename` and `references/notes.md`, and the full view contains the frontmatter name and `Do rename.` The supporting view equals `a note`. `../SKILL.md` is an error, and `secret.txt` is an error.

`a_new_skill_is_a_note_and_the_arc_stays` opens a session on `rename`, keeps the `Arc`, writes `sync`, and expects a note that contains `sync` and starts with `[System: The skills available`. `session.index().as_str()` still equals the kept text, and a second `change_note` against the stored text is still `Some`.

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

The skill is `rename`, and the body line is `Rename a file by its path.`

`load_render_500` writes 500 `SKILL.md` files before the bench, and category is `cat` plus `n % 10`.

The means are `mean.point_estimate` from [run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589), section `24`, and bench ids: `24/load_render_500`, `22/load_render_500`. How to read the `time:` line is in [How the numbers were measured](appendix-benchmark-method.md).

## Pitfalls

`change_note` compares names. A description that changes, and a `requires_mcp` server that becomes connected, leave the name list alone. The function returns `None`, and the `Arc` still has the first render. A new `open` is how the block itself changes.

A second folder with the same frontmatter name is skipped by `load_tree` and ignored by `find_dir` after the first hit. The index and the view agree on the first directory in sorted path order, and the skipped file is still on disk.

One file over the body cap, or one name over 64 characters, fails `load_tree` for the whole root. Skills that parsed earlier in the walk are dropped with the `Err`.

`support_path` treats `..` as a substring, and a supporting file whose name contains those two characters is refused. The check isn't a component-only check.

## Guardrails in this chapter

The catalog is [`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Skills sit on L6 and L8 once they can be written. This chapter only reads.

**Path.** `support_path` refuses `..`, an absolute path, and a first component outside `references`, `templates`, `scripts`, and `assets`. That is the read side of the path check.

**Size.** `MAX_NAME_LENGTH` is 64. `MAX_DESCRIPTION_LENGTH` is 1024. `MAX_BODY_CHARS` is 100_000, the same number as `MAX_SKILL_CONTENT_CHARS`. A file over the body cap returns `SkillError::Body` and does not enter the vec.

**Index.** `SkillSession` stores one `Arc<String>`. `change_note` returns a separate `String`. The bytes inside the `Arc` stay. That is the note from `stage_skills_index_note`, with the index left where `_render_skills_index` put it.

**Left for later.** The skills-guard scan (H1) and the write-approval gate (A9) are chapter 25. The live hub is deferred. Prompt-cache byte identity with the parent model request stays a note in chapter 26. Secret redaction (E1, E2) is chapter 20, already in this crate. `skills.rs` does not call it.

## Chapter summary

- **Layout.** `<category>/<name>/SKILL.md`. Frontmatter `name`, `description`, and optional `requires_mcp`.
- **Caps.** Name 64. Description 1,024. Body 100,000 characters.
- **Index.** `render_index` writes `<available_skills>`. `SkillSession` stores an `Arc`. The same inputs render the same bytes.
- **Tools.** `skills_list` filters by category. `skill_view` returns one whole file, or one supporting file under the four directories.
- **Note.** Added and removed names become the note. The `Arc` stays.
- **Time.** Loading and rendering 500 skills measured **4.15 ms**. That is the mean.

## What you implement

`my-harness/tests/ch24.rs` is the milestone, and the test is `loading_skills_does_not_change_the_index_bytes_when_one_is_viewed`. It builds an index string and asserts `chapter_24_index_changes_when_viewed(index, index)` is false, but the same bytes in and the same bytes out means viewing a skill didn't change the index.

The function in `my-harness` is a stub, and it ends in `todo!("chapter 24")` and panics until the assertion can pass.

## What's next

Chapter 25 writes skills. Create, patch, supporting files, and delete go through a scan and, when the gate is on, an approval hold. A reviewer has to read a file before it can patch it, but a write the model didn't ask for is a queue entry.