# Lesson 07: The agent loop against a mock model

**Goal:** Build the core loop: call the model, run tools, append results, repeat until the model answers.

## Concepts
- **A `Model` trait** with a `ScriptedModel` mock gives deterministic tests with no network and no cost. Writing a fake implementation of a trait is the standard way to test in Rust.
- **`&mut dyn Model`** passes "any model" by mutable reference.
- **Budgets.** A `max_iterations` cap stops the agent from looping forever.
- **`VecDeque`** is a double-ended queue; `pop_front` takes the next scripted reply.
- **`map_err(LoopError::Model)?`.** An enum variant can be used as a function.

## How Hermes does it
`agent/conversation_loop.py::run_conversation` is essentially:
```python
while api_call_count < max_iterations and budget.remaining > 0:
    response = client.chat.completions.create(model=..., messages=messages, tools=tool_schemas)
    if response.tool_calls:
        for tc in response.tool_calls:
            messages.append(tool_result(handle_function_call(tc.name, tc.args)))
    else:
        return response.content
```
- The default for `max_iterations` is 500.
- Tool errors are returned to the model as text, so the model can recover, rather than crashing the loop.

## Tasks
1. `ScriptedModel::new` and `complete`. Increment `calls` and pop the next reply.
2. `run_agent_loop`. Follow the doc comment exactly.
   - Hint: parse with `serde_json::from_str::<serde_json::Value>(&call.arguments)`.
   - The error text is `format!("error: {e}")`, which uses `ToolError`'s `Display`.

## Milestone
`cargo test --test l07_loop` is green. **You now have a working agent core.**

## Exercises
- Return per-tool timings in `LoopOutcome`.
- Add a "grace call" like Hermes: when the budget runs out, call the model once more with no tools so it can summarise.
- Add `on_event: &mut dyn FnMut(&str)` to print progress.
