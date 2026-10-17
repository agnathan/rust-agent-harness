# Social posts: Chapter 10, "The Turn State Machine"

## X thread
1/
Chapter 10 is up: The Turn State Machine.

Chapter 9 already returns one outcome when a turn ends. This chapter is the table of which phase may follow which event. The test walks every pair. 🧵

2/
Hermes has a TurnState in gateway/session_state.py. It is the pointers for one gateway turn, and clear() empties them. _LoopState in agent/conversation_loop.py is the bag of locals the phase helpers pass by name. Neither one refuses an illegal move.

3/
Fifteen moves are legal. A terminal phase accepts nothing. Soft stop and a budget wrap-up both enter WrappingUp. A text answer leaves as SoftStopped or Budget. A wrap-up that still wants tools is Partial.

4/
The fake model is a list of script steps. No key. One transition measured 1.103 ns. The three-step script measured 7.803 ns.

[personal note]

5/
Approvals are next. The wait for a human has to sit in this table without holding the tool permit. This chapter does not add that wait.

Read Chapter 10: [link]

#rustlang #AIagents

## LinkedIn
Chapter 10 is out: The Turn State Machine.

The agent loop in the previous chapter already stops for a cancel, a budget, a stall, or a bad tool call. What it did not have was a single match that says which of those exits can follow which phase. Hermes Agent keeps that information in a dataclass of pointers and a bag of loop locals. I wrote the match, and a test that tries every phase against every event.

A scripted turn with no model goes from a tool call to a tool result to a text answer and ends Finished. One transition measured 1.103 ns.

[personal note: one takeaway from writing this chapter]

Next: approvals. Read Chapter 10: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
