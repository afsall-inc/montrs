# Runner Package — Agent Guide

## Overview
`montrs-runner` provides the task runner for MontRS. It parses task definitions from `montrs.toml` and executes them with dependency resolution.

## Key Concepts
- **TaskConfig**: `Simple(String)` for inline commands, `Detailed` for structured tasks.
- **TaskRunner**: Executes tasks with dependency resolution.
- **Dependencies**: Tasks are run in topological order.

## Agent Usage
- Use `TaskRunner::new(tasks)` to create a runner.
- Use `runner.run(task_name)` to execute a task.
- Use `runner.list()` to list available tasks.

## Local Invariants
Read `docs/invariants.md` before modifying.