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

//! Generic OAuth plugin — register extra providers at runtime via config.
//! POST /sign-in/oauth2 (start), GET /oauth2/callback/:id (callback).

use crate::{
    AuthError,
    context::AuthState,
    entities::{DefaultAccount, DefaultUser, UserProfile},
    plugin::AuthPlugin,
    providers::{OAuthProfile, SocialProvider},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};

/// A runtime-registered OAuth provider.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuntimeProviderConfig {
    pub id: String,
    pub display_name: String,
    pub auth_url: String,
    pub token_url: String,
    pub userinfo_url: Option<String>,
    pub scopes: Vec<String>,
    pub client_id: String,
    pub client_secret: String,
    pub id_key: Option<String>,
    pub email_key: Option<String>,
    pub name_key: Option<String>,
    pub image_key: Option<String>,
}

impl RuntimeProviderConfig {
    /// Build the full authorization URL for this provider.
    fn authorization_url(
        &self,
        state_param: &str,
        redirect_uri: &str,
    ) -> String {
        let scopes = if self.scopes.is_empty() {
            "openid email profile"
        } else {
            return format!(
                "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&\
                 state={}",
                self.auth_url,
                url_encode(&self.client_id),
                url_encode(redirect_uri),
                url_encode(&self.scopes.join(" ")),
                url_encode(state_param),
            );
        };
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&\
             state={}",
            self.auth_url,
            url_encode(&self.client_id),
            url_encode(redirect_uri),
            url_encode(scopes),
            url_encode(state_param),
        )
    }

    /// Exchange the authorization code for tokens and fetch the userinfo.
    async fn exchange_code(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> anyhow::Result<OAuthProfile> {
        let client = reqwest::Client::new();
        let token_resp: Value = client
            .post(&self.token_url)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", redirect_uri),
            ])
            .send()
            .await?
            .json()
            .await?;

        let access_token = token_resp
            .get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let refresh_token = token_resp
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let id_token = token_resp
            .get("id_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut raw = token_resp;
        if let (Some(url), Some(at)) =
            (self.userinfo_url.as_ref(), access_token.as_ref())
        {
            let info: Value =
                client.get(url).bearer_auth(at).send().await?.json().await?;
            raw = info;
        }

        let id_key = self.id_key.as_deref().unwrap_or("id");
        let email_key = self.email_key.as_deref().unwrap_or("email");
        let name_key = self.name_key.as_deref().unwrap_or("name");
        let image_key = self.image_key.as_deref().unwrap_or("picture");

        let id = raw
            .get(id_key)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .or_else(|| {
                raw.get(id_key)
                    .and_then(|v| v.as_i64().map(|n| n.to_string()))
            })
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let email = raw
            .get(email_key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let name = raw
            .get(name_key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let image = raw
            .get(image_key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let email_verified = email.is_some();

        Ok(OAuthProfile {
            provider_account_id: id,
            email,
            email_verified,
            name,
            image,
            access_token,
            refresh_token,
            id_token,
            raw,
        })
    }
}

fn url_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

/// Generic OAuth plugin — register extra providers at runtime via config.
pub struct GenericOAuthPlugin {
    state: Option<AuthState>,
}

impl GenericOAuthPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for GenericOAuthPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for GenericOAuthPlugin {
    fn name(&self) -> &'static str {
        "generic_oauth"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self
            .state
            .clone()
            .expect("GenericOAuthPlugin: state not set");
        Router::new()
            .route("/sign-in/oauth2", post(sign_in_oauth2))
            .route("/oauth2/register-provider", post(register_provider))
            .route("/oauth2/callback/:id", get(oauth2_callback))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2StartRequest {
    pub provider_id: String,
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2CallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

async fn register_provider(
    State(state): State<AuthState>,
    Json(cfg): Json<RuntimeProviderConfig>,
) -> Result<Json<Value>, AuthError> {
    if cfg.id.is_empty() || cfg.client_id.is_empty() {
        return Err(AuthError::missing_field("id or clientId"));
    }
    state
        .db
        .plugin_set(
            "oauth_runtime",
            &cfg.id,
            serde_json::to_value(&cfg).unwrap(),
        )
        .await
        .map_err(|e| {
            AuthError::new(
                crate::error::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;
    Ok(Json(json!({ "success": true, "providerId": cfg.id })))
}

async fn get_runtime_provider(
    state: &AuthState,
    provider_id: &str,
) -> Result<Option<RuntimeProviderConfig>, AuthError> {
    let entry = state
        .db
        .plugin_get("oauth_runtime", provider_id)
        .await
        .map_err(|e| {
            AuthError::new(
                crate::error::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;
    Ok(entry.and_then(|v| serde_json::from_value(v).ok()))
}

async fn sign_in_oauth2(
    State(state): State<AuthState>,
    Json(req): Json<OAuth2StartRequest>,
) -> Result<Json<Value>, AuthError> {
    let state_param = crate::utils::generate_token();

    let (auth_url, redirect_uri) = if let Some(cfg) =
        get_runtime_provider(&state, &req.provider_id).await?
    {
        let redirect = req.redirect_uri.unwrap_or_else(|| {
            format!(
                "{}/api/auth/oauth2/callback/{}",
                state.config.base_url, cfg.id
            )
        });
        let url = cfg.authorization_url(&state_param, &redirect);
        (url, redirect)
    } else {
        // Built-in provider path.
        let provider = crate::providers::get_provider(&req.provider_id)
            .ok_or_else(AuthError::provider_not_configured)?;
        let oauth_cfg = state
            .config
            .oauth_providers
            .get(&req.provider_id)
            .cloned()
            .ok_or_else(AuthError::provider_not_configured)?;
        let redirect = format!(
            "{}/api/auth/oauth2/callback/{}",
            state.config.base_url, req.provider_id
        );
        let url =
            provider.authorization_url(&oauth_cfg, &state_param, &redirect);
        (url, redirect)
    };

    // Store the state for CSRF verification.
    let _ = crate::verification::create_verification(
        state.db.as_ref(),
        format!("oauth-state:{}", req.provider_id),
        Some(state_param.clone()),
        600,
    )
    .await;

    Ok(Json(json!({
        "url": auth_url,
        "state": state_param,
        "redirectUri": redirect_uri,
    })))
}

async fn oauth2_callback(
    State(state): State<AuthState>,
    Path(provider_id): Path<String>,
    Query(query): Query<OAuth2CallbackQuery>,
) -> Result<Json<Value>, AuthError> {
    if let Some(err) = &query.error {
        return Err(AuthError::new(
            crate::error::AuthErrorCode::OAuthError,
            format!("OAuth error: {err}"),
        ));
    }
    let code = query
        .code
        .clone()
        .ok_or_else(|| AuthError::missing_field("code"))?;

    // Verify state if present.
    if let Some(st) = &query.state {
        let _ = crate::verification::consume_verification(
            state.db.as_ref(),
            &format!("oauth-state:{}", provider_id),
            st,
        )
        .await;
    }

    let redirect_uri = format!(
        "{}/api/auth/oauth2/callback/{}",
        state.config.base_url, provider_id
    );

    // Runtime provider?
    let profile =
        if let Some(cfg) = get_runtime_provider(&state, &provider_id).await? {
            cfg.exchange_code(&code, &redirect_uri).await.map_err(|e| {
                AuthError::new(
                    crate::error::AuthErrorCode::OAuthError,
                    e.to_string(),
                )
            })?
        } else {
            let provider = crate::providers::get_provider(&provider_id)
                .ok_or_else(AuthError::provider_not_configured)?;
            let oauth_cfg = state
                .config
                .oauth_providers
                .get(&provider_id)
                .cloned()
                .ok_or_else(AuthError::provider_not_configured)?;
            provider
                .exchange_code(&oauth_cfg, &code, &redirect_uri)
                .await
                .map_err(|e| {
                    AuthError::new(
                        crate::error::AuthErrorCode::OAuthError,
                        e.to_string(),
                    )
                })?
        };

    // Find existing account or create user.
    let user = if let Some(account) = state
        .db
        .find_account(&provider_id, &profile.provider_account_id)
        .await?
    {
        state
            .db
            .find_user_by_id(&account.user_id)
            .await?
            .ok_or_else(AuthError::user_not_found)?
    } else {
        let email = profile.email.clone().unwrap_or_else(|| {
            format!(
                "{}-{}@oauth.local",
                provider_id, profile.provider_account_id
            )
        });
        let user_record = match state.db.find_user_by_email(&email).await? {
            Some(u) => u,
            None => {
                let mut nu = DefaultUser::new(&email, None);
                nu.email_verified = profile.email_verified;
                nu.name = profile.name.clone();
                nu.image = profile.image.clone();
                state.db.create_user(&nu).await.map_err(|e| {
                    AuthError::new(
                        crate::error::AuthErrorCode::InternalError,
                        e.to_string(),
                    )
                })?;
                state.db.find_user_by_email(&email).await?.ok_or_else(|| {
                    AuthError::new(
                        crate::error::AuthErrorCode::InternalError,
                        "Failed to create user",
                    )
                })?
            }
        };
        let account = DefaultAccount::new(
            &user_record.id,
            &provider_id,
            &profile.provider_account_id,
        );
        state.db.create_account(&account).await.map_err(|e| {
            AuthError::new(
                crate::error::AuthErrorCode::InternalError,
                e.to_string(),
            )
        })?;
        user_record
    };

    state
        .db
        .update_user(
            &user.id,
            crate::database::UserUpdate {
                last_login_method: Some(format!("oauth:{provider_id}")),
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

    let profile_out: UserProfile = (&user).into();
    Ok(Json(json!({
        "user": profile_out,
        "session": crate::session::session_json(&session),
        "token": session.token,
    })))
}
