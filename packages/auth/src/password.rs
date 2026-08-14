//! Password hashing and verification.

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
    Argon2,
};

/// Hash a password using Argon2id.
pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("password hashing failed: {e}"))?
        .to_string();
    Ok(hash)
}

/// Verify a password against a stored Argon2 hash.
pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

/// Check whether a stored hash looks like an Argon2 hash.
pub fn is_argon2_hash(hash: &str) -> bool {
    hash.starts_with("$argon2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() -> anyhow::Result<()> {
        let hash = hash_password("correct horse battery staple")?;
        assert!(is_argon2_hash(&hash));
        assert!(verify_password("correct horse battery staple", &hash));
        assert!(!verify_password("wrong password", &hash));
        Ok(())
    }

    #[test]
    fn test_distinct_hashes() -> anyhow::Result<()> {
        let a = hash_password("same password")?;
        let b = hash_password("same password")?;
        // Argon2 uses random salts, so hashes differ.
        assert_ne!(a, b);
        Ok(())
    }
}