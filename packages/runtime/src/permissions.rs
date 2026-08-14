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

//! Permission system — controls access to FS, net, env, run, and sys calls.
//!
//! Inspired by Deno's permission model. Each permission has a tri-state:
//! `Allow`, `Deny`, or `Prompt` (default).

use crate::error::{RuntimeError, RuntimeErrorKind};
use std::path::Path;

/// Tri-state permission value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    /// Granted (possibly with a blanket allow).
    Allow,
    /// Denied (possibly with a blanket deny).
    Deny,
    /// Not yet decided — caller should prompt or fall back to default.
    Prompt,
}

impl Default for PermissionState {
    fn default() -> Self {
        Self::Prompt
    }
}

/// A permission descriptor returned by check methods.
#[derive(Debug, Clone)]
pub struct PermissionCheck {
    pub state: PermissionState,
    pub description: String,
}

/// Granular permissions for a runtime worker.
#[derive(Debug, Clone)]
pub struct Permissions {
    /// Allowed file system paths (blanket: `["/"]` means all, `[]` means none).
    pub allow_fs: Vec<String>,
    /// Denied file system paths (checked after allow).
    pub deny_fs: Vec<String>,

    /// Allowed network hosts (e.g. `["github.com:443"]`, `["0.0.0.0:0"]` = all).
    pub allow_net: Vec<String>,
    pub deny_net: Vec<String>,

    /// Allowed environment variable names (`["*"]` = all).
    pub allow_env: Vec<String>,
    pub deny_env: Vec<String>,

    /// Allowed run commands (e.g. `["npm", "git"]`).
    pub allow_run: Vec<String>,

    /// Allow sys info access.
    pub allow_sys: bool,

    /// Allow reading (separate from write). If `allow_fs` contains a path,
    /// read is allowed by default.
    pub allow_read: bool,

    /// Allow writing.
    pub allow_write: bool,
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            allow_fs: Vec::new(),
            deny_fs: Vec::new(),
            allow_net: Vec::new(),
            deny_net: Vec::new(),
            allow_env: Vec::new(),
            deny_env: Vec::new(),
            allow_run: Vec::new(),
            allow_sys: false,
            allow_read: true,
            allow_write: false,
        }
    }
}

impl Permissions {
    /// Create a set of permissions that allows everything.
    pub fn all() -> Self {
        Self {
            allow_fs: vec!["/".to_string()],
            deny_fs: Vec::new(),
            allow_net: vec!["0.0.0.0:0".to_string()],
            deny_net: Vec::new(),
            allow_env: vec!["*".to_string()],
            deny_env: Vec::new(),
            allow_run: vec!["*".to_string()],
            allow_sys: true,
            allow_read: true,
            allow_write: true,
        }
    }

    /// Create a set of permissions that denies everything.
    pub fn none() -> Self {
        Self {
            allow_read: false,
            allow_write: false,
            ..Default::default()
        }
    }

    // ── FS checks ──────────────────────────────────────────────────────

    /// Check if a file system path is allowed for reading.
    pub fn check_fs_read(&self, path: &str) -> Result<(), RuntimeError> {
        self.check_fs_inner(path, false)
    }

    /// Check if a file system path is allowed for writing.
    pub fn check_fs_write(&self, path: &str) -> Result<(), RuntimeError> {
        self.check_fs_inner(path, true)
    }

    fn check_fs_inner(&self, path: &str, write: bool) -> Result<(), RuntimeError> {
        let p = Path::new(path);
        let canonical = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        let canonical_str = canonical.to_string_lossy();

        // Check deny list first.
        for denied in &self.deny_fs {
            if self.path_matches(canonical_str.as_ref(), denied) {
                return Err(self.denied_err("fs", path));
            }
        }

        // Check allow list.
        if self.allow_fs.is_empty() {
            // No blanket allowance.
            // If allow_read is true and this is a read, allow.
            if !write && self.allow_read {
                return Ok(());
            }
            if write && self.allow_write {
                return Ok(());
            }
            return Err(self.denied_err("fs", path));
        }

        for allowed in &self.allow_fs {
            if self.path_matches(canonical_str.as_ref(), allowed) {
                return Ok(());
            }
        }

        Err(self.denied_err("fs", path))
    }

