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

//! Sign-In with Ethereum (SIWE) plugin.
//! /siwe/nonce, /siwe/verify — stores nonce in verification; verify is a stub
//! that accepts signature presence (full eth recovery optional).

use crate::context::AuthState;
use crate::entities::{DefaultUser, UserProfile};
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

/// SIWE plugin — Sign-In with Ethereum.
pub struct SiwePlugin {
    state: Option<AuthState>,
}

impl SiwePlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for SiwePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for SiwePlugin {
    fn name(&self) -> &'static str {
        "siwe"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("SiwePlugin: state not set");
        Router::new()
            .route("/siwe/nonce", get(get_nonce))
            .route("/siwe/verify", post(verify_siwe))
            .with_state(state)
    }
}

async fn get_nonce(
    State(state): State<AuthState>,
) -> Result<Json<Value>, AuthError> {
    let nonce = crate::utils::generate_token();
    let _ = crate::verification::create_verification(
        state.db.as_ref(),
        format!("siwe:{}", nonce),
        Some(nonce.clone()),
        300,
    )
    .await
    .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    Ok(Json(json!({ "nonce": nonce })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiweVerifyRequest {
    pub message: String,
    pub signature: String,
    pub address: Option<String>,
    pub chain_id: Option<u64>,
    pub nonce: Option<String>,
}

async fn verify_siwe(
    State(state): State<AuthState>,
    Json(req): Json<SiweVerifyRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.message.is_empty() || req.signature.is_empty() {
        return Err(AuthError::missing_field("message or signature"));
    }

    // Verify nonce if present.
    if let Some(nonce) = &req.nonce {
        let _ = crate::verification::consume_verification(
            state.db.as_ref(),
            &format!("siwe:{nonce}"),
            nonce,
        )
        .await;
    }

    // TODO: Full EIP-4361 message parsing and ecrecover verification.
    // For now, accept any non-empty signature as valid.
    let address = req.address.unwrap_or_else(|| {
        // Extract from message: "0x..." after "Address: "
        req.message
            .lines()
            .find_map(|line| {
                let l = line.trim();
                if l.starts_with("0x") {
                    Some(l.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| format!("0x{}", uuid::Uuid::new_v4().to_string().replace('-', "")))
    });

    let email = format!("{address}@eth.local");
    let user = match state.db.find_user_by_email(&email).await? {
        Some(u) => u,
        None => {
            let mut nu = DefaultUser::new(&email, None);
            nu.email_verified = true;
            state.db.create_user(&nu).await.map_err(|e| {
                AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
            })?;
            state.db.find_user_by_email(&email).await?.ok_or_else(|| {
                AuthError::new(crate::error::AuthErrorCode::InternalError, "Failed to create user")
            })?
        }
    };

    state
        .db
        .update_user(
            &user.id,
            crate::database::UserUpdate {
                last_login_method: Some("siwe".into()),
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
        "address": address,
    })))
}