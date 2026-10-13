use std::sync::Arc;
use std::time::{Duration, Instant};

use harness_core::{Dispatcher, LatencyTool, Tool, ToolCall, ToolKind, ToolRegistry};
use serde_json::json;

/// Eight read tools that each wait 20 ms, run together and then one at a time.
/// The waits are the mock's stand-in for tool latency. Nothing here calls a model.
#[tokio::main]
async fn main() {
    let delay = Duration::from_millis(20);
    let mut registry = ToolRegistry::new();
    let calls = (0..8)
        .map(|index| {
            let name = format!("read-{index}");
            registry
                .register(
                    Arc::new(LatencyTool::new(name.clone(), ToolKind::Read, delay))
                        as Arc<dyn Tool>,
                )
                .expect("register");
            ToolCall {
                name,
                arguments: json!({}),
            }
        })
        .collect::<Vec<_>>();
    let dispatcher = Dispatcher::new(8);
    let started = Instant::now();
    let outputs = dispatcher.run(&registry, &calls).await;
    let concurrent = started.elapsed();
    let started = Instant::now();
    dispatcher.run_sequential(&registry, &calls).await;
    let sequential = started.elapsed();
    for output in outputs {
        let text = output.result.expect("read").content[0].text.clone();
        println!("{text}");
    }
    println!("concurrent_ms: {:.1}", concurrent.as_secs_f64() * 1000.0);
    println!("sequential_ms: {:.1}", sequential.as_secs_f64() * 1000.0);
}
