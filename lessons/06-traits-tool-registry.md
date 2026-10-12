# Lesson 06: Traits as Tool, plus a registry

**Goal:** Define what a tool *is* with a trait, and store different tools together.

## Concepts
- **Traits** are shared behaviour, similar to interfaces. They can have *default methods* (`name()` here).
- **Trait objects.** `Box<dyn Tool>` holds any type that implements `Tool` and calls its methods through dynamic dispatch. That's how one `HashMap` can hold different tool types.
- **`Send + Sync`** marks a type as safe to move or share between threads. We require it now because later lessons run tools on many threads.
- **`HashMap<String, Box<dyn Tool>>`** gives lookup by name. Iteration order is random, so sort before returning a list.

## How Hermes does it
- `tools/registry.py`: `ToolEntry` holds name, toolset, schema, handler, `check_fn` and more. Each `tools/*.py` file calls `registry.register(...)` when it's imported.
- `model_tools.handle_function_call` dispatches by name and wraps errors. Tool handlers always return a JSON string.
- Hermes cares about a stable tool list because the tool definitions are part of the prompt cache. That's why our `specs()` returns them sorted.

## Tasks
1. `EchoTool::call`.
   - Hint: `args.get("text").and_then(Value::as_str)`.
2. `AddTool::spec` and `call`. `format!("{}", 5.0_f64)` prints `5`.
3. `ToolRegistry::register` (reject duplicates), `specs` (sorted) and `dispatch`.

## Milestone
`cargo test --test l06_tools` is green.

## Exercises
- Add `check_available(&self) -> bool` with a default of `true` (Hermes's `check_fn`), and make `specs()` skip unavailable tools.
- Add toolsets: `register(tool, toolset)` and `specs_for(&["core"])`.
- Write a `ReadFileTool` using `std::fs::read_to_string` that refuses paths containing `..`.
