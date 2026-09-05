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

//! CLI commands for authentication (`montrs auth`).

use montrs_auth::{
    AuthError,
    config::AuthConfig,
    database::{DatabaseAdapter, MemoryDatabaseAdapter},
    entities::DefaultUser,
    password::hash_password,
};

/// Validate a session token and return the user.
pub async fn validate_token(token: &str) -> anyhow::Result<serde_json::Value> {
    let state = build_state()?;
    let session = state
        .session
        .validate(token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    let user = state
        .db
        .find_user_by_id(&session.user_id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;
    let profile: montrs_auth::entities::UserProfile = (&user).into();
    Ok(serde_json::json!({
        "session": {
            "id": session.id,
            "userId": session.user_id,
            "expiresAt": session.expires_at.format(&time::format_description::well_known::Rfc3339).unwrap(),
        },
        "user": profile,
    }))
}

/// Sign in with email + password.
pub async fn sign_in(
    email: &str,
    password: &str,
) -> anyhow::Result<serde_json::Value> {
    let state = build_state()?;
    let user = state
        .db
        .find_user_by_email(email)
        .await?
        .ok_or_else(AuthError::invalid_credentials)?;
    let hash = user
        .password_hash
        .as_deref()
        .ok_or_else(AuthError::invalid_credentials)?;
    if !montrs_auth::password::verify_password(password, hash) {
        return Err(AuthError::invalid_credentials().into());
    }
    let session = state
        .session
        .create(&user.id, state.session_expires_secs())
        .await
        .map_err(|e| {
            AuthError::new(
                montrs_auth::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;
    let profile: montrs_auth::entities::UserProfile = (&user).into();
    Ok(serde_json::json!({
        "user": profile,
        "token": session.token,
        "session": montrs_auth::session::session_json(&session),
    }))
}

/// Sign up a new user with email + password.
pub async fn sign_up(
    email: &str,
    password: &str,
    name: Option<&str>,
) -> anyhow::Result<serde_json::Value> {
    let state = build_state()?;
    if state.db.find_user_by_email(email).await?.is_some() {
        return Err(AuthError::email_in_use().into());
    }
    let hash = hash_password(password).map_err(|e| {
        AuthError::new(montrs_auth::AuthErrorCode::InternalError, e.to_string())
    })?;
    let mut user = DefaultUser::new(email, Some(hash));
    user.name = name.map(|s| s.to_string());
    state.db.create_user(&user).await?;
    let session = state
        .session
        .create(&user.id, state.session_expires_secs())
        .await
        .map_err(|e| {
            AuthError::new(
                montrs_auth::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;
    Ok(serde_json::json!({
        "user": {
            "id": user.id,
            "email": user.email,
            "name": user.name,
            "emailVerified": user.email_verified,
        },
        "token": session.token,
    }))
}

/// Shared auth state singleton — the in-memory DB persists across calls
/// within one process (important for tests and the MCP server session).
static STATE: std::sync::OnceLock<montrs_auth::context::AuthState> =
    std::sync::OnceLock::new();

/// Build the auth state — reads config from env or uses defaults.
fn build_state() -> anyhow::Result<montrs_auth::context::AuthState> {
    match STATE.get() {
        Some(state) => Ok(state.clone()),
        None => {
            let state = build_state_inner()?;
            Ok(STATE.get_or_init(|| state).clone())
        }
    }
}

fn build_state_inner() -> anyhow::Result<montrs_auth::context::AuthState> {
    let secret = std::env::var("MONTRS_AUTH_SECRET").unwrap_or_else(|_| {
        "dev-secret-key-change-me-32-chars-minimum!!".into()
    });
    let config = AuthConfig::new(secret).base_url(
        std::env::var("MONTRS_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string()),
    );
    let db: std::sync::Arc<dyn DatabaseAdapter> =
        std::sync::Arc::new(MemoryDatabaseAdapter::new());
    let email: std::sync::Arc<dyn montrs_auth::email::EmailProvider> =
        std::sync::Arc::new(montrs_auth::email::ConsoleEmailProvider::new());
    let session = montrs_auth::session::SessionManager::new(
        config.secret.clone(),
        db.clone(),
    );
    let rate_limit =
        std::sync::Arc::new(montrs_auth::rate_limit::RateLimiter::new(
            config.rate_limit_max,
            config.rate_limit_window_secs,
        ));
    Ok(montrs_auth::context::AuthState {
        config,
        db,
        session,
        email,
        rate_limit,
    })
}

/// List configured auth plugins (for diagnostics).
pub async fn status() -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::json!({
        "provider": "montrs-auth",
        "plugins": [
            "email-password", "sessions", "social-oauth",
            "two-factor", "magic-link", "organization", "admin"
        ],
        "features": {
            "emailVerification": false,
            "2fa": true,
            "rbac": true,
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sign_up_then_validate() -> anyhow::Result<()> {
        let res = sign_up(
            "mcp-test@example.com",
            "correct-horse-battery",
            Some("Mcp Test"),
        )
        .await?;
        let token = res["token"].as_str().unwrap().to_string();
        let validated = validate_token(&token).await?;
        assert_eq!(validated["user"]["email"], "mcp-test@example.com");
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_sign_in() {
        let res = sign_in("nobody@example.com", "wrong").await;
        assert!(res.is_err());
    }
}
