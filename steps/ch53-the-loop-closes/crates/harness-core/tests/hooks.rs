use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use harness_core::approval::Session;
use harness_core::hooks::{
    cron_policy, dispatch_post, dispatch_pre, empty_pre_post, escalation_block, gate_failed,
    object_map, run_api_hook, run_script, ApiHook, BlockName, HookError, MemoryBackend,
    MemoryQueue, Message, NopHook, PreAction, Registry, ScriptedCall, ToolHook, TurnRecord,
};
use serde_json::{json, Value};

struct ScriptHook {
    actions: Vec<PreAction>,
    fail: bool,
    rewrite: Option<String>,
    set: Option<String>,
}

impl ToolHook for ScriptHook {
    fn pre_tool_call(&self, _name: &str, _args: &Value) -> Result<Vec<PreAction>, HookError> {
        if self.fail {
            return Err(HookError {
                message: "plugin down".into(),
            });
        }
        Ok(self.actions.clone())
    }

    fn post_tool_call(&self, _name: &str, _result: &str) -> Option<String> {
        self.rewrite.clone()
    }

    fn enable_toolset(&self) -> Option<&str> {
        self.set.as_deref()
    }
}

struct SeenHook {
    ptr: AtomicUsize,
    len: AtomicUsize,
}

impl ApiHook for SeenHook {
    fn pre_api_request(&self, messages: &[Message]) {
        self.len.store(messages.len(), Ordering::SeqCst);
        self.ptr.store(messages.as_ptr() as usize, Ordering::SeqCst);
    }
}

#[test]
fn a_block_beats_an_earlier_approve_and_keeps_the_modify() {
    let hooks: Vec<Arc<dyn ToolHook>> = vec![
        Arc::new(ScriptHook {
            actions: vec![
                PreAction::Modify {
                    patch: json!({"path": "notes.md"}),
                },
                PreAction::Approve {
                    message: "please".into(),
                    rule_key: "rm".into(),
                },
            ],
            fail: false,
            rewrite: None,
            set: None,
        }),
        Arc::new(ScriptHook {
            actions: vec![PreAction::Block {
                message: "no".into(),
            }],
            fail: false,
            rewrite: None,
            set: None,
        }),
    ];
    let dispatch = dispatch_pre(&hooks, "rm", json!({}));
    assert_eq!(dispatch.block.as_deref(), Some("no"));
    assert!(dispatch.approve.is_none());
    assert_eq!(dispatch.args["path"], "notes.md");
}

#[test]
fn the_first_approve_wins_and_a_failed_hook_is_skipped() {
    let hooks: Vec<Arc<dyn ToolHook>> = vec![
        Arc::new(ScriptHook {
            actions: vec![],
            fail: true,
            rewrite: None,
            set: None,
        }),
        Arc::new(ScriptHook {
            actions: vec![PreAction::Approve {
                message: "first".into(),
                rule_key: "a".into(),
            }],
            fail: false,
            rewrite: None,
            set: None,
        }),
        Arc::new(ScriptHook {
            actions: vec![PreAction::Approve {
                message: "second".into(),
                rule_key: "b".into(),
            }],
            fail: false,
            rewrite: None,
            set: None,
        }),
    ];
    let dispatch = dispatch_pre(&hooks, "note", json!({}));
    assert!(dispatch.block.is_none());
    assert_eq!(dispatch.approve, Some(("first".into(), "a".into())));
}

#[test]
fn post_hook_rewrites_the_result() {
    let hooks: Vec<Arc<dyn ToolHook>> = vec![Arc::new(ScriptHook {
        actions: vec![],
        fail: false,
        rewrite: Some("rewritten".into()),
        set: None,
    })];
    assert_eq!(
        dispatch_post(&hooks, "echo", "ran echo".into()),
        "rewritten"
    );
}

#[test]
fn cron_escalation_denies_and_a_dead_gate_fails_closed() {
    let mut session = Session::new();
    let blocked = escalation_block(&cron_policy(), &mut session, "rm", "-rf ./build");
    let message = blocked.expect("deny");
    assert!(message.contains("has NOT consented") || message.contains("BLOCKED"));
    assert!(gate_failed("rm").contains("plugin approval gate failed"));
}

#[test]
fn enabling_a_toolset_does_not_override_a_failed_check() {
    let hook: Arc<dyn ToolHook> = Arc::new(ScriptHook {
        actions: vec![],
        fail: false,
        rewrite: None,
        set: Some("notes".into()),
    });
    let mut registry = Registry::new();
    registry.insert("notes", "remember", true);
    registry.insert("notes", "secret", false);
    assert!(!registry.available("remember"));
    registry.enable_from(std::slice::from_ref(&hook));
    assert!(registry.available("remember"));
    assert!(!registry.available("secret"));
}

#[test]
fn the_api_hook_borrows_the_messages() {
    let messages = vec![Message {
        role: "user".into(),
        content: "hello".into(),
    }];
    let hook = SeenHook {
        ptr: AtomicUsize::new(0),
        len: AtomicUsize::new(0),
    };
    run_api_hook(&hook, &messages);
    assert_eq!(hook.len.load(Ordering::SeqCst), 1);
    assert_eq!(hook.ptr.load(Ordering::SeqCst), messages.as_ptr() as usize);
}

#[test]
fn the_memory_queue_records_a_turn_without_running_it() {
    let queue = MemoryQueue::new();
    queue
        .enqueue_turn(&TurnRecord {
            user: "hello",
            assistant: "ok",
        })
        .expect("enqueue");
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.name(), "queue");
}

#[test]
fn an_empty_hook_adds_nothing_and_a_block_stops_the_script() {
    let nop = NopHook;
    assert_eq!(empty_pre_post(&nop, &object_map(&[])), 0);
    let hook: Arc<dyn ToolHook> = Arc::new(BlockName {
        name: "rm".into(),
        message: "BLOCKED: hook refused rm".into(),
    });
    let calls = [
        ScriptedCall {
            name: "rm".into(),
            args: object_map(&[("path", "/")]),
        },
        ScriptedCall {
            name: "echo".into(),
            args: object_map(&[("text", "hello")]),
        },
    ];
    let outcomes = run_script(std::slice::from_ref(&hook), &calls);
    assert_eq!(outcomes.len(), 1);
    assert!(outcomes[0].result.contains("refused"));
}
