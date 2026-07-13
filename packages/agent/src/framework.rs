//! Framework Invariants & Documentation
//!
//! This module embeds core framework invariants and guides directly into the binary.
//! This ensures that the agent always has access to the fundamental rules of the framework,
//! even when a local `.agent` folder is missing or when the package is distributed via crates.io.
//!
//! ### 🛠️ Adding a New Package
//! When adding a new package to the MontRS framework:
//! 1. Create `packages/<name>/docs/invariants.md`.
//! 2. Add a new `pub const <NAME>_INVARIANTS` below using `include_str!`.
//! 3. Update `get_framework_invariants()` to include the new invariants.
//! 4. (Optional) If the package has specialized workflows, add them to `docs/agent/workflows/` and embed them here.

pub const CORE_INVARIANTS: &str =
    include_str!("../../../packages/core/docs/invariants.md");
pub const AGENT_INVARIANTS: &str =
    include_str!("../../../packages/agent/docs/invariants.md");
pub const CLI_INVARIANTS: &str =
    include_str!("../../../packages/cli/docs/invariants.md");
pub const ORM_INVARIANTS: &str =
    include_str!("../../../packages/orm/docs/invariants.md");
pub const VALIDATOR_INVARIANTS: &str =
    include_str!("../../../packages/validator/docs/invariants.md");
pub const TEST_INVARIANTS: &str =
    include_str!("../../../packages/test/docs/invariants.md");
pub const UTILS_INVARIANTS: &str =
    include_str!("../../../packages/utils/docs/invariants.md");
pub const FMT_INVARIANTS: &str =
    include_str!("../../../packages/fmt/docs/invariants.md");
pub const BENCH_INVARIANTS: &str =
    include_str!("../../../packages/bench/docs/invariants.md");
pub const MONTRS_INVARIANTS: &str =
    include_str!("../../../packages/montrs/docs/invariants.md");

pub const AGENT_INDEX: &str = include_str!("../../../docs/agent/index.md");
pub const APP_DEVELOPER_PROMPT: &str =
    include_str!("../../../docs/agent/app-developer-prompt.md");
pub const FRAMEWORK_CONTRIBUTOR_PROMPT: &str =
    include_str!("../../../docs/agent/framework-contributor-prompt.md");
pub const APP_DEVELOPER_RULE: &str =
    include_str!("../../../.trae/rules/app-developer.md");
pub const FRAMEWORK_CONTRIBUTOR_RULE: &str =
    include_str!("../../../.trae/rules/framework-contributor.md");
pub const FIXING_ERRORS_WORKFLOW: &str =
    include_str!("../../../docs/agent/workflows/fixing-errors.md");
pub const ADDING_FEATURES_WORKFLOW: &str =
    include_str!("../../../docs/agent/workflows/adding-features.md");

pub fn get_framework_invariants()
-> std::collections::HashMap<&'static str, &'static str> {
    let mut m = std::collections::HashMap::new();
    m.insert("core", CORE_INVARIANTS);
    m.insert("agent", AGENT_INVARIANTS);
    m.insert("cli", CLI_INVARIANTS);
    m.insert("orm", ORM_INVARIANTS);
    m.insert("validator", VALIDATOR_INVARIANTS);
    m.insert("test", TEST_INVARIANTS);
    m.insert("utils", UTILS_INVARIANTS);
    m.insert("fmt", FMT_INVARIANTS);
    m.insert("bench", BENCH_INVARIANTS);
    m.insert("montrs", MONTRS_INVARIANTS);
    m
}
