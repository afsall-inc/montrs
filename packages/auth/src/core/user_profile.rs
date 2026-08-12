//! User profile: update, change email, delete user.

use crate::context::AuthState;
use crate::database::UserUpdate;
use axum::extract::State;
use axum::Json;
use axum::routing::post;
use axum::Router;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/update-user", post(update_user))
        .route("/change-email", post(change_email))
        .route("/delete-user", post(delete_user))
        .with_state(state)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateUserBody {
    name: Option<String>,
    image: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangeEmailBody {
    new_email: String,
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .or_else(|| {
            headers.get("cookie").and_then(|v| v.to_str().ok()).and_then(|c| {
                c.split(';').find_map(|p| {
                    let p = p.trim();
                    p.strip_prefix("session=")
                        .or_else(|| p.strip_prefix("__montrs_session="))
                        .map(|s| s.to_string())
                })
            })
        })
}

async fn update_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<UpdateUserBody>,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers).ok_or_else(crate::AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;

    let updates = UserUpdate {
        name: body.name.clone(),
        image: body.image.clone(),
        ..Default::default()
    };
    state.db.update_user(&user.id, updates).await?;
    let updated = state
        .db
        .find_user_by_id(&user.id)
        .await?
        .ok_or_else(crate::AuthError::user_not_found)?;
    let profile: crate::entities::UserProfile = (&updated).into();
    Ok(Json(json!({ "user": profile })))
}

async fn change_email(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<ChangeEmailBody>,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers).ok_or_else(crate::AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;

    if state.db.find_user_by_email(&body.new_email).await?.is_some() {
        return Err(crate::AuthError::email_in_use());
    }

    state
        .db
        .update_user(
            &user.id,
            UserUpdate {
                email: Some(body.new_email.clone()),
                email_verified: Some(false),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(json!({ "success": true, "email": body.new_email })))
}

async fn delete_user(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, crate::AuthError> {
    let token = extract_token(&headers).ok_or_else(crate::AuthError::invalid_session)?;
    let user = state
        .session
        .get_user(&token)
        .await?
        .ok_or_else(crate::AuthError::invalid_session)?;

    state.db.delete_user(&user.id).await?;
    state.session.revoke_all(&user.id).await?;
    Ok(Json(json!({ "success": true })))
}