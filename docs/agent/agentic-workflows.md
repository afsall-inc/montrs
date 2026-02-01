# Agentic CLI & MCP Workflows

MontRS provides a powerful suite of tools designed specifically for agentic development. This guide teaches agents how to leverage the CLI and Model Context Protocol (MCP) server to automate their work and ensure architectural compliance.

## 🤖 The Agentic Loop

Agents should follow specific workflows based on the task at hand. See the detailed guides below:

-   [**Fixing Errors**](workflows/fixing-errors.md): The self-correction loop for bugs and structural violations.
-   [**New Projects & Packages**](workflows/new-projects.md): How to initialize and onboard new projects.
-   [**Restructuring Projects**](workflows/restructuring.md): Safely refactoring and reorganizing code.
-   [**Adding New Features**](workflows/adding-features.md): The "Golden Path" for extending functionality.

## 🛠️ CLI Power Tools for Agents

| Command | Purpose | When to Use |
| :--- | :--- | :--- |
| `montrs agent list-errors` | Lists all tracked errors and their status. | Start of every task. |
| `montrs agent diff <path>` | Generates a diagnostic report for a specific error file. | When fixing a reported bug. |
| `montrs agent check` | Validates the project against MontRS invariants. | After making code changes. |
| `montrs agent doctor` | Runs a health check on the project/package. | When the environment feels unstable. |
| `montrs spec` | Refreshes the machine-readable project snapshot. | Before analyzing project structure. |

## 🔌 The MCP Advantage

The MCP server (`montrs mcp serve`) allows agents to interact with the framework as if it were a native set of functions. Instead of parsing CLI output, agents can call tools directly:

-   **`get_project_snapshot`**: Returns the full JSON structure of the app.
-   **`agent_list_errors`**: Returns structured error data for direct processing.
-   **`agent_diff`**: Provides a step-by-step plan for fixing an error.
-   **`list_router_structure`**: Deep-dives into the routing table.

### Example Agentic Workflow (via MCP)
1.  **Call** `agent_list_errors({"status": "Pending"})`.
2.  **Pick** an error and **Call** `agent_diff({"path": ".agent/errorfiles/v1/ERR-001.json"})`.
3.  **Read** the suggested plan.
4.  **Call** `get_project_snapshot({})` to find relevant files.
5.  **Apply** the fix.
6.  **Call** `agent_check({})` to verify.

## 📝 Best Practices for Agents

-   **Refresh Often**: Always run `montrs spec` after significant structural changes.
-   **Trust the Metadata**: Use `description()` and `validator` info from `agent.json` instead of guessing function signatures.
-   **Incremental Fixes**: Use `agent check` after every file modification to catch regressions early.
-   **Document Fixes**: When fixing an error, ensure your commit message or internal logs reference the Error ID (e.g., `ERR-AGENT-001`).

---
*By mastering these tools, you transform from a text-generator into a precise architectural engineer.*
