use std::sync::Arc;
use std::time::{Duration, Instant};

use harness_core::{
    CallOutput, Dispatcher, EchoTool, LatencyTool, StaticTool, Tool, ToolCall, ToolKind,
    ToolRegistry, DISPATCH_BOUND,
};
use serde_json::json;

fn reads(count: usize, delay: Duration) -> (ToolRegistry, Vec<ToolCall>, Vec<Arc<LatencyTool>>) {
    let mut registry = ToolRegistry::new();
    let mut tools = Vec::new();
    let mut calls = Vec::new();
    for index in 0..count {
        let name = format!("read-{index}");
        let tool = Arc::new(LatencyTool::new(name.clone(), ToolKind::Read, delay));
        registry
            .register(Arc::clone(&tool) as Arc<dyn Tool>)
            .unwrap();
        tools.push(tool);
        calls.push(ToolCall {
            name,
            arguments: json!({}),
        });
    }
    (registry, calls, tools)
}

async fn time_of(
    fut: impl std::future::Future<Output = Vec<CallOutput>>,
) -> (Duration, Vec<CallOutput>) {
    let started = Instant::now();
    let outputs = fut.await;
    (started.elapsed(), outputs)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn eight_reads_finish_in_about_one_call() {
    let delay = Duration::from_millis(30);
    let (registry, calls, _) = reads(8, delay);
    let dispatcher = Dispatcher::new(DISPATCH_BOUND);
    let (concurrent, outputs) = time_of(dispatcher.run(&registry, &calls)).await;
    let (sequential, _) = time_of(dispatcher.run_sequential(&registry, &calls)).await;
    assert_eq!(outputs.len(), 8);
    assert!(
        concurrent < delay * 4,
        "concurrent {concurrent:?} should be about one call ({delay:?})"
    );
    assert!(
        sequential > delay * 6,
        "sequential {sequential:?} should be about eight calls"
    );
    assert!(
        concurrent * 2 < sequential,
        "concurrent {concurrent:?} sequential {sequential:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_write_does_not_overlap_other_calls() {
    let delay = Duration::from_millis(20);
    let mut registry = ToolRegistry::new();
    let before = Arc::new(LatencyTool::new("before", ToolKind::Read, delay));
    let write = Arc::new(LatencyTool::new("write", ToolKind::Write, delay));
    let after = Arc::new(LatencyTool::new("after", ToolKind::Read, delay));
    registry
        .register(Arc::clone(&before) as Arc<dyn Tool>)
        .unwrap();
    registry
        .register(Arc::clone(&write) as Arc<dyn Tool>)
        .unwrap();
    registry
        .register(Arc::clone(&after) as Arc<dyn Tool>)
        .unwrap();
    let calls = ["before", "write", "after"]
        .into_iter()
        .map(|name| ToolCall {
            name: name.into(),
            arguments: json!({}),
        })
        .collect::<Vec<_>>();
    let outputs = Dispatcher::new(8).run(&registry, &calls).await;
    assert_eq!(
        outputs
            .iter()
            .map(|output| output.name.as_str())
            .collect::<Vec<_>>(),
        ["before", "write", "after"]
    );
    let before_span = before.spans()[0];
    let write_span = write.spans()[0];
    let after_span = after.spans()[0];
    assert!(
        before_span.1 <= write_span.0,
        "write started before the earlier read finished"
    );
    assert!(
        write_span.1 <= after_span.0,
        "a later read started before the write finished"
    );
}

#[tokio::test]
async fn results_stay_in_call_order_and_unknown_tools_fail() {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(EchoTool) as Arc<dyn Tool>)
        .unwrap();
    registry
        .register(StaticTool::arc("note", ToolKind::Write))
        .unwrap();
    let calls = vec![
        ToolCall {
            name: "echo".into(),
            arguments: json!({"text": "hi"}),
        },
        ToolCall {
            name: "missing".into(),
            arguments: json!({}),
        },
        ToolCall {
            name: "note".into(),
            arguments: json!({}),
        },
    ];
    let outputs = Dispatcher::default().run(&registry, &calls).await;
    assert_eq!(outputs[0].result.as_ref().unwrap().content[0].text, "hi");
    assert!(outputs[1]
        .result
        .as_ref()
        .unwrap_err()
        .to_string()
        .contains("unknown"));
    assert_eq!(outputs[2].result.as_ref().unwrap().content[0].text, "note");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_bound_limits_a_read_wave() {
    let delay = Duration::from_millis(20);
    let (registry, calls, tools) = reads(4, delay);
    let (elapsed, _) = time_of(Dispatcher::new(2).run(&registry, &calls)).await;
    assert!(elapsed < delay * 5, "took {elapsed:?}");
    let spans: Vec<_> = tools.iter().flat_map(|tool| tool.spans()).collect();
    assert_eq!(spans.len(), 4);
    let max_in_flight = spans
        .iter()
        .map(|span| {
            spans
                .iter()
                .filter(|other| other.0 <= span.0 && span.0 < other.1)
                .count()
        })
        .max()
        .unwrap();
    assert_eq!(
        max_in_flight, 2,
        "the bound should keep exactly two reads in flight"
    );
}
