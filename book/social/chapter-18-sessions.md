# Social posts: Chapter 18, "Sessions on Disk"

## X thread
1/
Chapter 18 is up: Sessions on Disk.

A new process used to start from nothing. A turn now appends to SQLite in WAL mode, and the same session can be read back. Search is a queue of row ids. The index is the next chapter. 🧵

2/
Hermes Agent keeps sessions and messages in SQLite and sets WAL on the writer connection (hermes_state.py). The FTS tables, including messages_fts_trigram, are separate from that insert.

3/
This crate writes the two tables and queues the new ids, capped at 64. It does not create the virtual table. The chapter 9 eval set still passes on the mock.

4/
One WAL append measured CI_MEAN_18_WAL. The demo reply is pong. Two rows. Journal mode wal. No key.

[personal note]

5/
Next is MEMORY.md and USER.md: a frozen snapshot, a char budget, and a scan before the write.

Read Chapter 18: [link]

#rustlang #AIagents

## LinkedIn
Chapter 18 is out: Sessions on Disk.

The conversation finally survives the process. One turn is a single SQLite transaction: a user row and an assistant row, in WAL mode. The session row is inserted once. Search does not run on that path. The new row ids sit on a queue until the recall chapter builds the index.

I re-ran the chapter 9 eval set on the mock. It still passes. One WAL append measured CI_MEAN_18_WAL.

[personal note: one takeaway from writing this chapter]

Next: memory files, and a snapshot that a mid-session write does not change. Read Chapter 18: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
