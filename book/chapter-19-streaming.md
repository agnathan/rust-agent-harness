# Chapter 19: Streaming

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 18 builds a request and waits for a whole reply. A stream arrives as deltas. This chapter folds those deltas into the same reply string, strips think blocks as the chunks arrive, and lets a read-only tool start before the stream finishes. A write waits until `finish`. Nothing here opens a socket.

The step crate is `steps/ch19-streaming`. It is chapter 18's harness copied forward, plus `stream`. The package name stays `ch19-streaming`.

| Section | Step | What it adds |
|---|---|---|
| 19 | `steps/ch19-streaming` | think-block scrubbing, folded deltas, read-only overlap |

> **How Hermes does it**
>
> I read `agent/think_scrubber.py` (`StreamingThinkScrubber`, `THINK_TAGS`, `feed`, `flush`, `reset`) on Hermes `main` at `cf71b1aa`.
>
> The tags are `think`, `thinking`, `reasoning`, `thought`, `REASONING_SCRATCHPAD`, and the CJK tags `思考`, `反思`, `推理`, `推敲`. `feed` returns the visible part of one delta. A tag split across deltas is held so the opening `<think>` is not emitted on its own. A closed pair is removed wherever it sits. An opening tag that never closes is entered only at the start of the buffer or after a newline, so the prose `see <think> later` is not stripped. `flush` drops an unterminated block. Leaking a partial thought is worse than a short reply. `reset` clears the state for the next turn.
>
> **Adopted.** Those nine tags. `feed`, `flush`, and `reset`. Holding a partial tag. Stripping a closed pair. Leaving an inline `<think>` that never closes. Dropping an unterminated block on flush. A chunk with no `<` and an empty hold buffer is returned as-is.
>
> **Adapted.** The newline-boundary flag and the orphan-close regex are not ported. Case folding is ASCII, which covers the English tags. The CJK tags do not change under that fold. Overlap is a rule on the delta: a read-only tool is recorded while text is still arriving, and a write is stored until `finish`. Hermes starts safe tools from the stream in the tool executor. This chapter does not call that executor.
>
> **Deferred.** The streaming repetition watch from chapter 14's guard stays a full-string check. Provider SSE parsers stay out, with the socket.

[personal note: a think tag that arrived in two chunks, and what leaked before the hold]

## What this chapter decides

`ThinkScrubber::feed("<think>secret</think>hello")` plus `flush` is `hello`.

`feed("<thi")` returns nothing. `feed("nk>secret</think>ok")` returns `ok`. The secret is not in the output.

`<思考>秘密</思考>ok` is `ok`. `see <think> later` is kept, because the tag is not at the start or after a newline and it never closes. A newline, then `<think>secret` with no close, flushes to an empty string.

`Assembler` pushes `Delta::Text` through the scrubber. `Delta::Tool { read_only: true }` is appended to `started` immediately. A write goes to a deferred list. `finish` flushes the scrubber and then appends the deferred names. The test sees `echo` before `finish`, and `echo` then `write` after.

The demo chunks are `hel`, `<thi`, `nk>no</think>`, `lo`, and a read-only `echo`. The reply is `hello`. Started tools are `echo`.

`fold_text` and `buffer_join` on 100 copies of `hello ` are the same string. The fold is the streamed assembly. The join is the buffered reply. Neither calls a model.

## 19.1 `stream` is not in `drive_loop`

The file is `steps/ch19-streaming/crates/harness-core/src/stream.rs`. The module comment is the policy.

```rust
//! Book chapter 19. Stream deltas fold into one reply. Think blocks are
//! stripped as the chunks arrive. A read-only tool may start before the
//! stream finishes. A write waits.
```

`lib.rs` in this step declares `pub mod stream;`. The `pub use` list does not re-export the names. Callers write `harness_core::stream::ThinkScrubber`. This crate is chapter 18 copied forward, so `pub mod adapters;` is still there too. Streaming does not go through the adapters. It has its own module.

I searched `agent.rs` in `steps/ch19-streaming`. The names `stream`, `ThinkScrubber`, `Assembler`, and `fold_text` do not appear. `drive_loop` is unchanged from the previous step. It still takes `&dyn Provider` and still calls `provider.complete(&send)` inside the cancel `select`. `budget_wrap_up` and `soft_wrap_up` each call `provider.complete` the same way. Those three calls are every model call in the file. `complete` returns `Vec<Event>` for a finished reply. It does not return a `Delta`.

That is intentional. Chapter 9's loop, and the evals that drive it, still see one reply string on a `Message`. This module is how a stream of chunks becomes that string, and how a think block is removed while the chunks are still arriving. The tests construct the chunks themselves. Wiring `Assembler` into `drive_loop` would mean the loop consumes deltas the scripted provider does not yield. The folder can wait. The scrubber is already testable.

