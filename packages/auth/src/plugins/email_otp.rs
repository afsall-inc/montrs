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

//! Email OTP plugin — send and verify 6-digit OTP codes via email.
//! POST /email-otp/send-verification-otp, /email-otp/verify-email,
//! /sign-in/email-otp, and password reset via OTP.

use crate::{
    AuthError,
    context::AuthState,
    entities::{DefaultUser, UserProfile},
    plugin::AuthPlugin,
};
use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

/// Email OTP plugin — 6-digit OTP codes sent via email.
pub struct EmailOtpPlugin {
    state: Option<AuthState>,
}

impl EmailOtpPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for EmailOtpPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for EmailOtpPlugin {
    fn name(&self) -> &'static str {
        "email_otp"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("EmailOtpPlugin: state not set");
        Router::new()
            .route(
                "/email-otp/send-verification-otp",
                post(send_verification_otp),
            )
            .route("/email-otp/verify-email", post(verify_email_otp))
            .route("/sign-in/email-otp", post(sign_in_email_otp))
            .route("/email-otp/reset-password", post(reset_password_otp))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendOtpRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOtpRequest {
    pub email: String,
    pub otp: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInOtpRequest {
    pub email: String,
    pub otp: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordOtpRequest {
    pub email: String,
    pub otp: String,
    pub new_password: String,
}

async fn send_verification_otp(
    State(state): State<AuthState>,
    Json(req): Json<SendOtpRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.email.is_empty() {
        return Err(AuthError::missing_field("email"));
    }

    let otp = crate::verification::create_otp(
        state.db.as_ref(),
        format!("email-otp:{}", req.email),
        6,
        300, // 5 minutes
    )
    .await
    .map_err(|e| {
        AuthError::new(
            crate::error::AuthErrorCode::InternalError,
            e.to_string(),
        )
    })?;

    let _ = state
        .email
        .send(crate::email::EmailMessage {
            to: req.email.clone(),
            subject: "Your verification code".into(),
            body_text: format!(
                "Your verification code is: {}\n\nIt expires in 5 minutes.",
                otp.value
            ),
            body_html: None,
        })
        .await;

    Ok(Json(json!({ "success": true, "message": "OTP sent" })))
}

async fn verify_email_otp(
    State(state): State<AuthState>,
    Json(req): Json<VerifyOtpRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.email.is_empty() || req.otp.is_empty() {
        return Err(AuthError::missing_field("email or otp"));
    }

    let _rec = crate::verification::consume_verification(
        state.db.as_ref(),
        &format!("email-otp:{}", req.email),
        &req.otp,
    )
    .await
    .map_err(|_| AuthError::invalid_token())?;

    // Mark email as verified.
    if let Some(user) = state.db.find_user_by_email(&req.email).await? {
        state
            .db
            .update_user(
                &user.id,
                crate::database::UserUpdate {
                    email_verified: Some(true),
                    ..Default::default()
                },
            )
            .await?;
    }

    Ok(Json(json!({ "success": true, "emailVerified": true })))
}

async fn sign_in_email_otp(
    State(state): State<AuthState>,
    Json(req): Json<SignInOtpRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.email.is_empty() || req.otp.is_empty() {
        return Err(AuthError::missing_field("email or otp"));
    }

    let _rec = crate::verification::consume_verification(
        state.db.as_ref(),
        &format!("email-otp:{}", req.email),
        &req.otp,
    )
    .await
    .map_err(|_| AuthError::invalid_token())?;

    // Find or create user.
    let user =
        match state.db.find_user_by_email(&req.email).await? {
            Some(u) => u,
            None => {
                let new_user = DefaultUser::new(&req.email, None);
                state.db.create_user(&new_user).await.map_err(|e| {
                    AuthError::new(
                        crate::error::AuthErrorCode::InternalError,
                        e.to_string(),
                    )
                })?;
                state.db.find_user_by_email(&req.email).await?.ok_or_else(
                    || {
                        AuthError::new(
                            crate::error::AuthErrorCode::InternalError,
                            "Failed to create user",
                        )
                    },
                )?
            }
        };

    state
        .db
        .update_user(
            &user.id,
            crate::database::UserUpdate {
                last_login_method: Some("email-otp".into()),
                email_verified: Some(true),
                ..Default::default()
            },
        )
        .await?;

    let session = state
        .session
        .create(&user.id, state.session_expires_secs())
        .await
        .map_err(|e| {
            AuthError::new(
                crate::error::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;

    let profile: UserProfile = (&user).into();
    Ok(Json(json!({
        "user": profile,
        "session": crate::session::session_json(&session),
        "token": session.token,
    })))
}

async fn reset_password_otp(
    State(state): State<AuthState>,
    Json(req): Json<ResetPasswordOtpRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.email.is_empty() || req.otp.is_empty() || req.new_password.is_empty()
    {
        return Err(AuthError::missing_field("email, otp, or newPassword"));
    }

    let _rec = crate::verification::consume_verification(
        state.db.as_ref(),
        &format!("email-otp:{}", req.email),
        &req.otp,
    )
    .await
    .map_err(|_| AuthError::invalid_token())?;

    let user = state
        .db
        .find_user_by_email(&req.email)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    let hash =
        crate::password::hash_password(&req.new_password).map_err(|e| {
            AuthError::new(
                crate::error::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;

    state
        .db
        .update_user(
            &user.id,
            crate::database::UserUpdate {
                password_hash: Some(hash),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(
        json!({ "success": true, "message": "Password reset successful" }),
    ))
}
