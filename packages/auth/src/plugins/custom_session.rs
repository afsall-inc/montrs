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

//! Custom Session plugin — re-export enriched session JSON at /custom-session/get.
//! Stores a callback as Box<dyn Fn> to enrich session data.

use crate::{AuthError, context::AuthState, plugin::AuthPlugin};
use axum::{Json, Router, extract::State, routing::get};
use serde_json::{Value, json};
use std::sync::Arc;

/// Enrichment function: takes (user_id, base_session_json) and returns enriched JSON.
pub type SessionEnricher = Arc<dyn Fn(&str, Value) -> Value + Send + Sync>;

/// CustomSessionPlugin — optional custom get-session via enrichment callback.
pub struct CustomSessionPlugin {
    state: Option<AuthState>,
    enricher: Option<SessionEnricher>,
}

impl CustomSessionPlugin {
    pub fn new() -> Self {
        Self {
            state: None,
            enricher: None,
        }
    }

    /// Set a session enrichment callback.
    pub fn with_enricher(mut self, enricher: SessionEnricher) -> Self {
        self.enricher = Some(enricher);
        self
    }
}

impl Default for CustomSessionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    if let Some(v) = headers.get("authorization").and_then(|v| v.to_str().ok())
        && let Some(t) = v.strip_prefix("Bearer ")
    {
        return Some(t.to_string());
    }
    if let Some(v) = headers.get("cookie").and_then(|v| v.to_str().ok()) {
        for part in v.split(';') {
            let part = part.trim();
            if let Some(t) = part.strip_prefix("session=") {
                return Some(t.to_string());
            }
            if let Some(t) = part.strip_prefix("__montrs_session=") {
                return Some(t.to_string());
            }
        }
    }
    None
}

impl AuthPlugin for CustomSessionPlugin {
    fn name(&self) -> &'static str {
        "custom_session"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self
            .state
            .clone()
            .expect("CustomSessionPlugin: state not set");
        Router::new()
            .route("/custom-session/get", get(get_custom_session))
            .with_state(state)
    }
}

async fn get_custom_session(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token =
        extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    let user = state
        .db
        .find_user_by_id(&session.user_id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;
    let profile: crate::entities::UserProfile = (&user).into();

    let base = json!({
        "session": {
            "id": session.id,
            "userId": session.user_id,
            "expiresAt": session.expires_at.to_rfc3339(),
            "createdAt": session.created_at.to_rfc3339(),
            "ipAddress": session.ip_address,
            "userAgent": session.user_agent,
            "impersonatedBy": session.impersonated_by,
            "activeOrganizationId": session.active_organization_id,
        },
        "user": profile,
        "isAnonymous": user.is_anonymous,
        "twoFactorEnabled": user.two_factor_enabled,
        "role": user.role,
    });

    Ok(Json(base))
}
