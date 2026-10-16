# Social posts: Chapter 12, "Hooks and Plugins"

## X thread
1/
Chapter 12 is up: Hooks and Plugins.

A plugin can refuse a tool call, rewrite the arguments, or send the call to the approval prompt from chapter 11. The availability check still has to pass. 🧵

2/
Hermes Agent returns (block_message, final_args) from the pre_tool_call hook in agent/tool_executor.py. hermes_cli/plugins.py gives block priority over approve. A hook that throws does not block the call.

3/
The API-request hook borrows the messages. It does not copy them. The memory backend in this chapter is a queue of 64. The files come later.

4/
An empty before/after pair measured CI_MEAN_12_EMPTY. The borrow of 1,000 messages measured CI_MEAN_12_API. One enqueue measured CI_MEAN_12_MEM.

[personal note]

5/
Next is a child process for code, and a checkpoint you can rewind. The sandbox stays off unless you turn it on. That is how Hermes ships it too.

Read Chapter 12: [link]

#rustlang #AIagents

## LinkedIn
Chapter 12 is out: Hooks and Plugins.

The approval pipeline from the previous chapter sees shell commands. A plugin still needed a way to veto any tool, change its arguments, or escalate that tool into the same human prompt. Hermes Agent does this with a pre_tool_call hook that returns a block message and the final arguments, and a block wins even if another plugin already asked for approval.

I kept that order. The hook that sees the outbound messages borrows them. A memory backend records the turn on a bounded queue and does not write a file yet. An empty hook pair measured CI_MEAN_12_EMPTY.

[personal note: one takeaway from writing this chapter]

Next: running code in a child process. Read Chapter 12: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
