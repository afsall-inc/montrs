//! Anonymous plugin — anonymous sessions and account deletion.
//! POST /sign-in/anonymous, POST /delete-anonymous-user.

use crate::context::AuthState;
use crate::entities::{DefaultUser, UserProfile};
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};

/// Anonymous plugin — create anonymous sessions and delete anonymous users.
pub struct AnonymousPlugin {
    state: Option<AuthState>,
}

impl AnonymousPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for AnonymousPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for AnonymousPlugin {
    fn name(&self) -> &'static str {
        "anonymous"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("AnonymousPlugin: state not set");
        Router::new()
            .route("/sign-in/anonymous", post(sign_in_anonymous))
            .route("/delete-anonymous-user", post(delete_anonymous_user))
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

async fn sign_in_anonymous(
    State(state): State<AuthState>,
) -> Result<Json<Value>, AuthError> {
    let user = DefaultUser::anonymous();
    state.db.create_user(&user).await.map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;

    let session = state
        .session
        .create(&user.id, state.session_expires_secs())
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let profile = UserProfile {
        id: user.id.clone(),
        email: user.email.clone(),
        name: user.name.clone(),
        image: user.image.clone(),
        email_verified: user.email_verified,
        username: user.username.clone(),
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(Json(json!({
        "user": profile,
        "session": crate::session::session_json(&session),
        "token": session.token,
        "isAnonymous": true,
    })))
}

async fn delete_anonymous_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let user = state
        .db
        .find_user_by_id(&session.user_id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    if !user.is_anonymous {
        return Err(AuthError::forbidden());
    }

    state.db.delete_user(&user.id).await.map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;
    state.session.revoke_all(&user.id).await.ok();

    Ok(Json(json!({ "success": true, "deleted": true })))
}