The callers in this step are `tests/streaming.rs`, `benches/streaming.rs`, and `src/bin/chapter_demo.rs`. Integration tests and the bin sit outside the library, so they use the `pub` items. The tag search helpers are private. The tests reach them only by what `feed` and `flush` return.

`fold_deltas` in the bench is not a second parser for the final answer. It calls `fold_text`. Chapter 9 still sees one reply. The stream is how that reply was assembled. Nothing here opens a socket, and nothing here parses server-sent events.

## 19.2 The nine tags

The tag list is a constant slice of string slices.

```rust
pub const THINK_TAGS: &[&str] = &[
    "think",
    "thinking",
    "reasoning",
    "thought",
    "REASONING_SCRATCHPAD",
    "思考",
    "反思",
    "推理",
    "推敲",
];
```

`&[&str]` is a shared reference to an array of shared string references. The array lives for the whole program. Nothing in the file pushes a tenth tag. Every search walks this slice from the start.

The English names are the words inside the angle brackets. `REASONING_SCRATCHPAD` is stored in uppercase. The search lowercases both the buffer and the tag with `to_ascii_lowercase` before it compares, so `<reasoning_scratchpad>` matches this entry. The four CJK names do not change under ASCII lowercase. They match as written.

`<think>` is not a prefix of `<thinking>`. Both spellings share `<think` and then differ: the shorter tag has `>` where the longer tag has `i`. `str::find("<think>")` does not hit inside `<thinking>`. The same is true of the close tags. `</think>` is not a substring of `</thinking>`, because the `>` does not sit immediately after `k`. The list can contain both names. A buffer that holds a finished `<thinking>` pair is stripped as that longer tag. I say this because the search really is `str::find` of the whole tag, and a reader who sees `think` listed before `thinking` should know the shorter spelling does not eat the longer one.

## 19.3 `ThinkScrubber`: `feed`, `flush`, and `reset`

The scrubber is two fields.

```rust
#[derive(Debug, Default)]
pub struct ThinkScrubber {
    in_block: bool,
    buf: String,
}

impl ThinkScrubber {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.in_block = false;
        self.buf.clear();
    }
```

`Default` sets `in_block` to `false` and `buf` to an empty `String`. `new` is that default. The fields are private. A test cannot set `in_block` from outside. It can only see the strings `feed` and `flush` return.

`reset` forces both fields back to the start and returns nothing. Text sitting in `buf` is dropped, including a partial tag that had not been emitted yet. `clear` keeps the allocation and sets the length to zero. No test calls `reset`. It is the public way to drop state between turns without emitting the tail. `flush` is the one that can still return text.

`feed` is the whole state machine.

```rust
    pub fn feed(&mut self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }
        if !self.in_block && self.buf.is_empty() && !text.contains('<') {
            return text.to_string();
        }
        self.buf.push_str(text);
        let mut out = String::new();
        loop {
            if self.buf.is_empty() {
                break;
            }
            if self.in_block {
                if let Some((idx, len)) = find_tag(&self.buf, false) {
                    self.buf.drain(..idx + len);
                    self.in_block = false;
                    continue;
                }
                let hold = partial_suffix(&self.buf, false);
                self.buf.drain(..self.buf.len() - hold);
                break;
            }
            if let Some((start, end)) = find_closed_pair(&self.buf) {
                out.push_str(&self.buf[..start]);
                self.buf.drain(..end);
                continue;
            }
            if let Some((idx, len)) = find_open_at_boundary(&self.buf) {
                out.push_str(&self.buf[..idx]);
                self.buf.drain(..idx + len);
                self.in_block = true;
                continue;
            }
            let hold = partial_suffix(&self.buf, true);
            let emit = self.buf.len() - hold;
            out.push_str(&self.buf[..emit]);
            self.buf.drain(..emit);
            break;
        }
        out
    }
```

An empty chunk returns `String::new()` before anything else. The hold buffer is not flushed and not appended to. `feed("")` while `buf` holds `<thi` leaves `<thi` there. The caller gets an empty string. No test passes an empty chunk. The branch is first so a blank delta cannot take the fast path or the scanner.

The fast path is the next `if`. All three conditions have to be true: not inside a block, nothing held, and the chunk has no `<`. Then `feed` returns `text.to_string()` and does not call `find_tag`. The tag list is not walked. A plain delta pays one allocation, the copy into the returned `String`, and nothing else. `contains` scans the chunk for the byte `<`. These tags all use `<`. A chunk that hides a tag without that character takes the fast path and is returned as-is. The pitfall section already says a fullwidth bracket is not caught. The condition is the character `<`, ASCII `0x3C`.

