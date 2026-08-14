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

//! Structured runtime errors with stable codes and suggestions.
//!
//! Replaces the plain `OpError(String)` with a classified error type so
//! extensions and callers can match on error kinds programmatically.

use std::fmt;

/// Stable error classification for runtime failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeErrorKind {
    /// The requested op does not exist.
    OpNotFound,
    /// A sync op was invoked as async (or vice versa).
    OpMismatch,
    /// Extension dependency cycle detected.
    ExtensionCycle,
    /// Extension dependency is missing.
    MissingDependency,
    /// Permission denied by the runtime permission checker.
    PermissionDenied,
    /// A module could not be resolved or loaded.
    ModuleLoad,
    /// A module failed to evaluate.
    ModuleEvaluation,
    /// An op failed internally.
    OpExecution,
    /// The resource table operation failed (missing/closed resource).
    Resource,
    /// The arena/allocator ran out of space.
    OutOfMemory,
    /// A timeout was exceeded.
    Timeout,
    /// The runtime was shut down.
    Shutdown,
    /// Internal/unclassified error.
    Internal,
}

impl RuntimeErrorKind {
    /// Stable string code (used by agents and API consumers).
    pub fn code(&self) -> &'static str {
        match self {
            Self::OpNotFound => "op_not_found",
            Self::OpMismatch => "op_mismatch",
            Self::ExtensionCycle => "extension_cycle",
            Self::MissingDependency => "missing_dependency",
            Self::PermissionDenied => "permission_denied",
            Self::ModuleLoad => "module_load",
            Self::ModuleEvaluation => "module_evaluation",
            Self::OpExecution => "op_execution",
            Self::Resource => "resource",
            Self::OutOfMemory => "out_of_memory",
            Self::Timeout => "timeout",
            Self::Shutdown => "shutdown",
            Self::Internal => "internal",
        }
    }

    /// Suggested fixes for agent error tracking.
    pub fn suggested_fixes(&self) -> &'static [&'static str] {
        match self {
            Self::OpNotFound => &[
                "Check the op name is registered in an extension.",
                "Ensure the extension supplying the op is included in RuntimeOptions.",
            ],
            Self::OpMismatch => &[
                "Use op_sync for sync ops and op_async for async ops.",
            ],
            Self::ExtensionCycle => &[
                "Remove the cyclic dependency between extensions.",
            ],
            Self::MissingDependency => &[
                "Add the dependency extension to RuntimeOptions.extensions.",
            ],
            Self::PermissionDenied => &[
                "Configure Permissions to allow the requested operation.",
            ],
            Self::ModuleLoad => &[
                "Check the module specifier resolves to an existing file.",
                "Add the module's root directory to the ModuleLoader roots.",
            ],
            Self::ModuleEvaluation => &[
                "Inspect the module code for panics or invalid imports.",
            ],
            Self::Resource => &[
                "Check the resource ID was returned by the resource table.",
                "Ensure the resource has not already been closed.",
            ],
            Self::OutOfMemory => &[
                "Increase the arena size in RuntimeOptions.",
            ],
            _ => &[],
        }
    }
}

impl fmt::Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// A structured runtime error.
#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub message: String,
    /// The op or module name that caused the error, if any.
    pub source: Option<String>,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn kind(&self) -> RuntimeErrorKind {
        self.kind
    }

    pub fn code(&self) -> &'static str {
        self.kind.code()
    }

    pub fn suggested_fixes(&self) -> &'static [&'static str] {
        self.kind.suggested_fixes()
    }

    /// Convenience constructors.
    pub fn op_not_found(name: &str) -> Self {
        Self::new(
            RuntimeErrorKind::OpNotFound,
            format!("op not found: {name}"),
        )
        .with_source(name)
    }

    pub fn op_mismatch(name: &str, expected: &str) -> Self {
        Self::new(
            RuntimeErrorKind::OpMismatch,
            format!("op '{name}' requires {expected}"),
        )
        .with_source(name)
    }

    pub fn extension_cycle(nodes: &[&str]) -> Self {
        Self::new(
            RuntimeErrorKind::ExtensionCycle,
            format!("extension dependency cycle detected: {}", nodes.join(" -> ")),
        )
    }

    pub fn missing_dependency(ext: &str, dep: &str) -> Self {
        Self::new(
            RuntimeErrorKind::MissingDependency,
            format!("extension '{ext}' depends on missing extension '{dep}'"),
        )
    }

    pub fn out_of_memory() -> Self {
        Self::new(RuntimeErrorKind::OutOfMemory, "arena allocation failed")
    }

    pub fn resource(message: impl Into<String>) -> Self {
        Self::new(RuntimeErrorKind::Resource, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(RuntimeErrorKind::Internal, message)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            Some(src) => write!(f, "[{}] {} (in {src})", self.kind, self.message),
            None => write!(f, "[{}] {}", self.kind, self.message),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<std::io::Error> for RuntimeError {
    fn from(e: std::io::Error) -> Self {
        Self::new(RuntimeErrorKind::Internal, e.to_string())
    }
}

impl From<serde_json::Error> for RuntimeError {
    fn from(e: serde_json::Error) -> Self {
        Self::new(RuntimeErrorKind::Internal, e.to_string())
    }
}

impl From<anyhow::Error> for RuntimeError {
    fn from(e: anyhow::Error) -> Self {
        Self::new(RuntimeErrorKind::Internal, e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codes() {
        assert_eq!(RuntimeErrorKind::OpNotFound.code(), "op_not_found");
        assert_eq!(RuntimeErrorKind::ExtensionCycle.code(), "extension_cycle");
    }

    #[test]
    fn test_display() {
        let err = RuntimeError::op_not_found("foo");
        assert!(err.to_string().contains("foo"));
        assert_eq!(err.code(), "op_not_found");
    }

    #[test]
    fn test_suggested_fixes() {
        let err = RuntimeError::op_not_found("x");
        assert!(!err.suggested_fixes().is_empty());
    }
}