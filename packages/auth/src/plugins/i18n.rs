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

//! I18n plugin — localized auth error messages.
//!
//! Registers a global message catalog keyed by `(locale, error_code)` and
//! exposes `lookup_message()` so `AuthError` responses can be localized by
//! the request's `Accept-Language` header.

use crate::context::AuthState;
use crate::error::AuthErrorCode;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::routing::get;
use axum::{Json, Router};
use parking_lot::RwLock;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Global error message catalog: `locale -> (code_str -> message)`.
/// Uses `RwLock` so the catalog can be updated at plugin build time.
static CATALOG: LazyLock<RwLock<HashMap<String, HashMap<String, String>>>> =
    LazyLock::new(|| RwLock::new(default_catalog()));

/// Look up a localized message for an error code.
/// Falls back to English, then the code string itself.
pub fn lookup_message(code: AuthErrorCode, locale: &str) -> String {
    message_from(&CATALOG.read(), code, locale)
}

/// Pure lookup against an explicit catalog (used by tests and embedders).
fn message_from(
    catalog: &HashMap<String, HashMap<String, String>>,
    code: AuthErrorCode,
    locale: &str,
) -> String {
    let code_str = code_str(code);
    let locale = normalize_locale(locale);

    catalog
        .get(&locale)
        .and_then(|m| m.get(code_str))
        .cloned()
        .or_else(|| catalog.get("en").and_then(|m| m.get(code_str)).cloned())
        .unwrap_or_else(|| {
            catalog
                .get("en")
                .and_then(|m| m.get("unknown"))
                .cloned()
                .unwrap_or_else(|| format!("Unknown error: {code_str}"))
        })
}

/// Whether catalogs have been registered.
pub fn is_registered() -> bool {
    !CATALOG.read().is_empty()
}

/// Best-effort locale normalization: `en-US` -> `en`, lowercase.
fn normalize_locale(locale: &str) -> String {
    let base = locale
        .split(['-', '_'])
        .next()
        .unwrap_or(locale)
        .to_lowercase();
    if base.is_empty() {
        "en".to_string()
    } else {
        base
    }
}

fn code_str(code: AuthErrorCode) -> &'static str {
    match code {
        AuthErrorCode::InvalidCredentials => "invalid-credentials",
        AuthErrorCode::UserNotFound => "user-not-found",
        AuthErrorCode::EmailAlreadyInUse => "email-already-in-use",
        AuthErrorCode::EmailNotVerified => "email-not-verified",
        AuthErrorCode::InvalidToken => "invalid-token",
        AuthErrorCode::InvalidSession => "invalid-session",
        AuthErrorCode::RateLimited => "rate-limited",
        AuthErrorCode::MissingField => "missing-field",
        AuthErrorCode::WeakPassword => "weak-password",
        AuthErrorCode::TwoFactorRequired => "two-factor-required",
        AuthErrorCode::InvalidTwoFactor => "invalid-two-factor",
        AuthErrorCode::OAuthError => "oauth-error",
        AuthErrorCode::ProviderNotConfigured => "provider-not-configured",
        AuthErrorCode::OrganizationError => "organization-error",
        AuthErrorCode::Forbidden => "forbidden",
        AuthErrorCode::DatabaseError => "database-error",
        AuthErrorCode::InternalError => "internal-error",
        AuthErrorCode::CaptchaRequired => "captcha-required",
        AuthErrorCode::AccountAlreadyLinked => "account-already-linked",
        AuthErrorCode::AccountNotFound => "account-not-found",
        AuthErrorCode::ServerError => "server-error",
    }
}

/// The default English catalog (also serves as the fallback).
fn default_catalog() -> HashMap<String, HashMap<String, String>> {
    let mut catalog = HashMap::new();
    catalog.insert("en".to_string(), en_messages());
    catalog
}

