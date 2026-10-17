# Social posts: Chapter 44, "Shared Team Skills"

## X thread
1/
Chapter 44 is up: Shared Team Skills.

Workers enqueue a proposal. The librarian is the only writer. The index the workers read is an Arc, swapped after the model returns. 🧵

2/
Hermes Agent syncs bundled skills into a library and hashes a manifest. tools/skills_guard.py refuses a dangerous community install, and force does not override that verdict. tools/write_approval.py stages a skill write when the gate is on.

3/
This crate rejects an empty body and a community body that contains rm -rf / or the injection phrase, even with force. TEAM.md is rendered from the new index. Fake replies. No key.

4/
One skill_propose measured 73.7 ns. One index swap measured 199.9 ns. The demo prints "skills: v2 3".

[personal note]

5/
Next is A2A for agents outside this process. A local:// name is refused before it takes a permit.

Read Chapter 44: [link]

#rustlang #AIagents

## LinkedIn
Chapter 44 is out: Shared Team Skills.

A team that shares skills still needs one writer. Hermes Agent installs bundled skills into the user library and blocks a dangerous community verdict even when the caller passes force. This chapter splits the same idea into a queue and a pointer. `skill_propose` pushes onto a deque of 64. `librarian_accept` is the only function that builds the next index. The model runs before `swap_index`. Workers keep the old `Arc` until that replace.

A community proposal whose body is `rm -rf /` or `ignore previous instructions` is rejected with force set. A builtin dangerous body is allowed. The rendered team file is a `# TEAM` list.

One enqueue measured 73.7 ns. One index swap measured 199.9 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: A2A for external agents. Read Chapter 44: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
