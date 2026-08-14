# MontRS SaaS Template

Full SaaS starter: auth, organizations, admin, API keys, services.

## Getting started

```bash
montrs services start     # start postgres + redis + api
montrs run seed           # seed data
montrs serve              # run API + web
```

## Auth plugins

- Email/password
- Two-factor (TOTP)
- Organizations + RBAC
- Admin panel
- API keys