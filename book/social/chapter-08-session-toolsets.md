# Social posts: Chapter 8, "Session Toolsets"

## X thread
1/
Chapter 8 is up: Session Toolsets.

A session starts with no tools. search_tools reveals names. describe fetches the schema. call refuses a name the session was not shown. 🧵

2/
Hermes caps one tool-search call at 7 queries and one describe at 10 names (tools/tool_search.py). It rebuilds the catalog every assembly because a session-keyed copy drifts.

This server is the source of the list, so the session stores names and the catalog stores cards. When the names change, a list_changed flag flips. The wire notification is later.

3/
Hermes does not have set_task. I searched. The starter set here is a token overlap that prefers the files server for "rename a file", up to 8 names.

4/
A fake model, no key, 2,000 tools, three arms, all successful:

every tool: 45,104 schema tokens
static: 153
dynamic: 135

The dynamic arm measured 4.124 ms. Revealing every tool measured 5.996 ms. Those times include building the catalog.

5/
[personal note]

6/
The agent loop is already written. It does not call this gateway yet. Chapter 10 is the transition table around the loop's exits.

Read Chapter 8: [link]

#rustlang #AIagents

## LinkedIn
Chapter 8 is out: Session Toolsets.

The expensive part of a tool catalog is showing the schema. Hermes Agent caps search at 7 queries and describe at 10 names, and it rebuilds the catalog so a session-scoped copy cannot drift. I kept the caps. The session stores revealed names. The catalog stores the cards. A call to a name that was not revealed fails.

On 2,000 synthetic tools a fake task succeeded three ways. Showing every schema was 45,104 estimated tokens. A static set was 153. Search plus one describe was 135. The estimate is bytes divided by 4. It is not a bill.

[personal note: one takeaway from writing this chapter]

Next: the turn state machine. Read Chapter 8: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
