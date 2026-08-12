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