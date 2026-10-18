# Social posts: Chapter 46, "The Kanban Board"

## X thread
1/
Chapter 46 is up: The Kanban Board.

One JSON file. One lock file. Claim, complete, review, or block is one transaction. The loser of the compare-and-swap does not become the owner. 🧵

2/
I looked at Hermes main at 9e01b44. hermes_cli/kanban_db.py uses WAL, BEGIN IMMEDIATE, and a compare-and-swap on tasks.status and claim_lock. SQLite serializes writers. One claimer wins. Losers see zero rows.

3/
_claim_and_open_run sets status to running only when claim_lock is null. rowcount other than 1 returns None. This crate uses create_new on board.lock. An empty result is refused. Pause blocks a new claim. Parents must be done.

4/
The demo prints "kanban: done". The bench id is 46/claim_transaction. The old ids 44/claim_transaction and 42/claim_transaction are aliases. The mean is 328.02 µs, mean.point_estimate from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

[personal note]

5/
Next is chapter 47, the message bus, and redact_for_egress on every body.

Read Chapter 46: [link]

#rustlang #AIagents

## LinkedIn
Chapter 46 is out: The Kanban Board.

The harness is the chapter 45 tree copied forward, plus a kanban module. The board is board.json. The transaction is board.lock, created with create_new. apply claims only when status is ready and the owner is empty. Anything else is Lost. An empty result is Empty and does not write done. A parent that is not done blocks the claim. Pause blocks Claim and still allows Complete. The model runs after the lock file is gone.

Hermes Agent at 9e01b44 keeps the card in SQLite. The hermes_cli/kanban_db.py module docstring names WAL, BEGIN IMMEDIATE, and the compare-and-swap on tasks.status and claim_lock. One claimer wins. Losers see zero rows. _claim_and_open_run updates a row to running where claim_lock is null, and returns None when rowcount is not 1. This chapter is that rule on a JSON file and a lock file.

The demo prints "kanban: done". The bench id is 46/claim_transaction. Aliases: 44/claim_transaction and 42/claim_transaction. The mean is 328.02 µs, mean.point_estimate from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

[personal note]

Next: chapter 47, the message bus, and redact_for_egress on every body. Read Chapter 46: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
