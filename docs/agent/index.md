# 🤖 MontRS Agent Entry Point

Welcome to the unified command center for MontRS Agents. This document is the "Map of Maps"—it defines the operational framework for how you should interact with the codebase based on your current intent.

---

## 🧭 Operational Framework: "What are you doing?"

Choose your path based on the task at hand. Do not attempt to "wing it"—follow the established workflows to maintain architectural integrity.

### 1. I am fixing a Bug or Error
- **Goal**: Identify, diagnose, and resolve a failure (compilation, runtime, or architectural).
- **Primary Guide**: [Workflow: Fixing Errors](workflows/fixing-errors.md)
- **Key Tools**: `montrs agent list-errors`, `montrs agent diff`, `montrs agent resolve`.

### 2. I am adding a New Feature
- **Goal**: Extend the application or framework with new capabilities.
- **Primary Guide**: [Workflow: Adding Features](workflows/adding-features.md)
- **Key Tools**: `montrs generate plate`, `montrs generate route`, `montrs spec`.

### 3. I am starting a New Project
- **Goal**: Scaffold a fresh MontRS application from scratch.
- **Primary Guide**: [Workflow: New Projects](workflows/new-projects.md)
- **Key Tools**: `montrs new`.

### 4. I am restructuring or Refactoring
- **Goal**: Move logic, split plates, or improve architectural health without changing behavior.
- **Primary Guide**: [Workflow: Restructuring](workflows/restructuring.md)
- **Key Tools**: `montrs agent check`.

### 5. I am using a Skill
- **Goal**: Load a composable, multi-step agent capability.
- **Primary Guide**: [Skills System](skills.md)
- **Key Tools**: `montrs agent skills`, `montrs agent skills --name <skill>`.

### 6. I am documenting a Pull Request
- **Goal**: Create structured, machine-readable PR documentation for agent review.
- **Primary Guide**: [PRDoc Guide](../contributor/prdoc.md)
- **Key Tools**: `montrs agent prdoc`, `montrs agent prdoc --validate`.

---

## 🎭 System Prompts (Your Identity)

Your behavior is governed by your specific role. Read these to understand your constraints and core identity:

- **[App Developer Prompt](app-developer-prompt.md)**: For building applications *using* MontRS.
- **[Framework Contributor Prompt](framework-contributor-prompt.md)**: For developing and maintaining the *framework itself*.

---

## 🛡️ Foundational Knowledge

Before performing any action, ensure you are grounded in these core principles:

- **[Agent-First Philosophy](agent-first.md)**: Why we prioritize machine-readability.
- **[Framework Invariants](onboarding.md)**: The rules you must never break.
- **[Spec Snapshot (agent.json)](spec.md)**: How to read the project's current state.
- **[Metadata Standards](metadata.md)**: How to annotate code for discovery (`@agent-tool`, `@agent-skill`).
- **[Skills System](skills.md)**: Composable, reusable agent capabilities with workflow steps.
- **[PRDoc Guide](../contributor/prdoc.md)**: Structured PR documentation for agent review.

---

## 💡 Token Optimization & Efficiency

To minimize context window usage and improve accuracy, follow these efficiency rules:

- **Scoped Invariants**: Do NOT read all framework invariants by default. Only read the `invariants` field in `agent.json` for the specific package(s) you are currently modifying or interacting with.
- **Incremental Context**: Start with the `agent_entry_point` and the relevant workflow guide. Only pull in full package documentation if the workflow guide or `agent.json` summary is insufficient.
- **Targeted Discovery**: Use `montrs agent check` or `montrs agent list-errors` to narrow down the specific files and packages that require your attention.

---

## 🌍 Context Boundary: Framework vs. Application

It is critical to distinguish between working **on** the MontRS framework and working **with** it to build an application. Confusing these two will lead to architectural violations.

### 1. Working ON the Framework (Contributor Mode)
- **Target**: The `montrs` repository itself (e.g., `packages/core`, `packages/cli`).
- **Goal**: Modifying the engine, CLI, or core traits.
- **Prompt**: Use the [Framework Contributor Prompt](framework-contributor-prompt.md).
- **Rule**: Your changes affect *all* downstream users. Prioritize stability and zero-cost abstractions.

### 2. Working WITH the Framework (App Developer Mode)
- **Target**: A user project created via `montrs new`.
- **Goal**: Building business logic, UI, and features using MontRS primitives.
- **Prompt**: Use the [App Developer Prompt](app-developer-prompt.md).
- **Rule**: Treat the framework as a stable API. 
- **Consumer vs. Contributor**: If you are in a standalone app project, you are a "Consumer". You will not see the framework's source code in your file tree. Use the `agent.json` snapshot to discover framework invariants and documentation embedded by the CLI.

---

## 🔌 Tooling & Integration

- **[MCP Setup & Access](mcp-setup.md)**: How to connect your capabilities to the project.
- **[Agentic CLI Guide](agentic-workflows.md)**: Mastering the command-line loop.