Any other chunk falls through. `self.buf.push_str(text)` appends the new bytes onto the hold. `push_str` copies the chunk onto the end of the existing allocation. When the allocation is too small, `String` grows. Growth is amortized: the buffer usually has spare capacity, and a grow copies the old bytes once so the next several appends fit. The next chunk is appended to the suffix already held, so `<thi` plus `nk>` becomes `<think>` in one buffer. Clearing `buf` at the start of every `feed` would drop that suffix. The second half of the tag would arrive with no memory of the first half, and the split-tag test would see `nk>secret` in the output.

> **Rust sidebar: the hold is the tail of the `String`**
> - `buf` is one owned `String`. The bytes at the front are text the scrubber has not decided about yet. The bytes at the end may be the start of a tag.
> - `push_str` adds the new chunk after that tail. The unfinished `<thi` and the next chunk's `nk>` are neighbors in the same allocation, which is what makes them one tag.
> - Emitting means `push_str` of a prefix of `buf` into `out`, then `drain` of that same prefix. `drain(..emit)` removes the bytes the caller is allowed to see and shifts the tail down to index 0. The tail stays in `buf`.
> - The tail's length comes from `partial_suffix`. It is the longest proper prefix of a tag that `buf` currently ends with. A finished tag is not a hold. The scanners above the hold have already had a chance to match it.
> - Slicing `&self.buf[..emit]` requires `emit` to fall on a character boundary. The suffix helper checks that before it reports a hold length. A CJK tag is three bytes per character. Cutting one byte into `思` would panic.

`out` starts empty on every `feed`. It is the visible text from this chunk only, not the whole reply. The caller appends the returned `String` onto its own buffer. The `loop` walks `buf` until it has to wait.

Inside a block, `find_tag(..., false)` looks for any close tag. On a hit, `drain(..idx + len)` throws away everything up to and including that close tag. The text before the close is not copied into `out`. That is the secret disappearing. `in_block` becomes false, and `continue` runs the loop again on whatever followed the close tag. A bool does not remember which open tag started the block. The first close tag in the list-search order that `find` locates ends it. The tests close the same name they opened.

If no close tag is present, `partial_suffix(..., false)` measures a trailing prefix of a close tag only. `drain(..len - hold)` deletes the rest. The secret in front of a partial `</thi` is deleted. `</thi` stays, so the next chunk can finish `</think>`. Then `break`. Nothing from inside the block was pushed to `out`.

Outside a block, the loop tries three things in order.

`find_closed_pair` wins first. A finished `<think>secret</think>` anywhere in the buffer is removed, including after ordinary words. `out.push_str(&self.buf[..start])` keeps the text before the open tag. `drain(..end)` drops the open tag, the inside, and the close tag. `continue` looks at what followed. A second pair in the same buffer is handled on the next turn of the loop. `a<think>x</think>b<think>y</think>c` becomes `abc` by that `continue`. The tests pass one pair. The `continue` is why a second pair does not need a second `feed`.

`find_open_at_boundary` runs only when no closed pair exists. It enters the block when an open tag sits at index 0 or the byte before it is `\n`. Text before that index is kept. The open tag itself is drained and not emitted. `in_block` becomes true, and `continue` processes the inside with the in-block branch, which drops it if the close has not arrived.

`partial_suffix(..., true)` runs when the buffer has no closed pair and no boundary open. `emit = buf.len() - hold` is the visible prefix. That prefix is pushed to `out` and drained. The hold stays. Then `break`, because the next bytes have to come from a later `feed` or from `flush`.

`flush` is the end of the stream.

```rust
    pub fn flush(&mut self) -> String {
        let tail = if self.in_block {
            String::new()
        } else {
            std::mem::take(&mut self.buf)
        };
        self.buf.clear();
        self.in_block = false;
        tail
    }
```

`std::mem::take` replaces `buf` with `String::default()`, which is empty, and returns the old `String`. When `in_block` is false, that old string is the return value. A held partial tag that never became a block is emitted. `feed("<thi")` returns `""` and leaves `<thi` in `buf`. `flush` then returns `"<thi"`. The hold protected the stream while another chunk might still arrive. Flush means no more chunks. A partial that never opened a block is ordinary text again.

When `in_block` is true, `tail` is a fresh empty string. The old `buf` is then `clear`ed, which drops the unclosed thought. `in_block` is set false either way. The following `clear` is redundant after `take`, because `take` already left an empty buffer, and it is what drops the secret when `take` was skipped. An unterminated block comes back as `""`. Leaking that tail would put the thought in the reply. The function drops it.

