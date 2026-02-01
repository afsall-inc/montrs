# Getting Started with MontRS

Welcome to MontRS! This guide will help you get your first application up and running in minutes.

## 🛠️ Prerequisites

- **Rust**: Ensure you have the latest stable version of Rust installed.
- **Node.js** (Optional): Required if you are building a frontend with Leptos.
- **MontRS CLI**: Install our command-line tool.

```bash
cargo install --path packages/cli
```

## 🚀 Creating Your First Project

Use the `new` command to scaffold a project from a template.

```bash
montrs new my-awesome-app
cd my-awesome-app
```

## 🏗️ Project Structure

A typical MontRS project looks like this:

- `src/`: Your application source code.
  - `main.rs`: Entry point.
  - `plates/`: Your business logic organized into plates.
- `montrs.toml`: Framework and task configuration.
- `.agent/`: (Auto-generated) agent-readable project specifications.

## 🏃 Running the App

Start the development server with hot-reloading:

```bash
montrs serve
```

Your app will be available at `http://localhost:3000`.

## 🤖 Agent-first development

MontRS is designed to work with agents. Every time you run a CLI command, it updates `.agent/agent.json`. This file gives your agent coding partner a perfect understanding of your project's routes, validators, and state.

## 🛠️ Troubleshooting

### 1. `montrs: command not found`
Ensure that your Cargo binary directory is in your `PATH`. On most systems, this is `~/.cargo/bin`.

### 2. Compilation Errors in `view!` Macros
The `view!` macro is very strict about HTML syntax. If you get a cryptic error, try running `montrs fmt` to see if it can identify a malformed tag or missing closing brace.

### 3. `.agent` Folder Not Found

The `.agent` folder is created the first time you run `montrs serve`, `montrs build`, or `montrs spec`. If it's missing, try running `montrs spec` manually.

---

## 📚 Next Steps

- [The Golden Path](golden-path.md): Learn the best practices for building with MontRS.
- [Router & Plates](router.md): Deep dive into our data-first architecture.
- [Testing](testing.md): Write your first deterministic test.
