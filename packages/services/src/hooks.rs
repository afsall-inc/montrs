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

//! Lifecycle hooks — run commands on service state transitions.

use crate::config::LifecycleHooks;
use tracing::error;

/// The set of lifecycle events that can trigger hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleEvent {
    Ready,
    Fail,
    Retry,
    Stop,
    Exit,
}

impl LifecycleEvent {
    fn command_key(&self) -> &'static str {
        match self {
            LifecycleEvent::Ready => "on_ready",
            LifecycleEvent::Fail => "on_fail",
            LifecycleEvent::Retry => "on_retry",
            LifecycleEvent::Stop => "on_stop",
            LifecycleEvent::Exit => "on_exit",
        }
    }
}

/// Runs lifecycle hooks as shell commands.
pub struct HookRunner;

impl HookRunner {
    /// Get the command for a given event, if configured.
    pub fn command_for(hooks: &LifecycleHooks, event: LifecycleEvent) -> Option<String> {
        match event {
            LifecycleEvent::Ready => hooks.on_ready.clone(),
            LifecycleEvent::Fail => hooks.on_fail.clone(),
            LifecycleEvent::Retry => hooks.on_retry.clone(),
            LifecycleEvent::Stop => hooks.on_stop.clone(),
            LifecycleEvent::Exit => hooks.on_exit.clone(),
        }
    }

    /// Run the hook for an event, if configured. Non-blocking.
    pub async fn run_if_present(
        service: &str,
        hooks: &LifecycleHooks,
        event: LifecycleEvent,
    ) {
        if let Some(cmd) = Self::command_for(hooks, event) {
            let result = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .spawn();
            match result {
                Ok(mut child) => {
                    if let Err(e) = child.wait().await {
                        error!(
                            "service {}: hook {} failed to run: {e}",
                            service,
                            event.command_key()
                        );
                    }
                }
                Err(e) => {
                    error!(
                        "service {}: failed to spawn hook {}: {e}",
                        service,
                        event.command_key()
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_for() {
        let hooks = LifecycleHooks {
            on_ready: Some("echo ready".into()),
            ..Default::default()
        };
        assert_eq!(
            HookRunner::command_for(&hooks, LifecycleEvent::Ready),
            Some("echo ready".to_string())
        );
        assert_eq!(HookRunner::command_for(&hooks, LifecycleEvent::Fail), None);
    }
}