The newline test shows the split between `feed`'s return and `flush`. `feed("\n<think>secret")` sees an open tag after a newline, emits the `"\n"` that sits before the tag, enters the block, and drops `secret`. The test writes `let _ = scrubber.feed(...)` and asserts `flush()` equals `""`. The newline was already returned by `feed` and discarded by `let _`. `hello\n<think>secret` similarly returns `"hello\n"` from `feed` and `""` from `flush`. The decision line in this chapter talks about `flush`. The bytes before the tag are the `feed` return.

## 19.4 The four searches, and why a partial token waits

The tags are built with two formatters.

```rust
fn open_tag(name: &str) -> String {
    format!("<{name}>")
}

fn close_tag(name: &str) -> String {
    format!("</{name}>")
}
```

Each call allocates. `find_tag` builds nine strings per call, either every open tag or every close tag. `find_closed_pair` builds both for each name, eighteen strings. `partial_suffix` also builds both, and when `open_side` is false it still allocates the open tag before the `continue` skips it. The plain-text bench never gets here. A chunk with a `<` does.

```rust
fn find_tag(buf: &str, open: bool) -> Option<(usize, usize)> {
    let lower = buf.to_ascii_lowercase();
    let mut best: Option<(usize, usize)> = None;
    for name in THINK_TAGS {
        let tag = if open {
            open_tag(name)
        } else {
            close_tag(name)
        };
        let tag_l = tag.to_ascii_lowercase();
        if let Some(idx) = lower.find(&tag_l) {
            let better = best.is_none_or(|(at, _)| idx < at);
            if better {
                best = Some((idx, tag_l.len()));
            }
        }
    }
    best
}
```

`to_ascii_lowercase` allocates a copy and changes only ASCII letters. The copy's length equals the original's, so an index into `lower` is an index into `buf`. CJK bytes are unchanged. `for name in THINK_TAGS` walks the nine names in order. `THINK_TAGS` is `&[&str]`. Iterating it yields `&&str`, and `open_tag` accepts `&str`, so the extra reference is derefenced by the argument.

`lower.find(&tag_l)` returns `Option<usize>`, the byte index of the first match, or `None`. The length stored beside it is `tag_l.len()`, in bytes. `<think>` is 7 bytes. `<思考>` is 8 bytes: `<`, then `思` (3), then `考` (3), then `>`.

`best.is_none_or(|(at, _)| idx < at)` is true when no tag has matched yet, or when this index is strictly smaller. An equal index keeps the earlier tag in the list. The first name in `THINK_TAGS` wins a tie. The tests do not build a buffer where two different tags start at the same byte. The comparison is still `<`, not `<=`.

`find` returns the earliest close tag when `open` is false, and the earliest open tag when `open` is true. That is what the in-block branch uses to decide how far to drain.

```rust
fn find_closed_pair(buf: &str) -> Option<(usize, usize)> {
    let lower = buf.to_ascii_lowercase();
    let mut best: Option<(usize, usize)> = None;
    for name in THINK_TAGS {
        let open = open_tag(name).to_ascii_lowercase();
        let close = close_tag(name).to_ascii_lowercase();
        let Some(start) = lower.find(&open) else {
            continue;
        };
        let Some(rel) = lower[start + open.len()..].find(&close) else {
            continue;
        };
        let end = start + open.len() + rel + close.len();
        let better = best.is_none_or(|(at, _)| start < at);
        if better {
            best = Some((start, end));
        }
    }
    best
}
```

This one requires both tags, and the close has to occur after the open of the same name. `let Some(start) = ... else { continue }` skips this name when the open tag is absent. The slice `lower[start + open.len()..]` is the bytes after the open tag, so a close tag that somehow overlapped the open tag is not considered. `rel` is the index of the close tag inside that tail. `end` is the byte just past the close tag. The returned pair is `(start, end)`, and `feed` drains `..end`, which is exclusive. The inside, from `start + open.len()` to the close, is inside the drained range. It is not copied to `out`.

The earliest `start` wins. A later pair in the same buffer waits for the `continue` in `feed`. `<THINK>secret</THINK>ok` matches because both the buffer and the tags are lowercased: the open becomes `<think>` and the close becomes `</think>`. The test uses lowercase. The lowercase copy is why uppercase works too. ASCII lowercase does not fold characters outside `A`–`Z`. The CJK tags do not need it.

`<思考>秘密</思考>ok` is a closed pair at byte 0. The open tag is 8 bytes, `秘密` is 6 bytes, the close tag is 9 bytes, and `end` is 23. `feed` keeps `buf[..0]`, drains 23 bytes, and the next loop iteration emits `ok`. The test locks that result.

```rust
fn find_open_at_boundary(buf: &str) -> Option<(usize, usize)> {
    let (idx, len) = find_tag(buf, true)?;
    if idx == 0 || buf.as_bytes().get(idx - 1) == Some(&b'\n') {
        Some((idx, len))
    } else {
        None
    }
}
```

