# AgentIgnore Package — Agent Guide

## Overview
`montrs-agentignore` defines how agents decide which files to scan and include in project snapshots. It uses `.agentignore` as the canonical source of truth for exclusion patterns.

## Key Concepts
- **.agentignore**: The canonical file. Uses `.gitignore` syntax.
- **Pattern Matching**: Supports glob patterns, negation with `!`, and directory patterns.
- **IDE Export**: Can export patterns to IDE-specific configs (`.trae`, `.cursor`, `.opencode`).

## Agent Usage
- Always respect `.agentignore` when scanning a project.
- If `.agentignore` is missing, fall back to `.gitignore`.
- Use `IsIgnored` logic to quickly check whether a path is excluded.

## Local Invariants
Read `docs/invariants.md` before modifying.