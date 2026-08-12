//! Social OAuth providers — trait + built-in provider registry.

use crate::config::OAuthProviderConfig;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Profile returned after a successful OAuth exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProfile {
    pub provider_account_id: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub name: Option<String>,
    pub image: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub raw: serde_json::Value,
}

/// A social OAuth / OIDC provider.
#[async_trait]
pub trait SocialProvider: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn authorization_url(&self, config: &OAuthProviderConfig, state: &str, redirect_uri: &str) -> String;
    fn token_url(&self) -> &str;
    fn userinfo_url(&self) -> Option<&str>;
    fn default_scopes(&self) -> &[&str];
    async fn exchange_code(
        &self,
        config: &OAuthProviderConfig,
        code: &str,
        redirect_uri: &str,
    ) -> anyhow::Result<OAuthProfile>;
}

/// Generic OAuth2 provider driven by static endpoints.
pub struct GenericOAuthProvider {
    pub id: &'static str,
    pub display_name: &'static str,
    pub auth_url: &'static str,
    pub token_url: &'static str,
    pub userinfo_url: Option<&'static str>,
    pub scopes: &'static [&'static str],
    pub profile_map: ProfileMap,
}

/// How to map userinfo JSON into OAuthProfile fields.
#[derive(Debug, Clone, Copy)]
pub struct ProfileMap {
    pub id_key: &'static str,
    pub email_key: &'static str,
    pub name_key: &'static str,
    pub image_key: &'static str,
    pub email_verified_key: Option<&'static str>,
}

impl Default for ProfileMap {
    fn default() -> Self {
        Self {
            id_key: "id",
            email_key: "email",
            name_key: "name",
            image_key: "picture",
            email_verified_key: Some("email_verified"),
        }
    }
}

