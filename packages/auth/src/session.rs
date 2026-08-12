//! Session management — create, validate, refresh, and revoke sessions.

use crate::database::{DatabaseAdapter, SessionRecord, UserRecord};
use crate::entities::DefaultSession;
use chrono::Utc;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

/// Manages session lifecycle.
#[derive(Clone)]
pub struct SessionManager {
    secret: String,
    adapter: Arc<dyn DatabaseAdapter>,
}

impl SessionManager {
    /// Create a new session manager.
    pub fn new(secret: String, adapter: Arc<dyn DatabaseAdapter>) -> Self {
        Self { secret, adapter }
    }

    /// Create a new session for a user.
    pub async fn create(
        &self,
        user_id: &str,
        expires_in_secs: u64,
    ) -> anyhow::Result<DefaultSession> {
        let session = DefaultSession::new(user_id, expires_in_secs);
        self.adapter.create_session(&session).await?;
        Ok(session)
    }

    /// Validate a session token. Returns the session if valid and not expired.
    pub async fn validate(&self, token: &str) -> anyhow::Result<Option<SessionRecord>> {
        let Some(session) = self.adapter.find_session_by_token(token).await? else {
            return Ok(None);
        };
        if session.expires_at <= Utc::now() {
            let _ = self.adapter.delete_session(&session.id).await;
            return Ok(None);
        }
        Ok(Some(session))
    }

    /// Get the user associated with a valid session.
    pub async fn get_user(&self, token: &str) -> anyhow::Result<Option<UserRecord>> {
        let Some(session) = self.validate(token).await? else {
            return Ok(None);
        };
        self.adapter.find_user_by_id(&session.user_id).await
    }

    /// Revoke a session.
    pub async fn revoke(&self, token: &str) -> anyhow::Result<()> {
        self.adapter.delete_session(token).await
    }

    /// Revoke all sessions for a user.
    pub async fn revoke_all(&self, user_id: &str) -> anyhow::Result<()> {
        self.adapter.delete_user_sessions(user_id).await
    }

    /// List sessions for a user.
    pub async fn list(&self, user_id: &str) -> anyhow::Result<Vec<SessionRecord>> {
        self.adapter.list_sessions(user_id).await
    }

    /// CORS layer placeholder for middleware chain stability.
    pub fn middleware(&self) -> CorsLayer {
        CorsLayer::permissive()
    }

    /// The signing secret.
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// Underlying adapter.
    pub fn adapter(&self) -> &Arc<dyn DatabaseAdapter> {
        &self.adapter
    }
}

/// JSON helper for session responses.
pub fn session_json(session: &DefaultSession) -> serde_json::Value {
    serde_json::json!({
        "id": session.id,
        "userId": session.user_id,
        "token": session.token,
        "expiresAt": session.expires_at.to_rfc3339(),
        "createdAt": session.created_at.to_rfc3339(),
    })
}
