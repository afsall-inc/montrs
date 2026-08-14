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

//! Authentication error types.

use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// Authentication error codes (stable, part of the agent contract).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthErrorCode {
    /// Invalid credentials.
    InvalidCredentials,
    /// User not found.
    UserNotFound,
    /// Email already in use.
    EmailAlreadyInUse,
    /// Email not verified.
    EmailNotVerified,
    /// Invalid or expired token.
    InvalidToken,
    /// Session expired or invalid.
    InvalidSession,
    /// Rate limit exceeded.
    RateLimited,
    /// Required field missing.
    MissingField,
    /// Password too weak.
    WeakPassword,
    /// 2FA required.
    TwoFactorRequired,
    /// 2FA verification failed.
    InvalidTwoFactor,
    /// OAuth provider error.
    OAuthError,
    /// Provider not configured.
    ProviderNotConfigured,
    /// Organization error.
    OrganizationError,
    /// Permission denied.
    Forbidden,
    /// Database error.
    DatabaseError,
    /// Internal server error.
    InternalError,
    /// Captcha required.
    CaptchaRequired,
    /// Account already linked.
    AccountAlreadyLinked,
    /// Account not found.
    AccountNotFound,
    /// Server misconfiguration.
    ServerError,
}

/// The full auth error.
#[derive(Debug, Clone)]
pub struct AuthError {
    pub code: AuthErrorCode,
    pub message: String,
    pub status: u16,
    pub details: Option<serde_json::Value>,
}

impl AuthError {
    pub fn new(code: AuthErrorCode, message: impl Into<String>) -> Self {
        let status = match code {
            AuthErrorCode::InvalidCredentials
            | AuthErrorCode::InvalidToken
            | AuthErrorCode::InvalidSession
            | AuthErrorCode::InvalidTwoFactor => 401,
            AuthErrorCode::Forbidden => 403,
            AuthErrorCode::RateLimited => 429,
            AuthErrorCode::MissingField
            | AuthErrorCode::WeakPassword
            | AuthErrorCode::EmailAlreadyInUse
            | AuthErrorCode::TwoFactorRequired
            | AuthErrorCode::CaptchaRequired => 400,
            AuthErrorCode::UserNotFound
            | AuthErrorCode::EmailNotVerified
            | AuthErrorCode::AccountNotFound
            | AuthErrorCode::AccountAlreadyLinked => 400,
            AuthErrorCode::OAuthError
            | AuthErrorCode::ProviderNotConfigured
            | AuthErrorCode::OrganizationError
            | AuthErrorCode::DatabaseError
            | AuthErrorCode::InternalError
            | AuthErrorCode::ServerError => 500,
        };
        Self {
            code,
            message: message.into(),
            status,
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Serialize) -> Self {
        self.details = serde_json::to_value(details).ok();
        self
    }

    /// The stable error code (for agents and API consumers).
    pub fn code(&self) -> AuthErrorCode {
        self.code
    }

    /// Suggested fixes for agent error tracking.
    pub fn suggested_fixes(&self) -> Vec<String> {
        match self.code {
            AuthErrorCode::InvalidCredentials => vec![
                "Check the email/password combination.".into(),
                "Ensure the user's email is verified.".into(),
            ],
            AuthErrorCode::EmailNotVerified => vec![
                "Send a verification email to the user.".into(),
            ],
            AuthErrorCode::WeakPassword => vec![
                "Use at least 8 characters with letters and numbers.".into(),
            ],
            AuthErrorCode::ProviderNotConfigured => vec![
                "Add the OAuth provider credentials to the auth config.".into(),
            ],
            AuthErrorCode::RateLimited => vec![
                "Wait before retrying, or increase the rate limit window.".into(),
            ],
            _ => vec![],
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.code, self.message)
    }
}

impl std::error::Error for AuthError {}

impl From<anyhow::Error> for AuthError {
    fn from(e: anyhow::Error) -> Self {
        Self::new(AuthErrorCode::InternalError, e.to_string())
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorBody {
            error: String,
            message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            details: Option<serde_json::Value>,
        }

        // Try to localize via the i18n catalog if registered.
        let message = crate::plugins::i18n::lookup_message(self.code, "en");

        let body = ErrorBody {
            error: serde_json::to_string(&self.code).unwrap_or_default(),
            message,
            details: self.details,
        };
        (axum::http::StatusCode::from_u16(self.status).unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR), Json(body)).into_response()
    }
}

/// Convenience constructors.
impl AuthError {
    pub fn invalid_credentials() -> Self {
        Self::new(AuthErrorCode::InvalidCredentials, "Invalid email or password")
    }
    pub fn user_not_found() -> Self {
        Self::new(AuthErrorCode::UserNotFound, "User not found")
    }
    pub fn email_in_use() -> Self {
        Self::new(AuthErrorCode::EmailAlreadyInUse, "Email already in use")
    }
    pub fn email_not_verified() -> Self {
        Self::new(AuthErrorCode::EmailNotVerified, "Email not verified")
    }
    pub fn invalid_token() -> Self {
        Self::new(AuthErrorCode::InvalidToken, "Invalid or expired token")
    }
    pub fn invalid_session() -> Self {
        Self::new(AuthErrorCode::InvalidSession, "Invalid or expired session")
    }
    pub fn rate_limited() -> Self {
        Self::new(AuthErrorCode::RateLimited, "Too many requests")
    }
    pub fn missing_field(field: &str) -> Self {
        Self::new(AuthErrorCode::MissingField, format!("Missing required field: {field}"))
    }
    pub fn two_factor_required() -> Self {
        Self::new(AuthErrorCode::TwoFactorRequired, "Two-factor authentication required")
    }
    pub fn invalid_two_factor() -> Self {
        Self::new(AuthErrorCode::InvalidTwoFactor, "Invalid two-factor code")
    }
    pub fn forbidden() -> Self {
        Self::new(AuthErrorCode::Forbidden, "Permission denied")
    }
    pub fn provider_not_configured() -> Self {
        Self::new(AuthErrorCode::ProviderNotConfigured, "OAuth provider not configured")
    }
}