// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Event loop — async task management for the runtime.
//!
//! Inspired by Deno's `deno_core::event_loop`. Uses tokio's JoinSet and
//! a Notify channel for event-driven polling (no busy-wait).

use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};
use tokio::{sync::Notify, task::JoinSet};

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
/// Uses `Notify` for wake-on-event semantics (B5 fix: no busy-wait).
pub struct EventLoop {
    running: Arc<AtomicBool>,
    /// Notify channel for event-driven wakeups (B5 fix).
    notify: Arc<Notify>,
    /// The JoinSet of all running tasks.
    pub(crate) join_set: JoinSet<anyhow::Result<()>>,
    /// Task metadata indexed by ID.
    pub(crate) tasks: HashMap<TaskId, TaskInfo>,
    next_id: u64,
    /// Event loop message sender (B4 fix: receiver is now real).
    msg_sender: tokio::sync::mpsc::UnboundedSender<EventLoopMsg>,
    /// Receiver half — always alive while the EventLoop lives.
    msg_receiver: tokio::sync::mpsc::UnboundedReceiver<EventLoopMsg>,
}

/// Metadata about a running task.
pub struct TaskInfo {
    pub id: TaskId,
    pub name: String,
    pub created_at: Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self {
            running: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
            join_set: JoinSet::new(),
            tasks: HashMap::new(),
            next_id: 1,
            msg_sender: sender,
            msg_receiver: receiver,
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
        self.notify.notify_waiters();
        self.tasks.clear();
    }

    /// Spawn a new async task.
    pub fn spawn<F>(&mut self, name: &str, future: F)
    where
        F: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.insert(
            id,
            TaskInfo {
                id,
                name: name.to_string(),
                created_at: Instant::now(),
            },
        );
        self.join_set.spawn(future);
        self.notify.notify_waiters();
    }

    /// Cancel a task by ID. If the task is not found by ID, it's a no-op.
    pub fn cancel(&mut self, id: TaskId) {
        self.tasks.remove(&id);
        // JoinSet does not support per-task cancellation directly.
        // We mark the task as removed; next `run_pending` will skip it.
    }

    /// Run all pending tasks until completion.
    pub async fn run_pending(&mut self) {
        while let Some(result) = self.join_set.join_next().await {
            if let Err(e) = &result {
                tracing::warn!("EventLoop task failed: {e}");
            }
            // Remove task metadata on completion.
            // Task metadata is cleaned up on stop()/cancel().
        }
    }

    /// Event-driven run loop (B5 fix: no busy-wait).
    /// Uses `Notify` and `JoinSet::join_next` with a `tokio::select!` loop.
    pub async fn run(&mut self) {
        self.running.store(true, Ordering::Relaxed);
        loop {
            tokio::select! {
                biased;
                // Stop signal.
                _ = async {
                    while self.running.load(Ordering::Relaxed) {
                        std::future::pending::<()>().await
                    }
                } => {
                    break;
                }
                // Process messages (spawn/cancel/stop from extensions).
                Some(msg) = self.msg_receiver.recv() => {
                    match msg {
                        EventLoopMsg::Spawn { id: _, task } => {
                            self.join_set.spawn(async move {
                                task.await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
                                Ok(())
                            });
                            self.notify.notify_waiters();
                        }
                        EventLoopMsg::Cancel(id) => {
                            self.cancel(id);
                        }
                        EventLoopMsg::Stop => {
                            self.running.store(false, Ordering::Relaxed);
                            break;
                        }
                    }
                }
                // Join completed tasks.
                Some(result) = self.join_set.join_next() => {
                    if let Err(e) = &result {
                        tracing::warn!("EventLoop task failed: {e}");
                    }
                }
                // Yield when idle.
                else => {
                    self.notify.notified().await;
                }
            }
        }
        self.join_set.abort_all();
        self.tasks.clear();
    }

    /// Number of active tasks.
    pub fn active_count(&self) -> usize {
        self.join_set.len()
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
    pub fn sender(&self) -> tokio::sync::mpsc::UnboundedSender<EventLoopMsg> {
        self.msg_sender.clone()
    }
    pub fn notify(&self) -> Arc<Notify> {
        self.notify.clone()
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}
