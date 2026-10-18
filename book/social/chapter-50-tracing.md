# Social posts: Chapter 50, "Seeing Inside"

## X thread
1/
Chapter 50 is up: Seeing Inside.

A span per event. The field is the event name. The sink holds request, response, and gateway. The prompt is not a span field. 🧵

2/
I read agent/monitoring/otlp_exporter.py at 9e01b44. It maps gateway events to OTel spans. The OTel SDK is optional (hermes-agent[otlp]). headers_env values are read at export time and never logged or stored. No default destination ships. I checked that module doc.

3/
redact_bounded redacts str(raw or "") and length-bounds it. moa_trace.py is opt-in via moa.save_traces, off by default. _traces_enabled_and_dir returns None when the flag is off. The side channel is a file under the trace dir.

4/
The demo prints "trace: ok events=3 spans=5 team=101". The bench ids are 50/event_without_span and 50/span_per_event. Aliases: 48/event_without_span, 46/event_without_span, 48/span_per_event, and 46/span_per_event. The means are MEAN_PENDING_50_BARE and MEAN_PENDING_50_SPAN.

[personal note]

5/
Next is chapter 51, the server capstone: load and tokens. Chapter 52 copies this harness, not chapter 51.

Read Chapter 50: [link]

#rustlang #AIagents

## LinkedIn
Chapter 50 is out: Seeing Inside.

The harness is the chapter 49 tree copied forward, plus spans. tracing = "0.1" on this crate. safe_field discards the prompt and returns "request". emit pushes the name onto a sink and enters a span named event. mock_turn enters a turn, emits request, completes FakeModel, emits response, enters a gateway span, and emits gateway. The sink holds those three names. CountingSubscriber counts span ids from 1 and does not export. Five spans open on the demo turn: turn, request event, response event, gateway, gateway event. research_team(100) then research_team(1) leaves 101 notes in one Memory. The notes are the mock replies. FakeModel ignores the prompt. In the demo those replies are ok and kept.

I read Hermes at 9e01b44. The OTLP exporter maps gateway events to spans, the SDK is optional, and headers_env is read at export time and never logged or stored. redact_bounded redacts and length-bounds. moa.save_traces is off by default. The side channel is a file under the trace dir, not a row in the messages table.

The demo prints "trace: ok events=3 spans=5 team=101". The bench ids are 50/event_without_span and 50/span_per_event. The means are MEAN_PENDING_50_BARE and MEAN_PENDING_50_SPAN. Subscriber install sits outside both samples.

[personal note]

Next: chapter 51, the server capstone, load and tokens. Chapter 52 copies this harness, not chapter 51. Read Chapter 50: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
