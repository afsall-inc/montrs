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

//! Email verification: send verification and verify.

use crate::{context::AuthState, database::UserUpdate};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/send-verification-email", post(send_verification_email))
        .route("/verify-email", get(verify_email))
        .with_state(state)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendVerificationBody {
    email: String,
    callback_url: Option<String>,
}

#[derive(Deserialize)]
struct VerifyQuery {
    token: String,
    #[serde(default)]
    email: Option<String>,
}

async fn send_verification_email(
    State(state): State<AuthState>,
    Json(body): Json<SendVerificationBody>,
) -> Result<Json<Value>, crate::AuthError> {
    let user = state
        .db
        .find_user_by_email(&body.email)
        .await?
        .ok_or_else(crate::AuthError::user_not_found)?;

    if user.email_verified {
        return Ok(Json(json!({ "verified": true })));
    }

    let ver = crate::verification::create_verification(
        state.db.as_ref(),
        format!("email-verify:{}", user.email),
        None,
        3600 * 48,
    )
    .await
    .map_err(|e| {
        crate::AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    let _ = state
        .email
        .send(crate::email::EmailMessage {
            to: user.email.clone(),
            subject: "Verify your email".into(),
            body_text: format!(
                "Click to verify: {}/verify-email?token={}&email={}",
                state.config.base_url, ver.value, user.email
            ),
            body_html: None,
        })
        .await;

    Ok(Json(json!({ "success": true })))
}

async fn verify_email(
    State(state): State<AuthState>,
    Query(query): Query<VerifyQuery>,
) -> Result<Json<Value>, crate::AuthError> {
    let email = query.email.clone().unwrap_or_default();
    let ver = crate::verification::consume_verification(
        state.db.as_ref(),
        &format!("email-verify:{}", email),
        &query.token,
    )
    .await
    .map_err(|_| crate::AuthError::invalid_token())?;

    let email = ver
        .identifier
        .strip_prefix("email-verify:")
        .unwrap_or(&ver.identifier)
        .to_string();

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
                email_verified: Some(true),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(
        json!({ "verified": true, "message": "Email verified successfully" }),
    ))
}
