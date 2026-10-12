# Social posts: Chapter 2, "Conversations"

## X thread
1/
Chapter 2 of "learning Rust by building a multi-agent harness": Conversations.

LLM APIs are stateless. "Memory" is just a list of messages you resend every time, and that list has more rules than you'd think. 🧵

2/
Hermes Agent (Nous Research) writes the rules down in agent/AGENTS.md:
• roles strictly alternate
• the system prompt never changes mid-conversation (prompt caching!)
• the only allowed rewrite of history is compression

3/
In Rust I made those rules part of the type. `Conversation` has private fields and one `push()` gate that checks alternation.

If you hold a Conversation, it's valid. Even loading a hand-edited JSON file goes back through the same checks.

4/
The system prompt lives outside the history and is resent byte-for-byte identical on every request.

A small design choice, but it's what keeps a provider's prompt cache working.

5/
New: a scriptable MockProvider. Queue replies, queue a failure, and inspect exactly what the model would have seen.

Writing the failure test caught a real bug: a failed call left an unanswered user turn and broke alternation. Now it rolls back.

6/
And a real REPL: `harness chat` with /help /reset /history /save /load /quit (rustyline), plus rough token tracking and trimming of old turns.

11 tests, all offline.

7/
Next, Chapter 3: tools. The model stops just talking and starts acting.

Read Chapter 2: [link]

#rustlang #AIagents

## LinkedIn
Chapter 2 of my "learning Rust by building a multi-agent harness" series is out: Conversations.

The key fact: language-model APIs are stateless. A chatbot's "memory" is just the message history the program resends on every request. That history has rules, and Nous Research's Hermes Agent documents them clearly: roles must strictly alternate, the system prompt must stay stable so provider prompt caching keeps working, and compression is the only sanctioned way to rewrite the past.

In this chapter I encoded those rules in Rust's type system. The `Conversation` type has private fields and a single validated entry point, so an invalid history can't be constructed, not even by loading a hand-edited file. I also built a scriptable mock provider for multi-turn tests, rough token tracking with trimming, JSON save and load, and an interactive `harness chat` REPL with slash commands.

One lesson I didn't expect: writing a test for a failed API call exposed a subtle bug, where the unanswered user turn would break alternation on retry. The fix is a rollback, and the test now guards it.

[personal note: one takeaway from this chapter]

Next up: tools. Read Chapter 2: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
