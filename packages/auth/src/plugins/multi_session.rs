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

//! Multi-Session plugin — list device sessions, set active, revoke.
//! POST /multi-session/list-device-sessions, /multi-session/set-active, /multi-session/revoke.

use crate::{AuthError, context::AuthState, plugin::AuthPlugin};
use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

/// Multi-Session plugin — device session management.
pub struct MultiSessionPlugin {
    state: Option<AuthState>,
    /// Optional device info headers: "user-agent" and "x-forwarded-for".
    device_headers: bool,
}

impl MultiSessionPlugin {
    pub fn new() -> Self {
        Self {
            state: None,
            device_headers: true,
        }
    }

    /// Disable device info reading from headers.
    pub fn no_device_headers(mut self) -> Self {
        self.device_headers = false;
        self
    }
}

impl Default for MultiSessionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for MultiSessionPlugin {
    fn name(&self) -> &'static str {
        "multi_session"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self
            .state
            .clone()
            .expect("MultiSessionPlugin: state not set");
        Router::new()
            .route(
                "/multi-session/list-device-sessions",
                post(list_device_sessions),
            )
            .route("/multi-session/set-active", post(set_active))
            .route("/multi-session/revoke", post(revoke_session))
            .with_state(state)
    }
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    if let Some(v) = headers.get("authorization").and_then(|v| v.to_str().ok())
    {
        if let Some(t) = v.strip_prefix("Bearer ") {
            return Some(t.to_string());
        }
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

async fn list_device_sessions(
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

    let sessions = state.session.list(&session.user_id).await?;
    let mut device_sessions = Vec::new();
    for s in sessions {
        // Device info stored in the plugin store keyed by session id.
        let device: Option<Value> = state
            .db
            .plugin_get("device_session", &s.id)
            .await
            .ok()
            .flatten();
        let device_name = device
            .and_then(|d| {
                d.get("name").and_then(|v| v.as_str()).map(String::from)
            })
            .unwrap_or_else(|| "unknown device".into());
        device_sessions.push(json!({
            "id": s.id,
            "userId": s.user_id,
            "expiresAt": s.expires_at.to_rfc3339(),
            "createdAt": s.created_at.to_rfc3339(),
            "deviceName": device_name,
            "current": s.id == session.id,
        }));
    }

    let mut info = HashMap::new();
    info.insert("sessions".to_string(), serde_json::json!(device_sessions));
    info.insert(
        "currentSessionId".to_string(),
        serde_json::json!(session.id),
    );
    Ok(Json(serde_json::to_value(info).unwrap_or_default()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetActiveRequest {
    pub session_id: String,
    pub device_name: Option<String>,
}

async fn set_active(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<SetActiveRequest>,
) -> Result<Json<Value>, AuthError> {
    let token =
        extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    // Verify the target session belongs to the same user.
    let target = state
        .db
        .find_session(&req.session_id)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    if target.user_id != session.user_id {
        return Err(AuthError::forbidden());
    }

    // Record device info.
    let device_json = json!({
        "name": req.device_name.unwrap_or_else(|| "unknown device".into()),
        "active": true,
    });
    state
        .db
        .plugin_set("device_session", &req.session_id, device_json)
        .await
        .map_err(|e| {
            AuthError::new(
                crate::error::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;

    Ok(Json(
        json!({ "success": true, "activeSessionId": req.session_id }),
    ))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeRequest {
    pub session_id: String,
}

async fn revoke_session(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RevokeRequest>,
) -> Result<Json<Value>, AuthError> {
    let token =
        extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let target = state
        .db
        .find_session(&req.session_id)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    if target.user_id != session.user_id {
        return Err(AuthError::forbidden());
    }

    state.session.revoke(&req.session_id).await?;
    let _ = state
        .db
        .plugin_delete("device_session", &req.session_id)
        .await;

    Ok(Json(json!({ "success": true, "revoked": req.session_id })))
}
