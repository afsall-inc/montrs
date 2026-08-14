//! Username plugin — sign-in by username, check availability.
//! POST /sign-in/username, GET /is-username-available?username=

use crate::context::AuthState;
use crate::database::UserUpdate;
use crate::entities::UserProfile;
use crate::password::verify_password;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

/// Username plugin — sign-in by username, check availability.
pub struct UsernamePlugin {
    state: Option<AuthState>,
}

impl UsernamePlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for UsernamePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for UsernamePlugin {
    fn name(&self) -> &'static str {
        "username"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("UsernamePlugin: state not set");
        Router::new()
            .route("/sign-in/username", post(sign_in_username))
            .route("/is-username-available", get(is_username_available))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInUsernameRequest {
    pub username: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UsernameQuery {
    pub username: String,
}

async fn sign_in_username(
    State(state): State<AuthState>,
    Json(req): Json<SignInUsernameRequest>,
) -> Result<Json<Value>, AuthError> {
    if req.username.is_empty() {
        return Err(AuthError::missing_field("username"));
    }
    if req.password.is_empty() {
        return Err(AuthError::missing_field("password"));
    }

    let user = state
        .db
        .find_user_by_username(&req.username)
        .await?
        .ok_or_else(AuthError::invalid_credentials)?;

    if user.banned {
        return Err(AuthError::forbidden());
    }

    let hash = user
        .password_hash
        .as_deref()
        .ok_or_else(AuthError::invalid_credentials)?;

    if !verify_password(&req.password, hash) {
        return Err(AuthError::invalid_credentials());
    }

    if state.config.email_verification && !user.email_verified {
        return Err(AuthError::email_not_verified());
    }

    let _ = state
        .db
        .update_user(
            &user.id,
            UserUpdate {
                last_login_method: Some("username".into()),
                ..Default::default()
            },
        )
        .await;

    let session = state
        .session
        .create(&user.id, state.session_expires_secs())
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let profile: UserProfile = (&user).into();
    Ok(Json(json!({
        "user": profile,
        "session": crate::session::session_json(&session),
        "token": session.token,
    })))
}

async fn is_username_available(
    State(state): State<AuthState>,
    Query(query): Query<UsernameQuery>,
) -> Result<Json<Value>, AuthError> {
    if query.username.is_empty() {
        return Err(AuthError::missing_field("username"));
    }
    let existing = state
        .db
        .find_user_by_username(&query.username)
        .await?;
    Ok(Json(json!({
        "available": existing.is_none(),
        "username": query.username,
    })))
}