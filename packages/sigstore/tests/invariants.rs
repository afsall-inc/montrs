// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
