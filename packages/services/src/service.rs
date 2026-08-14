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

//! Runtime state for a single service.

use crate::config::ServiceConfig;
use crate::retry::RetryState;
use crate::service_id::ServiceId;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::process::Child;
use tokio::sync::Mutex;

/// The current status of a service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceStatus {
    /// Not started.
    Stopped,
    /// Starting up (ready checks in progress).
    Starting,
    /// Running and ready.
    Running,
    /// Stopping.
    Stopping,
    /// Failed (exited with error).
    Failed,
    /// Waiting for dependencies.
    Waiting,
}

impl ServiceStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, ServiceStatus::Starting | ServiceStatus::Running | ServiceStatus::Stopping)
    }
}

/// Runtime state for a single managed service.
#[derive(Debug)]
pub struct Service {
    /// Unique identifier.
    pub id: ServiceId,
    /// Configuration.
    pub config: ServiceConfig,
    /// Current status.
    pub status: ServiceStatus,
    /// Process handle (None if not running).
    pub child: Option<Child>,
    /// Retry state.
    pub retry: RetryState,
    /// Accumulated stdout/stderr for ready checking.
    pub output_buffer: Arc<Mutex<String>>,
    /// Whether the service should be kept alive (cancellation token).
    pub keep_alive: Arc<AtomicBool>,
    /// PID of the running process.
    pub pid: Option<u32>,
    /// Start timestamp.
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Ports that this service is known to listen on.
    pub ports: Vec<u16>,
}

impl Service {
    /// Create a new service from its config.
    pub fn new(id: ServiceId, config: ServiceConfig) -> Self {
        Self {
            id,
            config,
            status: ServiceStatus::Stopped,
            child: None,
            retry: RetryState::new(),
            output_buffer: Arc::new(Mutex::new(String::new())),
            keep_alive: Arc::new(AtomicBool::new(true)),
            pid: None,
            started_at: None,
            ports: Vec::new(),
        }
    }

    /// Mark the service as started.
    pub fn mark_started(&mut self, child: Child) {
        self.child = Some(child);
        self.status = ServiceStatus::Starting;
        self.pid = None; // will be set when child is polled
        self.started_at = Some(chrono::Utc::now());
        self.retry.reset();
    }

    /// Mark the service as running/ready.
    pub fn mark_ready(&mut self) {
        self.status = ServiceStatus::Running;
    }

    /// Mark as stopping.
    pub fn mark_stopping(&mut self) {
        self.status = ServiceStatus::Stopping;
    }

    /// Mark as stopped.
    pub fn mark_stopped(&mut self) {
        self.status = ServiceStatus::Stopped;
        self.child = None;
        self.keep_alive.store(false, Ordering::SeqCst);
    }

    /// Mark as failed.
    pub fn mark_failed(&mut self) {
        self.status = ServiceStatus::Failed;
        self.child = None;
    }

    /// Append output to the buffer (for ready checking).
    pub async fn append_output(&self, line: &str) {
        let mut buf = self.output_buffer.lock().await;
        buf.push_str(line);
        buf.push('\n');
        // Keep only the last 64KB.
        if buf.len() > 65_536 {
            let keep_len = buf.len() - 65_536;
            *buf = buf.split_off(keep_len);
        }
    }

    /// Check if the service should be kept alive.
    pub fn is_alive(&self) -> bool {
        self.keep_alive.load(Ordering::SeqCst)
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.id, self.status_str())
    }
}

impl Service {
    fn status_str(&self) -> &'static str {
        match self.status {
            ServiceStatus::Stopped => "stopped",
            ServiceStatus::Starting => "starting",
            ServiceStatus::Running => "running",
            ServiceStatus::Stopping => "stopping",
            ServiceStatus::Failed => "failed",
            ServiceStatus::Waiting => "waiting",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_transitions() {
        let id = ServiceId::from_name("test");
        let config = ServiceConfig::default();
        let mut svc = Service::new(id, config);
        assert_eq!(svc.status, ServiceStatus::Stopped);
        svc.mark_ready();
        assert_eq!(svc.status, ServiceStatus::Running);
        svc.mark_stopped();
        assert_eq!(svc.status, ServiceStatus::Stopped);
    }
}