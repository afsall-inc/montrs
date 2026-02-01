# AppSpec: The Blueprint of a MontRS Application

The `AppSpec` is the central source of truth for a MontRS project. It is a serializable data structure that captures the "Shape" of the application, including its plates, routes, validators, and metadata.

## 🔍 How Discovery Works

MontRS uses a **heuristic discovery engine** to map your source code into an `AppSpec`. This happens in several phases:

1.  **Static Analysis**: The CLI scans your `src/` directory for implementations of core traits (`Plate`, `Route`, `RouteLoader`, `RouteAction`).
2.  **Metadata Extraction**: It invokes the `description()`, `path()`, and validator methods on these implementations.
3.  **Dependency Mapping**: It identifies how plates are composed and which external plates are being used.
4.  **Serialization**: The resulting graph is serialized into `.agent/agent.json` (for agent context) and used internally by the CLI to orchestrate the build.

## 🛠️ The Structure of a Spec

An `AppSpec` contains:

-   **Metadata**: Project name, version, and agent-specific instructions.
-   **Plates**: A list of all registered plates and their internal configurations.
-   **Routes**: A mapping of paths to **Unified Routes**, which bundle Params, Loaders, Actions, and Views.
-   **Environment**: Required environment variables and their validation rules.

## 🤖 Why it matters for Agents

For an agent, the `AppSpec` is the **Project Manual**. Instead of grep-ing through thousands of lines of code to find where a route is defined, the agent can simply query the spec:

```json
{
  "path": "/api/users",
  "name": "UserRoute",
  "loader": {
    "description": "Returns a paginated list of users",
    "output_validator": { ... }
  },
  "action": {
    "description": "Updates a user",
    "input_validator": { ... }
  }
}
```

## 🔄 Refreshing the Spec

Whenever you make structural changes to your code (adding a route, changing a validator), you should run:

```bash
montrs spec
```

This ensures that the `.agent/agent.json` file is in sync with your source code, providing the most accurate context for your agent coding partner.
