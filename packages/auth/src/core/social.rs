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

//! Social OAuth sign-in, callback, account link/unlink.

use crate::context::AuthState;
use crate::database::UserUpdate;
use crate::entities::{DefaultAccount, DefaultUser, UserProfile};
use crate::providers::{self, SocialProvider};
use axum::extract::{Path, Query, State};
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/sign-in/social", post(sign_in_social))
        .route("/callback/:provider", get(oauth_callback))
        .route("/link-social", post(link_social))
        .route("/unlink-account", post(unlink_account))
        .route("/list-accounts", get(list_accounts))
        .route("/get-access-token", post(get_access_token))
        .with_state(state)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialSignInBody {
    provider: String,
    callback_url: Option<String>,
    #[serde(default)]
    disable_redirect: bool,
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkBody {
    provider: String,
    callback_url: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnlinkBody {
    provider_id: String,
    account_id: Option<String>,
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

async fn sign_in_social(
    State(state): State<AuthState>,
    Json(body): Json<SocialSignInBody>,
) -> Result<Json<Value>, crate::AuthError> {
    let provider = providers::get_provider(&body.provider)
        .ok_or_else(crate::AuthError::provider_not_configured)?;
    let config = state
        .config
        .oauth_providers
        .get(&body.provider)
        .cloned()
        .ok_or_else(crate::AuthError::provider_not_configured)?;

    let oauth_state = crate::utils::generate_token();
    let _ = crate::verification::create_verification(
        state.db.as_ref(),
        format!("oauth-state:{}", body.provider),
        Some(oauth_state.clone()),
        600,
    )
    .await;

    let redirect_uri = config.redirect_uri.clone().unwrap_or_else(|| {
        format!(
            "{}/api/auth/callback/{}",
            state.config.base_url, body.provider
        )
    });

    let url = provider.authorization_url(&config, &oauth_state, &redirect_uri);

    if body.disable_redirect {
        return Ok(Json(json!({ "url": url, "redirect": false })));
    }
    Ok(Json(json!({ "url": url, "redirect": true })))
}

async fn oauth_callback(
    State(state): State<AuthState>,
    Path(provider_id): Path<String>,
    Query(query): Query<CallbackQuery>,
) -> Result<Json<Value>, crate::AuthError> {
    if let Some(err) = query.error {
        return Err(crate::AuthError::new(
            crate::error::AuthErrorCode::OAuthError,
            err,
        ));
    }
    let code = query.code.ok_or_else(|| {
        crate::AuthError::missing_field("code")
    })?;
    let oauth_state = query.state.ok_or_else(|| {
        crate::AuthError::missing_field("state")
    })?;

    // Validate state.
    let _ = crate::verification::consume_verification(
        state.db.as_ref(),
        &format!("oauth-state:{}", provider_id),
        &oauth_state,
    )
    .await
    .map_err(|_| crate::AuthError::invalid_token())?;

    let provider = providers::get_provider(&provider_id)
        .ok_or_else(crate::AuthError::provider_not_configured)?;
    let config = state
        .config
        .oauth_providers
        .get(&provider_id)
        .cloned()
        .ok_or_else(crate::AuthError::provider_not_configured)?;

    let redirect_uri = config.redirect_uri.clone().unwrap_or_else(|| {
        format!(
            "{}/api/auth/callback/{}",
            state.config.base_url, provider_id
        )
    });

    let profile = provider
        .exchange_code(&config, &code, &redirect_uri)
        .await
        .map_err(|e| {
            crate::AuthError::new(crate::error::AuthErrorCode::OAuthError, e.to_string())
        })?;

    // Find or create user.
    let existing = state
        .db
        .find_account(&provider_id, &profile.provider_account_id)
        .await?;

    let user_id = if let Some(acc) = existing {
        acc.user_id
    } else {
        let email = profile
            .email
            .clone()
            .unwrap_or_else(|| format!("{}@oauth.local", profile.provider_account_id));
        let mut user = if let Some(u) = state.db.find_user_by_email(&email).await? {
            // Link to existing email user.
            DefaultUser {
                id: u.id,
                email: u.email,
                name: u.name.or(profile.name.clone()),
                image: u.image.or(profile.image.clone()),
                email_verified: u.email_verified || profile.email_verified,
                password_hash: u.password_hash,
                username: u.username,
                phone_number: u.phone_number,
                phone_verified: u.phone_verified,
                role: u.role,
                banned: u.banned,
                ban_reason: u.ban_reason,
                two_factor_enabled: u.two_factor_enabled,
                is_anonymous: u.is_anonymous,
                last_login_method: Some(provider_id.clone()),
                metadata: u.metadata,
                created_at: u.created_at,
                updated_at: chrono::Utc::now(),
            }
        } else {
            let mut u = DefaultUser::new(&email, None);
            u.name = profile.name.clone();
            u.image = profile.image.clone();
            u.email_verified = profile.email_verified;
            u.last_login_method = Some(provider_id.clone());
            state.db.create_user(&u).await?;
            u
        };

        // If we matched existing, just update last login.
        if state.db.find_user_by_id(&user.id).await?.is_some() {
            let _ = state
                .db
                .update_user(
                    &user.id,
                    UserUpdate {
                        last_login_method: Some(provider_id.clone()),
                        email_verified: Some(user.email_verified),
                        ..Default::default()
                    },
                )
                .await;
        }

        let mut acc = DefaultAccount::new(&user.id, &provider_id, &profile.provider_account_id);
        acc.access_token = profile.access_token.clone();
        acc.refresh_token = profile.refresh_token.clone();
        acc.id_token = profile.id_token.clone();
        state.db.create_account(&acc).await?;
        user.id
    };

    let session = state
        .session
        .create(&user_id, state.session_expires_secs())
        .await
        .map_err(|e| {
            crate::AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
        })?;

    let user = state
        .db
        .find_user_by_id(&user_id)
        .await?
        .ok_or_else(crate::AuthError::user_not_found)?;
    let profile_out: UserProfile = (&user).into();

    Ok(Json(json!({
        "user": profile_out,
        "session": crate::session::session_json(&session),
        "token": session.token,
    })))
}

async fn link_social(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<LinkBody>,
) -> Result<Json<Value>, crate::AuthError> {
    let _token = extract_token(&headers).ok_or_else(crate::AuthError::invalid_session)?;
    // Start OAuth flow similar to sign-in; linking completed in callback with session present.
    let provider = providers::get_provider(&body.provider)
        .ok_or_else(crate::AuthError::provider_not_configured)?;
    let config = state
        .config
        .oauth_providers
        .get(&body.provider)
        .cloned()
        .ok_or_else(crate::AuthError::provider_not_configured)?;
    let oauth_state = crate::utils::generate_token();
    let _ = crate::verification::create_verification(
        state.db.as_ref(),
        format!("oauth-link:{}", body.provider),
        Some(oauth_state.clone()),
        600,
    )
    .await;
    let redirect_uri = config.redirect_uri.clone().unwrap_or_else(|| {
        format!(
            "{}/api/auth/callback/{}",
            state.config.base_url, body.provider
        )
    });
    let url = provider.authorization_url(&config, &oauth_state, &redirect_uri);
    Ok(Json(json!({ "url": url })))
}

async fn unlink_account(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<UnlinkBody>,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers).ok_or_else(crate::AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let accounts = state.db.list_accounts(&user.id).await?;
    for acc in accounts {
        if acc.provider_id == body.provider_id {
            if body.account_id.as_ref().is_none_or(|id| id == &acc.id) {
                state.db.delete_account(&acc.id).await?;
            }
        }
    }
    Ok(Json(json!({ "success": true })))
}

async fn list_accounts(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers).ok_or_else(crate::AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let accounts = state.db.list_accounts(&user.id).await?;
    Ok(Json(json!({
        "accounts": accounts.iter().map(|a| json!({
            "id": a.id,
            "providerId": a.provider_id,
            "accountId": a.provider_account_id,
        })).collect::<Vec<_>>(),
    })))
}

async fn get_access_token(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<UnlinkBody>,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers).ok_or_else(crate::AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let accounts = state.db.list_accounts(&user.id).await?;
    let acc = accounts
        .into_iter()
        .find(|a| a.provider_id == body.provider_id)
        .ok_or_else(|| {
            crate::AuthError::new(
                crate::error::AuthErrorCode::AccountNotFound,
                "Account not found",
            )
        })?;
    Ok(Json(json!({
        "accessToken": acc.access_token,
        "refreshToken": acc.refresh_token,
        "idToken": acc.id_token,
    })))
}
