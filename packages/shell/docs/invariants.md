# montrs-shell — Invariants

## 1. Responsibility
Generate shell-specific activation scripts and manage tool shims.

## 2. Invariants
- **Shell-agnostic trait**: All shells implement `Shell` trait. No shell-specific logic outside the shell module.
- **No side effects**: Activation scripts are printed to stdout for shell evaluation. The package itself does not modify `.bashrc`/`.zshrc`.
- **Shim simplicity**: Shims are shell scripts on Unix, copy on Windows. No binary detection logic in the shim itself.
- **Default directories**: Shims go in `~/.local/share/montrs/shims`, installs in `~/.local/share/montrs/installs`.

## 3. Boundary
- **In-Scope**: Shell activation scripts, deactivation, env set/unset, PATH prepend, shim creation/removal.
- **Out-of-Scope**: Shell configuration file editing, prompt formatting, autocompletion generation.

## 4. Agent Guidelines
- Use `ShellType::detect()` to auto-detect the current shell.
- Use `shell.activate(&opts)` to generate the activation script.
- Use `shims::reshim_all()` to rebuild shims after tool installs.