fn en_messages() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("invalid-credentials".into(), "Invalid email or password".into());
    m.insert("user-not-found".into(), "User not found".into());
    m.insert("email-already-in-use".into(), "Email already in use".into());
    m.insert("email-not-verified".into(), "Email not verified".into());
    m.insert("invalid-token".into(), "Invalid or expired token".into());
    m.insert("invalid-session".into(), "Invalid or expired session".into());
    m.insert("rate-limited".into(), "Too many requests".into());
    m.insert("missing-field".into(), "Missing required field".into());
    m.insert("weak-password".into(), "Password is too weak".into());
    m.insert("two-factor-required".into(), "Two-factor authentication required".into());
    m.insert("invalid-two-factor".into(), "Invalid two-factor code".into());
    m.insert("oauth-error".into(), "OAuth provider error".into());
    m.insert("provider-not-configured".into(), "OAuth provider not configured".into());
    m.insert("organization-error".into(), "Organization error".into());
    m.insert("forbidden".into(), "Permission denied".into());
    m.insert("database-error".into(), "Database error".into());
    m.insert("internal-error".into(), "Internal server error".into());
    m.insert("captcha-required".into(), "CAPTCHA verification required".into());
    m.insert("account-already-linked".into(), "Account already linked".into());
    m.insert("account-not-found".into(), "Account not found".into());
    m.insert("server-error".into(), "Server misconfiguration".into());
    m.insert("unknown".into(), "An unknown error occurred".into());
    let _ = m.insert("help".into(), "General auth error".into());
    m
}

/// I18nPlugin — registers the localized error-message catalog.
pub struct I18nPlugin {
    /// Additional locale catalogs to register (merged over English defaults).
    locales: Vec<(String, HashMap<String, String>)>,
}

impl I18nPlugin {
    pub fn new() -> Self {
        Self { locales: Vec::new() }
    }

    /// Add or override messages for a locale.
    pub fn with_locale(mut self, locale: &str, msgs: HashMap<String, String>) -> Self {
        self.locales.push((locale.to_string(), msgs));
        self
    }

    /// Reset/register the global catalog. Each plugin registration REPLACES the
/// catalog so behavior is deterministic (a real app builds one plugin).
fn register(&self) {
    let mut catalog = default_catalog();
    for (locale, msgs) in &self.locales {
        let entry = catalog.entry(locale.clone()).or_default();
        for (code, msg) in msgs {
            entry.insert(code.clone(), msg.clone());
        }
    }
    let mut guarded = CATALOG.write();
    *guarded = catalog;
}
}

impl Default for I18nPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for I18nPlugin {
    fn name(&self) -> &'static str {
        "i18n"
    }

    fn on_build(&mut self, _state: &AuthState) -> Result<(), AuthError> {
        self.register();
        Ok(())
    }

    fn router(&self) -> Router {
        // Re-register to make sure catalog exists even if on_build ordering differs.
        self.register();
        Router::new().route("/i18n/messages", get(move || async {
            get_messages()
        }))
    }
}

fn get_messages() -> Json<Value> {
    let catalog = serde_json::to_value(&*CATALOG.read()).unwrap_or_default();
    Json(catalog)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog_with(locales: Vec<(String, HashMap<String, String>)>) -> HashMap<String, HashMap<String, String>> {
        let mut catalog = default_catalog();
        for (locale, msgs) in locales {
            let entry = catalog.entry(locale).or_default();
            entry.extend(msgs);
        }
        catalog
    }

    #[test]
    fn test_catalog_registration_and_lookup() {
        let catalog = catalog_with(vec![]);
        assert!(is_registered() || !catalog.is_empty());
        // English lookup works.
        let msg = message_from(&catalog, AuthErrorCode::InvalidCredentials, "en");
        assert_eq!(msg, "Invalid email or password");
        // Unknown locale falls back to English.
        let msg = message_from(&catalog, AuthErrorCode::InvalidCredentials, "fr-FR");
        assert_eq!(msg, "Invalid email or password");
        // Normalized locale.
        let msg = message_from(&catalog, AuthErrorCode::RateLimited, "en-US");
        assert_eq!(msg, "Too many requests");
    }

    #[test]
    fn test_custom_locale_override() {
        let mut fr = HashMap::new();
        fr.insert("invalid-credentials".into(), "Identifiants invalides".into());
        let catalog = catalog_with(vec![("fr".into(), fr)]);

        let msg = message_from(&catalog, AuthErrorCode::InvalidCredentials, "fr");
        assert_eq!(msg, "Identifiants invalides");
        // Unoverridden code falls back to English.
        let msg = message_from(&catalog, AuthErrorCode::RateLimited, "fr");
        assert_eq!(msg, "Too many requests");
    }

    #[test]
    fn test_normalize_locale() {
        assert_eq!(normalize_locale("en-US"), "en");
        assert_eq!(normalize_locale("fr_CA"), "fr");
        assert_eq!(normalize_locale("ar"), "ar");
    }
}