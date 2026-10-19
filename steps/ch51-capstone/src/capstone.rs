//! Chapter 51. Three token arms and an in-memory load. No socket.
//! The arms and the load call the chapter 8 session gateway that this crate vendors.

use std::time::Instant;

use serde_json::json;

use crate::catalog::{synthetic, Load};
use crate::session::{self, approx_tokens, SessionGateway};

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

pub fn schema_tokens(schema: &str) -> u64 {
    approx_tokens(schema)
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

/// The three arms on the real session gateway. Each report has tokens and a success bit.
pub fn task_arms(count: usize) -> [session::TaskReport; 3] {
    [
        session::run_rename_task(session::Arm::Every, count),
        session::run_rename_task(session::Arm::Static, count),
        session::run_rename_task(session::Arm::Dynamic, count),
    ]
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

/// One hundred in-memory calls through `SessionGateway`. Nothing listens.
pub fn load(requests: u32) -> LoadReport {
    let gateway = SessionGateway::new();
    let cards = synthetic(CATALOG_LEN);
    gateway
        .reload(Load {
            native: cards,
            plugins: Vec::new(),
            upstreams: Vec::new(),
        })
        .expect("reload");
    gateway.open_session("load");
    let listing = gateway
        .search_tools("load", &["file".into()])
        .expect("search");
    let name = listing
        .lines()
        .next()
        .and_then(|line| line.split(" — ").next())
        .expect("tool")
        .to_string();
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
        let result = gateway
            .call("load", &name, &json!({"text": "a.txt"}))
            .expect("call");
        if !result.is_empty() {
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
    let tasks = task_arms(CATALOG_LEN);
    let report = load(100);
    format!(
        "capstone: every={} static={} dynamic={} success={}/{}/{} task={}/{}/{} load={}/{} waves={}",
        arms[0].tokens,
        arms[1].tokens,
        arms[2].tokens,
        arms[0].success as u8,
        arms[1].success as u8,
        arms[2].success as u8,
        tasks[0].success as u8,
        tasks[1].success as u8,
        tasks[2].success as u8,
        report.successes,
        report.requests,
        report.waves
    )
}
