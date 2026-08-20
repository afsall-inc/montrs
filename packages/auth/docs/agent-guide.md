# Agent Guide: montrs-auth

## Core Concepts
MontRS authentication system supporting email/password, OAuth, 2FA, sessions, and RBAC.

## Important Rules
- Always validate input before authentication operations.
- Session management is handled through the auth context.
- RBAC permissions are checked at the route level.