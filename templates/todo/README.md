# todo-example

A practical demonstration of the MontRS framework in action.

## Overview

This example showcases how to combine the various crates within the MontRS ecosystem to build a real-world application. It covers everything from state management to database persistence.

## Features Demonstrated

- **Signal Reactivity**: Using the `Signal` type for thread-safe, reactive state.
- **Validator Validation**: Using `#[derive(Validator)]` to enforce data constraints (like minimum title length).
- **ORM Integration**: Performing asynchronous database operations using `SqliteBackend`.
- **Composition**: Structuring the application using the `Plate` trait and `AppSpec`.

## Running the Example

From the root of the workspace, run:

```bash
cargo run -p todo-example
```

## Code Highlights

- **`CreateTodo`**: Demonstrates field-level validation attributes.
- **`Todo`**: Shows how to implement `FromRow` for different database backends.
- **`TodoPlate`**: Illustrates the unit of composition in a MontRS app.

## Tailwind Support

This template includes [`tailwind-fuse`](https://github.com/gaucho-labs/tailwind-fuse) for advanced styling capabilities:

- **Type-Safe Variants**: Use `#[derive(TwClass)]` and `#[derive(TwVariant)]` to build robust UI components.
- **Smart Merging**: `tw_merge!` handles class conflicts automatically (e.g., `p-4` overrides `px-2`).
- **Rust-First Config**: Optional `tailwind.toml` support for defining themes and merge options without JavaScript.

### Configuration Options

1. **Tailwind v4 (Recommended)**: Use pure CSS for configuration. No JS files required.
2. **tailwind.toml**: Create a `tailwind.toml` for a pure-Rust configuration experience. `montrs` generates the JS config automatically.

```toml
content = ["src/**/*.rs", "*.html"]

[merge]
prefix = "tw-"
separator = ":"
```