`?` returns `None` from this function when `find_tag` finds no open tag. Otherwise the tag counts only at the start of the buffer, or when the byte immediately before it is `\n` (`b'\n'`, byte `0x0A`). `idx == 0` is checked first, so the subtraction `idx - 1` does not underflow. `as_bytes().get` returns `Option<&u8>`. A missing byte is `None`, and `None` fails the comparison.

The byte before the tag is the rule. A space fails it. `see <think> later` has a space before `<`, `find_open_at_boundary` returns `None`, and there is no close tag, so the hold logic emits the whole string. The test locks `see <think> later`. A carriage return fails the check too. The source compares against `\n` only. `\r\n<think>` still matches, because the byte directly before `<` is `\n`.

An open tag that fails this test is left for `partial_suffix`. If the tag is already complete, `partial_suffix` will not hold it. The `>` is emitted. That is why an inline `<think>` that never closes survives `flush`. A closed pair never reaches this function: `feed` tries `find_closed_pair` first. `see <think>secret</think>ok` loses the pair and keeps `see ok`. The boundary rule saves an unclosed mention. It does not save a mention that has its close tag in the same buffer.

```rust
fn partial_suffix(buf: &str, open_side: bool) -> usize {
    let lower = buf.to_ascii_lowercase();
    let mut best = 0usize;
    for name in THINK_TAGS {
        for tag in [open_tag(name), close_tag(name)] {
            if !open_side && !tag.starts_with("</") {
                continue;
            }
            let tag_l = tag.to_ascii_lowercase();
            let mut len = 0usize;
            for ch in tag_l.chars() {
                let next = len + ch.len_utf8();
                if next >= tag_l.len() || next > lower.len() {
                    break;
                }
                if lower.is_char_boundary(lower.len() - next)
                    && buf.is_char_boundary(buf.len() - next)
                    && lower.ends_with(&tag_l[..next])
                {
                    best = best.max(next);
                }
                len = next;
            }
        }
    }
    best
}
```

The return value is a byte length. `0` means "emit the whole buffer." A positive number is how many bytes at the end of `buf` must wait.

`open_side == false` is the in-block call. The `continue` skips open tags, because `open_tag` does not start with `</`. Only a partial close tag can be held while the secret is being dropped. `open_side == true` is the outside call. Both open and close tags are candidates. A trailing `</thi` is held outside a block too, so a close tag split across chunks is not emitted early.

The inner `for ch in tag_l.chars()` walks Unicode scalar values, not bytes. `ch.len_utf8()` is 1 for `<` and for ASCII letters, and 3 for `思`. `next` is the byte length of the prefix of the tag considered so far. Two conditions abort that tag before a match is recorded. `next >= tag_l.len()` refuses the full tag. A finished `<think>` is seven bytes, and the loop breaks when `next` reaches 7, so the hold is at most `<think` without the `>`. `next > lower.len()` means the prefix is already longer than the buffer.

The three-part `if` records the prefix only when it is safe to slice and the buffer actually ends with it. `is_char_boundary` is true when the index is not in the middle of a UTF-8 character. Both the lowercased copy and the original `buf` are checked, at `len - next`. `ends_with` then asks whether those last `next` bytes equal the tag prefix. `best.max(next)` keeps the longest success. A buffer can end with a prefix of several tags at once. `<thi` is a prefix of `<think>` and of `<thinking>`. Both yield length 4. `best` stays 4.

That is why a partial token is held back. `<thi` is not yet `<think>`, and it is not yet ordinary text. Emitting it would show the start of a tag the next chunk might finish. If the next chunk is `nk>secret</think>ok`, the held bytes plus the new bytes are one buffer, `find_closed_pair` removes the block, and the caller sees `ok`. If `feed` had returned `<thi` immediately, that return value would already be in the reply, and the secret test would fail. The hold is the memory between those two chunks.

`feed("<thi")` takes the slow path, finds no pair and no boundary open, holds 4 bytes, emits nothing, and leaves `<thi` in `buf`. The following `feed("nk>secret</think>ok")` appends, strips the pair, and returns `ok`. `flush` adds nothing. The joined output is `ok`, and `secret` is not in it. That is the test.

A chunk that ends in `<` holds 1 byte. Open tags and close tags both start with `<`. `text<` emits `text` and keeps `<`. The next chunk decides whether that `<` was a tag. `flush` with no further chunk returns the `<`, because `in_block` is still false. The character was held for one chunk and then released.

