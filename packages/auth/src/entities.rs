//! Entity traits and default records for the auth system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A user profile (returned to clients — no sensitive data).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&crate::database::UserRecord> for UserProfile {
    fn from(u: &crate::database::UserRecord) -> Self {
        Self {
            id: u.id.clone(),
            email: u.email.clone(),
            name: u.name.clone(),
            image: u.image.clone(),
            email_verified: u.email_verified,
            username: u.username.clone(),
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

/// A session profile (returned to clients).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionProfile {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<&crate::database::SessionRecord> for SessionProfile {
    fn from(s: &crate::database::SessionRecord) -> Self {
        Self {
            id: s.id.clone(),
            user_id: s.user_id.clone(),
            token: s.token.clone(),
            expires_at: s.expires_at,
            created_at: s.created_at,
        }
    }
}

/// Core entity trait for users.
pub trait AuthUser: Send + Sync + 'static {
    fn id(&self) -> &str;
    fn email(&self) -> &str;
    fn email_verified(&self) -> bool;
    fn name(&self) -> Option<&str>;
    fn image(&self) -> Option<&str>;
    fn password_hash(&self) -> Option<&str>;
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
}

/// Core entity trait for sessions.
pub trait AuthSession: Send + Sync + 'static {
    fn id(&self) -> &str;
    fn user_id(&self) -> &str;
    fn expires_at(&self) -> &DateTime<Utc>;
    fn created_at(&self) -> &DateTime<Utc>;
}

/// Core entity trait for accounts (OAuth links).
pub trait AuthAccount: Send + Sync + 'static {
    fn id(&self) -> &str;
    fn user_id(&self) -> &str;
    fn provider_id(&self) -> &str;
    fn provider_account_id(&self) -> &str;
}

/// Default user record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub password_hash: Option<String>,
    pub username: Option<String>,
    pub phone_number: Option<String>,
    pub phone_verified: bool,
    pub role: Option<String>,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub two_factor_enabled: bool,
    pub is_anonymous: bool,
    pub last_login_method: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DefaultUser {
    pub fn new(email: impl Into<String>, password_hash: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.into(),
            name: None,
            image: None,
            email_verified: false,
            password_hash,
            username: None,
            phone_number: None,
            phone_verified: false,
            role: Some("user".into()),
            banned: false,
            ban_reason: None,
            two_factor_enabled: false,
            is_anonymous: false,
            last_login_method: None,
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn anonymous() -> Self {
        let mut u = Self::new(
            format!("anon-{}@anonymous.local", uuid::Uuid::new_v4()),
            None,
        );
        u.is_anonymous = true;
        u.email_verified = true;
        u
    }
}

impl AuthUser for DefaultUser {
    fn id(&self) -> &str {
        &self.id
    }
    fn email(&self) -> &str {
        &self.email
    }
    fn email_verified(&self) -> bool {
        self.email_verified
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }
    fn password_hash(&self) -> Option<&str> {
        self.password_hash.as_deref()
    }
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

/// Default session record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSession {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub impersonated_by: Option<String>,
    pub active_organization_id: Option<String>,
}

impl DefaultSession {
    pub fn new(user_id: impl Into<String>, expires_in_secs: u64) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.into(),
            token: crate::utils::generate_token(),
            expires_at: now + chrono::TimeDelta::seconds(expires_in_secs as i64),
            created_at: now,
            ip_address: None,
            user_agent: None,
            impersonated_by: None,
            active_organization_id: None,
        }
    }
}

impl AuthSession for DefaultSession {
    fn id(&self) -> &str {
        &self.id
    }
    fn user_id(&self) -> &str {
        &self.user_id
    }
    fn expires_at(&self) -> &DateTime<Utc> {
        &self.expires_at
    }
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

/// Default account record (OAuth / credential).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultAccount {
    pub id: String,
    pub user_id: String,
    pub provider_id: String,
    pub provider_account_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub access_token_expires_at: Option<DateTime<Utc>>,
    pub password: Option<String>,
}

impl DefaultAccount {
    pub fn new(
        user_id: impl Into<String>,
        provider_id: impl Into<String>,
        provider_account_id: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.into(),
            provider_id: provider_id.into(),
            provider_account_id: provider_account_id.into(),
            access_token: None,
            refresh_token: None,
            id_token: None,
            access_token_expires_at: None,
            password: None,
        }
    }

    pub fn credential(user_id: impl Into<String>, password_hash: String) -> Self {
        let uid = user_id.into();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: uid.clone(),
            provider_id: "credential".into(),
            provider_account_id: uid,
            access_token: None,
            refresh_token: None,
            id_token: None,
            access_token_expires_at: None,
            password: Some(password_hash),
        }
    }
}

impl AuthAccount for DefaultAccount {
    fn id(&self) -> &str {
        &self.id
    }
    fn user_id(&self) -> &str {
        &self.user_id
    }
    fn provider_id(&self) -> &str {
        &self.provider_id
    }
    fn provider_account_id(&self) -> &str {
        &self.provider_account_id
    }
}
