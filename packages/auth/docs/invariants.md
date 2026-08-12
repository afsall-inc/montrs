# montrs-auth — Invariants

- **Layer**: 2 (depends on orm, sigstore, i18n)
- **Architecture**: Plugin-based; core always-on routes + 32 optional plugins
- **Core**: `MontrsAuth` builder, `AuthConfig`, `AuthState`, `DatabaseAdapter`, `SessionManager`
- **Password hashing**: Argon2id via argon2 crate
- **Sessions**: Database-backed with configurable expiry
- **JWT**: jsonwebtoken crate for token signing/verification
- **TOTP**: totp-rs crate for 2FA
- **Verification**: Shared `VerificationRecord` store for OTPs, magic links, resets
- **Email**: `EmailProvider` trait + `ConsoleEmailProvider` for development
- **Database**: `MemoryDatabaseAdapter` for dev, `DatabaseAdapter` trait for production
- **Error codes**: Stable `AuthErrorCode` enum (20+ variants)
- **Security**: CSRF check, bearer token extraction, session cookies, rate limiting
- **Social providers**: 35 built-in (GitHub, Google, Apple, etc.) via `SocialProvider` trait
- **Plugins**: 32 feature-gated modules under `plugins/` implementing `AuthPlugin`
- **Naming**: No external product names in types; this is montrs-auth

## Plugin catalog

| Plugin | Feature | Endpoints |
|--------|---------|-----------|
| access | RBAC helpers | none (pure library) |
| admin | User management | 7 admin routes |
| agent_auth | AI agent auth | register, token, capability |
| anonymous | Guest sessions | sign-in-anonymous, delete-anonymous-user |
| api_key | API key CRUD | 6 api-key routes |
| bearer | Bearer token | hook-only |
| captcha | Captcha verification | POST /captcha/verify |
| custom_session | Custom session shape | GET /custom-session/get |
| device_authorization | OAuth Device Grant | 4 device routes |
| email_otp | Email OTP flow | 6 email-otp routes |
| generic_oauth | Any OAuth provider | sign-in/oauth2, callback |
| haveibeenpwned | HIBP check | pure function |
| i18n | Error i18n | GET /i18n/messages |
| jwt | JWT issuance | /token, /jwks |
| last_login_method | Track login method | hook-only |
| magic_link | Passwordless email | sign-in-magic-link, verify |
| mcp | MCP OAuth AS | MCP well-known + authorize/token |
| multi_session | Multi-device sessions | 3 multi-session routes |
| oauth_popup | Popup helper | /oauth-popup/start |
| oauth_provider | OIDC Provider | full OIDC AS |
| oauth_proxy | Proxy callback | /oauth-proxy-callback |
| one_tap | Google One Tap | /one-tap/callback |
| one_time_token | Single-use token | generate, verify |
| open_api | OpenAPI schema | GET /reference |
| organization | Orgs & teams | 8 org routes (CRUD + invite + members) |
| passkey | WebAuthn | 6 passkey routes (scaffold) |
| phone_number | Phone OTP | send-otp, sign-in-phone, verify |
| scim | SCIM 2.0 | /scim/v2/Users CRUD |
| siwe | Ethereum sign-in | /siwe/nonce, /siwe/verify |
| sso | Enterprise SSO | OIDC/SAML provider config |
| stripe | Stripe billing | subscription + webhook |
| two_factor | 2FA (TOTP/OTP) | 8 two-factor routes |
| username | Username auth | sign-in-username, available |

## Core always-on routes

| Endpoint | Method | Purpose |
|----------|--------|---------|
| /sign-up/email | POST | Email + password sign-up |
| /sign-in/email | POST | Email + password sign-in |
| /change-password | POST | Authenticated password change |
| /set-password | POST | Set password for social-only accounts |
| /verify-password | POST | Verify current password |
| /get-session | GET | Get current session + user |
| /list-sessions | POST | List user sessions |
| /revoke-session | POST | Revoke session by token |
| /revoke-other-sessions | POST | Revoke all other sessions |
| /sign-out | POST | Revoke current session |
| /forget-password | POST | Request password reset |
| /reset-password | POST | Complete password reset |
| /send-verification-email | POST | Send email verification |
| /verify-email | GET | Verify email with token |
| /update-user | POST | Update user profile |
| /change-email | POST | Change user email |
| /delete-user | POST | Delete user account |
| /sign-in/social | POST | Initiate OAuth sign-in |
| /callback/:provider | GET | OAuth callback |
| /link-social | POST | Link social account |
| /unlink-account | POST | Unlink social account |
| /list-accounts | GET | List linked accounts |
| /get-access-token | POST | Get OAuth access token |
| /ok | GET | Health check |
| /error | POST | Error test endpoint |