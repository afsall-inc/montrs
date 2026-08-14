//! TLS certificate generation for the proxy.

use rcgen::{CertificateParams, KeyPair};
use std::path::Path;

/// Generate a self-signed certificate for localhost development.
pub fn generate_self_signed() -> anyhow::Result<(String, String)> {
    let params = CertificateParams::new(vec!["localhost".to_string(), "127.0.0.1".to_string()])?;
    let key_pair = KeyPair::generate()?;
    let cert = params.self_signed(&key_pair)?;

    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();

    Ok((cert_pem, key_pem))
}

/// Write cert and key to files.
pub fn write_cert_files(
    cert_path: impl AsRef<Path>,
    key_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let (cert_pem, key_pem) = generate_self_signed()?;
    std::fs::write(cert_path.as_ref(), cert_pem)?;
    std::fs::write(key_path.as_ref(), key_pem)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cert() -> anyhow::Result<()> {
        let (cert, key) = generate_self_signed()?;
        assert!(cert.starts_with("-----BEGIN CERTIFICATE-----"));
        assert!(key.starts_with("-----BEGIN PRIVATE KEY-----"));
        Ok(())
    }
}