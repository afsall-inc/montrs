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

//! One Tap plugin — Google One Tap sign-in via id_token.
//! POST /one-tap/callback with id_token; verifies JWT loosely (decode without
//! full Google cert validation for now, or accept pre-validated claims JSON).

use crate::context::AuthState;
use crate::entities::{DefaultAccount, DefaultUser, UserProfile};
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

/// One Tap plugin — Google One Tap sign-in.
pub struct OneTapPlugin {
    state: Option<AuthState>,
}

impl OneTapPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for OneTapPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for OneTapPlugin {
    fn name(&self) -> &'static str {
        "one_tap"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("OneTapPlugin: state not set");
        Router::new()
            .route("/one-tap/callback", post(one_tap_callback))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneTapCallbackRequest {
    /// Raw Google id_token.
    pub id_token: Option<String>,
    /// Pre-validated claims JSON (accepted as an alternative to raw token).
    pub claims: Option<Value>,
}

/// Decode the payload of a JWT without verifying the signature.
/// Returns the parsed claims JSON.
fn decode_loose(token: &str) -> anyhow::Result<Value> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        anyhow::bail!("invalid JWT structure");
    }
    use base64::Engine as _;
    let payload = parts[1];
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload)?;
    let value: Value = serde_json::from_slice(&decoded)?;
    Ok(value)
}

async fn one_tap_callback(
    State(state): State<AuthState>,
    Json(req): Json<OneTapCallbackRequest>,
) -> Result<Json<Value>, AuthError> {
    let claims = if let Some(claims) = req.claims {
        claims
    } else if let Some(token) = req.id_token {
        decode_loose(&token).map_err(|_| AuthError::invalid_token())?
    } else {
        return Err(AuthError::missing_field("idToken or claims"));
    };

    // Extract identity claims.
    let sub = claims
        .get("sub")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AuthError::new(crate::error::AuthErrorCode::InvalidToken, "Missing sub claim"))?;
    let email = claims.get("email").and_then(|v| v.as_str()).unwrap_or("");
    let email_verified = claims
        .get("email_verified")
        .and_then(|v| v.as_bool())
        .unwrap_or(email.is_empty());
    let name = claims.get("name").and_then(|v| v.as_str()).map(String::from);
    let picture = claims.get("picture").and_then(|v| v.as_str()).map(String::from);

    // Find or create user linked to the "google" provider account.
    let user = if let Some(account) = state.db.find_account("google", sub).await? {
        state
            .db
            .find_user_by_id(&account.user_id)
            .await?
            .ok_or_else(AuthError::user_not_found)?
    } else {
        let email = if email.is_empty() {
            format!("{sub}@google.local")
        } else {
            email.to_string()
        };
        let user_record = match state.db.find_user_by_email(&email).await? {
            Some(u) => u,
            None => {
                let mut nu = DefaultUser::new(&email, None);
                nu.email_verified = email_verified;
                nu.name = name.clone();
                nu.image = picture.clone();
                state.db.create_user(&nu).await.map_err(|e| {
                    AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
                })?;
                state.db.find_user_by_email(&email).await?.ok_or_else(|| {
                    AuthError::new(crate::error::AuthErrorCode::InternalError, "Failed to create user")
                })?
            }
        };
        let account = DefaultAccount::new(&user_record.id, "google", sub);
        state.db.create_account(&account).await.map_err(|e| {
            AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
        })?;
        user_record
    };

    state
        .db
        .update_user(
            &user.id,
            crate::database::UserUpdate {
                last_login_method: Some("one-tap".into()),
                email_verified: Some(true),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_loose() -> anyhow::Result<()> {
        // Build a fake JWT: header.payload.signature
        use base64::Engine as _;
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b"{\"alg\":\"HS256\"}");
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b"{\"sub\":\"abc123\",\"email\":\"a@b.c\"}");
        let token = format!("{header}.{payload}.sig");
        let claims = decode_loose(&token)?;
        assert_eq!(claims["sub"], "abc123");
        Ok(())
    }
}