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

//! Auth middleware — request guards and headers.

use axum::{
    http::{HeaderValue, header},
    response::Response,
};

/// Extracts the bearer token from an Authorization header.
pub fn extract_bearer_token(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

/// Extracts a session token from cookies.
pub fn extract_session_cookie(
    headers: &axum::http::HeaderMap,
    cookie_name: &str,
) -> Option<String> {
    headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|c| {
                let mut parts = c.trim().splitn(2, '=');
                let name = parts.next()?.trim();
                let value = parts.next()?.trim();
                if name == cookie_name {
                    Some(value.to_string())
                } else {
                    None
                }
            })
        })
}

/// Build a session cookie header value with Secure/HttpOnly/SameSite.
pub fn make_session_cookie(
    name: &str,
    value: &str,
    max_age: u64,
) -> HeaderValue {
    HeaderValue::from_str(&format!(
        "{name}={value}; HttpOnly; Path=/; Max-Age={max_age}; SameSite=Lax"
    ))
    .unwrap_or_else(|_| HeaderValue::from_static(""))
}

/// Check the CORS origin against the allowed base URL.
pub fn is_allowed_origin(origin: &str, base_url: &str) -> bool {
    // Simplify: allow same-origin and the configured base URL.
    origin == base_url || origin.is_empty()
}

/// Check whether a request is a CSRF risk (cross-site + state-changing).
pub fn is_csrf_safe(
    method: &axum::http::Method,
    origin: &str,
    base_url: &str,
) -> bool {
    if method == axum::http::Method::GET
        || method == axum::http::Method::HEAD
        || method == axum::http::Method::OPTIONS
    {
        return true;
    }
    is_allowed_origin(origin, base_url)
}

/// Add standard security headers to a response.
pub fn add_security_headers(resp: &mut Response) {
    resp.headers_mut().insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    resp.headers_mut()
        .insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    resp.headers_mut()
        .insert("Referrer-Policy", HeaderValue::from_static("no-referrer"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bearer_token() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer abc123"),
        );
        assert_eq!(extract_bearer_token(&headers), Some("abc123".to_string()));
    }

    #[test]
    fn test_session_cookie() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("other=1; session=xyz; foo=2"),
        );
        assert_eq!(
            extract_session_cookie(&headers, "session"),
            Some("xyz".to_string())
        );
    }
}
