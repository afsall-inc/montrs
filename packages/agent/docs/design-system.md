# MontRS Agent Messaging Design System

This document defines the structured way MontRS articulates information, documentation, and errors to agents.

## 1. Tone and Articulation

### Human-Facing Tone
- **Professional & Helpful**: Concise instructions, clear warnings.
- **Action-Oriented**: Focus on what the user needs to do next.

### Agent-Facing Articulation (The "Model-First" Layer)
- **Deterministic**: Use stable identifiers (error codes, trait names).
- **Context-Rich**: Always include surrounding context (code snippets, file paths, line numbers).
- **Remediation-Focused**: Provide "suggested_fixes" that are machine-actionable.

## 2. Metadata Structure

All framework components (Plates, Loaders, Actions) must expose metadata via the following structure:

| Field | Type | Description |
|-------|------|-------------|
| `description` | String | A high-level summary of the component's purpose. |
| `input_validator` | JSON | (Optional) Validated structure for input data. |
| `output_validator`| JSON | (Optional) Validated structure for output data. |
| `tags` | Array | Categorical tags (e.g., "auth", "db", "ui"). |

## 3. Error Articulation (AgentError)

Errors are the primary communication channel during failure. Every `AgentError` must follow this structure:

```json
{
  "error_code": "STABLE_ID_UPPERCASE",
  "explanation": "A detailed explanation of the cause.",
  "suggested_fixes": [
    "Step-by-step fix 1",
    "Step-by-step fix 2"
  ],
  "subsystem": "package-name",
  "rustc_error": "Optional raw compiler output"
}
```

## 4. Documentation Mapping

Documentation is curated into the `.agent/agent.json` snapshot using these priorities:
1. **Explicit Metadata**: Values returned by `metadata()` methods.
2. **Doc Comments**: Triple-slash `///` comments on public items.
3. **Agent Guides**: Markdown files in `/docs/agent-guide.md`.
4. **READMEs**: Key sections (Key Features, Usage).

## 5. Learning from Resolution

When an error is resolved, the CLI generates a diff. This diff is structured as:
- **Base**: The error record and the state of the code at the time of failure.
- **Fix**: The commit/changes that resolved the error.
- **Lesson**: A summary of why the fix worked.

Agents should prioritize these "Resolution Diffs" as the most reliable source of truth for debugging.
