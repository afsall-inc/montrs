# montrs-registry — Agent Guide

## Overview
Contains the canonical list of tools that MontRS can manage. Each tool entry specifies available backends, installed binaries, and detection patterns.

## Key Concepts
- **RegistryTool**: name, backends, bins, detect patterns, idiomatic files, aliases, platform overrides
- **Registry**: Map of tool names to RegistryTool, with search and backend selection
- **BAKED_REGISTRY**: Compiled from `registry/*.toml` at build time
- **Floating registry**: Downloadable JSON from `montrs.com/registry/`

## Agent Usage
- `BAKED_REGISTRY.get("tool_name")` for tool lookup
- `BAKED_REGISTRY.search("query")` for fuzzy search
- `BAKED_REGISTRY.best_backend("tool_name")` for platform-aware backend selection

## Local Invariants
Read `docs/invariants.md` before modifying.