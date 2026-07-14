# MontRS Skills System

Skills are composable, reusable capabilities that agents can load and execute. Unlike tools (which are single-shot function calls), skills provide multi-step workflows, contextual knowledge, and structured execution plans.

## Skill Manifest Format

Each skill is defined by a `skill.toml` manifest:

```toml
[skill]
name = "database-setup"
version = "1.0.0"
description = "Guides the agent through setting up and configuring a database backend for a MontRS application."
author = "MontRS"
tags = ["database", "setup", "configuration"]

[workflow]
steps = [
    "Check montrs.toml for database configuration section",
    "If no database configured, guide user to add [database] section with backend and URL",
    "Run `cargo add` for the chosen backend driver",
    "Verify connection with TestRuntime",
]

[context]
prompts = [
    "Which database backend would you like to use? (sqlite, postgres, mysql)",
    "What is the connection URL or path for your database?",
]
invariants = [
    "Database configuration must be in montrs.toml, not hardcoded",
    "Connection strings must not be committed to version control",
]
```

## Skill Directory Structure

```
skills/
  database-setup/
    skill.toml
  testing/
    skill.toml
  deployment/
    skill.toml
```

## Agent Usage

Skills are registered via `@agent-skill` markers in Rust source or discovered from the `skills/` directory. They appear in `tools.json` as structured entries with workflow steps and context prompts.