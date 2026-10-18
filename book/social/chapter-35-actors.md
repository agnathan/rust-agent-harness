# Social posts: Chapter 35, "Agents as Actors"

## X thread
1/
Chapter 35 is up: Agents as Actors.

The inbox holds 64 messages. try_submit of the next one returns Full, and len stays 64. A steer waits for the turn. An interrupt returns "interrupted" and does not call the model. 🧵

2/
Hermes has no actor inbox. I read gateway/turn_executor.py at 9e01b44: one thread per submitted work item, no queue, no cap. gateway/session_bot.py says the mailbox is a delivery receipt, not an execution queue.

3/
agent/prompt_builder.py wraps a steer with STEER_MARKER_OPEN. This mailbox appends the trimmed text on the turn that runs. fan starts one tokio task per agent. Each channel is created with INBOX_CAP. One hundred tasks, one reply each.

4/
The bench id is 35/inbox_submit. The mean is @@MEAN:35/inbox_submit@@ (`mean.point_estimate`). The demo prints "agent: ok". No key.

[personal note]

5/
Next is supervision: restart a failed child. join_next surfaces a panic. This chapter does not start the agent again.

Read Chapter 35: [link]

#rustlang #AIagents

## LinkedIn
Chapter 35 is out: Agents as Actors.

Each agent needs a mailbox with a size. Hermes has no actor inbox. gateway/turn_executor.py starts one thread per submitted work item and says there is no queue and no cap. gateway/session_bot.py describes a delivery receipt, not an execution queue. I gave the harness an inbox of 64. A full inbox refuses the next message and does not grow. A steer is stored until the turn. An interrupt drops that turn before the model. One hundred tasks each answer once, on their own channel of the same cap. The steer marker in agent/prompt_builder.py is not wrapped here.

The bench id is 35/inbox_submit. The mean is @@MEAN:35/inbox_submit@@ (`mean.point_estimate`).

[personal note]

Next: supervision and restart. Read Chapter 35: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
