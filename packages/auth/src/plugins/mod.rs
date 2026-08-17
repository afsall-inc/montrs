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

//! Auth plugins — modular features implementing [`AuthPlugin`].
//!
//! Enable only what you need via [`crate::AuthBuilder::plugin`].

pub mod access;
pub mod admin;
pub mod agent_auth;
pub mod anonymous;
pub mod api_key;
pub mod bearer;
pub mod captcha;
pub mod custom_session;
pub mod device_authorization;
pub mod email_otp;
pub mod generic_oauth;
pub mod haveibeenpwned;
pub mod i18n;
pub mod jwt;
pub mod last_login_method;
pub mod magic_link;
pub mod mcp;
pub mod multi_session;
pub mod oauth_popup;
pub mod oauth_provider;
pub mod oauth_proxy;
pub mod one_tap;
pub mod one_time_token;
pub mod open_api;
pub mod organization;
pub mod passkey;
pub mod phone_number;
pub mod scim;
pub mod siwe;
pub mod sso;
pub mod stripe;
pub mod two_factor;
pub mod username;

pub use access::AccessPlugin;
pub use admin::AdminPlugin;
pub use agent_auth::AgentAuthPlugin;
pub use anonymous::AnonymousPlugin;
pub use api_key::ApiKeyPlugin;
pub use bearer::BearerPlugin;
pub use captcha::CaptchaPlugin;
pub use custom_session::CustomSessionPlugin;
pub use device_authorization::DeviceAuthorizationPlugin;
pub use email_otp::EmailOtpPlugin;
pub use generic_oauth::GenericOAuthPlugin;
pub use haveibeenpwned::HaveIBeenPwnedPlugin;
pub use i18n::I18nPlugin;
pub use jwt::JwtPlugin;
pub use last_login_method::LastLoginMethodPlugin;
pub use magic_link::MagicLinkPlugin;
pub use mcp::McpPlugin;
pub use multi_session::MultiSessionPlugin;
pub use oauth_popup::OAuthPopupPlugin;
pub use oauth_provider::OAuthProviderPlugin;
pub use oauth_proxy::OAuthProxyPlugin;
pub use one_tap::OneTapPlugin;
pub use one_time_token::OneTimeTokenPlugin;
pub use open_api::OpenApiPlugin;
pub use organization::OrganizationPlugin;
pub use passkey::PasskeyPlugin;
pub use phone_number::PhoneNumberPlugin;
pub use scim::ScimPlugin;
pub use siwe::SiwePlugin;
pub use sso::SsoPlugin;
pub use stripe::StripePlugin;
pub use two_factor::TwoFactorPlugin;
pub use username::UsernamePlugin;
