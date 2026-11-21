# Social posts: Chapter 6, "Thousands of Tools"

## X thread
1/
Chapter 6 of my "learning Rust by building a multi-agent harness" series is up: Thousands of Tools.

Ten thousand MCP-shaped schemas were 517,572 estimated tokens. A 50-tool registry was 2,508. The model does not need to see all of them. 🧵

2/
Hermes Agent (Nous Research) filters the registry with a toolset and a check_fn, then rewrites descriptions so they only mention tools that are actually available.

I did the same with a check on the index, and a rewrite that drops a sentence naming a tool the agent cannot call.

3/
The first turn carries one tool, search_tools. That prompt was 55 tokens. Search stages matches. The next turn appends their schemas. The core prefix stays byte-identical so a provider can keep its prompt cache.

On 2,000 tools the full list was 103,063 tokens. After revealing five schemas it was 305.

4/
BM25 over 10,000 tools answered in 146.8 µs. Routing to one server first, then searching only there, answered in 16.00 µs.

A cached tools/list hit was 17.56 ns. The cold clone of 10,000 cards was 12.07 ms. Filtering one agent's servers was 77.00 µs.

5/
Chapter 4 is the MCP client. The 10,000-tool benches here still generate the catalog. A real tools/list enters the same cache through cards_from_listed.

No live model call. --mock is the demo. Without a key the CLI prints the missing OPENROUTER_API_KEY error and stops.

6/
Next, the agent loop: the model picks a tool, including search_tools, and the result comes back as a message.

Read Chapter 6: [link]

#rustlang #AIagents

## LinkedIn
Chapter 6 of my "learning Rust by building a multi-agent harness" series is out: Thousands of Tools.

Putting every tool schema in the prompt gets expensive, and it makes the model worse at choosing. I measured a synthetic catalog in the same shape MCP uses. Ten thousand tools were 517,572 estimated tokens. Fifty tools were 2,508. The words "rename a file" still matched 192 of the ten thousand, so the model would be choosing among a crowd after reading everyone else.

Nous Research's Hermes Agent already filters by toolset and check_fn, and it rewrites descriptions so they only mention tools that are available. I built that gate into the index, and a rewrite that drops a sentence pointing at a tool the agent cannot call.

The model starts with one meta-tool, search_tools. Matched schemas are appended on the next turn, after a stable prefix, with a cap that evicts the oldest revealed names. On 2,000 tools the first turn was 55 tokens and the next turn, with five schemas revealed, was 305. The full list was 103,063.

Search over 10,000 tools was 146.8 µs. Routing by server first brought that to 16.00 µs. A cached tool list hit was 17.56 ns, against 12.07 ms to clone the catalog the first time.

The 10,000-tool benches still generate the catalog. Chapter 4's client hands a real list to the same cache. I'm new to Rust, so the inverted index, the append-only prefix, and Arc show up in short sidebars when they're needed. The demo is offline. I did not paste a live model transcript.

Next up: the server-side catalog (chapter 7), then the agent loop (chapter 9). Read Chapter 6: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
