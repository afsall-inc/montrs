//! Bearer token plugin — hook-only.
//! Documents that the core reads Bearer tokens from the Authorization header.
//! No routes needed — the empty router is sufficient.

use crate::plugin::AuthPlugin;

/// BearerPlugin documents bearer token support. No routes; core already handles Bearer.
pub struct BearerPlugin;

impl AuthPlugin for BearerPlugin {
    fn name(&self) -> &'static str {
        "bearer"
    }
}