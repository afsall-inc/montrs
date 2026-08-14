//! Auth configuration — the `AuthConfig` builder.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OAuth provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OAuthProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    /// Optional: the redirect URI. If not set, uses the default.
    pub redirect_uri: Option<String>,
    /// Scopes to request.
    #[serde(default)]
    pub scopes: Vec<String>,
}

/// Session configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// How long a session lasts (in seconds). Default: 7 days.
    #[serde(default = "default_session_expiry")]
    pub expires_in_secs: u64,
    /// How long before expiry to refresh the session.
    #[serde(default = "default_refresh_threshold")]
    pub refresh_threshold_secs: u64,
    /// Whether to update the session expiry on each request.
    #[serde(default = "default_true")]
    pub update_expiry: bool,
}

fn default_session_expiry() -> u64 { 7 * 24 * 3600 }
fn default_refresh_threshold() -> u64 { 24 * 3600 }
fn default_true() -> bool { true }

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            expires_in_secs: default_session_expiry(),
            refresh_threshold_secs: default_refresh_threshold(),
            update_expiry: true,
        }
    }
}

/// Password policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    /// Minimum password length (default: 8).
    #[serde(default = "default_min_length")]
    pub min_length: usize,
    /// Require at least one uppercase letter.
    #[serde(default)]
    pub require_uppercase: bool,
    /// Require at least one lowercase letter.
    #[serde(default)]
    pub require_lowercase: bool,
    /// Require at least one digit.
    #[serde(default)]
    pub require_digit: bool,
    /// Require at least one special character.
    #[serde(default)]
    pub require_special: bool,
}

fn default_min_length() -> usize { 8 }

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: default_min_length(),
            require_uppercase: false,
            require_lowercase: false,
            require_digit: false,
            require_special: false,
        }
    }
}

impl PasswordPolicy {
    /// Validate a password against the policy. Returns Ok(()) or an error.
    pub fn validate(&self, password: &str) -> Result<(), crate::AuthError> {
        if password.len() < self.min_length {
            return Err(crate::AuthError::new(
                crate::error::AuthErrorCode::WeakPassword,
                format!("Password must be at least {} characters", self.min_length),
            ));
        }
        if self.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(crate::AuthError::new(
                crate::error::AuthErrorCode::WeakPassword,
                "Password must contain an uppercase letter",
            ));
        }
        if self.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err(crate::AuthError::new(
                crate::error::AuthErrorCode::WeakPassword,
                "Password must contain a lowercase letter",
            ));
        }
        if self.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(crate::AuthError::new(
                crate::error::AuthErrorCode::WeakPassword,
                "Password must contain a digit",
            ));
        }
        if self.require_special && !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(crate::AuthError::new(
                crate::error::AuthErrorCode::WeakPassword,
                "Password must contain a special character",
            ));
        }
        Ok(())
    }
}

/// The main auth configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Secret key for signing tokens and cookies.
    pub secret: String,
    /// Base URL of the application (e.g., "http://localhost:3000").
    pub base_url: String,
    /// Session configuration.
    #[serde(default)]
    pub session: SessionConfig,
    /// Password policy.
    #[serde(default)]
    pub password: PasswordPolicy,
    /// OAuth provider configurations.
    #[serde(default)]
    pub oauth_providers: HashMap<String, OAuthProviderConfig>,
    /// Whether to enable email verification.
    #[serde(default)]
    pub email_verification: bool,
    /// Whether to trust the X-Forwarded-For header.
    #[serde(default)]
    pub trust_forwarded: bool,
    /// Rate limit: max requests per window.
    #[serde(default = "default_rate_limit")]
    pub rate_limit_max: u32,
    /// Rate limit window in seconds.
    #[serde(default = "default_rate_window")]
    pub rate_limit_window_secs: u64,
}

fn default_rate_limit() -> u32 { 10 }
fn default_rate_window() -> u64 { 60 }

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            secret: String::new(),
            base_url: "http://localhost:3000".to_string(),
            session: SessionConfig::default(),
            password: PasswordPolicy::default(),
            oauth_providers: HashMap::new(),
            email_verification: false,
            trust_forwarded: false,
            rate_limit_max: default_rate_limit(),
            rate_limit_window_secs: default_rate_window(),
        }
    }
}

impl AuthConfig {
    /// Create a new config with the given secret.
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
            ..Default::default()
        }
    }

    /// Set the base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Add an OAuth provider.
    pub fn add_oauth_provider(
        mut self,
        name: impl Into<String>,
        config: OAuthProviderConfig,
    ) -> Self {
        self.oauth_providers.insert(name.into(), config);
        self
    }
}