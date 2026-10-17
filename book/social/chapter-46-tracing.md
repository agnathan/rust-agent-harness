# Social posts: Chapter 46, "Seeing Inside"

## X thread
1/
Chapter 46 is up: Seeing Inside.

A turn span, then request, response, and a gateway child. The sink stores those names. The prompt is not a field. 🧵

2/
Hermes Agent emits monitoring events without blocking and without raising. GatewayHealthEvent is content-free. The OTLP exporter is optional and has no default endpoint. Redaction happens before egress.

3/
This crate installs a counting subscriber. One hundred mock researchers append notes into one memory. Fake replies. No key. No exporter.

4/
A bare complete measured 12.23 ns. One spanned turn measured 126.5 ns. The demo prints "trace: ok events=3 spans=5 team=101".

[personal note]

5/
Next is the load and token capstone: every tool, a static set, and a dynamic set, still with no socket.

Read Chapter 46: [link]

#rustlang #AIagents

## LinkedIn
Chapter 46 is out: Seeing Inside.

A trace that copies the prompt into a span is a leak, and a trace that waits on a collector stalls the turn. Hermes Agent emits into a bounded queue, drops when that queue is full, and keeps gateway health events free of content. This chapter counts spans in process. `mock_turn` enters a turn, records the names `request`, `response`, and `gateway`, and calls `FakeModel`. The span field is the word `request`, not the prompt. A secret in the prompt does not appear in the sink.

The same subscriber wraps a research team. One hundred mock agents each append one note, and a second call sees the earlier notes. No socket and no exporter.

A bare complete measured 12.23 ns. The spanned turn measured 126.5 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: the load and token capstone. Read Chapter 46: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
