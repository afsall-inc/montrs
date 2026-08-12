//! Two-Factor Authentication plugin — TOTP + backup codes + OTP.
//! Full: enable, disable, get-totp-uri, verify-totp, send-otp, verify-otp,
//! verify-backup-code, generate-backup-codes.

use crate::context::AuthState;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use base64::Engine;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
/// TOTP configuration stored in plugin_store namespace "2fa".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorConfig {
    pub secret: Vec<u8>,
    pub enabled: bool,
    pub backup_codes: Vec<String>,
}

/// Two-Factor Authentication plugin.
pub struct TwoFactorPlugin {
    state: Option<AuthState>,
}

impl TwoFactorPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for TwoFactorPlugin {
    fn default() -> Self {
        Self::new()
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

impl AuthPlugin for TwoFactorPlugin {
    fn name(&self) -> &'static str {
        "two_factor"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("TwoFactorPlugin: state not set");
        Router::new()
            .route("/two-factor/enable", post(enable_2fa))
            .route("/two-factor/disable", post(disable_2fa))
            .route("/two-factor/get-totp-uri", post(get_totp_uri))
            .route("/two-factor/verify-totp", post(verify_totp))
            .route("/two-factor/send-otp", post(send_otp))
            .route("/two-factor/verify-otp", post(verify_otp))
            .route("/two-factor/verify-backup-code", post(verify_backup_code))
            .route("/two-factor/generate-backup-codes", post(generate_backup_codes))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enable2faRequest {
    pub password: Option<String>,
}

async fn enable_2fa(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<Enable2faRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;
    let user = state.db.find_user_by_id(&session.user_id).await?.ok_or_else(AuthError::user_not_found)?;

    // Verify password if provided.
    if let Some(pw) = &req.password {
        if let Some(hash) = &user.password_hash {
            if !crate::password::verify_password(pw, hash) {
                return Err(AuthError::invalid_credentials());
            }
        }
    }

    // Check if already enabled.
    if let Some(existing) = state.db.plugin_get("2fa", &user.id).await.ok().flatten() {
        let cfg: TwoFactorConfig = serde_json::from_value(existing).unwrap_or(TwoFactorConfig {
            secret: vec![],
            enabled: false,
            backup_codes: vec![],
        });
        if cfg.enabled {
            return Err(AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "2FA already enabled"));
        }
    }

    let secret = crate::utils::totp::generate_secret();
    let backup_codes: Vec<String> = (0..8).map(|_| crate::utils::generate_token()).collect();
    let cfg = TwoFactorConfig {
        secret: secret.clone(),
        enabled: false, // not yet enabled (needs verify-totp first)
        backup_codes: backup_codes.clone(),
    };

    state
        .db
        .plugin_set("2fa", &user.id, serde_json::to_value(&cfg).unwrap())
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let uri = crate::utils::totp::provisioning_uri(&secret, &user.email, "MontRS");

    Ok(Json(json!({
        "totpUri": uri,
        "secret": base64::engine::general_purpose::STANDARD.encode(&secret),
        "backupCodes": backup_codes,
    })))
}

async fn disable_2fa(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;

    state.db.plugin_delete("2fa", &session.user_id).await.ok();
    state
        .db
        .update_user(
            &session.user_id,
            crate::database::UserUpdate {
                two_factor_enabled: Some(false),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(json!({ "success": true, "twoFactorEnabled": false })))
}

async fn get_totp_uri(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;
    let user = state.db.find_user_by_id(&session.user_id).await?.ok_or_else(AuthError::user_not_found)?;

    let cfg_val = state.db.plugin_get("2fa", &user.id).await?.ok_or_else(|| {
        AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "2FA not configured")
    })?;
    let cfg: TwoFactorConfig = serde_json::from_value(cfg_val)
        .map_err(|_| AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "Invalid 2FA config"))?;

    let uri = crate::utils::totp::provisioning_uri(&cfg.secret, &user.email, "MontRS");
    Ok(Json(json!({ "totpUri": uri })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTotpRequest {
    pub code: String,
}

async fn verify_totp(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<VerifyTotpRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;

    let cfg_val = state.db.plugin_get("2fa", &session.user_id).await?.ok_or_else(|| {
        AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "2FA not configured")
    })?;
    let mut cfg: TwoFactorConfig = serde_json::from_value(cfg_val)
        .map_err(|_| AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "Invalid 2FA config"))?;

    if !crate::utils::totp::verify_code(&cfg.secret, &req.code) {
        return Err(AuthError::invalid_two_factor());
    }

    // Enable 2FA on first successful verification.
    cfg.enabled = true;
    state
        .db
        .plugin_set("2fa", &session.user_id, serde_json::to_value(&cfg).unwrap())
        .await
        .ok();
    state
        .db
        .update_user(
            &session.user_id,
            crate::database::UserUpdate {
                two_factor_enabled: Some(true),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(json!({ "success": true, "twoFactorEnabled": true })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendOtpRequest {
    pub user_id: Option<String>,
}

async fn send_otp(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<SendOtpRequest>,
) -> Result<Json<Value>, AuthError> {
    let uid = if let Some(uid) = &req.user_id {
        uid.clone()
    } else {
        let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
        let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;
        session.user_id
    };

    let user = state.db.find_user_by_id(&uid).await?.ok_or_else(AuthError::user_not_found)?;

    let otp = crate::verification::create_otp(
        state.db.as_ref(),
        format!("2fa-otp:{}", uid),
        6,
        300,
    )
    .await
    .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    let _ = state
        .email
        .send(crate::email::EmailMessage {
            to: user.email.clone(),
            subject: "Your two-factor code".into(),
            body_text: format!("Your 2FA code is: {}\n\nIt expires in 5 minutes.", otp.value),
            body_html: None,
        })
        .await;

    Ok(Json(json!({ "success": true, "message": "OTP sent to email" })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOtpRequest {
    pub user_id: Option<String>,
    pub otp: String,
}

async fn verify_otp(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<VerifyOtpRequest>,
) -> Result<Json<Value>, AuthError> {
    let uid = if let Some(uid) = &req.user_id {
        uid.clone()
    } else {
        let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
        let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;
        session.user_id
    };

    let _rec = crate::verification::consume_verification(
        state.db.as_ref(),
        &format!("2fa-otp:{}", uid),
        &req.otp,
    )
    .await
    .map_err(|_| AuthError::invalid_two_factor())?;

    Ok(Json(json!({ "success": true, "verified": true })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyBackupCodeRequest {
    pub code: String,
}

async fn verify_backup_code(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<VerifyBackupCodeRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;

    let cfg_val = state.db.plugin_get("2fa", &session.user_id).await?.ok_or_else(|| {
        AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "2FA not configured")
    })?;
    let mut cfg: TwoFactorConfig = serde_json::from_value(cfg_val)
        .map_err(|_| AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "Invalid 2FA config"))?;

    let pos = cfg.backup_codes.iter().position(|c| c == &req.code);
    match pos {
        Some(idx) => {
            cfg.backup_codes.remove(idx);
            state
                .db
                .plugin_set("2fa", &session.user_id, serde_json::to_value(&cfg).unwrap())
                .await
                .ok();
            Ok(Json(json!({ "success": true, "remainingCodes": cfg.backup_codes.len() })))
        }
        None => Err(AuthError::invalid_two_factor()),
    }
}

async fn generate_backup_codes(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;

    let cfg_val = state.db.plugin_get("2fa", &session.user_id).await?.ok_or_else(|| {
        AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "2FA not configured")
    })?;
    let mut cfg: TwoFactorConfig = serde_json::from_value(cfg_val)
        .map_err(|_| AuthError::new(crate::error::AuthErrorCode::InvalidTwoFactor, "Invalid 2FA config"))?;

    cfg.backup_codes = (0..8).map(|_| crate::utils::generate_token()).collect();
    state
        .db
        .plugin_set("2fa", &session.user_id, serde_json::to_value(&cfg).unwrap())
        .await
        .ok();

    Ok(Json(json!({
        "backupCodes": cfg.backup_codes,
        "message": "Store these codes safely. Each can be used once.",
    })))
}