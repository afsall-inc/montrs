//! Password reset: request and reset.

use crate::context::AuthState;
use crate::database::UserUpdate;
use crate::password::hash_password;
use axum::extract::State;
use axum::Json;
use axum::routing::post;
use axum::Router;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/forget-password", post(forget_password))
        .route("/reset-password", post(reset_password))
        .with_state(state)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ForgetPasswordRequest {
    email: String,
    redirect_to: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResetPasswordRequest {
    token: String,
    new_password: String,
}

async fn forget_password(
    State(state): State<AuthState>,
    Json(req): Json<ForgetPasswordRequest>,
) -> Result<Json<Value>, crate::AuthError> {
    let user = state.db.find_user_by_email(&req.email).await?;
    if user.is_none() {
        // Don't reveal whether the email exists.
        return Ok(Json(json!({ "success": true, "message": "If the email exists, a reset link was sent." })));
    }

    let ver = crate::verification::create_verification(
        state.db.as_ref(),
        format!("reset:{}", req.email),
        None,
        3600,
    )
    .await
    .map_err(|e| crate::AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let link = format!(
        "{}/reset-password?token={}",
        state.config.base_url, ver.value
    );
    let _ = state
        .email
        .send(crate::email::EmailMessage {
            to: req.email,
            subject: "Password Reset".into(),
            body_text: format!("Click to reset your password: {link}"),
            body_html: None,
        })
        .await;

    Ok(Json(json!({ "success": true, "message": "If the email exists, a reset link was sent." })))
}

async fn reset_password(
    State(state): State<AuthState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<Json<Value>, crate::AuthError> {
    let ver = crate::verification::consume_verification_by_value(
        state.db.as_ref(),
        &req.token,
    )
    .await
    .map_err(|_| crate::AuthError::invalid_token())?;

    if !ver.identifier.starts_with("reset:") {
        return Err(crate::AuthError::invalid_token());
    }
    let email = ver
        .identifier
        .strip_prefix("reset:")
        .unwrap_or(&ver.identifier)
        .to_string();

    state.config.password.validate(&req.new_password)?;
    let hash = hash_password(&req.new_password).map_err(|e| {
        crate::AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;

    let user = state
        .db
        .find_user_by_email(&email)
        .await?
        .ok_or_else(crate::AuthError::user_not_found)?;

    state
        .db
        .update_user(
            &user.id,
            UserUpdate {
                password_hash: Some(hash),
                ..Default::default()
            },
        )
        .await?;

    // Revoke all existing sessions.
    let _ = state.session.revoke_all(&user.id).await;

    Ok(Json(json!({ "success": true, "message": "Password reset successfully" })))
}