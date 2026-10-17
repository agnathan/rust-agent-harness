//! Book chapter 47. No socket. Three arms count schema characters.
//! A load of in-memory dispatches reports successes and a nearest-rank p99.

use std::time::Instant;

/// Hermes `tools/tool_search_catalog.py` uses 4 characters per token.
pub const CHARS_PER_TOKEN: usize = 4;
pub const CALL_BOUND: usize = 8;
pub const CATALOG_LEN: usize = 32;
pub const STATIC_LEN: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub name: String,
    pub schema: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arm {
    pub name: &'static str,
    pub tokens: u64,
    pub success: bool,
    pub offered: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadReport {
    pub requests: u32,
    pub successes: u32,
    pub p99_ns: u128,
    pub elapsed_ns: u128,
    pub waves: u32,
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

pub fn schema_tokens(schema: &str) -> u64 {
    (schema.len() / CHARS_PER_TOKEN) as u64
}

pub fn sample_catalog() -> Vec<Card> {
    (0..CATALOG_LEN)
        .map(|index| {
            let name = format!("tool_{index:02}");
            Card {
                schema: format!(r#"{{"name":"{name}"}}"#),
                name,
            }
        })
        .collect()
}

fn tokens_of(cards: &[&Card]) -> u64 {
    cards.iter().map(|card| schema_tokens(&card.schema)).sum()
}

fn offers_needed(cards: &[&Card], needed: &str) -> bool {
    cards.iter().any(|card| card.name == needed)
}

pub fn compare(catalog: &[Card], needed: &str, deadline: Option<u64>) -> [Arm; 3] {
    let every: Vec<&Card> = catalog.iter().collect();
    let static_set: Vec<&Card> = catalog.iter().take(STATIC_LEN).collect();
    let dynamic: Vec<&Card> = catalog.iter().filter(|card| card.name == needed).collect();
    [
        arm("every", &every, needed, deadline),
        arm("static", &static_set, needed, deadline),
        arm("dynamic", &dynamic, needed, deadline),
    ]
}

fn arm(name: &'static str, cards: &[&Card], needed: &str, deadline: Option<u64>) -> Arm {
    let tokens = tokens_of(cards);
    let present = offers_needed(cards, needed);
    let within = deadline.is_none_or(|limit| tokens <= limit);
    Arm {
        name,
        tokens,
        success: present && within,
        offered: cards.len(),
    }
}

pub fn admit(in_flight: usize) -> bool {
    in_flight < CALL_BOUND
}

/// Nearest rank. For 100 samples the index is 98.
pub fn p99(mut samples: Vec<u128>) -> u128 {
    if samples.is_empty() {
        return 0;
    }
    samples.sort_unstable();
    let rank = samples.len().saturating_mul(99).div_ceil(100);
    let index = rank.saturating_sub(1);
    samples[index]
}

pub fn load(catalog: &[Card], requests: u32) -> LoadReport {
    let model = FakeModel { reply: "tool_07" };
    let mut samples = Vec::with_capacity(requests as usize);
    let mut successes = 0u32;
    let mut waves = 0u32;
    let mut in_flight = 0usize;
    let started = Instant::now();
    for _ in 0..requests {
        if in_flight == 0 {
            waves += 1;
        }
        assert!(admit(in_flight));
        in_flight += 1;
        let one = Instant::now();
        let arms = compare(catalog, "tool_07", None);
        let reply = model.complete("tool_07");
        if arms.iter().all(|arm| arm.success) && reply == "tool_07" {
            successes += 1;
        }
        samples.push(one.elapsed().as_nanos());
        if in_flight == CALL_BOUND {
            in_flight = 0;
        }
    }
    LoadReport {
        requests,
        successes,
        p99_ns: p99(samples),
        elapsed_ns: started.elapsed().as_nanos(),
        waves,
    }
}

pub fn demo() -> String {
    let catalog = sample_catalog();
    let arms = compare(&catalog, "tool_07", None);
    let report = load(&catalog, 100);
    format!(
        "capstone: every={} static={} dynamic={} success={}/{}/{} load={}/{} p99_ns={}",
        arms[0].tokens,
        arms[1].tokens,
        arms[2].tokens,
        arms[0].success as u8,
        arms[1].success as u8,
        arms[2].success as u8,
        report.successes,
        report.requests,
        report.p99_ns
    )
}
