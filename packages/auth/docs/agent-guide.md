# Agent Guide: montrs-auth

## Core Concepts
MontRS authentication system supporting email/password, OAuth, 2FA, sessions, and RBAC.

### Authentication Providers
- **Email/Password** — standard credential-based login with hashed passwords.
- **OAuth** — third-party provider integration (Google, GitHub, etc.).
- **2FA** — time-based one-time password (TOTP) support.

### Session Management
- Sessions are tracked via secure tokens stored in configurable backends.
- Session expiration and refresh are handled automatically.
- Use `AuthContext` to access the current session from routes.

### RBAC (Role-Based Access Control)
- Roles are defined at the application level.
- Permissions are checked at the route level via guards.
- Use `require_role!` or `require_permission!` macros in route loaders/actions.

## Important Rules
- Always validate input before authentication operations.
- Session tokens must be kept confidential.
- RBAC permissions are checked at the route level, not in views.
- Password hashing uses a configurable algorithm (default: argon2).