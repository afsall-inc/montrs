//! I18n plugin — map AuthErrorCode to locale strings.
//! Empty router by default; optional GET /i18n/messages returns error code messages.

use crate::context::AuthState;
use crate::error::AuthErrorCode;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::Value;
use std::collections::HashMap;

/// I18nPlugin — provides error code to message mappings.
pub struct I18nPlugin {
    state: Option<AuthState>,
    /// Custom message overrides per locale.
    messages: HashMap<String, HashMap<String, String>>,
}

impl I18nPlugin {
    pub fn new() -> Self {
        let mut default_messages: HashMap<String, String> = HashMap::new();
        default_messages.insert("invalid-credentials".into(), "Invalid email or password".into());
        default_messages.insert("user-not-found".into(), "User not found".into());
        default_messages.insert("email-already-in-use".into(), "Email already in use".into());
        default_messages.insert("email-not-verified".into(), "Email not verified".into());
        default_messages.insert("invalid-token".into(), "Invalid or expired token".into());
        default_messages.insert("invalid-session".into(), "Invalid or expired session".into());
        default_messages.insert("rate-limited".into(), "Too many requests".into());
        default_messages.insert("missing-field".into(), "Missing required field".into());
        default_messages.insert("weak-password".into(), "Password is too weak".into());
        default_messages.insert("two-factor-required".into(), "Two-factor authentication required".into());
        default_messages.insert("invalid-two-factor".into(), "Invalid two-factor code".into());
        default_messages.insert("oauth-error".into(), "OAuth provider error".into());
        default_messages.insert("provider-not-configured".into(), "OAuth provider not configured".into());
        default_messages.insert("organization-error".into(), "Organization error".into());
        default_messages.insert("forbidden".into(), "Permission denied".into());
        default_messages.insert("database-error".into(), "Database error".into());
        default_messages.insert("internal-error".into(), "Internal server error".into());
        default_messages.insert("captcha-required".into(), "CAPTCHA verification required".into());
        default_messages.insert("account-already-linked".into(), "Account already linked".into());
        default_messages.insert("account-not-found".into(), "Account not found".into());
        default_messages.insert("server-error".into(), "Server misconfiguration".into());

        let mut messages = HashMap::new();
        messages.insert("en".into(), default_messages);
        // Allow more locales to be added via `with_locale`.
        Self {
            state: None,
            messages,
        }
    }

    /// Add or override messages for a locale.
    pub fn with_locale(mut self, locale: &str, msgs: HashMap<String, String>) -> Self {
        self.messages.insert(locale.to_string(), msgs);
        self
    }

    /// Get a localized message for an error code.
    pub fn message(&self, code: &AuthErrorCode, locale: &str) -> String {
        let code_str = serde_json::to_string(code).unwrap_or_default();
        let code_str = code_str.trim_matches('"');
        self.messages
            .get(locale)
            .and_then(|m| m.get(code_str))
            .cloned()
            .unwrap_or_else(|| {
                self.messages
                    .get("en")
                    .and_then(|m| m.get(code_str))
                    .cloned()
                    .unwrap_or_else(|| format!("Unknown error: {code_str}"))
            })
    }
}

impl Default for I18nPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for I18nPlugin {
    fn name(&self) -> &'static str {
        "i18n"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("I18nPlugin: state not set");
        Router::new()
            .route("/i18n/messages", get(get_messages))
            .with_state(state)
    }
}

async fn get_messages(
    State(_state): State<AuthState>,
) -> Result<Json<Value>, AuthError> {
    // Return default English messages. In production, read locale from query.
    let msgs = serde_json::json!({
        "locale": "en",
        "messages": {
            "invalid-credentials": "Invalid email or password",
            "user-not-found": "User not found",
            "email-already-in-use": "Email already in use",
            "email-not-verified": "Email not verified",
            "invalid-token": "Invalid or expired token",
            "invalid-session": "Invalid or expired session",
            "rate-limited": "Too many requests",
            "missing-field": "Missing required field",
            "weak-password": "Password is too weak",
            "two-factor-required": "Two-factor authentication required",
            "invalid-two-factor": "Invalid two-factor code",
            "oauth-error": "OAuth provider error",
            "provider-not-configured": "OAuth provider not configured",
            "organization-error": "Organization error",
            "forbidden": "Permission denied",
            "database-error": "Database error",
            "internal-error": "Internal server error",
            "captcha-required": "CAPTCHA verification required",
            "account-already-linked": "Account already linked",
            "account-not-found": "Account not found",
            "server-error": "Server misconfiguration",
        }
    });
    Ok(Json(msgs))
}