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

//! One-Time Token plugin — generate and verify single-use tokens.
//! POST /one-time-token/generate (needs session), POST /one-time-token/verify.
//! Uses verification store with identifier `ott:{userId}`.

use crate::{
    AuthError, context::AuthState, plugin::AuthPlugin, utils::generate_token,
};
use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
/// One-Time Token plugin.
pub struct OneTimeTokenPlugin {
    state: Option<AuthState>,
}

impl OneTimeTokenPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for OneTimeTokenPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for OneTimeTokenPlugin {
    fn name(&self) -> &'static str {
        "one_time_token"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self
            .state
            .clone()
            .expect("OneTimeTokenPlugin: state not set");
        Router::new()
            .route("/one-time-token/generate", post(generate_ott))
            .route("/one-time-token/verify", post(verify_ott))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateOttRequest {
    /// Optional custom expiry in seconds (default 300).
    pub expires_in_secs: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOttRequest {
    pub token: String,
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

async fn generate_ott(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<GenerateOttRequest>,
) -> Result<Json<Value>, AuthError> {
    let token =
        extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let ott = generate_token();
    let expires_in = req.expires_in_secs.unwrap_or(300);
    let identifier = format!("ott:{}", session.user_id);

    let rec = crate::verification::create_verification(
        state.db.as_ref(),
        &identifier,
        Some(ott.clone()),
        expires_in,
    )
    .await
    .map_err(|e| {
        AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    Ok(Json(json!({
        "token": rec.value,
        "expiresAt": rec.expires_at.to_rfc3339(),
    })))
}

async fn verify_ott(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<VerifyOttRequest>,
) -> Result<Json<Value>, AuthError> {
    let token =
        extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let identifier = format!("ott:{}", session.user_id);
    let _rec = crate::verification::consume_verification(
        state.db.as_ref(),
        &identifier,
        &req.token,
    )
    .await
    .map_err(|_| AuthError::invalid_token())?;

    Ok(Json(json!({ "valid": true })))
}