    fn path_matches(&self, path: &str, pattern: &str) -> bool {
        if pattern == "/" {
            return true;
        }
        path.starts_with(pattern) || path == pattern
    }

    // ── Net checks ─────────────────────────────────────────────────────

    /// Check if a network host:port is allowed.
    pub fn check_net(&self, host: &str, port: u16) -> Result<(), RuntimeError> {
        // Check deny list.
        for denied in &self.deny_net {
            if self.net_matches(host, port, denied) {
                return Err(self.denied_err("net", &format!("{host}:{port}")));
            }
        }

        if self.allow_net.is_empty() {
            return Err(self.denied_err("net", &format!("{host}:{port}")));
        }

        for allowed in &self.allow_net {
            if self.net_matches(host, port, allowed) {
                return Ok(());
            }
        }

        Err(self.denied_err("net", &format!("{host}:{port}")))
    }

    fn net_matches(&self, host: &str, port: u16, pattern: &str) -> bool {
        // "0.0.0.0:0" means all.
        if pattern == "0.0.0.0:0" {
            return true;
        }
        if let Some((p_host, p_port)) = pattern.split_once(':') {
            let p_port: u16 = p_port.parse().unwrap_or(0);
            if p_port != 0 && p_port != port {
                return false;
            }
            p_host == host || p_host == "*" || host.ends_with(&format!(".{p_host}"))
        } else {
            pattern == host || pattern == "*"
        }
    }

    // ── Env checks ─────────────────────────────────────────────────────

    /// Check if an environment variable is allowed.
    pub fn check_env(&self, var: &str) -> Result<(), RuntimeError> {
        for denied in &self.deny_env {
            if denied == var || denied == "*" {
                return Err(self.denied_err("env", var));
            }
        }
        if self.allow_env.is_empty() {
            return Err(self.denied_err("env", var));
        }
        for allowed in &self.allow_env {
            if allowed == var || allowed == "*" {
                return Ok(());
            }
        }
        Err(self.denied_err("env", var))
    }

    // ── Run checks ─────────────────────────────────────────────────────

    /// Check if a command is allowed to run.
    pub fn check_run(&self, cmd: &str) -> Result<(), RuntimeError> {
        if self.allow_run.is_empty() {
            return Err(self.denied_err("run", cmd));
        }
        for allowed in &self.allow_run {
            if allowed == cmd || allowed == "*" {
                return Ok(());
            }
        }
        Err(self.denied_err("run", cmd))
    }

    // ── Sys check ──────────────────────────────────────────────────────

    /// Check if sys info access is allowed.
    pub fn check_sys(&self) -> Result<(), RuntimeError> {
        if self.allow_sys {
            Ok(())
        } else {
            Err(self.denied_err("sys", "sysinfo"))
        }
    }

    fn denied_err(&self, kind: &str, target: &str) -> RuntimeError {
        RuntimeError::new(
            RuntimeErrorKind::PermissionDenied,
            format!("{kind} permission denied for '{target}'"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_permissions() {
        let p = Permissions::all();
        assert!(p.check_fs_read("/etc/passwd").is_ok());
        assert!(p.check_fs_write("/tmp").is_ok());
        assert!(p.check_net("github.com", 443).is_ok());
        assert!(p.check_env("HOME").is_ok());
        assert!(p.check_sys().is_ok());
    }

    #[test]
    fn test_none_permissions() {
        let p = Permissions::none();
        assert!(p.check_fs_read("/etc/passwd").is_err());
        assert!(p.check_net("localhost", 8080).is_err());
        assert!(p.check_env("PATH").is_err());
        assert!(p.check_sys().is_err());
    }

    #[test]
    fn test_net_matching() {
        let p = Permissions {
            allow_net: vec!["github.com:443".into()],
            ..Permissions::none()
        };
        assert!(p.check_net("github.com", 443).is_ok());
        assert!(p.check_net("github.com", 80).is_err());
        assert!(p.check_net("example.com", 443).is_err());
    }

    #[test]
    fn test_env_deny_overrides_allow() {
        let p = Permissions {
            allow_env: vec!["*".into()],
            deny_env: vec!["SECRET".into()],
            ..Permissions::none()
        };
        assert!(p.check_env("PATH").is_ok());
        assert!(p.check_env("SECRET").is_err());
    }
}