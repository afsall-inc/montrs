# How to Use MontRS Specialized Agent Prompts

This guide explains how to use the specialized system prompts for MontRS agents. These prompts are designed to turn generic LLMs into expert partners for either building apps with MontRS or contributing to the framework itself.

## 👥 Audience

-   **Application Developers**: Use the [App Developer Prompt](app-developer-prompt.md) (or [.trae/rules/app-developer.md](../../.trae/rules/app-developer.md)) to build features, fix bugs, and architect your MontRS applications.
-   **Framework Contributors**: Use the [Framework Contributor Prompt](framework-contributor-prompt.md) (or [.trae/rules/framework-contributor.md](../../.trae/rules/framework-contributor.md)) when working on the MontRS source code, adding new packages, or improving CLI tools.

## 🚀 For Users (Humans)

### 1. Choose the Right Prompt
-   If you are building an app (e.g., using `montrs new`): Use the **App Developer Prompt**.
-   If you are editing the MontRS workspace (e.g., in `packages/`): Use the **Framework Contributor Prompt**.

### 2. Setting Up the Agent
#### A. Using IDE Rules (Recommended for Trae)
MontRS provides pre-configured rule files in the `.trae/rules/` directory. These are automatically picked up by the Trae IDE to govern agent behavior.
- For App Dev: [app-developer.md](../../.trae/rules/app-developer.md)
- For Framework: [framework-contributor.md](../../.trae/rules/framework-contributor.md)

#### B. Manual Configuration
Copy the content of the selected prompt file and set it as the **System Prompt** (or "Instructions") in your AI chat interface, IDE (like Antigravity or Cursor), or custom agent configuration.

### 3. Contextualizing the Session
For the best results, provide the agent with the latest project state:
1.  Run `montrs spec` in your terminal.
2.  Point the agent to the `.agent/agent.json` (or `agent.txt`) file.
3.  If there are errors, show it the latest `errorfile.json` in `.agent/errorfiles/`.

## 🤖 For Agents (AI)

### 1. Internalizing the Identity
Once you receive these prompts, you are no longer a general-purpose assistant. Your primary goal is to uphold the **MontRS Golden Path** and **Architectural Invariants**.

### 2. Utilizing the .agent Folder
-   **Always check `agent.json`**: This is your map of the project.
-   **Respect `tools.json`**: Use the CLI tools described there instead of guessing commands.
-   **Look for `@agent-tool`**: These markers indicate functions you are encouraged to use.

### 3. Reporting and Correcting
-   If you identify a violation of MontRS principles (e.g., global state), report it immediately to the user.
-   Use the `agent check` and `agent doctor` tools (via MCP or CLI) to verify your own suggestions.

## 🛠️ Integration with IDEs (Trae/Cursor)

To use these prompts effectively in your IDE:

### 1. Automated Setup (CLI)
The fastest way to set up project-specific rules for both **Trae** and **Cursor** is via the MontRS CLI:
```bash
# Scaffolds .trae/rules and .cursorrules for your project
montrs agent rules setup
```

This command will:
1. Create `.trae/rules/` and populate it with multiple specialized rule files (best for Trae's multi-rule support).
2. Create `.cursorrules` in the root (best for Cursor's single-file system).

### 2. Manual Project Rules
If you prefer manual setup, copy the relevant prompt content into your project's rule folder:
- **For Trae**: Copy content to `.trae/rules/app-developer.md` or `.trae/rules/framework-contributor.md`.
- **For Cursor**: Copy content to `.cursorrules` in the project root.

### 3. Global Rules (IDE-Wide)
For those who want MontRS principles to apply to *all* projects without adding files to every repo, you can set up **Global Rules**, manually do it in your AI-powered IDE settings.

---
*By using these specialized prompts, you ensure that your MontRS project remains clean, maintainable, and perfectly tuned for the future of agentic development.*
