# Chapter 16: Providers

*Draft. Bracketed notes like [personal note] are where my own story goes.*

Chapter 1 talks to OpenRouter. A turn that needs Anthropic or Bedrock has a different JSON body, and a failed key should not build a new client. This chapter builds those bodies, an OpenRouter route, and a rotation that keeps one client id. The cheaper side model stays off unless the turn asked for a summary. Nothing here opens a socket.

The step crate is `steps/ch13-providers`. The book number is 16. The package is `ch13-providers`.

| Section | Step | What it adds |
|---|---|---|
| 16 | `steps/ch13-providers` | Anthropic and Bedrock bodies, a route, key rotation, outbound repair |

> **How Hermes does it**
>
> I read `agent/anthropic_adapter.py` (`build_anthropic_kwargs`), `agent/bedrock_adapter.py` (`build_converse_kwargs`), `agent/chat_completion_helpers.py` (`_provider_preferences_for_agent`, `_validated_openrouter_provider_sort`, `_OPENROUTER_PROVIDER_SORT_VALUES`), `agent/agent_init.py` (`fallback_model`), `agent/credential_pool.py` (`PooledCredential`, `mark_exhausted_and_rotate`), `agent/message_sanitization.py` (`uniquify_tool_call_ids`, `close_interrupted_tool_sequence`), `agent/agent_runtime_helpers_placeholders.py` (`_INTERRUPTED_PLACEHOLDER`), and `agent/ssl_verify.py` (`resolve_httpx_verify`, `install_truststore`) on Hermes `main` at `cf71b1aa`.
>
> `build_anthropic_kwargs` sends `model`, `messages`, and `max_tokens`. `system` is set only when it is non-empty. `max_tokens` has to be positive. `build_converse_kwargs` sends `modelId`, `messages`, and `inferenceConfig`, and the default `max_tokens` in that signature is 4096. Provider preferences keep `only`, `ignore`, `order`, `sort`, `require_parameters`, and `data_collection`, and drop empty values. Sort is `throughput`, `latency`, or `price`. Anything else is ignored. `require_parameters` is kept only when it is true. Duplicate tool-call ids gain a `_dN` suffix starting at 2. A transcript that ends on a tool row gets an assistant row whose text is `[interrupt: no assistant output for this turn]`. TLS uses the OS trust store through `truststore`. `ssl_verify: false` is for local development.
>
> **Adopted.** The Anthropic field names, including omitting an empty system prompt. The Bedrock field names `modelId` and `inferenceConfig.maxTokens`. The six preference keys, the three sort values, and dropping a false `require_parameters`. The `_d2` suffix. The interrupted-placeholder sentence. Refusing to disable TLS unless a dev flag is set.
>
> **Adapted.** There is no HTTP client in this crate. The per-model output-ceiling table is not ported. A `max_tokens` of 0 is an error. Fallbacks are a `models` array on the route object. I did not find Hermes writing that array inside `build_anthropic_kwargs` or `build_converse_kwargs`. Hermes keeps the list on `fallback_model`. Rotation advances an index in one `Pool` and the client id stays `1`. The side model is a string choice, not a second request.
>
> **Deferred.** OAuth transforms, thinking budgets, and cache points stay out. Live certificate verification stays out with the socket. Chapter 17 strips think blocks from a stream.

[personal note: a provider body that failed because one field was missing]

## What this chapter decides

`anthropic_body("claude", "be brief", messages, 128)` has `model`, `max_tokens`, `system`, and `messages`. An empty system string is omitted. `max_tokens` of 0 is `ProviderError::MaxTokens`.

`bedrock_body` uses `modelId` and `inferenceConfig.maxTokens`. The test passes 4096, which is the default on `build_converse_kwargs`. An empty system string is omitted here too.

`provider_preferences` drops empty lists. `sort: Some("Price")` becomes `"price"`. `sort: Some("fastest")` is omitted. `require_parameters: false` is omitted. `data_collection: Some("deny")` is kept.

`openrouter_route` puts the model list on `models` and the preference map on `provider`. `choose_next` with no failure returns the first model. `choose_next` after `"primary"` returns `"fallback"`.

`Pool::new` has client id 1. `mark_exhausted_and_rotate` moves from `sk-one` to `sk-two` and the client id is still 1.

`side_model(false, "primary", "side")` is `"primary"`. `side_model(true, ...)` is `"side"`.

`uniquify_tool_call_ids` turns `["call_0", "call_0", "call_1"]` into `["call_0", "call_0_d2", "call_1"]`. `close_interrupted` appends the placeholder when the last row is `tool`, and does nothing when the last row is already `assistant`.

