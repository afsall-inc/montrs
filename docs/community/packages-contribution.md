# Packages Contribution Guide

This guide is for contributors working **on** the MontRS framework itself. It defines the standards for creating, enhancing, and documenting packages within the MontRS workspace.

---

## 🛡️ The Core Principle: Agent-First by Design

Every package in MontRS must be machine-readable and predictable. When creating or updating a package, you must ensure:
1.  **Deterministic Behavior**: Avoid hidden state. All IO and side effects should be injectable or mockable via `TestRuntime`.
2.  **Trait-Based Interfaces**: Define core logic using traits. This allows `montrs-agent` to scan and understand the "capabilities" of your package.
3.  **Self-Documenting Metadata**: Always implement `description()` and other metadata methods in your traits. This is how agents understand the "Intent" of your code.

---

## 🏗️ Creating a New Package

Before adding a new package to the workspace, ensure it has a clearly defined **Boundary**.

### 1. File Structure
- Place the new package in the `packages/` directory.
- Follow the naming convention: `montrs-<name>`.
- Include a `README.md` within the package folder explaining its specific role.
- Include a `docs/invariants.md` file defining local architectural rules.

### 2. Mandatory Documentation Updates
When a new package is added, you **must** update the following global documentation:
- **[packages.md](../architecture/packages.md)**: Add a new section describing the responsibility, key components, and boundaries of the package.
- **[overview.md](../architecture/overview.md)**: Update the architecture diagram or description if the package changes the high-level flow.
- **[index.md](../index.md)**: Link any new user-facing guides related to this package.

---

## 📝 Documenting Improvements & Enhancements

When you update, upgrade, or improve an existing package, you must document the change based on its impact.

### 1. Internal Changes (Refactoring/Optimization)
- Update the internal documentation within the code using Rustdoc (`///`).
- Ensure any changes to internal traits are reflected in the package's local `README.md`.

### 2. Feature Enhancements (New Capabilities)
- **Code Comments**: Update trait descriptions to reflect new capabilities so that `montrs-agent` picks them up.
- **User Guides**: If the enhancement changes how developers use MontRS, update the relevant guide in `docs/core/` or `docs/guides/`.
- **Common Mistakes**: If the new feature introduces a potential pitfall, add it to **[common-mistakes.md](../guides/common-mistakes.md)**.

### 3. Breaking Changes
- Breaking changes must be documented in a "Migration" section in the package README.
- Update the **[philosophy.md](../architecture/philosophy.md)** if the change affects a core framework invariant.

---

## 🤖 Alignment with Agent-First Strategy

Contributors are responsible for maintaining the "Agent Source of Truth":
- **Run `montrs spec`**: After making changes to traits or validators, run the spec generator to ensure `.agent/agent.json` reflects your changes correctly.
- **Versioned Errors**: If you add new error types, ensure they follow the stable error code format in `montrs-core` so they are machine-actionable.
- **Metadata Standards**: Check **[metadata.md](../agent/metadata.md)** to ensure your new components provide the required info for AI coding partners.

---

## 🚀 Checklist for New Packages
- [ ] Added to `Cargo.toml` workspace members.
- [ ] Implements core metadata traits for agent-readiness.
- [ ] Contains a local `README.md` with boundary definitions.
- [ ] Contains `docs/invariants.md` with package-specific rules.
- [ ] Updated **[packages.md](../architecture/packages.md)**.
- [ ] Verified via `montrs spec` that the package is discoverable.
- [ ] Added deterministic tests using `montrs-test`.
