//! Auth plugin system — compose only the features you need.

use crate::context::AuthState;
use crate::AuthError;
use axum::extract::Request;
use axum::response::Response;
use axum::Router;

/// Schema extension declared by a plugin (for OpenAPI / migrations).
#[derive(Debug, Clone)]
pub struct SchemaExtension {
    pub table: String,
    pub description: String,
}

/// The auth plugin trait. Implement this to add auth features.
pub trait AuthPlugin: Send + Sync + 'static {
    /// A short name identifying the plugin.
    fn name(&self) -> &'static str;

    /// Called once when [`crate::MontrsAuth`] is built. Store `AuthState` if needed.
    fn on_build(&mut self, _state: &AuthState) -> Result<(), AuthError> {
        Ok(())
    }

    /// The axum router this plugin registers.
    fn router(&self) -> Router {
        Router::new()
    }

    /// Hooks run before a request is handled.
    fn before_request(&self, _req: &Request) -> Result<(), AuthError> {
        Ok(())
    }

    /// Hooks run after a request is handled.
    fn after_request(&self, _resp: &Response) {}

    /// Optional schema extensions for docs / migrations.
    fn schema_extensions(&self) -> Vec<SchemaExtension> {
        Vec::new()
    }
}

/// Attach a plugin's router to an existing router.
pub fn mount_plugin(router: Router, plugin: &dyn AuthPlugin) -> Router {
    router.merge(plugin.router())
}
