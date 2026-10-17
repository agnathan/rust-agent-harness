# Social posts: Chapter 40, "Editor Integration"

## X thread
1/
Chapter 40 is up: Editor Integration.

ACP is another client of the session inbox. An update becomes one message. A cancelled turn and a finished turn are different stop reasons. Logs stay off stdout. 🧵

2/
Hermes Agent reserves stdout for ACP JSON-RPC. acp_adapter/session.py sends incidental output to stderr. The turn's stop_reason is cancelled or end_turn. acp_adapter/permissions.py maps an unknown permission option to deny.

3/
The inbox cap is 64. The 65th update is refused. There is no ACP SDK in this crate. Fake replies. No key.

4/
One translate onto the inbox measured 119.9 ns. The demo prints "acp: end_turn".

[personal note]

5/
Next is the blackboard: shared state, and a lock that is not held across a model call.

Read Chapter 40: [link]

#rustlang #AIagents

## LinkedIn
Chapter 40 is out: Editor Integration.

An editor can drive the same session inbox the chat gateway uses. Hermes Agent speaks ACP on stdout and keeps logs on stderr, because the client parses every stdout line as JSON-RPC. A finished turn reports stop_reason end_turn. A cancelled turn reports cancelled. Permission choices map to once, session, always, or deny, and an option id the adapter does not know is a deny.

This chapter does that translation in memory. The update text becomes one mail. The inbox holds 64. The protocol line and the log line are different vecs, and a test checks they did not mix.

One translate measured 119.9 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: the blackboard. Read Chapter 40: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
