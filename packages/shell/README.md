# montrs-shell

Shell integration for MontRS. Provides activation scripts for bash, zsh, fish, and pwsh, plus shim management.

## Features

- **Shell activation**: `eval "$(montrs activate bash)"` for all supported shells
- **Shim management**: Auto-generated executable stubs in `~/.local/share/montrs/shims`
- **Hook env**: Automatic PATH/version syncing on every prompt
- **Reshim**: `montrs reshim` rebuilds all shims for all installed tools

## Supported Shells

| Shell | Activation | Hook Method |
|-------|-----------|-------------|
| bash | `eval "$(montrs activate bash)"` | `PROMPT_COMMAND` |
| zsh | `eval "$(montrs activate zsh)"` | `precmd` hook |
| fish | `eval (montrs activate fish)` | `fish_prompt` event |
| pwsh | `Invoke-Expression (montrs activate pwsh)` | Manual |

## Usage

```bash
# Add to your shell rc file:
eval "$(montrs activate bash)"
```