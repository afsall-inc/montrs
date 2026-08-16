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

//! JWT plugin — token issuance and JWKS endpoint.
//! GET /token, GET /jwks — uses utils::jwt; HS256 JWKS stub.

use crate::{AuthError, context::AuthState, plugin::AuthPlugin};
use axum::{Json, Router, extract::State, routing::get};
use serde_json::{Value, json};

/// JWT plugin — issue tokens and expose a JWKS document.
pub struct JwtPlugin {
    state: Option<AuthState>,
}

impl JwtPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for JwtPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for JwtPlugin {
    fn name(&self) -> &'static str {
        "jwt"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("JwtPlugin: state not set");
        Router::new()
            .route("/token", get(get_token))
            .route("/jwks", get(get_jwks))
            .with_state(state)
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

async fn get_token(
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

    // Issue a JWT signed with the configured secret.
    let jwt = crate::utils::jwt::create_token(
        &session.user_id,
        state.session.secret(),
        3600,
    )
    .map_err(|e| {
        AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    Ok(Json(json!({
        "token": jwt,
        "tokenType": "Bearer",
        "expiresIn": 3600,
        "sessionId": session.id,
    })))
}

async fn get_jwks() -> Json<Value> {
    // HS256 symmetric keys have no public key material; return an empty
    // keys list with a note. In production, swap in an RSA keypair and
    // expose the public key here.
    Json(json!({
        "keys": [],
        "note": "This server signs JWTs with HS256. No public JWK is available for symmetric keys. Configure an RSA keypair for asymmetric JWKS support.",
    }))
}