`<思` holds 4 bytes: one for `<`, three for `思`. The boundary checks are what make that cut legal. `buf.len() - 4` lands on a character boundary. `buf.len() - 1` would land inside `思`, `is_char_boundary` would be false, and that length would not be stored in `best`. The slice in `feed` never sees it.

## 19.5 `Delta` and `Assembler`

Deltas are an enum. Text and tools are different shapes, so one vec can carry both without a flag field that is sometimes meaningless.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delta {
    Text(String),
    Tool { name: String, read_only: bool },
}
```

> **Rust sidebar: `match` takes the enum apart**
> - `Delta::Text(String)` holds one string. `Delta::Tool { name, read_only }` holds a name and a bool. A value is one of those, never both.
> - `match delta` names the variant and binds the fields. The compiler requires every variant. Forgetting `Tool` is a compile error, which is the point of using an enum here.
> - `Clone` and `PartialEq` are derived so a test can build a delta and compare started-tool lists. `push` takes the delta by value. The `String` inside moves into the scrubber or into a vec. It is not borrowed from the caller after `push` returns.

```rust
#[derive(Debug, Default)]
pub struct Assembler {
    scrubber: ThinkScrubber,
    pub text: String,
    pub started: Vec<String>,
    deferred: Vec<String>,
}

impl Assembler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, delta: Delta) {
        match delta {
            Delta::Text(chunk) => self.text.push_str(&self.scrubber.feed(&chunk)),
            Delta::Tool { name, read_only } => {
                if read_only {
                    self.started.push(name);
                } else {
                    self.deferred.push(name);
                }
            }
        }
    }

    pub fn finish(&mut self) -> &str {
        self.text.push_str(&self.scrubber.flush());
        self.started.append(&mut self.deferred);
        &self.text
    }
}
```

`scrubber` and `deferred` are private. `text` and `started` are public, so the overlap test can read `started` before `finish`. `Default` builds an empty scrubber, an empty reply, and two empty vecs.

`push` on `Delta::Text` feeds the chunk through the scrubber and `push_str`s the visible part onto `text`. The tool name does not go through the scrubber. A name that contained `<think>` would be stored as the tool name. No test does that. Text and tools are separate match arms so a tool cannot be stripped as if it were prose, and prose cannot be recorded as a started tool.

`read_only == true` pushes the name onto `started` immediately. The stream can still be open. `read_only == false` pushes onto `deferred`. The write's name sits there until `finish`. This chapter does not run the tool. It records the name. Starting a write on a partial argument string would run a tool whose JSON was not closed. The assembler never sees argument JSON. The bool is the whole gate, the same read/write split as chapter 3, applied to a reply that is not done yet. The dispatcher from chapter 3 is not called.

`finish` flushes the scrubber into `text` first, so a held visible tail joins the reply before the function returns. Then `started.append(&mut self.deferred)` moves every deferred name onto the end of `started` and leaves `deferred` empty. `append` on `Vec` takes `&mut Vec` and drains the source. Order is the reads that already started, then the writes in the order they were pushed. The test pushes `echo` (read) and then `write`, sees `started == ["echo"]` before `finish`, and sees `["echo", "write"]` after.

The return value is `&self.text`, a borrow of the assembler's string. The caller has to finish using that borrow before the next `&mut self` call. `demo_reply` does `assembler.finish().to_string()`, which copies the reply into an owned `String` so the borrow ends, and then reads `assembler.started`. A second `finish` flushes an empty scrubber and appends an empty `deferred`. The text stays as it was. The tests call `finish` once.

## 19.6 Folding, joining, and the demo

Two functions build one string from the same slice of chunks. One of them scrubs.

```rust
pub fn buffer_join(chunks: &[String]) -> String {
    let mut out = String::new();
    for chunk in chunks {
        out.push_str(chunk);
    }
    out
}

