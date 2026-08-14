//! Utility functions for the auth system.

use base64::{Engine as _, engine::general_purpose};

/// Generate a cryptographically random token string.
pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::Rng::fill(&mut rand::thread_rng(), &mut bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Generate a numeric OTP code of the given length.
pub fn generate_otp(length: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}

/// Time-based one-time password (TOTP) utilities.
pub mod totp {
    use totp_rs::{TOTP, Algorithm};

    /// Generate a new TOTP secret (raw bytes).
    pub fn generate_secret() -> Vec<u8> {
        use rand::Rng;
        let mut bytes = [0u8; 20];
        rand::thread_rng().fill(&mut bytes);
        bytes.to_vec()
    }

    /// Build a TOTP instance from raw secret bytes.
    pub fn from_secret(secret: &[u8]) -> TOTP {
        TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_vec(),
            None,
            String::new(),
        )
        .expect("TOTP creation should not fail")
    }

    /// Generate a provisioning URI for authenticator apps.
    pub fn provisioning_uri(secret: &[u8], email: &str, issuer: &str) -> String {
        let base32_secret = base32_encode(secret);
        format!(
            "otpauth://totp/{issuer}:{email}?secret={base32_secret}&issuer={issuer}&algorithm=SHA1&digits=6&period=30"
        )
    }

    /// Verify a TOTP code.
    pub fn verify_code(secret: &[u8], code: &str) -> bool {
        let totp = from_secret(secret);
        let now = chrono::Utc::now().timestamp() as u64;
        for offset in 0..=1 {
            let time = now + offset * 30;
            if totp.generate(time) == code {
                return true;
            }
        }
        false
    }

    pub fn base32_encode(bytes: &[u8]) -> String {
        const BASE32: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut result = String::new();
        let mut buffer = 0u64;
        let mut bits = 0;

        for &byte in bytes {
            buffer = (buffer << 8) | (byte as u64);
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                let idx = ((buffer >> bits) & 0x1F) as usize;
                result.push(BASE32[idx] as char);
            }
        }
        if bits > 0 {
            buffer <<= 5 - bits;
            let idx = (buffer & 0x1F) as usize;
            result.push(BASE32[idx] as char);
        }
        // Pad to multiple of 8.
        while result.len() % 8 != 0 {
            result.push('=');
        }
        result
    }
}

/// JWT token utilities.
pub mod jwt {
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
    use serde::{Deserialize, Serialize};

    /// Standard JWT claims.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub sub: String,
        pub exp: usize,
        pub iat: usize,
    }

    /// Create a signed JWT token.
    pub fn create_token(sub: &str, secret: &str, expires_in_secs: u64) -> anyhow::Result<String> {
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: sub.to_string(),
            exp: now + expires_in_secs as usize,
            iat: now,
        };
        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?)
    }

    /// Verify and decode a JWT token.
    pub fn verify_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let a = generate_token();
        let b = generate_token();
        assert_eq!(a.len(), 43);
        assert_ne!(a, b);
    }

    #[test]
    fn test_jwt_roundtrip() -> anyhow::Result<()> {
        let token = jwt::create_token("user123", "my-secret-key", 3600)?;
        let claims = jwt::verify_token(&token, "my-secret-key")?;
        assert_eq!(claims.sub, "user123");
        Ok(())
    }

    #[test]
    fn test_totp_verify() {
        let secret = totp::generate_secret();
        let totp = totp::from_secret(&secret);
        let code = totp.generate(chrono::Utc::now().timestamp() as u64);
        assert!(totp::verify_code(&secret, &code));
    }

    #[test]
    fn test_base32_encode() {
        // Test vector: "foobar" -> base32
        let encoded = totp::base32_encode(b"foobar");
        assert_eq!(encoded.len() % 8, 0, "base32 must be padded to multiple of 8");
        assert!(encoded.ends_with("="), "base32 must be padded");
    }
}