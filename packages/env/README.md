# montrs-env

Environment variable manager for MontRS. Parses the `[env]` section from `montrs.toml`, renders Tera templates, loads `.env` files, and applies environment variables to the process.

## Features

- Parse `[env]` section: `FOO = "bar"`, `FOO = { value, export, redact, required }`, `_.path`
- Tera template rendering in env values with `env`/`cwd` custom functions
- `.env` file loading with layered resolution
- `EnvDiff` computation for set/unset detection
- `montrs env list`, `montrs env set`, `montrs env unset`, `montrs env exec` CLI commands
- PATH manipulation via `_.path` directive (prepend, append, remove)

## Usage

```toml
[env]
RUST_BACKTRACE = "1"
MONTRS_LOG = "info"
MY_VAR = { value = "secret", export = false }
_.path = ["/path/to/bin"]
```