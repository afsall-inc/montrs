//! montrs-auth: Authentication system for MontRS.
//!
//! Comprehensive, plugin-based authentication with email/password, OAuth, 2FA,
//! sessions, organizations, API keys, and more.

pub mod config;
pub mod context;
pub mod core;
pub mod database;
pub mod email;
pub mod entities;
pub mod error;
pub mod middleware;
pub mod password;
pub mod plugin;
pub mod plugins;
pub mod providers;
pub mod rate_limit;
pub mod session;
pub mod utils;
pub mod verification;

pub use config::AuthConfig;
pub use context::AuthState;
pub use error::AuthError;
pub use plugin::AuthPlugin;

/// Re-export of core entity traits.
pub use entities::{AuthAccount, AuthSession, AuthUser};

/// The main auth system — use [`MontrsAuth::builder`] to configure.
pub struct MontrsAuth {
    pub config: AuthConfig,
    pub state: AuthState,
    pub plugins: Vec<Box<dyn AuthPlugin>>,
}

impl MontrsAuth {
    /// Create a new auth builder.
    pub fn builder() -> AuthBuilder {
        AuthBuilder::new()
    }

    /// Get the axum router for all auth endpoints (core + plugins).
    pub fn axum_router(&self) -> axum::Router {
        let mut router = axum::Router::new();

        // Core always-on routes.
        router = router.merge(core::router(self.state.clone()));

        // Plugin routes.
        for plugin in &self.plugins {
            router = router.merge(plugin.router());
        }

        // Session middleware layer.
        router = router.layer(self.state.session.middleware());

        router
    }

    /// Access the shared auth state.
    pub fn state(&self) -> &AuthState {
        &self.state
    }
}

/// Builder for [`MontrsAuth`].
pub struct AuthBuilder {
    config: Option<AuthConfig>,
    plugins: Vec<Box<dyn AuthPlugin>>,
    database: Option<Box<dyn database::DatabaseAdapter>>,
    email: Option<Box<dyn email::EmailProvider>>,
}

impl AuthBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            plugins: Vec::new(),
            database: None,
            email: None,
        }
    }

    /// Set the auth configuration.
    pub fn config(mut self, config: AuthConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Add an auth plugin.
    pub fn plugin(mut self, plugin: Box<dyn AuthPlugin>) -> Self {
        self.plugins.push(plugin);
        self
    }

    /// Set the database adapter.
    pub fn database(mut self, adapter: Box<dyn database::DatabaseAdapter>) -> Self {
        self.database = Some(adapter);
        self
    }

    /// Set the email provider.
    pub fn email(mut self, provider: Box<dyn email::EmailProvider>) -> Self {
        self.email = Some(provider);
        self
    }

    /// Build the auth system.
    pub async fn build(self) -> anyhow::Result<MontrsAuth> {
        let config = self.config.unwrap_or_default();
        if config.secret.is_empty() {
            anyhow::bail!("AuthConfig.secret must be set (at least 32 characters recommended)");
        }

        let db: std::sync::Arc<dyn database::DatabaseAdapter> =
            if let Some(db) = self.database {
                std::sync::Arc::from(db)
            } else {
                std::sync::Arc::new(database::MemoryDatabaseAdapter::new())
            };

        let email: std::sync::Arc<dyn email::EmailProvider> =
            if let Some(e) = self.email {
                std::sync::Arc::from(e)
            } else {
                std::sync::Arc::new(email::ConsoleEmailProvider::new())
            };

        let session = session::SessionManager::new(config.secret.clone(), db.clone());
        let rate_limit = rate_limit::RateLimiter::new(
            config.rate_limit_max,
            config.rate_limit_window_secs,
        );

        let state = AuthState {
            config: config.clone(),
            db,
            session,
            email,
            rate_limit: std::sync::Arc::new(rate_limit),
        };

        // Give plugins a chance to initialize with state.
        let mut plugins = self.plugins;
        for plugin in &mut plugins {
            plugin.on_build(&state)?;
        }

        Ok(MontrsAuth {
            config,
            state,
            plugins,
        })
    }
}

impl Default for AuthBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The result of a sign-in or sign-up operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthResult {
    pub user: entities::UserProfile,
    pub session: Option<entities::SessionProfile>,
    pub token: Option<String>,
}
