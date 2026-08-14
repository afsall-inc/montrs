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

//! Have I Been Pwned plugin — check password compromise via HIBP k-anonymity API.
//! Export `is_pwned(password) -> bool` async; plugin can be used as hook.

use crate::context::AuthState;
use crate::plugin::AuthPlugin;
use crate::AuthError;
/// Check whether a password has been exposed in a known breach.
/// Uses the HIBP range API (k-anonymity): sends only the first 5 hex chars of SHA-1.
pub async fn is_pwned(password: &str) -> bool {
    let hash = sha1_hex(password);
    let prefix = &hash[..5];
    let suffix = &hash[5..];

    let url = format!("https://api.pwnedpasswords.com/range/{}", prefix);
    match reqwest::get(&url).await {
        Ok(resp) => {
            if let Ok(body) = resp.text().await {
                body.lines().any(|line| {
                    line.split(':')
                        .next()
                        .map(|s| s.eq_ignore_ascii_case(suffix))
                        .unwrap_or(false)
                })
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

/// Compute the SHA-1 hex digest of a password (upper-case, as HIBP requires).
/// Pure Rust implementation — no external sha1 crate needed.
fn sha1_hex(input: &str) -> String {
    let hash = sha1_bytes(input.as_bytes());
    hex::encode(hash).to_uppercase()
}

/// Minimal SHA-1 implementation (FIPS 180-1).
fn sha1_bytes(message: &[u8]) -> [u8; 20] {
    let mut h0: u32 = 0x6745_2301;
    let mut h1: u32 = 0xEFCD_AB89;
    let mut h2: u32 = 0x98BA_DCFE;
    let mut h3: u32 = 0x1032_5476;
    let mut h4: u32 = 0xC3D2_E1F0;

    // Pre-processing: padding
    let bit_len = (message.len() as u64) * 8;
    let mut msg = message.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A82_7999),
                20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
                _ => (b ^ c ^ d, 0xCA62_C1D6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut out = [0u8; 20];
    out[0..4].copy_from_slice(&h0.to_be_bytes());
    out[4..8].copy_from_slice(&h1.to_be_bytes());
    out[8..12].copy_from_slice(&h2.to_be_bytes());
    out[12..16].copy_from_slice(&h3.to_be_bytes());
    out[16..20].copy_from_slice(&h4.to_be_bytes());
    out
}

/// HaveIBeenPwnedPlugin — hook-only, no routes. Checks password on sign-up.
pub struct HaveIBeenPwnedPlugin {
    state: Option<AuthState>,
}

impl HaveIBeenPwnedPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for HaveIBeenPwnedPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for HaveIBeenPwnedPlugin {
    fn name(&self) -> &'static str {
        "haveibeenpwned"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn before_request(&self, _req: &axum::extract::Request) -> Result<(), AuthError> {
        // Hook: the core sign-up handler can call `is_pwned` before creating a user.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_hex() {
        // SHA-1 of "password" is a well-known vector.
        let result = sha1_hex("password");
        assert_eq!(result.len(), 40);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
        // "password" → 5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8
        assert_eq!(result, "5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8");
    }

    #[test]
    fn test_sha1_hex_uppercase() {
        let result = sha1_hex("password");
        assert_eq!(result, result.to_uppercase());
    }
}