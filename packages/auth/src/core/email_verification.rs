//! Email verification: send verification and verify.

use crate::context::AuthState;
use crate::database::UserUpdate;
use axum::extract::{Query, State};
use axum::Json;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/send-verification-email", post(send_verification_email))
        .route("/verify-email", get(verify_email))
        .with_state(state)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendVerificationBody {
    email: String,
    callback_url: Option<String>,
}

#[derive(Deserialize)]
struct VerifyQuery {
    token: String,
    #[serde(default)]
    email: Option<String>,
}

async fn send_verification_email(
    State(state): State<AuthState>,
    Json(body): Json<SendVerificationBody>,
) -> Result<Json<Value>, crate::AuthError> {
    let user = state
        .db
        .find_user_by_email(&body.email)
        .await?
        .ok_or_else(crate::AuthError::user_not_found)?;

    if user.email_verified {
        return Ok(Json(json!({ "verified": true })));
    }

    let ver = crate::verification::create_verification(
        state.db.as_ref(),
        format!("email-verify:{}", user.email),
        None,
        3600 * 48,
    )
    .await
    .map_err(|e| crate::AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let _ = state
        .email
        .send(crate::email::EmailMessage {
            to: user.email.clone(),
            subject: "Verify your email".into(),
            body_text: format!(
                "Click to verify: {}/verify-email?token={}&email={}",
                state.config.base_url, ver.value, user.email
            ),
            body_html: None,
        })
        .await;

    Ok(Json(json!({ "success": true })))
}

async fn verify_email(
    State(state): State<AuthState>,
    Query(query): Query<VerifyQuery>,
) -> Result<Json<Value>, crate::AuthError> {
    let email = query.email.clone().unwrap_or_default();
    let ver = crate::verification::consume_verification(
        state.db.as_ref(),
        &format!("email-verify:{}", email),
        &query.token,
    )
    .await
    .map_err(|_| crate::AuthError::invalid_token())?;

    let email = ver
        .identifier
        .strip_prefix("email-verify:")
        .unwrap_or(&ver.identifier)
        .to_string();

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
                email_verified: Some(true),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(json!({ "verified": true, "message": "Email verified successfully" })))
}