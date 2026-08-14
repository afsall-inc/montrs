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

//! Email/password authentication: sign-up, sign-in, change/set/verify password.

use crate::context::AuthState;
use crate::database::UserUpdate;
use crate::entities::{DefaultAccount, DefaultUser, UserProfile};
use crate::password::{hash_password, verify_password};
use crate::AuthError;
use axum::extract::State;
use axum::Json;
use axum::routing::post;
use axum::Router;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/sign-up/email", post(sign_up))
        .route("/sign-in/email", post(sign_in))
        .route("/change-password", post(change_password))
        .route("/set-password", post(set_password))
        .route("/verify-password", post(verify_password_endpoint))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub image: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub revoke_other_sessions: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPasswordRequest {
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyPasswordRequest {
    pub password: String,
}

async fn sign_up(
    State(state): State<AuthState>,
    Json(req): Json<SignUpRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.email.is_empty() {
        return Err(AuthError::missing_field("email"));
    }
    if req.password.is_empty() {
        return Err(AuthError::missing_field("password"));
    }
    state.config.password.validate(&req.password)?;

    if !state.rate_limit.check(&format!("signup:{}", req.email)) {
        return Err(AuthError::rate_limited());
    }

    if state.db.find_user_by_email(&req.email).await?.is_some() {
        return Err(AuthError::email_in_use());
    }

    let hash = hash_password(&req.password).map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;

    let mut user = DefaultUser::new(&req.email, Some(hash.clone()));
    user.name = req.name;
    user.image = req.image;
    user.last_login_method = Some("email".into());

    state.db.create_user(&user).await?;
    let account = DefaultAccount::credential(&user.id, hash);
    state.db.create_account(&account).await?;

    let session = state
        .session
        .create(&user.id, state.session_expires_secs())
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    if state.config.email_verification {
        let ver = crate::verification::create_verification(
            state.db.as_ref(),
            format!("email-verify:{}", user.email),
            None,
            3600 * 24,
        )
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;
        let link = format!(
            "{}/api/auth/verify-email?token={}&email={}",
            state.config.base_url, ver.value, user.email
        );
        let _ = state
            .email
            .send(crate::email::EmailMessage {
                to: user.email.clone(),
                subject: "Verify your email".into(),
                body_text: format!("Click to verify: {link}"),
                body_html: None,
            })
            .await;
    }

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
    })))
}

async fn sign_in(
    State(state): State<AuthState>,
    Json(req): Json<SignInRequest>,
) -> Result<Json<Value>, AuthError> {
    if !state.rate_limit.check(&format!("signin:{}", req.email)) {
        return Err(AuthError::rate_limited());
    }

    let user = state
        .db
        .find_user_by_email(&req.email)
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

    if user.two_factor_enabled {
        // Return a pending 2FA token instead of a full session.
        let pending = crate::utils::generate_token();
        let _ = crate::verification::create_verification(
            state.db.as_ref(),
            format!("2fa:{}", user.id),
            Some(pending.clone()),
            600,
        )
        .await;
        return Ok(Json(json!({
            "twoFactorRedirect": true,
            "token": pending,
            "userId": user.id,
        })));
    }

    let _ = state
        .db
        .update_user(
            &user.id,
            UserUpdate {
                last_login_method: Some("email".into()),
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

async fn change_password(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_bearer(&headers).ok_or_else(AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let hash = user
        .password_hash
        .as_deref()
        .ok_or_else(AuthError::invalid_credentials)?;
    if !verify_password(&req.current_password, hash) {
        return Err(AuthError::invalid_credentials());
    }
    state.config.password.validate(&req.new_password)?;
    let new_hash = hash_password(&req.new_password).map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;
    state
        .db
        .update_user(
            &user.id,
            UserUpdate {
                password_hash: Some(new_hash),
                ..Default::default()
            },
        )
        .await?;

    if req.revoke_other_sessions.unwrap_or(false) {
        let _ = state.session.revoke_all(&user.id).await;
        let session = state
            .session
            .create(&user.id, state.session_expires_secs())
            .await
            .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;
        return Ok(Json(json!({
            "success": true,
            "token": session.token,
        })));
    }

    Ok(Json(json!({ "success": true })))
}

async fn set_password(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<SetPasswordRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_bearer(&headers).ok_or_else(AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    state.config.password.validate(&req.new_password)?;
    let new_hash = hash_password(&req.new_password).map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;
    state
        .db
        .update_user(
            &user.id,
            UserUpdate {
                password_hash: Some(new_hash.clone()),
                ..Default::default()
            },
        )
        .await?;
    // Ensure credential account exists.
    if state
        .db
        .find_account("credential", &user.id)
        .await?
        .is_none()
    {
        let acc = DefaultAccount::credential(&user.id, new_hash);
        state.db.create_account(&acc).await?;
    }
    Ok(Json(json!({ "success": true })))
}

async fn verify_password_endpoint(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<VerifyPasswordRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_bearer(&headers).ok_or_else(AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    let hash = user
        .password_hash
        .as_deref()
        .ok_or_else(AuthError::invalid_credentials)?;
    let ok = verify_password(&req.password, hash);
    if !ok {
        return Err(AuthError::invalid_credentials());
    }
    Ok(Json(json!({ "valid": true })))
}

fn extract_bearer(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .or_else(|| {
            headers.get("cookie").and_then(|v| v.to_str().ok()).and_then(|c| {
                c.split(';').find_map(|p| {
                    let p = p.trim();
                    p.strip_prefix("session=")
                        .or_else(|| p.strip_prefix("__montrs_session="))
                        .map(|s| s.to_string())
                })
            })
        })
}