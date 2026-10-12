# Lesson 05: serde and the OpenAI wire format

**Goal:** Convert messages and tool schemas to and from exactly the JSON that model APIs expect.

## Concepts
- **serde** generates serialization code: `#[derive(Serialize, Deserialize)]`.
- **Attributes** customise the JSON:
  - `#[serde(rename = "type")]` maps a field to a JSON key that is a reserved word in Rust.
  - `skip_serializing_if = "Option::is_none"` leaves out fields that are `None`.
  - `default` fills missing fields when reading JSON.
- **`serde_json::Value`** is untyped JSON. `json!({...})` builds it, and `v["a"]["b"]` indexing returns `Null` instead of panicking when a key is missing.
- **`From<&A> for B`** is the standard conversion trait. **Generics** with trait bounds (`T: DeserializeOwned`) give you one function that parses into any type.
- We keep an internal type (`Message`) separate from the wire type (`WireMessage`). Keeping the API's JSON format at the boundary is a good habit.

## How Hermes does it
- Tool schemas are JSON Schema dicts registered in `tools/registry.py` (`ToolEntry.schema`) and wrapped as `{"type":"function","function":{...}}`.
- Anthropic, Gemini and Codex need conversions (`agent/anthropic_message_convert.py`, `agent/gemini_schema.py`), so Hermes also keeps one internal format (OpenAI-shaped) and converts at the edge.

## Tasks
1. `From<&Message> for WireMessage`. `content` is `None` only when the message has tool calls and empty content.
2. `role_from_str` and `WireMessage::to_message`.
   - Hint: `.ok_or_else(|| format!(..))?`.
3. `ToolSpec::to_openai` using `json!`.
4. `object_schema`.
   - Hint: build a `serde_json::Map` and insert each property.
5. `parse_args<T>`: one line.

## Milestone
`cargo test --test l05_json` is green. Print `serde_json::to_string_pretty` of a tool call and compare it with the OpenAI docs.

## Exercises
- Use the `schemars` crate to derive the JSON Schema from `AddArgs` instead of writing it by hand.
- Add an `image_url` content part (multimodal content as an array).
