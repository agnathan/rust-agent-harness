//! Book chapter 31. A board accepts 100 tasks and refuses the 101st.
//! Gather runs them under the supervisor's bound and returns summaries
//! in task_index order. The parent does not see child tool traces.

use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use supervisor::{Pool, WORKER_BOUND};

pub const QUEUE_CAP: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub task_index: usize,
    pub reply: String,
    pub tool_trace: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanError {
    Full,
}

pub struct Board {
    queue: Mutex<VecDeque<Task>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.lock().expect("queue").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn schedule(&self, task: Task) -> Result<(), FanError> {
        let mut queue = self.queue.lock().expect("queue");
        if queue.len() == QUEUE_CAP {
            return Err(FanError::Full);
        }
        queue.push_back(task);
        Ok(())
    }

    pub fn gather(&self, pool: &Pool) -> String {
        self.gather_holding(pool, Duration::ZERO)
    }

    pub fn gather_holding(&self, pool: &Pool, hold: Duration) -> String {
        let tasks: Vec<Task> = self.queue.lock().expect("queue").drain(..).collect();
        let mut rows: Vec<(usize, String)> = thread::scope(|scope| {
            let handles: Vec<_> = tasks
                .iter()
                .map(|task| {
                    scope.spawn(|| {
                        let _permit = pool.acquire();
                        if !hold.is_zero() {
                            thread::sleep(hold);
                        }
                        (task.task_index, task.reply.clone())
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|handle| handle.join().expect("worker"))
                .collect()
        });
        rows.sort_by_key(|(index, _)| *index);
        rows.into_iter()
            .map(|(_, reply)| reply)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

pub fn demo() -> String {
    let board = Board::new();
    let pool = Pool::new(WORKER_BOUND);
    for (index, reply) in [(2, "c"), (0, "a"), (1, "b")] {
        board
            .schedule(Task {
                task_index: index,
                reply: reply.into(),
                tool_trace: "secret-tool".into(),
            })
            .expect("schedule");
    }
    board.gather(&pool)
}
