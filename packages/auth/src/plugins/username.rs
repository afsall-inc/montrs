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

//! Username plugin — sign-in by username, check availability.
//! POST /sign-in/username, GET /is-username-available?username=

use crate::context::AuthState;
use crate::database::UserUpdate;
use crate::entities::UserProfile;
use crate::password::verify_password;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

/// Username plugin — sign-in by username, check availability.
pub struct UsernamePlugin {
    state: Option<AuthState>,
}

impl UsernamePlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for UsernamePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for UsernamePlugin {
    fn name(&self) -> &'static str {
        "username"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("UsernamePlugin: state not set");
        Router::new()
            .route("/sign-in/username", post(sign_in_username))
            .route("/is-username-available", get(is_username_available))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInUsernameRequest {
    pub username: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UsernameQuery {
    pub username: String,
}

async fn sign_in_username(
    State(state): State<AuthState>,
    Json(req): Json<SignInUsernameRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.username.is_empty() {
        return Err(AuthError::missing_field("username"));
    }
    if req.password.is_empty() {
        return Err(AuthError::missing_field("password"));
    }

    let user = state
        .db
        .find_user_by_username(&req.username)
        .await?
        .ok_or_else(AuthError::invalid_credentials)?;

    if user.banned {
        return Err(AuthError::forbidden());
    }

    let hash = user
        .password_hash
        .as_deref()
        .ok_or_else(AuthError::invalid_credentials)?;

    if !verify_password(&req.password, hash) {
        return Err(AuthError::invalid_credentials());
    }

    if state.config.email_verification && !user.email_verified {
        return Err(AuthError::email_not_verified());
    }

    let _ = state
        .db
        .update_user(
            &user.id,
            UserUpdate {
                last_login_method: Some("username".into()),
                ..Default::default()
            },
        )
        .await;

    let session = state
        .session
        .create(&user.id, state.session_expires_secs())
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let profile: UserProfile = (&user).into();
    Ok(Json(json!({
        "user": profile,
        "session": crate::session::session_json(&session),
        "token": session.token,
    })))
}

async fn is_username_available(
    State(state): State<AuthState>,
    Query(query): Query<UsernameQuery>,
) -> Result<Json<Value>, AuthError> {
    if query.username.is_empty() {
        return Err(AuthError::missing_field("username"));
    }
    let existing = state
        .db
        .find_user_by_username(&query.username)
        .await?;
    Ok(Json(json!({
        "available": existing.is_none(),
        "username": query.username,
    })))
}