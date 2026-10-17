# Social posts: Chapter 17, "Streaming"

## X thread
1/
Chapter 17 is up: Streaming.

The reply no longer arrives in one buffer. Deltas fold into one string. A think block is stripped as the chunks arrive, including a tag that is split across two of them. 🧵

2/
Hermes Agent's StreamingThinkScrubber (agent/think_scrubber.py) holds a partial tag so an opening <think> is not emitted on its own. The tags include think, thinking, reasoning, thought, REASONING_SCRATCHPAD, and four CJK tags. An unterminated block is dropped on flush.

3/
A read-only tool can be recorded while text is still arriving. A write waits until finish. This chapter does not execute the tool.

4/
Folding 100 plain chunks measured 2.832 µs. Joining the same chunks with no scrubber measured 730.1 ns. The demo reply is hello. No socket.

[personal note]

5/
Next is sessions on disk: one WAL append per turn, and the chapter 9 eval set run again.

Read Chapter 17: [link]

#rustlang #AIagents

## LinkedIn
Chapter 17 is out: Streaming.

A streamed reply is the same text as a buffered one, plus the work of folding deltas. I strip think blocks as the chunks arrive, and I hold a tag that shows up half in one chunk and half in the next. The tag list is the one in Hermes Agent's StreamingThinkScrubber, including the CJK tags.

A read-only tool is recorded before the stream finishes. A write waits. Folding 100 plain chunks measured 2.832 µs, against a plain join of 730.1 ns.

[personal note: one takeaway from writing this chapter]

Next: an append-only session store. Read Chapter 17: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
