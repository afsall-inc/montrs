# montrs-validator

Procedural macros for validation in MontRS.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-validator` provides the `#[derive(Validator)]` macro, which enables declarative, type-safe validation of data structures. It is the primary tool for defining the "shape" and constraints of data in a MontRS application. It is our schema validation tool.

## 2. What problems it solves
- **Validation Boilerplate**: Replaces repetitive `if` statements with concise, readable attributes.
- **Data Integrity**: Ensures that only valid data enters your `Action`s and `Plate`s.
- **Machine Readability**: The validator attributes are not just for validation; they also serve as metadata that agents can use to generate valid inputs.

## 3. What it intentionally does NOT do
- **Data Parsing**: It validates data that is already in a Rust struct; it does not handle the initial parsing from JSON or other formats (use `serde` for that).
- **Complex Cross-Field Validation**: While it supports `custom` methods, it is optimized for field-level constraints.
- **Database Schema Generation**: It defines validation rules, not database table structures (though they often overlap).

## 4. How it fits into the MontRS system
It is used in the **Data Layer**. It integrates with `montrs-core` to provide validation results that are automatically handled by the framework's routing and error systems.

## 5. When a user should reach for this package
- When defining an input struct for a `Loader` or `Action`.
- When modeling business entities that require strict constraints (e.g., User, Order).
- When they want to provide clear validation metadata to an agent.

## 6. Deeper Documentation
- [Validator Attributes Reference](../../docs/core/validator.md)
- [Custom Validation Logic](../../docs/core/validator.md#custom-validation)
- [Agent-first validation metadata](../../docs/core/validator.md#agent-integration)

## 7. Notes for Agents
- **Constraint Discovery**: Use the `#[validator(...)]` attributes to understand the valid range and format of any field.
- **Input Generation**: When tasked with calling an `Action`, always refer to the `Validator` of the input struct to ensure your request is valid.
- **Error Handling**: Look for `ValidationError` with `AgentError` metadata if `validate()` fails. It will tell you exactly which field failed and why.
- **Zero-Overhead**: Validation logic is generated at compile-time and has minimal runtime impact.
