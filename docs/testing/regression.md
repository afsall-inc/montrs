# Deterministic Regression CI: `montrs verify`

MontRS provides a built-in regression detection command — `montrs verify` — designed for both application developers and framework contributors.

Unlike standard correctness tests (`cargo test`) which verify whether code runs without failing, `montrs verify` detects whether your application has **deteriorated** in performance, responsiveness, layout bounds, or contract shapes compared to committed baselines.

---

## 1. Why Deterministic Regression?

In typical web frameworks, regression tests are notoriously flaky because they rely on:
- Live network sockets and ports
- Headless browser drivers and GPU layout timing
- Wall-clock timers subject to CPU frequency scaling and CI runner contention

The MontRS Test Fabric solves this by running **completely in-process** using:
- Pure-Rust box-model geometry computation (`taffy`)
- Seeded pseudo-randomness (`TestRng`)
- Injectable virtual time (`TestClock`)
- Stable HTML marker stripping for snapshot invariance

Because inputs and simulation are deterministic, running the verification twice yields **byte-identical results**.

---

## 2. Using `montrs verify`

### Basic Execution

Run in your terminal or CI pipeline:

```bash
# Run all deterministic checks (self-check, responsive layout, API contracts)
montrs verify
```

### Targeted Scopes

Verify only specific subsystems when iterating:

```bash
# Verify UI layout and responsive overflow constraints down to 320px
montrs verify --ui

# Verify API contracts and loader/action routes
montrs verify --api

# Run the determinism self-consistency test (executes twice and verifies byte-identical output)
montrs verify --self-check
```

### Updating Baselines

When you make intentional design changes or add new routes, update the committed baselines:

```bash
montrs verify --update
```

This writes updated artifacts into `.montrs/baselines/` which should be committed to git so reviewers can inspect the diff during pull request review.

---

## 3. Performance Budgets vs. Weights

MontRS avoids blockchain-style "weights" (which calculate transaction gas fees) in favor of clear, actionable **latency budgets**:

- **P95 Latency Ceilings**: Set acceptable maximum response times per route.
- **Statistical Baselines**: Record median, P95, and P99 times in `.montrs/bench.json`.
- **Tolerance Bands**: Catch unexpected performance degradation in CI before merging.

Configure budgets in your `montrs.toml`:

```toml
[testing.budgets]
"route:/api/health".max-p95-ms = 10.0
"route:/users".max-p95-ms = 45.0
```

---

## 4. CI/CD Integration

Add `montrs verify` to your GitHub Actions workflow:

```yaml
- name: Deterministic Regression Check
  run: cargo run --package montrs-cli --bin montrs -- verify --self-check
```
