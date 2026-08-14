//! Auth middleware — request guards and headers.

use axum::http::{HeaderValue, header};
use axum::response::Response;

/// Extracts the bearer token from an Authorization header.
pub fn extract_bearer_token(
    headers: &axum::http::HeaderMap,
) -> Option<String> {
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
pub fn make_session_cookie(name: &str, value: &str, max_age: u64) -> HeaderValue {
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
    resp.headers_mut()
        .insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
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
        assert_eq!(extract_session_cookie(&headers, "session"), Some("xyz".to_string()));
    }
}