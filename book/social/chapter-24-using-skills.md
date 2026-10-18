# Social posts: Chapter 24, "Using Skills"

## X thread
1/
Chapter 24 is up: Using Skills.

A skill is a folder with SKILL.md. The index is one <available_skills> block, rendered once and stored in an Arc. A skill added later is a note. The block stays. 🧵

2/
Hermes Agent renders that block in agent/prompt_builder.py (_render_skills_index). A skill added after the prompt is a note (agent/skills_index_delta.py, stage_skills_index_note, _NOTE_PREFIX). Name limit 64, description 1024, body 100,000 (tools/skill_manager_tool.py).

3/
skills_list can filter by category. skill_view returns the whole file, or one supporting file under references, templates, scripts, or assets. A path with .. is refused.

4/
Loading and rendering 500 skills measured 4.15 ms ([run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589)). That figure is the mean. The demo views rename. No key.

[personal note]

5/
Next is writing skills: a scanner, an approval gate that starts off, and read-before-write for the reviewer.

Read Chapter 24: [link]

#rustlang #AIagents

## LinkedIn
Chapter 24 is out: Using Skills.

The session search could find an old conversation. It could not load a procedure from a folder. Hermes Agent puts an <available_skills> index in the prompt and loads SKILL.md when the model asks. I did the same: one sorted block, stored in an Arc for the session. A skill added after that render is the note from skills_index_delta.py. The Arc bytes stay.

skill_view returns the whole file. A supporting file has to sit in one of four directories. Loading and rendering 500 skills measured 4.15 ms ([run 39121462589](https://github.com/agnathan/rust-agent-harness/actions/runs/39121462589)). That figure is the mean.

[personal note: one takeaway from writing this chapter]

Next: creating and patching skills, with a scan before the write. Read Chapter 24: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
