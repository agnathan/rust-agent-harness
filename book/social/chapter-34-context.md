# Social posts: Chapter 34, "Managing Context"

## X thread
1/
Chapter 34 is up: Managing Context.

A project file with an injection is replaced by the BLOCKED line. A user-authored SOUL.md keeps its text and the finding. Over 20,000 characters, the file keeps a head of 14,000 and a tail of 4,000. share is Arc::clone. 🧵

2/
Hermes Agent at 9e01b44. agent/prompt_builder.py: _scan_context_content, CONTEXT_FILE_MAX_CHARS 20_000, head ratio 0.7, tail ratio 0.2. tools/threat_patterns.py: MAX_SCAN_CHARS is 65_536. The filler is {0,8}.

3/
agent/tool_dispatch_helpers.py wraps web_extract, web_search, browser_, and mcp_ once the text is at least 32 characters, and rewrites an embedded untrusted_tool_result token with hyphens. preflight calls the side model only over the cap. ptr_eq holds on both paths. agent/context_compressor.py builds the SKILL_PRUNED marker.

4/
The bench id is 34/share_snapshot. The mean is 4.86 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The demo prints "shared: clean". No key.

[personal note]

5/
Next is actors: one tokio task per agent, an inbox of 64, a steer that waits for the turn, an interrupt that skips the model.

Read Chapter 34: [link]

#rustlang #AIagents

## LinkedIn
Chapter 34 is out: Managing Context.

A rules file is not the same thing as a prompt you meant to send. Hermes Agent scans context files in agent/prompt_builder.py, replaces a project hit with a BLOCKED line, and still loads the user's own file. The cap is 20,000 characters, with a 70% head and a 20% tail. tools/threat_patterns.py stops the scan at 65,536. agent/tool_dispatch_helpers.py wraps high-risk tool output and neutralizes an embedded closer. I kept those checks, compiled four of the patterns, and shared the snapshot with Arc::clone. Compression runs only when the transcript is over the cap. The hot path does not rebuild that Arc.

The bench id is 34/share_snapshot. The mean is 4.86 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573).

[personal note]

Next: one mailbox per agent, capacity 64, steer and interrupt. Read Chapter 34: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
