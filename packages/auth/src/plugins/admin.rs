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

//! Admin plugin — user management for admin role.
//! /admin/list-users, create-user, ban-user, unban-user, set-role,
//! set-user-password, impersonate-user, revoke-user-sessions.
//! Require session user role == "admin".

use crate::{
    AuthError,
    context::AuthState,
    database::UserUpdate,
    entities::{DefaultUser, UserProfile},
    password::hash_password,
    plugin::AuthPlugin,
};
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};

/// Admin plugin — requires role == "admin".
pub struct AdminPlugin {
    state: Option<AuthState>,
}

impl AdminPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for AdminPlugin {
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

async fn require_admin(
    state: &AuthState,
    headers: &axum::http::HeaderMap,
) -> Result<crate::database::UserRecord, AuthError> {
    let token =
        extract_token(headers).ok_or_else(AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    if user.role.as_deref() != Some("admin") {
        return Err(AuthError::forbidden());
    }
    Ok(user)
}

impl AuthPlugin for AdminPlugin {
    fn name(&self) -> &'static str {
        "admin"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("AdminPlugin: state not set");
        Router::new()
            .route("/admin/list-users", get(list_users))
            .route("/admin/create-user", post(create_user))
            .route("/admin/ban-user", post(ban_user))
            .route("/admin/unban-user", post(unban_user))
            .route("/admin/set-role", post(set_role))
            .route("/admin/set-user-password", post(set_user_password))
            .route("/admin/impersonate-user", post(impersonate_user))
            .route("/admin/revoke-user-sessions", post(revoke_user_sessions))
            .with_state(state)
    }
}

async fn list_users(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    axum::extract::Query(q): axum::extract::Query<
        std::collections::HashMap<String, String>,
    >,
) -> Result<Json<Value>, AuthError> {
    require_admin(&state, &headers).await?;

    let limit: usize =
        q.get("limit").and_then(|s| s.parse().ok()).unwrap_or(50);
    let offset: usize =
        q.get("offset").and_then(|s| s.parse().ok()).unwrap_or(0);

    let users = state.db.list_users(limit, offset).await.map_err(|e| {
        AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    let profiles: Vec<UserProfile> =
        users.iter().map(UserProfile::from).collect();
    Ok(Json(
        json!({ "users": profiles, "limit": limit, "offset": offset }),
    ))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub email: String,
    pub password: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
    pub email_verified: Option<bool>,
}

async fn create_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<Value>, AuthError> {
    require_admin(&state, &headers).await?;

    if req.email.is_empty() {
        return Err(AuthError::missing_field("email"));
    }
    if state.db.find_user_by_email(&req.email).await?.is_some() {
        return Err(AuthError::email_in_use());
    }

    let hash = if let Some(pw) = &req.password {
        Some(hash_password(pw).map_err(|e| {
            AuthError::new(
                crate::error::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?)
    } else {
        None
    };

    let mut user = DefaultUser::new(&req.email, hash);
    user.name = req.name;
    user.role = Some(req.role.unwrap_or_else(|| "user".into()));
    user.email_verified = req.email_verified.unwrap_or(false);

    state.db.create_user(&user).await.map_err(|e| {
        AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

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

    Ok(Json(json!({ "user": profile })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BanUserRequest {
    pub user_id: String,
    pub reason: Option<String>,
}

async fn ban_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<BanUserRequest>,
) -> Result<Json<Value>, AuthError> {
    require_admin(&state, &headers).await?;

    state
        .db
        .update_user(
            &req.user_id,
            UserUpdate {
                banned: Some(true),
                ban_reason: req.reason,
                ..Default::default()
            },
        )
        .await?;

    // Revoke all sessions.
    state.session.revoke_all(&req.user_id).await.ok();

    Ok(Json(json!({ "success": true, "banned": req.user_id })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnbanUserRequest {
    pub user_id: String,
}

async fn unban_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<UnbanUserRequest>,
) -> Result<Json<Value>, AuthError> {
    require_admin(&state, &headers).await?;

    state
        .db
        .update_user(
            &req.user_id,
            UserUpdate {
                banned: Some(false),
                ban_reason: Some(String::new()),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(json!({ "success": true, "unbanned": req.user_id })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRoleRequest {
    pub user_id: String,
    pub role: String,
}

async fn set_role(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<SetRoleRequest>,
) -> Result<Json<Value>, AuthError> {
    require_admin(&state, &headers).await?;

    state
        .db
        .update_user(
            &req.user_id,
            UserUpdate {
                role: Some(req.role.clone()),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(
        json!({ "success": true, "userId": req.user_id, "role": req.role }),
    ))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUserPasswordRequest {
    pub user_id: String,
    pub new_password: String,
}

async fn set_user_password(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<SetUserPasswordRequest>,
) -> Result<Json<Value>, AuthError> {
    require_admin(&state, &headers).await?;

    state.config.password.validate(&req.new_password)?;
    let hash = hash_password(&req.new_password).map_err(|e| {
        AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    state
        .db
        .update_user(
            &req.user_id,
            UserUpdate {
                password_hash: Some(hash),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(json!({ "success": true })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImpersonateUserRequest {
    pub user_id: String,
}

async fn impersonate_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<ImpersonateUserRequest>,
) -> Result<Json<Value>, AuthError> {
    let admin = require_admin(&state, &headers).await?;

    let target = state
        .db
        .find_user_by_id(&req.user_id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    // Create a session for the target user, marked as impersonated.
    let session = state
        .session
        .create(&target.id, state.session_expires_secs())
        .await
        .map_err(|e| {
            AuthError::new(
                crate::error::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;

    // Store impersonation metadata.
    state
        .db
        .plugin_set(
            "impersonation",
            &session.id,
            json!({ "impersonatedBy": admin.id, "targetUserId": target.id }),
        )
        .await
        .ok();

    let profile: UserProfile = (&target).into();
    Ok(Json(json!({
        "user": profile,
        "session": crate::session::session_json(&session),
        "token": session.token,
        "impersonatedBy": admin.id,
    })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeUserSessionsRequest {
    pub user_id: String,
}

async fn revoke_user_sessions(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RevokeUserSessionsRequest>,
) -> Result<Json<Value>, AuthError> {
    require_admin(&state, &headers).await?;

    state.session.revoke_all(&req.user_id).await.map_err(|e| {
        AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    Ok(Json(
        json!({ "success": true, "revokedUserId": req.user_id }),
    ))
}
