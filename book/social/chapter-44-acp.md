# Social posts: Chapter 44, "Editor Integration"

## X thread
1/
Chapter 44 is up: Editor Integration.

One ACP update becomes one inbox submit. The inbox holds 64. A finished turn is end_turn. A cancelled turn is cancelled. JSON goes one way. The log goes the other. 🧵

2/
I looked for Hermes's ACP server. pyproject.toml names acp_adapter, and hermes_cli/main_agent_cmds.py imports acp_adapter.entry. The directory is not in the tree at 9e01b44. I checked that path.

3/
What is there: agent/copilot_acp_client.py appends session/update chunks, and it answers session/request_permission with cancelled. Stdout is the JSON queue. Stderr is a side deque. This chapter's map denies an option id it does not know.

4/
The demo prints "acp: end_turn". The bench id is 44/translate_update. The old ids 42/translate_update and 40/translate_update are aliases. The mean is @@MEAN:44/translate_update@@ (`mean.point_estimate`).

[personal note]

5/
Next is chapter 45, the blackboard: agents claim shared state, and the lock is not held across a model call.

Read Chapter 44: [link]

#rustlang #AIagents

## LinkedIn
Chapter 44 is out: Editor Integration.

The harness is the chapter 43 tree copied forward, plus an ACP module. translate copies one update into one mail. Inbox::submit takes that mail until 64, the same cap as the session inbox. stop_reason is end_turn or cancelled. write_result pushes a JSON-RPC line onto stdout. log_line pushes onto stderr. allow_once, allow_session, and allow_always keep their outcomes. deny, deny_always, and any unknown id deny.

Hermes Agent at 9e01b44 lists acp_adapter in pyproject.toml, and cmd_acp imports acp_adapter.entry. That directory is not in the checkout. The ACP code I could read is agent/copilot_acp_client.py: session/update chunks are appended, a permission request is answered cancelled, and stdout and stderr are pumped apart. gateway/session_acp_lifecycle.py ends an idle ACP session. It does not turn an update into a submit. The adapter in this chapter is the harness file.

The demo prints "acp: end_turn". The bench id is 44/translate_update. Aliases: 42/translate_update and 40/translate_update.

[personal note]

Next: chapter 45, the blackboard. Read Chapter 44: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
