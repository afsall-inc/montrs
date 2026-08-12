//! Multi-Session plugin — list device sessions, set active, revoke.
//! POST /multi-session/list-device-sessions, /multi-session/set-active, /multi-session/revoke.

use crate::context::AuthState;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Multi-Session plugin — device session management.
pub struct MultiSessionPlugin {
    state: Option<AuthState>,
    /// Optional device info headers: "user-agent" and "x-forwarded-for".
    device_headers: bool,
}

impl MultiSessionPlugin {
    pub fn new() -> Self {
        Self {
            state: None,
            device_headers: true,
        }
    }

    /// Disable device info reading from headers.
    pub fn no_device_headers(mut self) -> Self {
        self.device_headers = false;
        self
    }
}

impl Default for MultiSessionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for MultiSessionPlugin {
    fn name(&self) -> &'static str {
        "multi_session"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("MultiSessionPlugin: state not set");
        Router::new()
            .route("/multi-session/list-device-sessions", post(list_device_sessions))
            .route("/multi-session/set-active", post(set_active))
            .route("/multi-session/revoke", post(revoke_session))
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

async fn list_device_sessions(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let sessions = state.session.list(&session.user_id).await?;
    let mut device_sessions = Vec::new();
    for s in sessions {
        // Device info stored in the plugin store keyed by session id.
        let device: Option<Value> = state
            .db
            .plugin_get("device_session", &s.id)
            .await
            .ok()
            .flatten();
        let device_name = device
            .and_then(|d| d.get("name").and_then(|v| v.as_str()).map(String::from))
            .unwrap_or_else(|| "unknown device".into());
        device_sessions.push(json!({
            "id": s.id,
            "userId": s.user_id,
            "expiresAt": s.expires_at.to_rfc3339(),
            "createdAt": s.created_at.to_rfc3339(),
            "deviceName": device_name,
            "current": s.id == session.id,
        }));
    }

    let mut info = HashMap::new();
    info.insert("sessions".to_string(), serde_json::json!(device_sessions));
    info.insert("currentSessionId".to_string(), serde_json::json!(session.id));
    Ok(Json(serde_json::to_value(info).unwrap_or_default()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetActiveRequest {
    pub session_id: String,
    pub device_name: Option<String>,
}

async fn set_active(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<SetActiveRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    // Verify the target session belongs to the same user.
    let target = state
        .db
        .find_session(&req.session_id)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    if target.user_id != session.user_id {
        return Err(AuthError::forbidden());
    }

    // Record device info.
    let device_json = json!({
        "name": req.device_name.unwrap_or_else(|| "unknown device".into()),
        "active": true,
    });
    state
        .db
        .plugin_set("device_session", &req.session_id, device_json)
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    Ok(Json(json!({ "success": true, "activeSessionId": req.session_id })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeRequest {
    pub session_id: String,
}

async fn revoke_session(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RevokeRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let target = state
        .db
        .find_session(&req.session_id)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    if target.user_id != session.user_id {
        return Err(AuthError::forbidden());
    }

    state.session.revoke(&req.session_id).await?;
    let _ = state.db.plugin_delete("device_session", &req.session_id).await;

    Ok(Json(json!({ "success": true, "revoked": req.session_id })))
}