#[async_trait]
impl SocialProvider for GenericOAuthProvider {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        self.display_name
    }
    fn authorization_url(&self, config: &OAuthProviderConfig, state: &str, redirect_uri: &str) -> String {
        let scopes = if config.scopes.is_empty() {
            self.scopes.join(" ")
        } else {
            config.scopes.join(" ")
        };
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.auth_url,
            urlencoding_encode(&config.client_id),
            urlencoding_encode(redirect_uri),
            urlencoding_encode(&scopes),
            urlencoding_encode(state),
        )
    }
    fn token_url(&self) -> &str {
        self.token_url
    }
    fn userinfo_url(&self) -> Option<&str> {
        self.userinfo_url
    }
    fn default_scopes(&self) -> &[&str] {
        self.scopes
    }

    async fn exchange_code(
        &self,
        config: &OAuthProviderConfig,
        code: &str,
        redirect_uri: &str,
    ) -> anyhow::Result<OAuthProfile> {
        let client = reqwest::Client::new();
        let token_resp: serde_json::Value = client
            .post(self.token_url)
            .form(&[
                ("client_id", config.client_id.as_str()),
                ("client_secret", config.client_secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", redirect_uri),
            ])
            .send()
            .await?
            .json()
            .await?;

        let access_token = token_resp
            .get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let refresh_token = token_resp
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let id_token = token_resp
            .get("id_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut raw = token_resp;
        if let (Some(url), Some(at)) = (self.userinfo_url, access_token.as_ref()) {
            let info: serde_json::Value = client
                .get(url)
                .bearer_auth(at)
                .send()
                .await?
                .json()
                .await?;
            raw = info;
        }

        let id = raw
            .get(self.profile_map.id_key)
            .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| v.as_i64().map(|n| n.to_string())))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let email = raw
            .get(self.profile_map.email_key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let name = raw
            .get(self.profile_map.name_key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let image = raw
            .get(self.profile_map.image_key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let email_verified = self
            .profile_map
            .email_verified_key
            .and_then(|k| raw.get(k).and_then(|v| v.as_bool()))
            .unwrap_or(email.is_some());

        Ok(OAuthProfile {
            provider_account_id: id,
            email,
            email_verified,
            name,
            image,
            access_token,
            refresh_token,
            id_token,
            raw,
        })
    }
}

fn urlencoding_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

macro_rules! provider {
    ($id:expr, $name:expr, $auth:expr, $token:expr, $userinfo:expr, $scopes:expr) => {
        GenericOAuthProvider {
            id: $id,
            display_name: $name,
            auth_url: $auth,
            token_url: $token,
            userinfo_url: $userinfo,
            scopes: $scopes,
            profile_map: ProfileMap::default(),
        }
    };
    ($id:expr, $name:expr, $auth:expr, $token:expr, $userinfo:expr, $scopes:expr, $map:expr) => {
        GenericOAuthProvider {
            id: $id,
            display_name: $name,
            auth_url: $auth,
            token_url: $token,
            userinfo_url: $userinfo,
            scopes: $scopes,
            profile_map: $map,
        }
    };
}

/// All built-in social providers (35, matching better-auth coverage).
pub fn all_providers() -> Vec<GenericOAuthProvider> {
    let github_map = ProfileMap {
        id_key: "id",
        email_key: "email",
        name_key: "name",
        image_key: "avatar_url",
        email_verified_key: None,
    };
    let discord_map = ProfileMap {
        id_key: "id",
        email_key: "email",
        name_key: "username",
        image_key: "avatar",
        email_verified_key: Some("verified"),
    };

    vec![
        provider!("apple", "Apple", "https://appleid.apple.com/auth/authorize", "https://appleid.apple.com/auth/token", None, &["name", "email"]),
        provider!("atlassian", "Atlassian", "https://auth.atlassian.com/authorize", "https://auth.atlassian.com/oauth/token", Some("https://api.atlassian.com/me"), &["read:me"]),
        provider!("cognito", "Amazon Cognito", "https://cognito-idp.amazonaws.com/oauth2/authorize", "https://cognito-idp.amazonaws.com/oauth2/token", Some("https://cognito-idp.amazonaws.com/oauth2/userInfo"), &["openid", "email", "profile"]),
        provider!("discord", "Discord", "https://discord.com/api/oauth2/authorize", "https://discord.com/api/oauth2/token", Some("https://discord.com/api/users/@me"), &["identify", "email"], discord_map),
        provider!("dropbox", "Dropbox", "https://www.dropbox.com/oauth2/authorize", "https://api.dropboxapi.com/oauth2/token", Some("https://api.dropboxapi.com/2/users/get_current_account"), &["account_info.read"]),
        provider!("facebook", "Facebook", "https://www.facebook.com/v18.0/dialog/oauth", "https://graph.facebook.com/v18.0/oauth/access_token", Some("https://graph.facebook.com/me?fields=id,name,email,picture"), &["email", "public_profile"]),
        provider!("figma", "Figma", "https://www.figma.com/oauth", "https://www.figma.com/api/oauth/token", Some("https://api.figma.com/v1/me"), &["file_read"]),
        provider!("github", "GitHub", "https://github.com/login/oauth/authorize", "https://github.com/login/oauth/access_token", Some("https://api.github.com/user"), &["read:user", "user:email"], github_map),
        provider!("gitlab", "GitLab", "https://gitlab.com/oauth/authorize", "https://gitlab.com/oauth/token", Some("https://gitlab.com/api/v4/user"), &["read_user"]),
        provider!("google", "Google", "https://accounts.google.com/o/oauth2/v2/auth", "https://oauth2.googleapis.com/token", Some("https://openidconnect.googleapis.com/v1/userinfo"), &["openid", "email", "profile"]),
        provider!("huggingface", "Hugging Face", "https://huggingface.co/oauth/authorize", "https://huggingface.co/oauth/token", Some("https://huggingface.co/oauth/userinfo"), &["openid", "profile", "email"]),
        provider!("kakao", "Kakao", "https://kauth.kakao.com/oauth/authorize", "https://kauth.kakao.com/oauth/token", Some("https://kapi.kakao.com/v2/user/me"), &["profile_nickname", "account_email"]),
        provider!("kick", "Kick", "https://id.kick.com/oauth/authorize", "https://id.kick.com/oauth/token", Some("https://api.kick.com/public/v1/users"), &["user:read"]),
        provider!("line", "LINE", "https://access.line.me/oauth2/v2.1/authorize", "https://api.line.me/oauth2/v2.1/token", Some("https://api.line.me/v2/profile"), &["profile", "openid", "email"]),
        provider!("linear", "Linear", "https://linear.app/oauth/authorize", "https://api.linear.app/oauth/token", Some("https://api.linear.app/graphql"), &["read"]),
        provider!("linkedin", "LinkedIn", "https://www.linkedin.com/oauth/v2/authorization", "https://www.linkedin.com/oauth/v2/accessToken", Some("https://api.linkedin.com/v2/userinfo"), &["openid", "profile", "email"]),
        provider!("microsoft", "Microsoft", "https://login.microsoftonline.com/common/oauth2/v2.0/authorize", "https://login.microsoftonline.com/common/oauth2/v2.0/token", Some("https://graph.microsoft.com/v1.0/me"), &["openid", "email", "profile", "User.Read"]),
        provider!("naver", "Naver", "https://nid.naver.com/oauth2.0/authorize", "https://nid.naver.com/oauth2.0/token", Some("https://openapi.naver.com/v1/nid/me"), &["profile"]),
        provider!("notion", "Notion", "https://api.notion.com/v1/oauth/authorize", "https://api.notion.com/v1/oauth/token", None, &[]),
        provider!("paybin", "Paybin", "https://paybin.io/oauth/authorize", "https://paybin.io/oauth/token", Some("https://paybin.io/oauth/userinfo"), &["openid", "email"]),
        provider!("paypal", "PayPal", "https://www.paypal.com/signin/authorize", "https://api.paypal.com/v1/oauth2/token", Some("https://api.paypal.com/v1/identity/oauth2/userinfo"), &["openid", "email"]),
        provider!("polar", "Polar", "https://polar.sh/oauth2/authorize", "https://api.polar.sh/v1/oauth2/token", Some("https://api.polar.sh/v1/oauth2/userinfo"), &["openid", "email", "profile"]),
        provider!("railway", "Railway", "https://backboard.railway.app/oauth/authorize", "https://backboard.railway.app/oauth/token", Some("https://backboard.railway.app/graphql"), &["openid"]),
        provider!("reddit", "Reddit", "https://www.reddit.com/api/v1/authorize", "https://www.reddit.com/api/v1/access_token", Some("https://oauth.reddit.com/api/v1/me"), &["identity"]),
        provider!("roblox", "Roblox", "https://apis.roblox.com/oauth/v1/authorize", "https://apis.roblox.com/oauth/v1/token", Some("https://apis.roblox.com/oauth/v1/userinfo"), &["openid", "profile"]),
        provider!("salesforce", "Salesforce", "https://login.salesforce.com/services/oauth2/authorize", "https://login.salesforce.com/services/oauth2/token", Some("https://login.salesforce.com/services/oauth2/userinfo"), &["openid", "email", "profile"]),
        provider!("slack", "Slack", "https://slack.com/oauth/v2/authorize", "https://slack.com/api/oauth.v2.access", Some("https://slack.com/api/users.identity"), &["identity.basic", "identity.email"]),
        provider!("spotify", "Spotify", "https://accounts.spotify.com/authorize", "https://accounts.spotify.com/api/token", Some("https://api.spotify.com/v1/me"), &["user-read-email", "user-read-private"]),
        provider!("tiktok", "TikTok", "https://www.tiktok.com/v2/auth/authorize", "https://open.tiktokapis.com/v2/oauth/token", Some("https://open.tiktokapis.com/v2/user/info"), &["user.info.basic"]),
        provider!("twitch", "Twitch", "https://id.twitch.tv/oauth2/authorize", "https://id.twitch.tv/oauth2/token", Some("https://api.twitch.tv/helix/users"), &["user:read:email"]),
        provider!("twitter", "Twitter / X", "https://twitter.com/i/oauth2/authorize", "https://api.twitter.com/2/oauth2/token", Some("https://api.twitter.com/2/users/me"), &["users.read", "tweet.read", "offline.access"]),
        provider!("vercel", "Vercel", "https://vercel.com/oauth/authorize", "https://api.vercel.com/v2/oauth/access_token", Some("https://api.vercel.com/v2/user"), &[]),
        provider!("vk", "VK", "https://oauth.vk.com/authorize", "https://oauth.vk.com/access_token", Some("https://api.vk.com/method/users.get"), &["email"]),
        provider!("wechat", "WeChat", "https://open.weixin.qq.com/connect/qrconnect", "https://api.weixin.qq.com/sns/oauth2/access_token", Some("https://api.weixin.qq.com/sns/userinfo"), &["snsapi_login"]),
        provider!("zoom", "Zoom", "https://zoom.us/oauth/authorize", "https://zoom.us/oauth/token", Some("https://api.zoom.us/v2/users/me"), &["user:read"]),
    ]
}

/// Lookup a provider by id.
pub fn get_provider(id: &str) -> Option<GenericOAuthProvider> {
    all_providers().into_iter().find(|p| p.id == id)
}

/// List provider ids.
pub fn list_provider_ids() -> Vec<&'static str> {
    all_providers().into_iter().map(|p| p.id).collect()
}

/// Build a map of configured providers from AuthConfig.
pub fn configured_providers(
    configs: &HashMap<String, OAuthProviderConfig>,
) -> Vec<(GenericOAuthProvider, OAuthProviderConfig)> {
    let mut out = Vec::new();
    for (id, cfg) in configs {
        if let Some(p) = get_provider(id) {
            out.push((p, cfg.clone()));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thirty_five_providers() {
        assert_eq!(all_providers().len(), 35);
    }

    #[test]
    fn github_exists() {
        assert!(get_provider("github").is_some());
        assert!(get_provider("google").is_some());
    }
}
