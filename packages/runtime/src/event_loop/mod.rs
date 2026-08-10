//! Event loop — async task management for the runtime.
//!
//! Inspired by Deno's `deno_core::event_loop`. Uses tokio's JoinSet to
//! manage async tasks spawned by the runtime.

use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::{sync::mpsc, task::JoinSet};

/// A task handle for the event loop.
pub type TaskId = u64;

/// Messages sent to the event loop.
pub enum EventLoopMsg {
    /// Spawn a new async task.
    Spawn {
        id: TaskId,
        task: tokio::task::JoinHandle<()>,
    },
    /// Cancel a task by ID.
    Cancel(TaskId),
    /// Stop the event loop.
    Stop,
}

/// The event loop — manages async tasks spawned by the runtime.
pub struct EventLoop {
    running: Arc<AtomicBool>,
    sender: mpsc::UnboundedSender<EventLoopMsg>,
    /// The JoinSet of all running tasks.
    pub(crate) join_set: JoinSet<Result<(), anyhow::Error>>,
    /// Task metadata indexed by ID.
    pub(crate) tasks: HashMap<TaskId, TaskInfo>,
    next_id: u64,
}

/// Metadata about a running task.
pub struct TaskInfo {
    pub id: TaskId,
    pub name: String,
    pub created_at: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        let (sender, _receiver) = mpsc::unbounded_channel();
        Self {
            running: Arc::new(AtomicBool::new(false)),
            sender,
            join_set: JoinSet::new(),
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    /// Start the event loop.
    pub fn start(&mut self) {
        self.running.store(true, Ordering::Relaxed);
    }

    /// Stop the event loop.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        self.join_set.abort_all();
    }

    /// Spawn a new async task.
    pub fn spawn<F>(&mut self, name: &str, future: F)
    where
        F: std::future::Future<Output = Result<(), anyhow::Error>>
            + Send
            + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.insert(
            id,
            TaskInfo {
                id,
                name: name.to_string(),
                created_at: std::time::Instant::now(),
            },
        );
        self.join_set.spawn(future);
    }

    /// Run all pending tasks until completion.
    pub async fn run_pending(&mut self) {
        while let Some(result) = self.join_set.join_next().await {
            if let Err(e) = result {
                eprintln!("Task failed: {e}");
            }
        }
    }

    /// Run the event loop until stopped.
    pub async fn run(&mut self) {
        self.running.store(true, Ordering::Relaxed);
        while self.running.load(Ordering::Relaxed) {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }

    /// Number of active tasks.
    pub fn active_count(&self) -> usize {
        self.join_set.len()
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
    pub fn sender(&self) -> mpsc::UnboundedSender<EventLoopMsg> {
        self.sender.clone()
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}
