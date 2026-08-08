# montrs-sigstore

Signature verification for MontRS — cosign, SLSA, and GitHub attestations.

## Features

- **Cosign verification**: Verify cosign signatures (modern bundle format and legacy v1)
- **Cosign key verification**: Verify with a specific public key
- **SLSA provenance**: Verify SLSA provenance attestations
- **GitHub attestations**: OIDC-based attestation verification via GitHub API
- **AttestationClient**: HTTP client with retry/backoff for GitHub API
- **AttestationSource trait**: Pluggable attestation sources

## Usage

```rust
use montrs_sigstore::{verify_github_attestation, RetryConfig};

let result = verify_github_attestation(
    artifact_path, "owner", "repo", token, signer_workflow, RetryConfig::default()
).await?;
```