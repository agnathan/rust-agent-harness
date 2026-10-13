use std::sync::Arc;

use harness_core::{
    Budget, CancelFlag, Dispatcher, ScriptedProvider, ScriptedReply, Tool, ToolError, ToolKind,
    ToolRegistry, ToolResult, ToolSchema,
};
use serde_json::{json, Value};

struct SoftOnCall {
    flag: CancelFlag,
}

#[async_trait::async_trait]
impl Tool for SoftOnCall {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "soft_on_call".into(),
            description: "Sets the soft stop.".into(),
            input_schema: json!({"type": "object"}),
        }
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    async fn call(&self, _arguments: Value) -> Result<ToolResult, ToolError> {
        self.flag.soft_stop();
        Ok(ToolResult::text("finished the call"))
    }
}

#[tokio::main]
async fn main() {
    let cancel = CancelFlag::new();
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(SoftOnCall {
            flag: cancel.clone(),
        }) as Arc<dyn Tool>)
        .expect("register");
    let provider = ScriptedProvider::new(vec![
        ScriptedReply::tool("call-1", "soft_on_call", json!({})),
        ScriptedReply::text("wrapping up"),
    ]);
    let mut messages = vec![harness_core::Message::user("go")];
    let outcome = harness_core::run_loop(
        &mut messages,
        &provider,
        &registry,
        &Dispatcher::default(),
        &Budget::default(),
        &cancel,
        std::time::Instant::now(),
    )
    .await;
    for event in &outcome.events {
        println!("{}", event.report_line());
    }
    println!("stop: {:?}", outcome.stop);
    println!("final: {}", messages.last().unwrap().content);
}
