# Lesson 12: An async tool registry and safe parallel tool calls

**Goal:** Run independent tool calls at the same time without corrupting files.

## Concepts
- **`#[async_trait]`** allows `async fn` in trait objects (`Arc<dyn AsyncTool>`).
- **`Arc<T>`** is shared ownership across tasks; cloning it bumps a reference count.
- **`Semaphore`** caps concurrency: each task must acquire a permit before running.
- **Batch planning.**
  - Read-only calls can all run in parallel.
  - Writes to the *same* path must not overlap.
  - Some tools (`clarify`, anything that prompts the user) must run alone.
- **Order preservation.** Results must go back to the model in the same order as the calls, matched by `tool_call_id`.

## How Hermes does it
- `agent/tool_dispatch_helpers.py`:
  - `_PARALLEL_SAFE_TOOLS` lists read-only tools: `read_file`, `search_files`, `session_search`, and others.
  - `_NEVER_PARALLEL_TOOLS` lists tools that must run alone, such as `clarify`.
  - `_PATH_SCOPED_TOOLS` lists tools that conflict only on the same path.
- `agent/tool_executor.py` runs a batch on a `ThreadPoolExecutor` with `_MAX_TOOL_WORKERS = 8` and a batch deadline.

## Tasks
1. `plan_batches`.
   - Hint: keep a `current` vector, and `std::mem::take(&mut current)` when you need to flush it.
2. `SleepTool::call`.
3. `execute_parallel`.
   - Hint: wrap the semaphore in an `Arc`, `.cloned()` the tool `Arc`, and make each call an `async move { let _permit = sem.acquire().await...; tool.call(args).await }`. Then `join_all`.
4. `execute_planned`. Look up each tool's concurrency (an unknown tool counts as `Exclusive`), then run the batches in order and put the results back by index.

## Milestone
`cargo test --test l12_parallel` is green.

## Exercises
- Add a per-call timeout that returns `Err("timeout")` while the rest of the batch keeps going.
- Add a `pre_tool` hook (`Arc<dyn Fn(&str, &Value) -> Result<(), String>>`) that can block calls. This is a mini version of Hermes's approval and guard layer (`tools/approval*.py`).
