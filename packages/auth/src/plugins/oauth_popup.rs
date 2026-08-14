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

//! OAuth Popup plugin — DX helper: callback route for OAuth popup flows.
//! GET /oauth/popup/callback — returns a small HTML page that posts a message
//! to the opener window and closes the popup.

use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::Query;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

/// OAuth Popup plugin — small DX helper for popup-based OAuth flows.
pub struct OAuthPopupPlugin;

impl OAuthPopupPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OAuthPopupPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPlugin for OAuthPopupPlugin {
    fn name(&self) -> &'static str {
        "oauth_popup"
    }

    fn router(&self) -> Router {
        Router::new().route("/oauth/popup/callback", get(popup_callback))
    }
}

#[derive(Debug, Deserialize)]
pub struct PopupCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub provider: Option<String>,
}

async fn popup_callback(
    Query(q): Query<PopupCallbackQuery>,
) -> Result<Html<String>, AuthError> {
    let provider = q.provider.as_deref().unwrap_or("unknown");
    let code = q.code.as_deref().unwrap_or("");
    let error = q.error.as_deref().unwrap_or("");

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head><title>Sign in with {provider}</title></head>
<body>
<script>
(function() {{
    const payload = {{
        provider: "{provider}",
        code: "{code}",
        error: "{error}",
    }};
    if (window.opener) {{
        window.opener.postMessage(payload, "*");
        window.close();
    }} else {{
        document.body.textContent = JSON.stringify(payload);
    }}
}})();
</script>
<p>Signing you in with {provider}...</p>
</body>
</html>"#
    );

    Ok(Html(html))
}