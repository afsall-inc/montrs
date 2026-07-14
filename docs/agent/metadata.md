# Metadata Standards for Agents

To ensure that MontRS remains machine-readable, we follow a strict set of metadata standards across the codebase.

## 🏷️ The `@agent-tool` Marker

Any function or struct that is intended to be used as a tool by an agent must be marked with the `@agent-tool` comment. This allows `montrs-agent` to automatically curate a `tools.json` file.

```rust
// @agent-tool: Generates a new project from a template.
pub fn scaffold_project(name: &str, template: &str) -> Result<()> { ... }
```

## 🧩 The `@agent-skill` Marker

Any module or function that implements a multi-step agent capability should be marked with the `@agent-skill` comment. Skills differ from tools in that they provide workflows with contextual steps, prompts, and invariants rather than single-shot function calls.

```rust
// @agent-skill: database-setup — Guides database backend configuration.
pub fn setup_database(config: &DatabaseConfig) -> Result<()> { ... }
```

Skills are also discovered from `skill.toml` manifests in the `skills/` directory. See the [Skills System](skills.md) documentation for the full manifest format.

## 📝 Trait Metadata

Core traits (`Plate`, `Loader`, `Action`, `Limiter`, `Validate`) include a `description()` method. This should return a human-and-machine-readable summary of the component's purpose.

```rust
impl Loader for MyLoader {
    fn description(&self) -> Option<String> {
        Some("Fetches the latest 10 posts from the database.".to_string())
    }
}
```

## 🧬 Validator Definitions

Use `#[derive(Validator)]` on all input and output structs. This provides the agent with the exact data shape and validation rules required for interaction.

## 🔄 The Curation Process

How does your code end up in the `agent.json` or `tools.json`?

1.  **Scanning**: The `montrs-agent` scanner walks your source code and `skills/` directory.
2.  **Extraction**: It looks for explicit markers like `@agent-tool`, `@agent-skill`, `skill.toml` manifests, and implementations of traits like `Loader` or `AgentError`.
3.  **Heuristics**: If a component lacks explicit metadata, the scanner uses heuristics (like reading the README in the same package) to infer its purpose.
4.  **Serialization**: The extracted data is normalized into a standard JSON format and saved to the `.agent/` directory.

---

## 🔍 Best Practices for Metadata

-   **Be Descriptive**: Instead of `fn get_data()`, use `// @agent-tool: Fetches active user profiles from the primary database.`
-   **Include Constraints**: Mention any rate limits or side effects in the description.
-   **Keep it Up-to-Date**: If you change the behavior of a function, remember to update its `@agent-tool` comment or `description()` method.
-   **Use Skills for Workflows**: If a capability involves multiple steps or requires contextual prompts, mark it with `@agent-skill` and create a `skill.toml` manifest instead of a simple `@agent-tool`.
