//! JWT plugin — token issuance and JWKS endpoint.
//! GET /token, GET /jwks — uses utils::jwt; HS256 JWKS stub.

use crate::context::AuthState;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};

/// JWT plugin — issue tokens and expose a JWKS document.
pub struct JwtPlugin {
    state: Option<AuthState>,
}

impl JwtPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for JwtPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for JwtPlugin {
    fn name(&self) -> &'static str {
        "jwt"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("JwtPlugin: state not set");
        Router::new()
            .route("/token", get(get_token))
            .route("/jwks", get(get_jwks))
            .with_state(state)
    }
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    if let Some(v) = headers.get("authorization").and_then(|v| v.to_str().ok()) {
        if let Some(t) = v.strip_prefix("Bearer ") {
            return Some(t.to_string());
        }
    }
    if let Some(v) = headers.get("cookie").and_then(|v| v.to_str().ok()) {
        for part in v.split(';') {
            let part = part.trim();
            if let Some(t) = part.strip_prefix("session=") {
                return Some(t.to_string());
            }
            if let Some(t) = part.strip_prefix("__montrs_session=") {
                return Some(t.to_string());
            }
        }
    }
    None
}

async fn get_token(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    // Issue a JWT signed with the configured secret.
    let jwt = crate::utils::jwt::create_token(
        &session.user_id,
        state.session.secret(),
        3600,
    )
    .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    Ok(Json(json!({
        "token": jwt,
        "tokenType": "Bearer",
        "expiresIn": 3600,
        "sessionId": session.id,
    })))
}

async fn get_jwks() -> Json<Value> {
    // HS256 symmetric keys have no public key material; return an empty
    // keys list with a note. In production, swap in an RSA keypair and
    // expose the public key here.
    Json(json!({
        "keys": [],
        "note": "This server signs JWTs with HS256. No public JWK is available for symmetric keys. Configure an RSA keypair for asymmetric JWKS support.",
    }))
}