# Workflow: Adding New Features

This guide defines the "Golden Path" for adding functionality to an app or extending the framework.

---

## 📱 For Application Developers

1.  **Contextualize**: Read the `docs/invariants.md` of the MontRS framework packages you are utilizing. These define the "rules of engagement" for using the framework's features correctly.
2.  **Define Validator**: Create input/output structs with `#[derive(Validator)]`.
2.  **Generate Boilerplate**:
    -   Run `montrs generate plate <name>` for new features.
    -   Run `montrs generate route <path> --plate <name>` for new endpoints.
3.  **Implement Unified Route**: 
    -   The CLI generates `RouteLoader`, `RouteAction`, and `RouteView`.
    -   Implement the business logic in the `load` and `act` methods.
4.  **Register**:
    -   Add modules to `mod.rs` as instructed by the CLI.
    -   Register the plate in `main.rs` and the route in `Plate::register_routes`.
5.  **Annotate**: Implement `description()` on Loaders and Actions for agent discovery.
6.  **Verify**: Run `montrs spec` and `montrs agent check`.

---

## 🏗️ For Framework Contributors

1.  **Invariants Check**: Read `packages/<target>/docs/invariants.md` to understand the internal architectural rules and boundary constraints of the framework package you are modifying.
2.  **Trait Definition**: Define new core traits in `packages/core`.
2.  **Implementation**: Provide default or specialized implementations in relevant packages.
3.  **Macro Support**: If the feature requires automation, update `packages/validator` (procedural macros).
4.  **CLI Integration**: Update `montrs generate` commands in `packages/cli` if the feature requires boilerplate.
5.  **Agent Visibility**: 
    -   Update `packages/agent` to collect metadata for the new feature.
    -   Add `@agent-tool` to the new implementation.
6.  **Template Update**: Add an example of the feature to `templates/todo`.
7.  **Final Audit**: Run `montrs spec` and `agent doctor`.

---

## 🤖 Agent Instructions (MCP)
-   **Step 1**: Read the relevant framework `docs/invariants.md` before proposing changes to ensure you are using the framework's features as intended.
-   **Step 2**: Use `montrs generate` tools whenever possible to ensure architectural consistency.
-   **Step 3**: Follow the **Golden Path** (Validator -> Generation -> Implementation -> Registration).
-   **Step 4**: Always add `description()` to Loaders and Actions.
-   **Step 5**: `agent_check({})` to verify compliance.
