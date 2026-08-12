//! Shared verification token store helpers (OTP, magic link, email verify).

use crate::database::{DatabaseAdapter, VerificationRecord};
use crate::utils::generate_token;
use chrono::{Duration, Utc};
use uuid::Uuid;

/// Create a verification record and return its raw token value.
pub async fn create_verification(
    db: &dyn DatabaseAdapter,
    identifier: impl Into<String>,
    value: Option<String>,
    expires_in_secs: i64,
) -> anyhow::Result<VerificationRecord> {
    let token = value.unwrap_or_else(generate_token);
    let record = VerificationRecord {
        id: Uuid::new_v4().to_string(),
        identifier: identifier.into(),
        value: token,
        expires_at: Utc::now() + Duration::seconds(expires_in_secs),
        created_at: Utc::now(),
    };
    db.create_verification(&record).await?;
    Ok(record)
}

/// Consume a verification by identifier + value. Deletes on success.
pub async fn consume_verification(
    db: &dyn DatabaseAdapter,
    identifier: &str,
    value: &str,
) -> anyhow::Result<VerificationRecord> {
    let Some(rec) = db.find_verification(identifier, value).await? else {
        return Err(crate::AuthError::invalid_token().into());
    };
    if rec.expires_at <= Utc::now() {
        let _ = db.delete_verification(&rec.id).await;
        return Err(crate::AuthError::invalid_token().into());
    }
    db.delete_verification(&rec.id).await?;
    Ok(rec)
}

/// Consume a verification by token value only (looks up identifier).
pub async fn consume_verification_by_value(
    db: &dyn DatabaseAdapter,
    value: &str,
) -> anyhow::Result<VerificationRecord> {
    let Some(rec) = db.find_verification_by_value(value).await? else {
        return Err(crate::AuthError::invalid_token().into());
    };
    if rec.expires_at <= Utc::now() {
        let _ = db.delete_verification(&rec.id).await;
        return Err(crate::AuthError::invalid_token().into());
    }
    db.delete_verification(&rec.id).await?;
    Ok(rec)
}

/// Create a numeric OTP verification (for email-otp / phone).
pub async fn create_otp(
    db: &dyn DatabaseAdapter,
    identifier: impl Into<String>,
    length: usize,
    expires_in_secs: i64,
) -> anyhow::Result<VerificationRecord> {
    let otp = crate::utils::generate_otp(length);
    create_verification(db, identifier, Some(otp), expires_in_secs).await
}
