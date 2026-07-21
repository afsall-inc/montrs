# AgentIgnore Invariants

## What It Enforces
- .agentignore is the canonical source of truth for agent file exclusion
- All patterns follow .gitignore syntax
- Agents must respect .agentignore during project scanning and snapshot generation

## Rules

### Always
- The .agentignore file is read from the project root
- If .agentignore is missing, agents scan all non-gitignored files
- .agentignore patterns override .gitignore for agent behavior

### Never
- Never ignore .agentignore itself
- Never ignore .agent/ directory configuration
- Never ignore invariant files or agent rules

### IDE Export
- Export to .opencodeignore for opencode
- Export to .cursorignore for Cursor
- Export to .trae/.agentignore for Trae
