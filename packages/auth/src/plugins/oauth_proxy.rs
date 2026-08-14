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

//! OAuth Proxy plugin — DX helper: callback route for OAuth proxy flows.
//! GET /oauth/proxy/callback — receives the OAuth code and forwards to the main callback.

use crate::{AuthError, context::AuthState, plugin::AuthPlugin};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use serde::Deserialize;
use serde_json::{Value, json};

/// OAuth Proxy plugin — small DX helper for proxied OAuth flows.
pub struct OAuthProxyPlugin {
    state: Option<AuthState>,
}

impl OAuthProxyPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for OAuthProxyPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for OAuthProxyPlugin {
    fn name(&self) -> &'static str {
        "oauth_proxy"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state =
            self.state.clone().expect("OAuthProxyPlugin: state not set");
        Router::new()
            .route("/oauth/proxy/callback", get(proxy_callback))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
pub struct ProxyCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub provider: Option<String>,
    pub error: Option<String>,
}

async fn proxy_callback(
    State(state): State<AuthState>,
    Query(q): Query<ProxyCallbackQuery>,
) -> Result<Json<Value>, AuthError> {
    if let Some(err) = &q.error {
        return Err(AuthError::new(
            crate::error::AuthErrorCode::OAuthError,
            format!("OAuth proxy error: {err}"),
        ));
    }

    let provider = q.provider.clone().unwrap_or_else(|| "unknown".into());
    let code = q.code.clone().unwrap_or_default();

    // Forward to the main OAuth callback endpoint.
    let callback_url = format!(
        "{}/api/auth/oauth2/callback/{provider}?code={code}",
        state.config.base_url.trim_end_matches('/'),
    );

    Ok(Json(json!({
        "provider": provider,
        "forwardUrl": callback_url,
        "message": "OAuth proxy callback received. Forward to the main callback URL.",
    })))
}
