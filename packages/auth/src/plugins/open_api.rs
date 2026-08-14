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

//! OpenAPI reference plugin — GET /reference returns minimal OpenAPI JSON
//! listing known core auth paths.

use crate::{AuthError, context::AuthState, plugin::AuthPlugin};
use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

/// OpenApiPlugin — serves a minimal OpenAPI spec at /reference.
pub struct OpenApiPlugin {
    state: Option<AuthState>,
}

impl OpenApiPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for OpenApiPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for OpenApiPlugin {
    fn name(&self) -> &'static str {
        "open_api"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("OpenApiPlugin: state not set");
        Router::new()
            .route("/reference", get(reference))
            .with_state(state)
    }
}

async fn reference() -> Json<Value> {
    Json(json!({
        "openapi": "3.1.0",
        "info": {
            "title": "MontRS Auth API",
            "version": "1.0.0",
            "description": "Authentication endpoints for MontRS"
        },
        "paths": {
            "/api/auth/sign-up/email": {
                "post": { "summary": "Sign up with email/password", "tags": ["Auth"] }
            },
            "/api/auth/sign-in/email": {
                "post": { "summary": "Sign in with email/password", "tags": ["Auth"] }
            },
            "/api/auth/sign-in/social": {
                "post": { "summary": "Sign in with OAuth social provider", "tags": ["Auth"] }
            },
            "/api/auth/get-session": {
                "get": { "summary": "Get current session", "tags": ["Session"] }
            },
            "/api/auth/list-sessions": {
                "post": { "summary": "List all sessions for user", "tags": ["Session"] }
            },
            "/api/auth/revoke-session": {
                "post": { "summary": "Revoke a session", "tags": ["Session"] }
            },
            "/api/auth/sign-out": {
                "post": { "summary": "Sign out current session", "tags": ["Session"] }
            },
            "/api/auth/change-password": {
                "post": { "summary": "Change password", "tags": ["Password"] }
            },
            "/api/auth/set-password": {
                "post": { "summary": "Set password for OAuth-only accounts", "tags": ["Password"] }
            },
            "/api/auth/verify-password": {
                "post": { "summary": "Verify current password", "tags": ["Password"] }
            },
            "/api/auth/forgot-password": {
                "post": { "summary": "Send password reset email", "tags": ["Password"] }
            },
            "/api/auth/reset-password": {
                "post": { "summary": "Reset password with token", "tags": ["Password"] }
            },
            "/api/auth/send-verification-email": {
                "post": { "summary": "Send email verification", "tags": ["Email"] }
            },
            "/api/auth/verify-email": {
                "get": { "summary": "Verify email with token", "tags": ["Email"] }
            },
            "/api/auth/update-user": {
                "post": { "summary": "Update user profile", "tags": ["User"] }
            },
            "/api/auth/list-linked-accounts": {
                "get": { "summary": "List linked OAuth accounts", "tags": ["OAuth"] }
            },
            "/api/auth/link-social": {
                "post": { "summary": "Link OAuth account to user", "tags": ["OAuth"] }
            },
            "/api/auth/unlink-social": {
                "post": { "summary": "Unlink OAuth account", "tags": ["OAuth"] }
            },
            "/api/auth/health": {
                "get": { "summary": "Health check", "tags": ["System"] }
            }
        }
    }))
}
