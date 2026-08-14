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

//! SCIM 2.0 plugin — User provisioning endpoints.
//! /scim/v2/Users GET/POST, /scim/v2/Users/:id GET/PATCH/DELETE — map to UserRecord.

use crate::context::AuthState;
use crate::database::UserUpdate;
use crate::entities::DefaultUser;
use crate::password::hash_password;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::{Path, Query, State};
// Query is used for list filters.
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

/// SCIM plugin — System for Cross-domain Identity Management.
pub struct ScimPlugin {
    state: Option<AuthState>,
}

impl ScimPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for ScimPlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    if let Some(v) = headers.get("authorization").and_then(|v| v.to_str().ok()) {
        if let Some(t) = v.strip_prefix("Bearer ") {
            return Some(t.to_string());
        }
    }
    None
}

/// Require a valid bearer token (API key or session). For SCIM, typically a service token.
async fn require_auth(state: &AuthState, headers: &axum::http::HeaderMap) -> Result<(), AuthError> {
    let token = extract_token(headers).ok_or_else(AuthError::invalid_session)?;
    // Accept either a valid session or an API key.
    if state.session.validate(&token).await?.is_some() {
        return Ok(());
    }
    // Try API key.
    if crate::plugins::api_key::verify_api_key(state.db.as_ref(), &token)
        .await
        .ok()
        .flatten()
        .is_some()
    {
        return Ok(());
    }
    Err(AuthError::invalid_session())
}

impl AuthPlugin for ScimPlugin {
    fn name(&self) -> &'static str {
        "scim"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("ScimPlugin: state not set");
        Router::new()
            .route("/scim/v2/Users", get(list_users).post(create_user))
            .route(
                "/scim/v2/Users/:id",
                get(get_user).patch(patch_user).delete(delete_user),
            )
            .with_state(state)
    }
}

fn user_to_scim(user: &crate::database::UserRecord) -> Value {
    json!({
        "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
        "id": user.id,
        "userName": user.username.clone().unwrap_or_else(|| user.email.clone()),
        "name": {
            "formatted": user.name,
        },
        "emails": [{
            "value": user.email,
            "primary": true,
            "type": "work",
        }],
        "active": !user.banned,
        "meta": {
            "resourceType": "User",
            "created": user.created_at.to_rfc3339(),
            "lastModified": user.updated_at.to_rfc3339(),
        }
    })
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub start_index: Option<usize>,
    pub count: Option<usize>,
    pub filter: Option<String>,
}

async fn list_users(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Query(q): Query<ListQuery>,
) -> Result<Json<Value>, AuthError> {
    require_auth(&state, &headers).await?;

    let start = q.start_index.unwrap_or(1).saturating_sub(1);
    let count = q.count.unwrap_or(100);
    let users = state.db.list_users(count, start).await.map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;

    let resources: Vec<Value> = users.iter().map(user_to_scim).collect();
    let total = resources.len();

    Ok(Json(json!({
        "schemas": ["urn:ietf:params:scim:api:messages:2.0:ListResponse"],
        "totalResults": total,
        "startIndex": start + 1,
        "itemsPerPage": count,
        "Resources": resources,
    })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimCreateUser {
    pub user_name: Option<String>,
    pub name: Option<ScimName>,
    pub emails: Option<Vec<ScimEmail>>,
    pub password: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ScimName {
    pub formatted: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScimEmail {
    pub value: String,
    pub primary: Option<bool>,
    #[serde(rename = "type")]
    pub email_type: Option<String>,
}

async fn create_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<ScimCreateUser>,
) -> Result<Json<Value>, AuthError> {
    require_auth(&state, &headers).await?;

    let email = req
        .emails
        .as_ref()
        .and_then(|e| e.first())
        .map(|e| e.value.clone())
        .or_else(|| req.user_name.clone())
        .ok_or_else(|| AuthError::missing_field("emails or userName"))?;

    if state.db.find_user_by_email(&email).await?.is_some() {
        return Err(AuthError::email_in_use());
    }

    let hash = if let Some(pw) = &req.password {
        Some(hash_password(pw).map_err(|e| {
            AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
        })?)
    } else {
        None
    };

    let mut user = DefaultUser::new(&email, hash);
    user.name = req.name.and_then(|n| n.formatted);
    user.username = req.user_name;
    user.email_verified = true;
    if req.active == Some(false) {
        user.banned = true;
    }

    state.db.create_user(&user).await.map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;

    let record = state
        .db
        .find_user_by_id(&user.id)
        .await?
        .ok_or_else(|| AuthError::new(crate::error::AuthErrorCode::InternalError, "User not found after create"))?;

    Ok(Json(user_to_scim(&record)))
}

async fn get_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Value>, AuthError> {
    require_auth(&state, &headers).await?;

    let user = state
        .db
        .find_user_by_id(&id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    Ok(Json(user_to_scim(&user)))
}

#[derive(Debug, Deserialize)]
pub struct ScimPatchRequest {
    pub operations: Option<Vec<ScimPatchOp>>,
    #[serde(rename = "Operations")]
    pub operations_alt: Option<Vec<ScimPatchOp>>,
}

#[derive(Debug, Deserialize)]
pub struct ScimPatchOp {
    pub op: String,
    pub path: Option<String>,
    pub value: Option<Value>,
}

async fn patch_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
    Json(req): Json<ScimPatchRequest>,
) -> Result<Json<Value>, AuthError> {
    require_auth(&state, &headers).await?;

    let _user = state
        .db
        .find_user_by_id(&id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    let ops = req
        .operations
        .or(req.operations_alt)
        .unwrap_or_default();

    let mut update = UserUpdate::default();
    for op in ops {
        match op.op.to_lowercase().as_str() {
            "replace" | "add" => {
                if let Some(path) = &op.path {
                    match path.as_str() {
                        "active" => {
                            let active = op.value.as_ref().and_then(|v| v.as_bool()).unwrap_or(true);
                            update.banned = Some(!active);
                        }
                        "userName" => {
                            if let Some(v) = op.value.as_ref().and_then(|v| v.as_str()) {
                                update.username = Some(v.to_string());
                            }
                        }
                        "name.formatted" | "displayName" => {
                            if let Some(v) = op.value.as_ref().and_then(|v| v.as_str()) {
                                update.name = Some(v.to_string());
                            }
                        }
                        "emails[type eq \"work\"].value" | "emails" => {
                            if let Some(v) = op.value.as_ref() {
                                if let Some(email) = v.as_str() {
                                    update.email = Some(email.to_string());
                                } else if let Some(arr) = v.as_array() {
                                    if let Some(first) = arr.first() {
                                        if let Some(email) = first.get("value").and_then(|e| e.as_str()) {
                                            update.email = Some(email.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                } else if let Some(val) = &op.value {
                    // Whole-object replace.
                    if let Some(active) = val.get("active").and_then(|v| v.as_bool()) {
                        update.banned = Some(!active);
                    }
                    if let Some(name) = val.get("name").and_then(|n| n.get("formatted")).and_then(|v| v.as_str()) {
                        update.name = Some(name.to_string());
                    }
                }
            }
            "remove" => {
                // Limited remove support.
            }
            _ => {}
        }
    }

    state.db.update_user(&id, update).await?;

    let user = state
        .db
        .find_user_by_id(&id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    Ok(Json(user_to_scim(&user)))
}

async fn delete_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Value>, AuthError> {
    require_auth(&state, &headers).await?;

    let _user = state
        .db
        .find_user_by_id(&id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    state.db.delete_user(&id).await.map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;
    state.session.revoke_all(&id).await.ok();

    Ok(Json(json!({})))
}