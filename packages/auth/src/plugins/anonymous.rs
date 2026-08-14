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

//! Anonymous plugin — anonymous sessions and account deletion.
//! POST /sign-in/anonymous, POST /delete-anonymous-user.

use crate::context::AuthState;
use crate::entities::{DefaultUser, UserProfile};
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};

/// Anonymous plugin — create anonymous sessions and delete anonymous users.
pub struct AnonymousPlugin {
    state: Option<AuthState>,
}

impl AnonymousPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for AnonymousPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for AnonymousPlugin {
    fn name(&self) -> &'static str {
        "anonymous"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("AnonymousPlugin: state not set");
        Router::new()
            .route("/sign-in/anonymous", post(sign_in_anonymous))
            .route("/delete-anonymous-user", post(delete_anonymous_user))
            .with_state(state)
    }
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

async fn sign_in_anonymous(
    State(state): State<AuthState>,
) -> Result<Json<Value>, AuthError> {
    let user = DefaultUser::anonymous();
    state.db.create_user(&user).await.map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;

    let session = state
        .session
        .create(&user.id, state.session_expires_secs())
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let profile = UserProfile {
        id: user.id.clone(),
        email: user.email.clone(),
        name: user.name.clone(),
        image: user.image.clone(),
        email_verified: user.email_verified,
        username: user.username.clone(),
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(Json(json!({
        "user": profile,
        "session": crate::session::session_json(&session),
        "token": session.token,
        "isAnonymous": true,
    })))
}

async fn delete_anonymous_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
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

    if !user.is_anonymous {
        return Err(AuthError::forbidden());
    }

    state.db.delete_user(&user.id).await.map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;
    state.session.revoke_all(&user.id).await.ok();

    Ok(Json(json!({ "success": true, "deleted": true })))
}