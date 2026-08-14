# montrs-env — Agent Guide

## Overview
Manages environment variables for MontRS projects. The `[env]` section of `montrs.toml` defines variables that are applied before the application runs.

## Key Concepts
- **EnvDirective**: `Value(String)` or `Structured` (with export/redact/required/file/source options)
- **PathDirective**: `_.path` with prepend, append, remove, default
- **Environment**: Resolved vars with export flags and PATH state
- **EnvDiff**: Set/unset detection between two environment states

## Agent Usage
- `montrs env list` to show current env vars from montrs.toml
- `montrs env set KEY=value` to add a var
- `montrs env unset KEY` to remove a var
- `montrs env exec -- command` to run a command with resolved env

## Local Invariants
Read `docs/invariants.md` before modifying.