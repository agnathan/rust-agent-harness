# Social posts: Chapter 16, "Built-in Tools I: Web Search and Extract"

## X thread
1/
Chapter 16 is up: Built-in Tools I, Web Search and Extract.

The loop can call echo. It could not look anything up. `web_search` returns titles and urls. `web_extract` returns page text, after the URL is checked. 🧵

2/
Hermes Agent clamps limit to 1…100 (default 5) and returns metadata, not page text (tools/web_tools.py). A secret in the URL, or in one percent-decoded copy, refuses the extract. Link-local 169.254.0.0/16 and the metadata hosts stay blocked when private URLs are allowed (tools/url_safety.py).

3/
DNS failure fails closed. Every resolved address is checked. Over-budget text keeps a head and a tail, and a string of at least 32 characters is wrapped as untrusted data. The per-turn cap of 50 is still chapter 15, on the tool name `web_search`. This chapter does not add a second counter.

4/
Tests and benches call FakeSearch. EnvSearch reads WEB_SEARCH_API_KEY and does not dial in CI. `16/fake_search` measured 5.023 µs. `16/ssrf_block` measured 335.1 ns. No vendor client ships in this harness.

[personal note]

5/
Next is files, a terminal through the approval floor and the sandbox, a todo list, and clarify. A child agent still may not call clarify.

Read Chapter 16: [link]

#rustlang #AIagents

## LinkedIn
Chapter 16 is out: Built-in Tools I, Web Search and Extract.

The harness could retry and fall back. It still could not search. I added `web_search` and `web_extract` behind one trait. Tests use a fake backend. An env-key backend refuses to run unless a flag is set, and even then this crate does not ship a vendor client.

Hermes Agent already refuses a URL that looks like a token, blocks metadata addresses even when private URLs are allowed, and wraps long tool text as data rather than instructions. Those checks run before the backend. The cap of 50 searches in a turn stays on the tool name from the limits chapter. `16/fake_search` measured 5.023 µs. `16/ssrf_block` measured 335.1 ns.

[personal note: one takeaway from writing this chapter]

Next: files, a terminal, a todo list, and a clarify pause. Read Chapter 16: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
