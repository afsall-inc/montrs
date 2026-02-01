# Contributing to MontRS

Thank you for your interest in contributing to MontRS! This framework is built for the future of agent-assisted development, and we welcome contributions that strengthen this vision.

## 🛠️ Development Environment

1.  **Clone the Repo**:
    ```bash
    git clone https://github.com/montrs/montrs.git
    cd montrs
    ```
2.  **Install Dependencies**:
    Ensure you have Rust (latest stable) and the MontRS CLI installed locally.
3.  **Run Tests**:
    ```bash
    cargo test --workspace
    ```

## 🏗️ Project Structure

MontRS is a workspace with several packages. For a detailed breakdown of package boundaries and instructions on how to create new ones, see the **[Packages Contribution Guide](packages-contribution.md)**.

The workspace includes:
-   `packages/core`: The architectural engine.
-   `packages/cli`: The command-line interface.
-   `packages/agent`: The Agent-First sidecar.
-   `packages/validator`: Procedural macros for validation.
-   `packages/orm`: The database layer.
-   `packages/fmt`: The custom formatter.

## 🤖 Agent-First Contribution Guidelines

When adding new features or packages, you **must** ensure they are Agent-ready:

1.  **Implement `AgentError`**: All new error types should implement the `AgentError` trait with descriptive codes and suggested fixes.
2.  **Add Metadata**: Provide `description()` and validation information for all new traits or plates.
3.  **Update READMEs**: Follow the mandatory section structure for any new package README.
4.  **Local Invariants**: Every package must maintain a `docs/invariants.md` file that defines its specific "rules of engagement" and architectural boundaries.
5.  **Annotate Tools**: Use the `@agent-tool` marker in comments for any public function or struct that should be exposed to agents.

## 🧪 Testing Your Changes

We prioritize deterministic testing. If you add a new feature:
1.  Add unit tests in the same file or a `tests/` directory.
2.  Add integration tests in `packages/core/tests` or `packages/test`.
3.  If it's a CLI change, verify it with a real project scaffolded via `montrs new`.

## 📝 Pull Request Process

1.  **Fork and Branch**: Create a feature branch from `main`.
2.  **Format Code**: Run `montrs fmt` before committing.
3.  **Commit Messages**: Use clear, descriptive commit messages.
4.  **Self-Audit**: Run `montrs spec` and check if your changes are correctly reflected in the generated `agent.json`.

---

Together, we're building the first framework that truly speaks the language of both humans and machines.
