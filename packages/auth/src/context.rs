//! Shared auth state injected into handlers and plugins.

use crate::config::AuthConfig;
use crate::database::DatabaseAdapter;
use crate::email::EmailProvider;
use crate::rate_limit::RateLimiter;
use crate::session::SessionManager;
use std::sync::Arc;

/// Shared runtime state for core routes and plugins.
#[derive(Clone)]
pub struct AuthState {
    pub config: AuthConfig,
    pub db: Arc<dyn DatabaseAdapter>,
    pub session: SessionManager,
    pub email: Arc<dyn EmailProvider>,
    pub rate_limit: Arc<RateLimiter>,
}

impl AuthState {
    /// Convenience: session expiry from config.
    pub fn session_expires_secs(&self) -> u64 {
        self.config.session.expires_in_secs
    }
}
