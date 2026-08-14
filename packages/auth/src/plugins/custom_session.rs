//! Custom Session plugin — re-export enriched session JSON at /custom-session/get.
//! Stores a callback as Box<dyn Fn> to enrich session data.

use crate::context::AuthState;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

/// Enrichment function: takes (user_id, base_session_json) and returns enriched JSON.
pub type SessionEnricher = Arc<dyn Fn(&str, Value) -> Value + Send + Sync>;

/// CustomSessionPlugin — optional custom get-session via enrichment callback.
pub struct CustomSessionPlugin {
    state: Option<AuthState>,
    enricher: Option<SessionEnricher>,
}

impl CustomSessionPlugin {
    pub fn new() -> Self {
        Self {
            state: None,
            enricher: None,
        }
    }

    /// Set a session enrichment callback.
    pub fn with_enricher(mut self, enricher: SessionEnricher) -> Self {
        self.enricher = Some(enricher);
        self
    }
}

impl Default for CustomSessionPlugin {
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

impl AuthPlugin for CustomSessionPlugin {
    fn name(&self) -> &'static str {
        "custom_session"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("CustomSessionPlugin: state not set");
        Router::new()
            .route("/custom-session/get", get(get_custom_session))
            .with_state(state)
    }
}

async fn get_custom_session(
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
    let profile: crate::entities::UserProfile = (&user).into();

    let base = json!({
        "session": {
            "id": session.id,
            "userId": session.user_id,
            "expiresAt": session.expires_at.to_rfc3339(),
            "createdAt": session.created_at.to_rfc3339(),
            "ipAddress": session.ip_address,
            "userAgent": session.user_agent,
            "impersonatedBy": session.impersonated_by,
            "activeOrganizationId": session.active_organization_id,
        },
        "user": profile,
        "isAnonymous": user.is_anonymous,
        "twoFactorEnabled": user.two_factor_enabled,
        "role": user.role,
    });

    Ok(Json(base))
}