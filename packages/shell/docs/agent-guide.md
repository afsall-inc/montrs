# montrs-shell — Agent Guide

## Overview
Generates shell activation scripts so MontRS-managed tools are on your PATH. Also manages shim executables that forward to the correct tool versions.

## Key Concepts
- **Shell trait**: `activate()`, `deactivate()`, `set_env()`, `unset_env()`, `prepend_path()`
- **ShellType**: `Bash`, `Zsh`, `Fish`, `Pwsh` — detected from SHELL env var
- **ActivateOptions**: `exe` path, flags, `no_hook`
- **Shims**: Small scripts/binaries in `~/.local/share/montrs/shims` that exec the real tool

## Agent Usage
- `ShellType::detect()` to get the current shell
- `shell.activate(&opts)` to generate activation script
- `shims::reshim_all_default()` to regenerate all shims
- `montrs activate bash` to output activation script for a specific shell

## Local Invariants
Read `docs/invariants.md` before modifying.