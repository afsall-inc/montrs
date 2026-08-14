//! Invariant tests for montrs-sigstore.

use montrs_sigstore::*;
use std::path::Path;

#[test]
fn test_artifact_ref_from_digest() {
    let ar = ArtifactRef::from_digest("abc123");
    assert_eq!(ar.digest, "sha256:abc123");
}

#[test]
fn test_artifact_ref_from_full_digest() {
    let ar = ArtifactRef::from_digest("sha256:abc");
    assert_eq!(ar.digest, "sha256:abc");
}

#[test]
fn test_slsa_artifact_from_bytes() {
    let art = SlsaArtifact::from_bytes("file".to_string(), b"hello world");
    assert_eq!(art.name, "file");
    assert_eq!(art.sha256.len(), 64); // hex of 32 bytes
}

#[test]
fn test_retry_config_default() {
    let cfg = RetryConfig::default();
    assert_eq!(cfg.retries, 3);
    assert!(!cfg.timeout.is_zero());
}

#[test]
fn test_attestation_client_builder() {
    let client = AttestationClient::builder()
        .base_url("https://api.github.com")
        .github_token("test-token")
        .build()
        .unwrap();
    assert_eq!(client.base_url, "https://api.github.com");
    assert_eq!(client.github_token.as_deref(), Some("test-token"));
}

#[test]
fn test_github_source_config() {
    let source = GitHubSource::new("owner", "repo", None).unwrap();
    assert_eq!(source.owner, "owner");
    assert_eq!(source.repo, "repo");
}

#[test]
fn test_verify_functions_accept_paths() {
    // These verify functions read files; test they return errors for missing files
    // rather than panicking.
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(verify_cosign_signature(
        Path::new("/nonexistent/artifact"),
        Path::new("/nonexistent/sig"),
    ));
    assert!(result.is_err());
}

#[test]
fn test_github_attestation_missing_file() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(verify_github_attestation(
        Path::new("/nonexistent/artifact"),
        "owner",
        "repo",
        None,
        None,
        RetryConfig::default(),
    ));
    assert!(result.is_err());
}
