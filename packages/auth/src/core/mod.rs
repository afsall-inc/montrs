//! Core always-on auth routes.

pub mod email_password;
pub mod email_verification;
pub mod health;
pub mod password_reset;
pub mod sessions;
pub mod social;
pub mod user_profile;

use crate::context::AuthState;
use axum::Router;

/// Build the router with all core routes, bound to `state`.
pub fn router(state: AuthState) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(email_password::routes(state.clone()))
        .merge(sessions::routes(state.clone()))
        .merge(password_reset::routes(state.clone()))
        .merge(email_verification::routes(state.clone()))
        .merge(user_profile::routes(state.clone()))
        .merge(social::routes(state))
}