# PRDoc Package — Agent Guide

## Overview
`montrs-prdoc` provides PR documentation parsing, generation, and changelog management for MontRS.

## Key Concepts
- **PRDoc**: Structured TOML document with `title`, `audience`, `changes`, `impact`.
- **Audience**: `FrameworkDev`, `AppDev`, `Operator`.
- **Changelog Generation**: Multiple PR docs → changelog.

## Agent Usage
- Use `montrs agent prdoc validate` to check PR doc schema.
- Use `montrs agent prdoc generate` to auto-generate from context.
- Use `montrs agent prdoc show` to view as JSON.

## Local Invariants
Read `docs/invariants.md` before modifying.