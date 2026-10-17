# Social posts: Chapter 42, "The Kanban Board"

## X thread
1/
Chapter 42 is up: The Kanban Board.

One JSON file. One lock file. Claim, complete, review, or block is one transaction. The loser of a race does not become the owner. 🧵

2/
Hermes Agent keeps the card in SQLite. hermes_cli/kanban_db.py claims with BEGIN IMMEDIATE and a compare-and-swap on status and claim_lock. An empty result cannot complete a card. Pause refuses a new claim.

3/
This crate uses create_new on board.lock, then writes the JSON back. Two processes: one claimed, one lost. Fake replies. No key.

4/
One claim transaction measured 81.55 µs. The demo prints "kanban: done".

[personal note]

5/
Next is the message bus: one small envelope, and a subscriber that hits a bounded channel.

Read Chapter 42: [link]

#rustlang #AIagents

## LinkedIn
Chapter 42 is out: The Kanban Board.

A blackboard in one process disappears on a crash, and a second process cannot see it. Hermes Agent stores the kanban card in SQLite so one claimer wins and the others see zero rows. This chapter is that rule on a JSON file. `transact` creates `board.lock` with `create_new`, reads the board, applies one op, writes it back, and deletes the lock. The file handle is closed before the delete so Windows can unlink it.

An empty result stays off `done`. A parent that is not `done` blocks the claim. A paused board refuses `Claim` and still allows complete of a card already running. Two processes spawned together produce one `claimed` and one `lost`.

One claim transaction measured 81.55 µs. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: the message bus. Read Chapter 42: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
