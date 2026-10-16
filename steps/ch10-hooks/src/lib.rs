//! Book chapter 12. Hooks sit in front of a tool call. One of them can block,
//! rewrite the arguments, or escalate into the chapter 11 prompt. The
//! registry check from chapter 3 still decides whether a tool is available.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use approvals::{check, Policy, Presence, Session, Verdict};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreAction {
    Continue,
    Block { message: String },
    Approve { message: String, rule_key: String },
    Modify { patch: Value },
}

pub trait ToolHook: Send + Sync {
    fn pre_tool_call(&self, name: &str, args: &Value) -> Result<Vec<PreAction>, HookError> {
        let _ = (name, args);
        Ok(Vec::new())
    }

    /// `Some` replaces the tool result. `None` leaves it.
    fn post_tool_call(&self, name: &str, result: &str) -> Option<String> {
        let _ = (name, result);
        None
    }

    fn enable_toolset(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dispatch {
    pub block: Option<String>,
    pub args: Value,
    /// `(message, rule_key)` when a hook asked for the human gate and nothing blocked.
    pub approve: Option<(String, String)>,
}

/// Block wins over approve. Modify patches accumulate until a block.
/// A hook error is ignored, which is the same as "hook failures never block."
pub fn dispatch_pre(hooks: &[Arc<dyn ToolHook>], name: &str, mut args: Value) -> Dispatch {
    let mut first_approve: Option<(String, String)> = None;
    for hook in hooks {
        let actions = match hook.pre_tool_call(name, &args) {
            Ok(actions) => actions,
            Err(_) => continue,
        };
        for action in actions {
            match action {
                PreAction::Continue => {}
                PreAction::Modify { patch } => merge_args(&mut args, &patch),
                PreAction::Block { message } => {
                    return Dispatch {
                        block: Some(message),
                        args,
                        approve: None,
                    };
                }
                PreAction::Approve { message, rule_key } => {
                    if first_approve.is_none() {
                        first_approve = Some((message, rule_key));
                    }
                }
            }
        }
    }
    Dispatch {
        block: None,
        args,
        approve: first_approve,
    }
}

pub fn dispatch_post(hooks: &[Arc<dyn ToolHook>], name: &str, mut result: String) -> String {
    for hook in hooks {
        if let Some(next) = hook.post_tool_call(name, &result) {
            result = next;
        }
    }
    result
}

fn merge_args(args: &mut Value, patch: &Value) {
    let Some(base) = args.as_object_mut() else {
        *args = patch.clone();
        return;
    };
    let Some(extra) = patch.as_object() else {
        return;
    };
    for (key, value) in extra {
        base.insert(key.clone(), value.clone());
    }
}

/// An approve directive that the chapter 11 pipeline did not allow becomes a block.
/// `Ask` is also a block here: this function is the synchronous stand-in for a
/// human who is not in the test. `run_call` in chapter 11 is the real wait.
pub fn escalation_block(
    policy: &Policy,
    session: &mut Session,
    tool: &str,
    reason: &str,
) -> Option<String> {
    let command = format!("{tool} {reason}");
    match check(policy, session, &command) {
        Verdict::Allow { .. } => None,
        Verdict::Deny { message, .. } => Some(message),
        Verdict::Ask { .. } => Some(format!(
            "BLOCKED: plugin approval required for {tool}. The user has NOT consented."
        )),
    }
}

/// Fail closed when the gate itself cannot run.
pub fn gate_failed(tool: &str) -> String {
    format!("BLOCKED: plugin approval gate failed for {tool}")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub trait ApiHook: Send + Sync {
    fn pre_api_request(&self, messages: &[Message]);
}

pub struct NopApi;

impl ApiHook for NopApi {
    fn pre_api_request(&self, messages: &[Message]) {
        let _ = messages.len();
    }
}

pub fn run_api_hook(hook: &dyn ApiHook, messages: &[Message]) {
    hook.pre_api_request(messages);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnRecord<'a> {
    pub user: &'a str,
    pub assistant: &'a str,
}

pub trait MemoryBackend: Send + Sync {
    fn name(&self) -> &str;
    fn enqueue_turn(&self, record: &TurnRecord<'_>) -> Result<(), HookError>;
}

#[derive(Debug, Default)]
pub struct MemoryQueue {
    jobs: Mutex<VecDeque<String>>,
}

impl MemoryQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.jobs.lock().expect("queue").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl MemoryBackend for MemoryQueue {
    fn name(&self) -> &str {
        "queue"
    }

    fn enqueue_turn(&self, record: &TurnRecord<'_>) -> Result<(), HookError> {
        let mut jobs = self.jobs.lock().expect("queue");
        if jobs.len() == 64 {
            jobs.pop_front();
        }
        jobs.push_back(format!("{} {}", record.user, record.assistant));
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Registry {
    checks: HashMap<String, bool>,
    sets: HashMap<String, Vec<String>>,
    enabled: HashSet<String>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
            sets: HashMap::new(),
            enabled: HashSet::new(),
        }
    }

    pub fn insert(&mut self, set: impl Into<String>, name: impl Into<String>, check_ok: bool) {
        let set = set.into();
        let name = name.into();
        self.checks.insert(name.clone(), check_ok);
        self.sets.entry(set).or_default().push(name);
    }

    pub fn enable_from(&mut self, hooks: &[Arc<dyn ToolHook>]) {
        for hook in hooks {
            if let Some(set) = hook.enable_toolset() {
                self.enabled.insert(set.to_string());
            }
        }
    }

    pub fn available(&self, name: &str) -> bool {
        if self.checks.get(name).copied() != Some(true) {
            return false;
        }
        self.sets
            .iter()
            .any(|(set, tools)| self.enabled.contains(set) && tools.iter().any(|tool| tool == name))
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptedCall {
    pub name: String,
    pub args: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallOutcome {
    pub name: String,
    pub result: String,
}

pub struct NopHook;

impl ToolHook for NopHook {}

/// A hook that blocks one tool name. The fake model's script stops there.
pub struct BlockName {
    pub name: String,
    pub message: String,
}

impl ToolHook for BlockName {
    fn pre_tool_call(&self, name: &str, _args: &Value) -> Result<Vec<PreAction>, HookError> {
        if name == self.name {
            Ok(vec![PreAction::Block {
                message: self.message.clone(),
            }])
        } else {
            Ok(Vec::new())
        }
    }
}

pub fn run_script(hooks: &[Arc<dyn ToolHook>], calls: &[ScriptedCall]) -> Vec<CallOutcome> {
    let mut out = Vec::new();
    for call in calls {
        let dispatch = dispatch_pre(hooks, &call.name, call.args.clone());
        if let Some(message) = dispatch.block {
            out.push(CallOutcome {
                name: call.name.clone(),
                result: message,
            });
            break;
        }
        let result = dispatch_post(hooks, &call.name, format!("ran {}", call.name));
        out.push(CallOutcome {
            name: call.name.clone(),
            result,
        });
    }
    out
}

pub fn empty_pre_post(hook: &dyn ToolHook, args: &Value) -> usize {
    let pre = hook.pre_tool_call("echo", args).expect("nop");
    let post = hook.post_tool_call("echo", "ok");
    pre.len() + usize::from(post.is_some())
}

pub fn object_map(entries: &[(&str, &str)]) -> Value {
    let mut map = Map::new();
    for (key, value) in entries {
        map.insert((*key).to_string(), Value::String((*value).to_string()));
    }
    Value::Object(map)
}

pub fn cron_policy() -> Policy {
    Policy {
        presence: Presence::Cron,
        ..Policy::manual()
    }
}

/// Touch the map so a denied escalation still type-checks in the binary.
pub fn sample_block() -> String {
    let hook: Arc<dyn ToolHook> = Arc::new(BlockName {
        name: "rm".to_string(),
        message: "BLOCKED: hook refused rm".to_string(),
    });
    let calls = [ScriptedCall {
        name: "rm".to_string(),
        args: object_map(&[("path", "/")]),
    }];
    run_script(std::slice::from_ref(&hook), &calls)
        .pop()
        .expect("one call")
        .result
}
