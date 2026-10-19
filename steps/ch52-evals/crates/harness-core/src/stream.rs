//! Book chapter 19. Stream deltas fold into one reply. Think blocks are
//! stripped as the chunks arrive. A read-only tool may start before the
//! stream finishes. A write waits.

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
}

fn open_tag(name: &str) -> String {
    format!("<{name}>")
}

fn close_tag(name: &str) -> String {
    format!("</{name}>")
}

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

fn find_open_at_boundary(buf: &str) -> Option<(usize, usize)> {
    let (idx, len) = find_tag(buf, true)?;
    if idx == 0 || buf.as_bytes().get(idx - 1) == Some(&b'\n') {
        Some((idx, len))
    } else {
        None
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delta {
    Text(String),
    Tool { name: String, read_only: bool },
}

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
