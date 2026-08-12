//! One-Time Token plugin — generate and verify single-use tokens.
//! POST /one-time-token/generate (needs session), POST /one-time-token/verify.
//! Uses verification store with identifier `ott:{userId}`.

use crate::context::AuthState;
use crate::plugin::AuthPlugin;
use crate::utils::generate_token;
use crate::AuthError;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
/// One-Time Token plugin.
pub struct OneTimeTokenPlugin {
    state: Option<AuthState>,
}

impl OneTimeTokenPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for OneTimeTokenPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for OneTimeTokenPlugin {
    fn name(&self) -> &'static str {
        "one_time_token"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("OneTimeTokenPlugin: state not set");
        Router::new()
            .route("/one-time-token/generate", post(generate_ott))
            .route("/one-time-token/verify", post(verify_ott))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateOttRequest {
    /// Optional custom expiry in seconds (default 300).
    pub expires_in_secs: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOttRequest {
    pub token: String,
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

async fn generate_ott(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<GenerateOttRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let ott = generate_token();
    let expires_in = req.expires_in_secs.unwrap_or(300);
    let identifier = format!("ott:{}", session.user_id);

    let rec = crate::verification::create_verification(
        state.db.as_ref(),
        &identifier,
        Some(ott.clone()),
        expires_in,
    )
    .await
    .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    Ok(Json(json!({
        "token": rec.value,
        "expiresAt": rec.expires_at.to_rfc3339(),
    })))
}

async fn verify_ott(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<VerifyOttRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let identifier = format!("ott:{}", session.user_id);
    let _rec = crate::verification::consume_verification(
        state.db.as_ref(),
        &identifier,
        &req.token,
    )
    .await
    .map_err(|_| AuthError::invalid_token())?;

    Ok(Json(json!({ "valid": true })))
}