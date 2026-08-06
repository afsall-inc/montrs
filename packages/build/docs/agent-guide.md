# Build Package — Agent Guide

## Overview
`montrs-build` is the facade crate for the MontRS build system. It re-exports `montrs-build-core`, `montrs-build-watch`, and `montrs-build-serve`, and provides the concrete `Pipeline` struct.

## Key Concepts
- **Pipeline**: The concrete build pipeline that implements `BuildPipeline`.
- **Pipeline::from_root()**: Creates a pipeline from a project root directory.
- **build_all()**: Runs all build steps in order.

## Agent Usage
- Use `montrs_build::Pipeline` to build a MontRS project.
- Use `montrs_build::run_cargo()` to run cargo commands.
- Use `montrs_build::run_tailwind()` to process Tailwind CSS.

## Local Invariants
Read `docs/invariants.md` before modifying.