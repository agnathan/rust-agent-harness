# Social posts: Chapter 7, "Dynamic Catalog"

## X thread
1/
Chapter 7 is up: Dynamic Catalog.

The server holds native tools, plugin tools, and an in-memory upstream list. Search is BM25. A reload stores a new Arc. The call that already cloned the old one keeps it. 🧵

2/
Hermes searches in tools/tool_search_catalog.py. An exact name ranks first. The catalog is rebuilt on every assembly, on purpose, so a session-keyed copy cannot drift.

3/
I kept BM25 and the exact-name rank. I did not copy the rarest-token admission. A duplicate name fails the reload and the previous snapshot stays. Hermes overwrites a same-toolset duplicate.

4/
Search over 10,000 tools measured 299.0 µs. Rebuilding that index measured 27.42 ms. Swapping a one-tool snapshot while holding the old Arc measured 2.467 µs.

HashEmbedder is eight buckets of byte counts. Ranking does not use it.

5/
No socket and no API key. The demo loads two tools and searches "rename a file".

[personal note]

6/
Next, each session sees only the names it revealed.

Read Chapter 7: [link]

#rustlang #AIagents

## LinkedIn
Chapter 7 of the series is out: Dynamic Catalog.

Hermes Agent rebuilds its tool-search catalog from the live definitions on every assembly. The search puts an exact name match first. I kept both ideas and changed the lifetime: the server builds a snapshot, stores an Arc, and a call that already holds the previous Arc does not see the new index.

Plugins and upstreams in this chapter are lists in the load. Nothing is dialed. A duplicate name aborts the reload.

On the Benchmarks runner, search over 10,000 synthetic tools measured 299.0 µs. The rebuild measured 27.42 ms.

[personal note: one takeaway from writing this chapter]

Next: session toolsets. Read Chapter 7: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
