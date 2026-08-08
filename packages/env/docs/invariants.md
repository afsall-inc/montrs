# montrs-env — Invariants

## 1. Responsibility
Parse, render, and apply environment variables from `montrs.toml [env]`, `.env` files, and system environment.

## 2. Invariants
- **Single source**: `[env]` in `montrs.toml` is the canonical source. `.env` files are layered on top.
- **Tera rendering**: All env values support Tera templates with `env`/`cwd` variables.
- **PATH manipulation**: `_.path` directive supports prepend, append, remove.
- **No unsafe**: All env mutations use `unsafe { set_var }` only when applying to the process.

## 3. Boundary
- **In-Scope**: Parsing, rendering, resolving, applying, diffing, `.env` I/O.
- **Out-of-Scope**: File watching, secret management, shell activation.

## 4. Agent Guidelines
- Use `parse_env_section()` then `resolve_environment()` to get resolved env.
- Use `apply_environment()` before spawning child processes.
- Use `EnvDiff::compute()` to compare before/after state.