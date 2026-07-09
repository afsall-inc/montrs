# MontRS MCP Server Design Document

## Mental Model

The MontRS MCP (Model Context Protocol) server is not a standalone service but a **mode** of the MontRS CLI. It serves as a machine-readable bridge between AI agents and the real MontRS environment. Following the MontRS philosophy of "Real systems over simulations," the MCP server directly invokes the CLI's internal logic to ensure that agents see exactly what the human developer sees.

## CLI Integration

The MCP server is started via the CLI:
```bash
montrs mcp serve
```
This command initializes a JSON-RPC 2.0 server over stdio (standard for MCP), allowing agents to discover and invoke MontRS capabilities.

## API Surface (Conceptual)

### Resources

- `montrs://project/structure`: Returns a machine-readable tree of the workspace, packages, and their relationships.
- `montrs://project/router`: Returns the current deterministic router structure, including all registered routes, loaders, and actions.
- `montrs://docs/{package}`: Provides access to package-specific documentation and invariants.
- `montrs://config/current`: Returns the merged configuration from `montrs.toml` and environment variables.

### Tools

- `montrs_check`: Invokes `montrs agent check`. Validates project invariants.
- `montrs_doctor`: Invokes `montrs agent doctor`. Returns structured diagnostics on "agent-readability".
- `montrs_diff`: Invokes `montrs agent diff`. Generates a diagnostic diff for an error file, showing the error, offending code, and suggested fix.
- `montrs_spec`: Invokes `montrs spec`. Generates a snapshot of the project for agent context.

### ENDPOINTS (MCP)

- `list_tools`: Returns available agent tools.
- `call_tool`: Executes a specific CLI capability and returns structured JSON.
- `list_resources`: Returns available project metadata resources.
- `read_resource`: Fetches the content of a specific resource.

## Safety and Determinism Guarantees

1. **Read-Only by Default**: The MCP tools are read-only. Modification of the project is left to the human developer or standard IDE tools after reviewing agent suggestions.
2. **Standardized Errors**: All errors follow the `AgentError` trait, providing stable error codes and structured remediation steps.
3. **No Shadow State**: The MCP server always queries the current filesystem. There is no sandboxing or speculative execution.
4. **Human-in-the-Loop**: Agents propose fixes via `montrs_diff`, but the application of these fixes is external to the MontRS Agent CLI, ensuring the human developer remains in control.

## Human vs Agent Outputs

- **Human**: CLI output is optimized for terminal readability (colors, progress bars, tables).
- **Agent**: MCP response is optimized for token efficiency and structural parsing (JSON, minimal filler, explicit structure).