`resolve_tls(true, _)` is `OsTrust`. `resolve_tls(false, false)` is an error. `resolve_tls(false, true)` is `DevInsecure`. This crate does not build an `SslContext`.

The demo fails the first fake model and takes the fallback. The reply is `pong`. The client id is 1. No key.

## Rust: the body is a `serde_json::Value`

`json!` builds an object. Assigning `body["system"]` inserts a key. `get("system")` is how a test sees that the key was left out. `Map<String, Value>` is the preference object before it is wrapped. Empty keys never enter the map, so a later `serde` of the route does not send `"ignore": []`.

`Pool` owns the key strings. `mark_exhausted_and_rotate` returns a borrow of the current key. The client id is a `u64` that the method does not change. One pool is one client.

## Tests

Six tests. Both bodies have the fields above, and a zero `max_tokens` fails. An invalid sort is dropped and the fallback model is the next name. Rotation keeps client id 1, and the side model stays off. Duplicate ids and a dangling tool row are repaired. TLS stays on the OS decision unless the dev flag is set. The fake turn answers `pong` from the fallback.

## Build, run, and benchmark

```bash
cargo nextest run --locked -p ch13-providers
```

```text
Starting 6 tests across 3 binaries
     Summary [   0.006s] 6 tests run: 6 passed, 0 skipped
```

```bash
cargo run -p ch13-providers
```

```text
reply: pong
client: 1
```

The binary rotates once, then picks the second fake model because the first one failed. No HTTP.

| Benchmark | Mean | Target | What it is |
|---|---|---|---|
| `choose_route` | 6.229 ns | under 1 ms | the next model after `primary` |
| `rotate_key` | 3.471 ns | under 1 ms | advance the key, same client id |

`choose_route` builds a three-name list before the bench. The timed closure is `b.iter`. Inside: `choose_next` with that list and `Some("primary")` both passed through `black_box`. No failed HTTP call is inside the sample. The result is `fallback`.

`rotate_key` builds a pool of three keys before the bench. Each sample calls `mark_exhausted_and_rotate` and black-boxes the new key. The client id is not rebuilt. The pool wraps from the last key back to the first across samples, so the sample is the index update, not a client constructor.

The table is `mean.point_estimate` from the Benchmarks workflow on ubuntu-latest, [run 38967738024](https://github.com/agnathan/rust-agent-harness/actions/runs/38967738024), section `16`.

### Milestone demo

The `cargo run` above. The reply is `pong`. The client is `1`.

## Pitfalls

`choose_next` does not call the model. A failure you have not already classified still needs chapter 14. This function only picks the next name.

The `_d2` suffix looks off by one if you expected `_d1`. Hermes starts the range at 2. A second collision of the same id becomes `_d3`.

`ssl_verify: false` without the dev flag is an error, not a quiet downgrade. The flag is the only way this crate records that the caller meant to turn verification off. It still does not install a certificate store.

A side-model summary does not run because the turn printed text. It runs when `asked` is true. Leave that false on the user path.

## Guardrails in this chapter

[`docs/hermes-guardrails.md`](../docs/hermes-guardrails.md). Layer L3.

**K4.** Duplicate tool-call ids in one batch are renamed with `_dN` starting at 2. A transcript that ends on a tool row is closed with the interrupted placeholder so the next user message is not `tool` then `user`.

**TLS.** The decision is the OS trust store. `ssl_verify: false` is refused unless the dev flag is set. That matches `agent/ssl_verify.py`: verification off is for local development.

**Left for later.** Think-block stripping is chapter 17 (K5). The live TLS handshake is still absent, with the socket. Secret redaction (E1, E2) is still absent.

## Chapter summary

- **Bodies.** Anthropic has `max_tokens`. Bedrock has `modelId` and `inferenceConfig.maxTokens`.
- **Route.** Invalid sorts are dropped. The fallback is the next name in `models`.
- **Client.** Rotation changes the key and keeps client id 1. The side model stays off.
- **Wire.** `call_0` twice becomes `call_0` and `call_0_d2`. A dangling tool row is closed.
- **Time.** Choosing the next route measured **6.229 ns**. Rotating a key measured **3.471 ns**.

[personal note: a key rotation that built a second client, and what the pool was supposed to keep]

## What's next

Chapter 17 folds a stream of deltas into one reply. Think blocks are stripped as the tokens arrive, including a tag that is split across two chunks. A read-only tool can start before the stream finishes.
