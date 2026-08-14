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

//! Magic Link plugin — passwordless sign-in via email magic link.
//! POST /sign-in/magic-link, GET /magic-link/verify?token=

use crate::context::AuthState;
use crate::entities::{DefaultUser, UserProfile};
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

/// Magic Link plugin — passwordless sign-in via email.
pub struct MagicLinkPlugin {
    state: Option<AuthState>,
}

impl MagicLinkPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for MagicLinkPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for MagicLinkPlugin {
    fn name(&self) -> &'static str {
        "magic_link"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("MagicLinkPlugin: state not set");
        Router::new()
            .route("/sign-in/magic-link", post(send_magic_link))
            .route("/magic-link/verify", get(verify_magic_link))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMagicLinkRequest {
    pub email: String,
    pub callback_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyMagicLinkQuery {
    pub token: String,
    pub email: Option<String>,
}

async fn send_magic_link(
    State(state): State<AuthState>,
    Json(req): Json<SendMagicLinkRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.email.is_empty() {
        return Err(AuthError::missing_field("email"));
    }

    // Check if user exists; if not, create one.
    let user = match state.db.find_user_by_email(&req.email).await? {
        Some(u) => u,
        None => {
            let new_user = DefaultUser::new(&req.email, None);
            state.db.create_user(&new_user).await.map_err(|e| {
                AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
            })?;
            state.db.find_user_by_email(&req.email).await?.ok_or_else(|| {
                AuthError::new(crate::error::AuthErrorCode::InternalError, "Failed to create user")
            })?
        }
    };

    let token = crate::utils::generate_token();
    let _rec = crate::verification::create_verification(
        state.db.as_ref(),
        format!("magic-link:{}", user.email),
        Some(token.clone()),
        600, // 10 minutes
    )
    .await
    .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let callback = req.callback_url.unwrap_or_else(|| format!("{}/api/auth/magic-link/verify", state.config.base_url));
    let link = format!("{}?token={}&email={}", callback, token, user.email);

    let _ = state
        .email
        .send(crate::email::EmailMessage {
            to: user.email.clone(),
            subject: "Sign in to your account".into(),
            body_text: format!("Click here to sign in: {link}"),
            body_html: None,
        })
        .await;

    Ok(Json(json!({ "success": true, "message": "Magic link sent" })))
}

async fn verify_magic_link(
    State(state): State<AuthState>,
    Query(query): Query<VerifyMagicLinkQuery>,
) -> Result<Json<Value>, AuthError> {
    if query.token.is_empty() {
        return Err(AuthError::missing_field("token"));
    }

    // Find the verification record by value.
    let rec = crate::verification::consume_verification_by_value(
        state.db.as_ref(),
        &query.token,
    )
    .await
    .map_err(|_| AuthError::invalid_token())?;

    let email = query.email.as_ref().map(|s| s.as_str()).unwrap_or("");
    let user_email = if !email.is_empty() {
        email.to_string()
    } else {
        // Extract from identifier.
        rec.identifier.strip_prefix("magic-link:").unwrap_or("").to_string()
    };

    let user = state
        .db
        .find_user_by_email(&user_email)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    state
        .db
        .update_user(
            &user.id,
            crate::database::UserUpdate {
                last_login_method: Some("magic-link".into()),
                ..Default::default()
            },
        )
        .await?;

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