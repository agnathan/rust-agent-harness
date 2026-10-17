# Social posts: Chapter 13, "Sandboxed Execution"

## X thread
1/
Chapter 13 is up: Sandboxed Execution.

The tool still ran in-process. This chapter moves a job into a child process, builds the Docker argv, and adds a checkpoint you can rewind. The default backend is local. Local is not a sandbox. 🧵

2/
Hermes Agent sets terminal.backend to "local" in hermes_cli/config_defaults.py. tools/environments/docker.py drops every capability, sets no-new-privileges, and limits pids to 2048. Checkpoints in tools/checkpoint_manager.py are off until you enable them.

3/
The child env keeps PATH. It drops OPENAI_API_KEY even if a skill lists it as passthrough. A third-party key on that list is kept. That split is tools/env_passthrough.py.

4/
A warm ping measured 16.39 µs. Copying a 1,000-path manifest measured 41.90 µs. Putting it back measured 61.75 µs. No Docker daemon in the bench.

[personal note]

5/
Next is what to do when the call fails: retry, fall back, or stop. Empty replies and a repetition loop were waiting since the agent loop chapter.

Read Chapter 13: [link]

#rustlang #AIagents

## LinkedIn
Chapter 13 is out: Sandboxed Execution.

An approval check is not a container. Hermes Agent leaves the terminal backend on local, and only builds the hardened Docker command when that backend is Docker: drop every capability, no-new-privileges, and a pids limit. I kept that default. The chapter's demo is a small worker process that answers ping. Docker is an argument list a test can read, not a daemon the suite starts.

The child environment keeps PATH and drops a provider API key even when a caller asked to pass it through. Checkpoints are off until enabled, then a rewind puts the saved files back. A warm accept measured 16.39 µs.

[personal note: one takeaway from writing this chapter]

Next: classifying failures, and the empty-reply and repetition checks that were deferred from the agent loop. Read Chapter 13: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
