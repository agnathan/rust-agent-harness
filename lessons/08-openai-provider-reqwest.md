# Lesson 08: A real OpenAI-compatible provider with reqwest

**Goal:** Talk to a real model, whether OpenAI, OpenRouter, Ollama, LM Studio or vLLM. They all speak `/v1/chat/completions`.

## Concepts
- **reqwest** is an HTTP client. We use its `blocking` API so this lesson stays synchronous; lesson 09 introduces async.
- **Testability.** Keep the network out of the logic. `build_request_body` and `parse_chat_response` are pure functions with tests; the HTTP call is a thin shell around them.
- **Configuration through a function.** `from_lookup(|k| ...)` instead of reading env vars directly makes the code easy to test. `from_env` is the real-world wrapper.
- **`?` on `Option` inside a function that returns `Result`** works via `.ok_or(..)?`.

## How Hermes does it
- Providers are `ProviderProfile`s (`providers/base.py`), with one plugin per provider in `plugins/model-providers/<name>/`. Each has hooks: `prepare_messages`, `build_extra_body`, `fetch_models`.
- The transport is `agent/transports/chat_completions.py`.
- Errors such as rate limits and auth failures are classified by `agent/error_classifier.py`.

## Tasks
1. `OpenAiConfig::from_lookup`, including defaults and trimming a trailing `/`.
2. `build_request_body`. Add `"tools"` only when the list isn't empty.
3. `parse_chat_response`: handle the error body, `tool_calls` and plain text.
4. `impl Model for OpenAiModel`.
   - Hint: `client.post(url).bearer_auth(key).json(&body).send()?.json::<Value>()?`.
5. Try it for real: create `harness/examples/chat.rs` that builds `OpenAiModel::new(OpenAiConfig::from_env()?)` and calls `run_agent_loop` with `AddTool`. Run it with `OPENAI_API_KEY=... cargo run --example chat`.
   - For a local model with no key: `OPENAI_BASE_URL=http://localhost:11434/v1 OPENAI_API_KEY=x OPENAI_MODEL=llama3.1`.

## Milestone
The tests are green, and a real model calls your `add` tool.

## Exercises
- Return a typed `ProviderError { status, retryable }` instead of `String`. Classify 429 and 5xx as retryable.
- Add retries with exponential backoff (sleep 2^n × 250 ms).
