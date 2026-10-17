# Social posts: Chapter 20, "Recall"

## X thread
1/
Chapter 20 is up: Recall.

The session file can be searched. session_search has four shapes: discover, scroll, read, and browse. It returns rows from SQLite. It does not call a model. 🧵

2/
Hermes Agent hides kanban, subagent, and tool sessions (tools/session_search_tool.py). The trigram index is messages_fts_trigram with tokenize=trigram (hermes_state_common.py). Scroll looks around one message id, five on either side.

3/
A child session that contains the same token is not a hit. A cron session that contains it sorts after the user's session. The index is rebuilt before the search, not on every insert.

4/
Discover over 10,000 messages measured 22.95 µs. The demo hit is the user session named desk. No key.

[personal note]

5/
That is chapter 20. The next server chapter is learned tools. The next harness chapter is skills.

Read Chapter 20: [link]

#rustlang #AIagents

## LinkedIn
Chapter 20 is out: Recall.

Past sessions are searchable without another model call. An FTS5 trigram index sits on the messages table from the sessions chapter. A query discovers one hit per session. A session id reads it. A session id plus an anchor scrolls a window. No arguments browses recent sessions.

Subagent sessions stay out of the results, matching Hermes Agent's hidden sources. Cron sorts after the user's own session instead of being dropped from the index. Discover over 10,000 messages measured 22.95 µs.

[personal note: one takeaway from writing this chapter]

Next: skills, and learned tools on the server. Read Chapter 20: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
