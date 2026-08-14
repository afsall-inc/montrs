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