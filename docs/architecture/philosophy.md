# Invariants & Philosophy

MontRS is built on a set of core principles that guide every architectural decision. Understanding these will help you contribute to the framework and build better apps.

## 0. The MUST Convention

Everything in MontRS — every crate, trait, feature, and test — **MUST** be
**Modular, Universal, Simple, and Testable**. This is not a slogan; it is the
acceptance bar. A change that fails any letter is not finished.

- **Modular** — One isolated, composable unit behind an explicit boundary (a
  trait or a small type). No hidden coupling, no cross-cutting reach. It can be
  adopted, replaced, or **omitted** without disturbing anything else.
- **Universal** — One API serves every target (*web, desktop, mobile, TUI*) and
  every layer (*API, DOM, pixels, motion*). No per-target forks, no
  special-cases that only work in one place.
- **Simple** — The smallest concept that does the job. Explicit over clever, no
  magic, no surprising side effects. If a feature needs a paragraph of
  justification to exist, it does not ship.
- **Testable** — Hermetically verifiable in-process and deterministic by
  construction: seeded randomness, an injectable clock, no required
  server/browser/DB. Every feature ships with the means to prove itself — and,
  where possible, **the artifact _is_ the test** (a budget, a snapshot, a spec).

**Rule of thumb:** if it can't be tested in-process, it isn't finished; if it
only works on one target, it isn't universal; if it needs a new concept to
explain, it isn't simple; if it can't be removed, it isn't modular.

**How it is enforced:** reviewed at PR time; checked by `montrs agent check`
against package invariants; and demonstrated by the deterministic test fabric,
which makes MUST *observable* rather than aspirational. Regression against the
expected output is gated by `montrs verify` (see `docs/testing/regression.md`).

## 1. Determinism by Default

We believe that a framework should be predictable. Given the same input and environment, a MontRS component should produce the same output. This makes testing, debugging, and agent-assisted development significantly more reliable.

## 2. Specification-First (Model-First)

A MontRS application is not just a collection of code; it is a living specification. The `AppSpec` is a first-class citizen, enabling a "Model-First" approach where agents can reason about the application's structure as effectively as a human developer.

## 3. Trait-Driven Modularity

We prefer traits over macros for defining interfaces. This makes boundaries explicit, improves compile-time checks, and allows for easy swapping of components (e.g., changing a database backend or a logger).

## 4. SQL-Centric Persistence

While we provide an ORM, we don't hide SQL. We believe SQL is the most powerful and standardized way to interact with relational data. Our ORM focuses on making SQL safe and ergonomic in Rust.

## 5. Explicit Boundaries

Data entering or leaving the application must be validated. The `montrs-validator` package ensures that boundaries between the frontend, backend, and database are clearly defined and enforced.

## 6. Agents as First-Class Users

We design our tools and documentation not just for humans, but for agents. Structured metadata, versioned error files, and machine-readable snapshots are core features, not afterthoughts.

## 7. Productive Explicitness (Scaffolded Explicit)

We avoid "magic" behaviors that are hard to trace. We prefer explicit registration and configuration over implicit discovery. However, explicitness should not mean "tedious." 

MontRS follows the **Scaffolded Explicit** architecture:
1. **Sketch**: Start with a single-file, explicit, and deterministic blueprint (`montrs sketch`).
2. **Expand**: Grow into a full, structured workspace with explicit imports and manual registration (`montrs expand`).

This ensures that the developer (and the AI agent) always has a clear, readable source of truth while maintaining high iteration speed. All scaffolded code is production-ready and deterministic by construction.
