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

//! Password reset: request and reset.

use crate::{
    context::AuthState, database::UserUpdate, password::hash_password,
};
use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/forget-password", post(forget_password))
        .route("/reset-password", post(reset_password))
        .with_state(state)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ForgetPasswordRequest {
    email: String,
    redirect_to: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResetPasswordRequest {
    token: String,
    new_password: String,
}

async fn forget_password(
    State(state): State<AuthState>,
    Json(req): Json<ForgetPasswordRequest>,
) -> Result<Json<Value>, crate::AuthError> {
    let user = state.db.find_user_by_email(&req.email).await?;
    if user.is_none() {
        // Don't reveal whether the email exists.
        return Ok(Json(
            json!({ "success": true, "message": "If the email exists, a reset link was sent." }),
        ));
    }

    let ver = crate::verification::create_verification(
        state.db.as_ref(),
        format!("reset:{}", req.email),
        None,
        3600,
    )
    .await
    .map_err(|e| {
        crate::AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    let link = format!(
        "{}/reset-password?token={}",
        state.config.base_url, ver.value
    );
    let _ = state
        .email
        .send(crate::email::EmailMessage {
            to: req.email,
            subject: "Password Reset".into(),
            body_text: format!("Click to reset your password: {link}"),
            body_html: None,
        })
        .await;

    Ok(Json(
        json!({ "success": true, "message": "If the email exists, a reset link was sent." }),
    ))
}

async fn reset_password(
    State(state): State<AuthState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<Json<Value>, crate::AuthError> {
    let ver = crate::verification::consume_verification_by_value(
        state.db.as_ref(),
        &req.token,
    )
    .await
    .map_err(|_| crate::AuthError::invalid_token())?;

    if !ver.identifier.starts_with("reset:") {
        return Err(crate::AuthError::invalid_token());
    }
    let email = ver
        .identifier
        .strip_prefix("reset:")
        .unwrap_or(&ver.identifier)
        .to_string();

    state.config.password.validate(&req.new_password)?;
    let hash = hash_password(&req.new_password).map_err(|e| {
        crate::AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    let user = state
        .db
        .find_user_by_email(&email)
        .await?
        .ok_or_else(crate::AuthError::user_not_found)?;

    state
        .db
        .update_user(
            &user.id,
            UserUpdate {
                password_hash: Some(hash),
                ..Default::default()
            },
        )
        .await?;

    // Revoke all existing sessions.
    let _ = state.session.revoke_all(&user.id).await;

    Ok(Json(
        json!({ "success": true, "message": "Password reset successfully" }),
    ))
}
