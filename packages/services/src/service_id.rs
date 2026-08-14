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

//! Service identifier — namespace/name with safe-path encoding.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A unique identifier for a service, with optional namespace.
///
/// Format: `name` or `namespace/name`.
/// The name is encoded into a safe filesystem path.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceId {
    /// Optional namespace (e.g., "project", "system").
    pub namespace: Option<String>,
    /// Service name.
    pub name: String,
}

impl ServiceId {
    /// Create a new ServiceId from a namespace and name.
    pub fn new(namespace: Option<impl Into<String>>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.map(|n| n.into()),
            name: name.into(),
        }
    }

    /// Create a ServiceId from just a name (no namespace).
    pub fn from_name(name: impl Into<String>) -> Self {
        Self {
            namespace: None,
            name: name.into(),
        }
    }

    /// Return the safe-path-encoded form of this ID.
    /// Used for log files, state files, and socket paths.
    pub fn encoded(&self) -> String {
        match &self.namespace {
            Some(ns) => format!("{}/{}", sanitize(ns), sanitize(&self.name)),
            None => sanitize(&self.name),
        }
    }

    /// Return the display name: `namespace/name` or `name`.
    pub fn display_name(&self) -> String {
        match &self.namespace {
            Some(ns) => format!("{ns}/{}", self.name),
            None => self.name.clone(),
        }
    }

    /// Return this ID as a filesystem-safe path component.
    pub fn to_path_component(&self) -> String {
        self.encoded().replace('/', "_")
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for ServiceId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("service ID cannot be empty".to_string());
        }
        if let Some((ns, name)) = s.split_once('/') {
            if ns.is_empty() || name.is_empty() {
                return Err("namespace and name must not be empty".to_string());
            }
            Ok(Self {
                namespace: Some(ns.to_string()),
                name: name.to_string(),
            })
        } else {
            Ok(Self {
                namespace: None,
                name: s.to_string(),
            })
        }
    }
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let id: ServiceId = "redis".parse().unwrap();
        assert_eq!(id.name, "redis");
        assert!(id.namespace.is_none());

        let id: ServiceId = "project/api".parse().unwrap();
        assert_eq!(id.namespace, Some("project".into()));
        assert_eq!(id.name, "api");
    }

    #[test]
    fn test_encoded() {
        let id = ServiceId::new(Some("my-project"), "web-api");
        assert_eq!(id.encoded(), "my-project/web-api");
    }

    #[test]
    fn test_display() {
        let id = ServiceId::from_name("redis");
        assert_eq!(id.to_string(), "redis");

        let id = ServiceId::new(Some("dev"), "api");
        assert_eq!(id.to_string(), "dev/api");
    }

    #[test]
    fn test_path_component() {
        let id = ServiceId::new(Some("ns"), "svc");
        assert_eq!(id.to_path_component(), "ns_svc");
    }
}