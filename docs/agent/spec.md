# The Agent Specification Snapshot (`agent.json`)

The `agent.json` file, located in the `.agent/` directory, is the primary source of context for agents working on a MontRS project. It is a structured, machine-readable representation of the entire application.

## 📂 File Location
```text
.agent/
├── agent.json        # Primary JSON specification
├── agent.yaml        # YAML version (optional)
├── agent.txt         # Text summary (optional)
└── errorfiles/       # Versioned history of project errors
```

## 🏗️ Structure of `agent.json`

The file is divided into several key sections:

### 1. `metadata`
Contains project-wide information:
- `name`, `version`, `description`.
- Agent-specific instructions and project context.

### 2. `plates`
A list of all registered `Plate` implementations found in the project. For each plate:
- `name`: The Rust struct name.
- `description`: The string returned by `plate.description()`.

### 3. `routes`
The most critical section for application development. Each route is a unified unit containing:
- `path`: The URL pattern (e.g., `/api/users/:id`).
- `params`: Metadata about the required URL parameters.
- `loader`: Metadata about the data-fetching logic (Input/Output).
- `action`: Metadata about the mutation logic (Input/Output).
- `metadata`: Key-value pairs describing the route's purpose for agents.

### 4. `tools`
A curated list of CLI commands and framework capabilities that the agent can invoke as "tools."

## 🔄 Lifecycle

1. **Auto-Update**: The CLI automatically refreshes the spec whenever a command (`build`, `serve`, `test`) is run.
2. **Manual Update**: You can force a refresh using `montrs spec`.
3. **Agent Consumption**: Agents should read this file at the start of every session to ensure they have the latest context.

## 🤖 Why Not Just Read the Code?

While models *can* read source code, `agent.json` provides:
- **Pre-computed Metadata**: Descriptions and validators are extracted and ready to use.
- **Unified View**: It bridges the gap between different files and packages.
- **Standardization**: It follows a consistent structure regardless of how the underlying code is structured.
