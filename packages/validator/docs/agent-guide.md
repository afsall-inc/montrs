# Agent Guide: montrs-validator

This guide helps agents use the declarative validation system of MontRS.

## Core Concepts

### 1. `#[derive(Validator)]`
The primary macro for defining validation rules. It generates a `validate(&self) -> Result<(), Vec<ValidatorError>>` method.

### 2. Validation Attributes
- `min_len = N`: Validates string length.
- `max_len = N`: Validates string length.
- `min = N`: Validates numeric value.
- `max = N`: Validates numeric value.
- `email`: Validates email format.
- `regex = "..."`: Validates against a regular expression.
- `custom = "method"`: Delegates to a custom method returning `Result<(), String>`.

## Agent Usage Patterns

### Defining a Validated Struct
When generating data models, always include validation attributes to ensure data integrity.
```rust
#[derive(Validator, Serialize, Deserialize)]
pub struct ProjectConfig {
    #[validator(min_len = 3)]
    pub name: String,
    #[validator(email)]
    pub contact_email: String,
}
```

### Handling Validation Errors
If `validate()` returns an error, it will contain a list of `ValidatorError` variants. Use these to prompt the user or self-correct the input data.
- `MinLength { field, min, actual }`
- `MaxLength { field, max, actual }`
- `Min { field, min, actual }`
- `Max { field, max, actual }`
- `InvalidEmail { field }`
- `RegexMismatch { field, pattern }`
- `Custom { field, message }`
