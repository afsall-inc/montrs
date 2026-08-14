//! OAuth Proxy plugin — DX helper: callback route for OAuth proxy flows.
//! GET /oauth/proxy/callback — receives the OAuth code and forwards to the main callback.

use crate::context::AuthState;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

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
        let state = self.state.clone().expect("OAuthProxyPlugin: state not set");
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
        return Err(AuthError::new(crate::error::AuthErrorCode::OAuthError, format!("OAuth proxy error: {err}")));
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