# Social posts: Chapter 22, "Using Skills"

## X thread
1/
Chapter 22 is up: Using Skills.

A skill is a folder with SKILL.md. The index is one <available_skills> block, rendered once and stored in an Arc. A skill added later is a note. The block does not change. 🧵

2/
Hermes Agent renders that block in agent/prompt_builder.py (_render_skills_index). The prefix cache stays warm because the system prompt keeps those bytes (agent/skills_index_delta.py). Name limit 64, description 1024, body 100,000 (tools/skill_manager_tool.py).

3/
skills_list can filter by category. skill_view returns the whole file, or one supporting file under references, templates, scripts, or assets. A path with .. is refused.

4/
Loading and rendering 500 skills measured 1.946 ms. The demo views rename. No key.

[personal note]

5/
Next is writing skills: a scanner, an approval gate that is off unless you turn it on, and read-before-write for the reviewer.

Read Chapter 22: [link]

#rustlang #AIagents

## LinkedIn
Chapter 22 is out: Using Skills.

Procedures do not belong in the system prompt as a full document. Hermes Agent puts a short index in the prompt and loads SKILL.md on demand. I did the same: one sorted <available_skills> block, stored in an Arc for the session. Adding a skill after that render produces the note Hermes uses, and the cached bytes stay.

skill_view returns the whole file. A supporting file has to sit in one of four directories. Loading and rendering 500 skills measured 1.946 ms.

[personal note: one takeaway from writing this chapter]

Next: creating and patching skills, with a scan before the write. Read Chapter 22: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
