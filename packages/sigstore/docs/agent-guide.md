# montrs-sigstore — Agent Guide

## Overview
Verifies cryptographic signatures on downloaded artifacts before installation. Supports cosign, SLSA, and GitHub attestation verification.

## Key Concepts
- **AttestationClient**: HTTP client with retry/backoff for GitHub API
- **AttestationSource trait**: Pluggable source for fetching attestations
- **GitHubSource**: Fetches attestations from GitHub's attestations API
- **SlsaArtifact**: An artifact with its SHA256 digest for SLSA verification

## Agent Usage
- Fetch attestations with `GitHubSource::new(owner, repo, token)`
- Verify with `verify_github_attestation(artifact_path, owner, repo, token, workflow, retry_config)`

## Local Invariants
Read `docs/invariants.md` before modifying.