pub fn fold_text(chunks: &[String]) -> String {
    let mut scrubber = ThinkScrubber::new();
    let mut out = String::new();
    for chunk in chunks {
        out.push_str(&scrubber.feed(chunk));
    }
    out.push_str(&scrubber.flush());
    out
}
```

`buffer_join` is the buffered reply. The `for` loop is an iterator over `&String`. `push_str` copies each chunk into `out` once. No scrubber is constructed. No chunk is scanned for `<`.

`fold_text` is the streamed assembly. It owns a fresh scrubber. Each `feed` returns a new `String`, and `push_str` copies that into `out`. On the fast path, `feed` has already done `text.to_string()`, so a plain chunk is allocated as the `feed` result and then copied again into `out`. `flush` after the loop appends whatever the hold still had. For a slice of `hello ` with no `<`, each `feed` takes the fast path, `flush` returns an empty string, and the joined text equals `buffer_join`. The test builds 100 copies and asserts that equality. The bench times the same two functions. The fold is allowed to cost more. It is not allowed to return a different string on this input.

The demo is the split tag plus one read-only tool.

```rust
pub fn demo_reply() -> (String, Vec<String>) {
    let mut assembler = Assembler::new();
    for delta in [
        Delta::Text("hel".into()),
        Delta::Text("<thi".into()),
        Delta::Text("nk>no</think>".into()),
        Delta::Text("lo".into()),
        Delta::Tool {
            name: "echo".into(),
            read_only: true,
        },
    ] {
        assembler.push(delta);
    }
    let text = assembler.finish().to_string();
    (text, assembler.started)
}
```

The array is five deltas. `for delta in [...]` moves each one into `push`. Walk them.

`Text("hel")` has no `<`, the scrubber is empty and outside a block, and `feed` returns `"hel"`. `text` is `"hel"`.

`Text("<thi")` misses the fast path. No closed pair, no boundary open, hold length 4. `feed` returns `""`. `buf` is `"<thi"`. `text` is still `"hel"`.

`Text("nk>no</think>")` appends, so `buf` is `"<think>no</think>"`. `find_closed_pair` covers the whole buffer. The text before the pair is empty. The pair, including `no`, is drained. `feed` returns `""`. `text` is still `"hel"`. The secret in this demo is the word `no`. It never reaches `text`.

`Text("lo")` is a fast path again. `text` becomes `"hello"`.

`Tool { name: "echo", read_only: true }` pushes `"echo"` onto `started`. It does not touch the scrubber.

`finish` flushes an empty hold, appends an empty deferred list, and `to_string` copies `"hello"`. The returned vec is `assembler.started`, which is `["echo"]`. The test `the_demo_drops_the_split_think_block` locks both.

The binary is one call.

```rust
fn main() {
    let (reply, started) = demo_reply();
    println!("reply: {reply}");
    println!("started: {}", started.join(","));
}
```

`join(",")` puts a comma between names. One name prints as `echo` with no comma. The milestone output is `reply: hello` and `started: echo`. No key.

## 19.7 What each test locks

Five tests, all through the public functions.

`a_closed_think_block_is_removed_and_a_split_tag_does_not_leak` feeds `"<think>secret</think>hello"`, flushes, and expects `"hello"`. On a second scrubber it feeds `"<thi"` and then `"nk>secret</think>ok"`, flushes, and expects `"ok"`. It also asserts `!out.contains("secret")`. The first call is a pair in one chunk. The second is the hold: the first `feed` contributes nothing visible, and the secret is gone after the close tag arrives.

`cjk_tags_are_stripped_and_an_inline_mention_is_kept_until_it_closes` feeds `"<思考>秘密</思考>ok"`, flushes, and expects `"ok"`. It feeds `"see <think> later"`, flushes, and expects that same string back. It feeds `"\n<think>secret"`, ignores that return value, and expects `flush()` to be `""`. The function name says an inline mention is kept until it closes. The asserts cover an unclosed inline mention. A closed pair in the same buffer is removed wherever it sits, which is `find_closed_pair` running before the boundary check. This test does not feed `see <think>secret</think>ok`.

`a_read_starts_during_the_stream_and_a_write_waits` pushes `Text("hi")`, a read-only `echo`, and a write named `write`. Before `finish`, `started` must equal `["echo"]`. `finish` must return `"hi"`. After `finish`, `started` must equal `["echo", "write"]`. The write was stored on `deferred` and moved by `append`.

`the_demo_drops_the_split_think_block` expects `demo_reply()` to be `("hello", ["echo"])`.

`folding_plain_chunks_matches_a_buffer` builds `(0..100).map(|_| "hello ".into())`. The range is an iterator. `map` turns each integer into the same six-character string. The integer itself is unused, which is what `_` means. `collect` builds the `Vec`. `fold_text` and `buffer_join` must return equal strings. One hundred copies of `hello ` are 600 characters, no `<`, so every `feed` takes the fast path and `flush` adds nothing.

## 19.8 What the two benches time

`benches/streaming.rs` registers `fold_deltas` and `buffer_join`. Those strings are the benchmark names in `benchmarks/descriptions.json`. The means are in the table below. The source is short enough to read as the measurement.

```rust
fn chunks() -> Vec<String> {
    (0..100).map(|_| "hello ".to_string()).collect()
}

fn fold_deltas(c: &mut Criterion) {
    let chunks = chunks();
    c.bench_function("fold_deltas", |b| {
        b.iter(|| {
            let text = fold_text(black_box(&chunks));
            black_box(text);
        });
    });
}

