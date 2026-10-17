# Social posts: Chapter 33, "Agents as Actors"

## X thread
1/
Chapter 33 is up: Agents as Actors.

Each agent is a task with a mailbox. The inbox holds 64 messages. The 65th send is refused. A steer waits for the next turn. An interrupt skips the model. 🧵

2/
Hermes Agent's steer is agent/interrupt_control.py. Empty text returns false. Two steers join with a newline. A steer does not cancel the turn. redirect is the path that cancels a model request.

3/
The capacity of 64 is the harness budget, not a number I found in steer. One hundred tasks means one hundred inboxes. FakeModel answers. No key.

4/
One submit into an empty inbox measured 208.2 ns. The demo prints "agent: ok".

[personal note]

5/
Next is restarting a failed child, and keeping that restart off the child's own work.

Read Chapter 33: [link]

#rustlang #AIagents

## LinkedIn
Chapter 33 is out: Agents as Actors.

An agent that shares the caller's thread cannot tell a second message to wait. Hermes Agent queues a steer without cancelling the turn, and it refuses empty steer text. I kept that. The mailbox here is a bounded channel of 64. A full inbox returns Full instead of growing. An interrupt is a different message: the turn it marks does not call the model. One hundred agents means one hundred of those inboxes, each with a scripted reply.

One submit into an empty inbox measured 208.2 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: restart a failed child without pretending the restart did the child's work. Read Chapter 33: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
