# Validator Package Invariants

## 1. Responsibility
`montrs-validator` handles procedural macros for validation, serialization, and metadata generation.

## 2. Invariants
- **Compile-Time Validation**: Errors in validator definition must be caught at compile-time with clear error messages.
- **Agent-Readable Metadata**: Macros must generate the necessary hooks for `montrs-agent` to understand data structures.
- **Minimal Runtime Overhead**: Generated code should be as efficient as manually written equivalents.

## 3. Boundary Definitions
- **In-Scope**: Derive macros, attribute macros, validation logic generation.
- **Out-of-Scope**: Runtime data processing (delegated to core/orm).