fn buffer_join_bench(c: &mut Criterion) {
    let chunks = chunks();
    c.bench_function("buffer_join", |b| {
        b.iter(|| {
            let text = buffer_join(black_box(&chunks));
            black_box(text);
        });
    });
}
```

`chunks()` runs before `b.iter`. Building the vec is outside the sample in both benches. Each string is `hello `, six bytes, no `<`. `fold_text` constructs a `ThinkScrubber`, calls `feed` 100 times, and calls `flush`. Each `feed` takes the fast path: `contains('<')` is false, and the chunk is copied with `to_string`. `fold_text` then `push_str`s that copy into its output. `buffer_join` `push_str`s each original chunk once and does not construct a scrubber. `black_box` stops the compiler from deleting the work or from lifting it out of the sample. The two benches do not share one `Vec` at runtime. Each function calls `chunks()` itself. The strings are the same.

`criterion_group!(benches, fold_deltas, buffer_join_bench)` registers both. The bench function is named `buffer_join_bench` so it does not collide with the library function `buffer_join`. The measured name is the string `"buffer_join"`. The budget is the gap. Folding these 100 chunks should stay within 1 ms of the join. Both numbers exclude model time. There is no SSE parser in the sample. A gap of milliseconds would mean the plain chunks had started walking `THINK_TAGS`. They do not contain `<`, so `find_tag` is not in the sample.

## Tests

Five tests. A closed block is removed, and a tag split across two feeds does not leak. CJK tags are stripped, an inline mention is kept, and an unterminated block at a newline flushes empty. A read starts during the stream and a write waits for `finish`. The demo reply is `hello`. Folding the 100 plain chunks matches the buffer.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch19-streaming
```

```text
Starting 105 tests across 26 binaries
     Summary [   0.186s] 105 tests run: 105 passed, 0 skipped
```

```bash
cargo run -p ch19-streaming
```

```text
reply: hello
started: echo
```

The binary is the split think block plus one read-only tool. No key.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `fold_deltas` | 2.67 µs | within 1 ms of `buffer_join` | scrub and join 100 plain chunks |
| `buffer_join` | 689.3 ns | the buffered baseline | join the same 100 chunks, no scrubber |

`fold_deltas` builds the 100 strings before the bench. Each is `hello `, six characters, no `<`. The timed closure is `b.iter`. Inside: `fold_text` on that slice passed through `black_box`, which constructs a scrubber, calls `feed` 100 times, and `flush`. The fast path returns each chunk without scanning tags. Building the vec is excluded.

`buffer_join` uses the same vec, built before the bench. Each sample concatenates the 100 strings and does not construct a scrubber.

The budget is the difference. Assembling the streamed reply should add less than 1 ms over the buffered join. Both numbers exclude model time. There is no SSE parser in the sample.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 39104733918](https://github.com/agnathan/rust-agent-harness/actions/runs/39104733918), section `17`.

### Milestone demo

The `cargo run` above. The reply is `hello`. `echo` started.

## Pitfalls

The fast path is wrong for a chunk that hides a tag without a `<`. These tags all use `<`. A model that emits a fullwidth bracket is not caught. Hermes folds some of that with NFKC in the threat scanner, not in this scrubber.

An inline `<think>` that never closes is kept. That is the boundary rule. Do not point a one-line mention of the tag at this function and expect it to disappear.

`flush` drops an open block. If you needed the thought for a log, it is already gone. `last_hidden` from Hermes is not stored here.

A write recorded during the stream does not run. `finish` only moves the name onto `started`. This chapter does not execute tools.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L3.

**K5.** `ThinkScrubber` strips the nine tags in `agent/think_scrubber.py`, including a tag split across deltas. An unterminated block is dropped on flush.

**Overlap.** A read-only tool is recorded while the stream is open. A write waits until `finish`. The tool body is not this chapter.

**Left for later.** The repetition guard on a finished string stays chapter 14. A streaming runaway watch is not ported. Secret redaction (E1, E2) is still absent.

## Chapter summary

- **Fold.** Deltas become one string. A split `<think>` does not leak.
- **Tags.** English and the four CJK tags. An inline mention without a close is kept.
- **Overlap.** `echo` starts. `write` waits for `finish`.
- **Time.** Folding 100 plain chunks measured **2.67 µs**. Joining them measured **689.3 ns**.

[personal note: a stream that showed the thought before the close tag arrived]

## What you implement

`my-harness/tests/ch19.rs` is the milestone. It compiles, then panics in `todo!("chapter 19")` until `chapter_19_fold_matches_buffer(&["hel", "lo"])` is true. Folding those two plain chunks has to produce the same text as joining them. The step crate is the reference. Do not copy it into `my-harness/`.

## What's next

Chapter 20 puts the session on disk. One turn is a WAL append. Search stays a background job until chapter 22. The chapter 9 eval set runs again on the mock.
