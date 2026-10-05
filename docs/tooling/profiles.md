# Operating Profiles: Development & Production

MontRS defines two explicit execution modes that control developer tools, asset
caching, compilation profiles, and error surfacing:

1. **Development (`development` or `dev`)**
2. **Production (`production` or `prod`)**

---

## 1. Feature Matrix

| Capability | Development | Production |
|------------|-------------|------------|
| In-browser Dev Console & Overlay | Enabled | **Completely disabled** |
| Hot-reload WebSocket (`/_dioxus`) | Active on reload port | **Disabled** |
| Static Asset Cache (`/pkg/*`, `/main.css`) | `no-store, no-cache` | `public, max-age=31536000, immutable` |
| HTML Document Cache | `no-store, no-cache` | `no-cache` |
| Gzip Asset Compression | Enabled | Enabled |
| WASM Profile | `hot` (optimized with debug assertions) | `release` (fully optimized, zero dev markers) |
| SSR Binary | `debug` profile | `release` profile |

---

## 2. Resolving the Mode

MontRS determines the mode using the following precedence:

1. **CLI Invocation**:
   - `montrs serve` always forces `development`.
   - `montrs build` defaults to `production`. Pass `--dev` to force a development build.
2. **Environment Variable**: `MONTRS_MODE` (e.g. `MONTRS_MODE=production`).
3. **Configuration**: `montrs.toml [deploy] mode`.
4. **Binary Inference**: Release builds infer `production`; debug builds infer `development`.

### Setting via `montrs.toml`

```toml
[deploy]
mode = "production"
target = "ssr"
```

### Setting via Environment

```bash
# In your shell or CI/CD runner:
export MONTRS_MODE=production
```

---

## 3. Production Guardrail

To prevent accidentally shipping developer overlays, live-reload hooks, or
un-optimized code to end users, `montrs build` checks the resolved mode.

If `[deploy] mode = "development"` or `MONTRS_MODE=development`, `montrs build`
will refuse to run unless the `--dev` flag is supplied:

```bash
$ montrs build
Error: refusing to build in development mode: [deploy] mode = "development" (or MONTRS_MODE=development).
Pass `--dev` to build deliberately, or set mode = "production".
```
