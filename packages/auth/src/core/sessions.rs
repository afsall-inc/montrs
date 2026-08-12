//! Sessions core routes: get-session, list, revoke, sign-out.

use crate::context::AuthState;
use axum::extract::State;
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/get-session", get(get_session))
        .route("/list-sessions", post(list_sessions))
        .route("/revoke-session", post(revoke_session))
        .route("/revoke-other-sessions", post(revoke_other_sessions))
        .route("/sign-out", post(sign_out))
        .with_state(state)
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

async fn get_session(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers)
        .ok_or_else(crate::AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let user = state
        .db
        .find_user_by_id(&session.user_id)
        .await?
        .ok_or_else(crate::AuthError::user_not_found)?;
    let profile: crate::entities::UserProfile = (&user).into();
    Ok(Json(json!({
        "session": {
            "id": session.id,
            "userId": session.user_id,
            "expiresAt": session.expires_at.to_rfc3339(),
            "createdAt": session.created_at.to_rfc3339(),
        },
        "user": profile,
    })))
}

async fn list_sessions(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers)
        .ok_or_else(crate::AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let sessions = state.session.list(&session.user_id).await?;
    Ok(Json(json!({
        "sessions": sessions.iter().map(|s| json!({
            "id": s.id,
            "userId": s.user_id,
            "expiresAt": s.expires_at.to_rfc3339(),
            "createdAt": s.created_at.to_rfc3339(),
        })).collect::<Vec<_>>(),
    })))
}

#[derive(Deserialize)]
struct RevokeBody {
    token: String,
}

async fn revoke_session(
    State(state): State<AuthState>,
    Json(body): Json<RevokeBody>,
) -> Result<Json<Value>, crate::AuthError> {
    state.session.revoke(&body.token).await?;
    Ok(Json(json!({ "success": true })))
}

async fn revoke_other_sessions(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers)
        .ok_or_else(crate::AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;
    let sessions = state.session.list(&session.user_id).await?;
    for s in sessions {
        if s.id != session.id {
            let _ = state.session.revoke(&s.id).await;
        }
    }
    Ok(Json(json!({ "success": true })))
}

async fn sign_out(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers)
        .ok_or_else(crate::AuthError::invalid_session)?;
    state.session.revoke(&token).await?;
    Ok(Json(json!({ "success": true })))
}