# Social posts: Chapter 48, "Shared Team Skills"

## X thread
1/
Chapter 48 is up: Shared Team Skills.

Workers enqueue. The librarian is the only writer. The queue holds 64. A dangerous community skill is refused even with force. Readers keep an Arc. The swap is a pointer replace. 🧵

2/
I read Hermes at 9e01b44. tools/skills_sync.py tracks an origin hash in .bundled_manifest. An update runs only when the bundled copy changed and the user copy still matches. User-deleted skills are not re-added.

3/
tools/skills_guard.py should_allow_install lets force override every block except a dangerous verdict on a community or trusted source. The refusal ends "--force does not override a dangerous verdict." This crate also calls chapter 25's scanner. rm -rf / is a local check, because that scanner does not name the command.

4/
The demo prints "skills: v2 3". The bench id 48/skill_propose has aliases 46/skill_propose and 44/skill_propose. Its mean is 80.1 ns. The bench id 48/index_swap has aliases 46/index_swap and 44/index_swap. Its mean is 200.3 ns. Both are mean.point_estimate from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). The Criterion time: line is the slope.

5/
Next is chapter 49, A2A for agents outside this process.

Read Chapter 48: [link]

#rustlang #AIagents

## LinkedIn
Chapter 48 is out: Shared Team Skills.

The harness is the chapter 47 tree copied forward, plus a team skills module. skill_propose pushes onto a VecDeque until 64. The next call is Full and does not push. The push does not scan, does not take the library lock, and does not wait on the model.

allow_install rejects an empty body. It rejects a community body that contains rm -rf / or ignore previous instructions, even when force is true. Then it calls chapter 25's scan_text and should_allow_install. Builtin maps to that scanner's Trusted trust, so a dangerous builtin can still be allowed when the scanner does not flag it. Community plus force does not override Dangerous. skill_write does not flag rm -rf /. That check stays local.

librarian_accept runs FakeModel before swap_index. The swap is mem::replace of an Arc under a short mutex. Workers load an Arc and do not hold that lock across the model. The notice kind is skills.updated. render_team writes the heading and one bullet per skill. The demo line is "skills: v2 3".

Hermes Agent at 9e01b44 syncs bundled skills through a manifest hash in .bundled_manifest. I did not find a file in that tree that writes the heading this chapter renders. The sync path is the manifest, not that string. should_allow_install is the install decision I read. stage_write exists for pending skill writes. This chapter does not call it.

The demo prints "skills: v2 3". The bench ids are 48/skill_propose and 48/index_swap. Aliases: 46/skill_propose, 44/skill_propose, 46/index_swap, and 44/index_swap. The means are 80.1 ns and 200.3 ns, each mean.point_estimate from the Benchmarks workflow on ubuntu-latest, [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

Next: chapter 49, A2A for agents outside this process. Read Chapter 48: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
