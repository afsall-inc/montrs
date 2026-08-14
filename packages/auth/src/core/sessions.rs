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

//! Sessions core routes: get-session, list, revoke, sign-out.

use crate::context::AuthState;
use axum::extract::State;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/get-session", get(get_session))
        .route("/list-sessions", post(list_sessions))
        .route("/revoke-session", post(revoke_session))
        .route("/revoke-other-sessions", post(revoke_other_sessions))
        .route("/sign-out", post(sign_out))
        .with_state(state)
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    if let Some(v) = headers.get("authorization").and_then(|v| v.to_str().ok()) {
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

async fn get_session(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers)
        .ok_or_else(crate::AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let user = state
        .db
        .find_user_by_id(&session.user_id)
        .await?
        .ok_or_else(crate::AuthError::user_not_found)?;
    let profile: crate::entities::UserProfile = (&user).into();
    Ok(Json(json!({
        "session": {
            "id": session.id,
            "userId": session.user_id,
            "expiresAt": session.expires_at.to_rfc3339(),
            "createdAt": session.created_at.to_rfc3339(),
        },
        "user": profile,
    })))
}

async fn list_sessions(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers)
        .ok_or_else(crate::AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let sessions = state.session.list(&session.user_id).await?;
    Ok(Json(json!({
        "sessions": sessions.iter().map(|s| json!({
            "id": s.id,
            "userId": s.user_id,
            "expiresAt": s.expires_at.to_rfc3339(),
            "createdAt": s.created_at.to_rfc3339(),
        })).collect::<Vec<_>>(),
    })))
}

#[derive(Deserialize)]
struct RevokeBody {
    token: String,
}

async fn revoke_session(
    State(state): State<AuthState>,
    Json(body): Json<RevokeBody>,
) -> Result<Json<Value>, crate::AuthError> {
    state.session.revoke(&body.token).await?;
    Ok(Json(json!({ "success": true })))
}

async fn revoke_other_sessions(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers)
        .ok_or_else(crate::AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let sessions = state.session.list(&session.user_id).await?;
    for s in sessions {
        if s.id != session.id {
            let _ = state.session.revoke(&s.id).await;
        }
    }
    Ok(Json(json!({ "success": true })))
}

async fn sign_out(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers)
        .ok_or_else(crate::AuthError::invalid_session)?;
    state.session.revoke(&token).await?;
    Ok(Json(json!({ "success": true })))
}