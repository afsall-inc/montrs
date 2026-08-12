//! Last Login Method plugin — hook-only.
//! Core already sets `last_login_method` on UserRecord during sign-in.
//! This plugin registers no routes; it serves as documentation and
//! a hook point for custom last-login logic.

use crate::plugin::AuthPlugin;

/// LastLoginMethodPlugin — hook-only. Core already tracks last_login_method.
pub struct LastLoginMethodPlugin;

impl AuthPlugin for LastLoginMethodPlugin {
    fn name(&self) -> &'static str {
        "last_login_method"
    }
}