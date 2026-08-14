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

//! Access Control plugin — pure RBAC functions used by admin, organization, etc.
//! No HTTP routes — only the AccessPlugin empty router.

use crate::plugin::AuthPlugin;
use serde::{Deserialize, Serialize};

/// A single statement in an RBAC policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    /// Effect: "allow" or "deny".
    pub effect: String,
    /// Actions this statement applies to (e.g. ["org:create", "user:delete"]).
    pub actions: Vec<String>,
    /// Resources this statement applies to (e.g. ["org:*", "user:*"]).
    pub resources: Vec<String>,
}

/// A role definition with a set of statements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub statements: Vec<Statement>,
}

/// The result of an authorization check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authorization {
    Allowed,
    Denied,
}

/// Check whether a set of statements authorize a given action on a resource.
/// Uses AND logic within a single statement, OR logic across statements.
/// Deny statements override allow statements.
pub fn authorize(statements: &[Statement], action: &str, resource: &str) -> Authorization {
    let mut allowed = false;
    for stmt in statements {
        let action_match = stmt.actions.iter().any(|a| a == action || a == "*");
        let resource_match = stmt.resources.iter().any(|r| r == resource || r == "*");
        if action_match && resource_match {
            match stmt.effect.as_str() {
                "deny" => return Authorization::Denied,
                "allow" => allowed = true,
                _ => {}
            }
        }
    }
    if allowed {
        Authorization::Allowed
    } else {
        Authorization::Denied
    }
}

/// Check if a role allows a specific action on a resource.
pub fn role_allows(role: &Role, action: &str, resource: &str) -> bool {
    authorize(&role.statements, action, resource) == Authorization::Allowed
}

/// The built-in admin role with full access.
pub fn admin_role() -> Role {
    Role {
        name: "admin".into(),
        statements: vec![Statement {
            effect: "allow".into(),
            actions: vec!["*".into()],
            resources: vec!["*".into()],
        }],
    }
}

/// The built-in user role with basic self-service access.
pub fn user_role() -> Role {
    Role {
        name: "user".into(),
        statements: vec![Statement {
            effect: "allow".into(),
            actions: vec![
                "session:read".into(),
                "session:revoke".into(),
                "user:read".into(),
                "user:update".into(),
            ],
            resources: vec!["self:*".into()],
        }],
    }
}

/// The AccessPlugin registers no routes but provides RBAC utilities.
pub struct AccessPlugin;

impl AuthPlugin for AccessPlugin {
    fn name(&self) -> &'static str {
        "access"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_allowed() {
        let role = admin_role();
        assert!(role_allows(&role, "org:create", "org:*"));
        assert!(role_allows(&role, "user:delete", "user:*"));
    }

    #[test]
    fn test_user_self_only() {
        let role = user_role();
        assert!(role_allows(&role, "user:read", "self:*"));
        assert!(!role_allows(&role, "user:delete", "other:*"));
    }

    #[test]
    fn test_deny_overrides_allow() {
        let statements = vec![
            Statement {
                effect: "allow".into(),
                actions: vec!["read".into()],
                resources: vec!["doc:1".into()],
            },
            Statement {
                effect: "deny".into(),
                actions: vec!["read".into()],
                resources: vec!["doc:1".into()],
            },
        ];
        assert_eq!(authorize(&statements, "read", "doc:1"), Authorization::Denied);
    }
}