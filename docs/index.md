# MontRS Documentation Index

Welcome to the MontRS documentation. This folder contains deep-dive guides for both application developers and framework contributors, with a focus on our agent-first philosophy.

## 🎯 Audience Paths

### For Application Developers
- [First 30 Minutes](getting-started/first-30-minutes.md) - **Start here!** Your first onboarding experience.
- [Introduction](getting-started/introduction.md) - Build your first MontRS app.
- [The Golden Path](getting-started/golden-path.md) - Idiomatic MontRS development.
- [Common Mistakes](guides/common-mistakes.md) - Avoid frequent pitfalls.
- [Router](core/router.md) & [Plates](core/plates.md) - Understanding the Loader/Action pattern.
- [Validator & Validation](core/validator.md) - Input validation.
- [ORM Layer](orm/index.md) - Working with databases.
- [ORM Backends](orm/backends.md) - Supported databases.
- [Testing](testing/index.md) - Writing deterministic tests.
- [Benchmarking](testing/benchmarking.md) - Measuring performance.
- [Deployment](community/deployment.md) - Shipping to production.

### For Framework Contributors
- [Architecture Overview](architecture/overview.md) - The "Shape" of MontRS.
- [AppSpec & Discovery](agent/appspec.md) - How the framework maps itself.
- [Package Boundaries](architecture/packages.md) - Responsibility of each crate.
- [Invariants & Philosophy](architecture/philosophy.md) - The rules we don't break.
- [Agent-first design](agent/agent-first.md) - Principles of machine-readability.
- [CLI Internals](tooling/cli.md) - Orchestration and task runners.
- [Custom Tasks](tooling/tasks.md) - Automating workflows.
- [Contributing Guide](community/contributing.md) - How to build MontRS.
- [Packages Contribution](community/packages-contribution.md) - Guidelines for new framework packages.

### For Agents
- [**Agent Entry Point**](agent/index.md) - **Start here!** Unified operational framework.
- [Agent Condensed Onboarding](agent/onboarding.md) - Core architectural knowledge.
- [Agent Integration Guide](agent/agent-first.md) - Principles of machine-readability.
- [Specialized Prompts](agent/prompt-usage.md) - Identity and constraints.
- [Agentic CLI & Workflows](agent/agentic-workflows.md) - Mastering the agentic loop.
- [MCP Setup & Access](agent/mcp-setup.md) - **New!** Connecting agents to the framework.
- [Spec Snapshot](agent/spec.md) - Understanding `agent.json`.
- [Error Handling Spec](core/errors.md) - Versioned error files and diffs.
- [Metadata Standards](agent/metadata.md) - How we annotate code for you.

---

## 🏗️ Core Concepts

- **Determinism**: Every part of MontRS is designed to be predictable and testable in isolation.
- **Model-First**: We prioritize machine-readable metadata to enable agent-assisted development.
- **Trait-Driven**: Interfaces are defined by traits, allowing for modular and swappable components.
