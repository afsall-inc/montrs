# montrs-sigstore — Invariants

## 1. Responsibility
Verify cryptographic signatures on artifacts using cosign, SLSA, and GitHub attestations.

## 2. Invariants
- **No sigstore-verify dependency**: Bundle verification is done manually to avoid heavy deps. Full sigstore-verify integration is optional.
- **Retry/backoff**: All HTTP calls use exponential backoff with jitter and Retry-After header support.
- **Platform-independent**: All verification works on any platform where Rust compiles.
- **Async-first**: All verification functions are async.

## 3. Boundary
- **In-Scope**: Cosign signature verification, SLSA provenance, GitHub attestation API.
- **Out-of-Scope**: Key generation, signing, certificate management, TUF root management.

## 4. Agent Guidelines
- Use `verify_github_attestation()` for GitHub-signed artifacts.
- Use `verify_cosign_signature()` for cosign-signed artifacts.
- Use `verify_slsa_provenance()` for SLSA provenance.