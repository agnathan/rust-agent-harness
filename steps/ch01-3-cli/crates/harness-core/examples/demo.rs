use harness_core::{AgentUsage, Usage};

fn main() {
    let mut ledger = AgentUsage::default();
    ledger.record(&Usage {
        prompt_tokens: 5,
        completion_tokens: 2,
        total_tokens: 7,
        cost: Some(0.25),
    });
    ledger.record(&Usage {
        prompt_tokens: 1,
        completion_tokens: 1,
        total_tokens: 2,
        cost: None,
    });
    let cost = if ledger.cost_known {
        harness_core::format_cost(ledger.cost_usd)
    } else {
        "unknown".to_string()
    };
    println!(
        "agent: calls={} prompt={} completion={} total={} cost={cost}",
        ledger.calls, ledger.prompt_tokens, ledger.completion_tokens, ledger.total_tokens
    );
}
