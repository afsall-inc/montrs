//! Reusable utilities for the MontRS framework.

/// Converts a string to PascalCase.
pub fn to_pascal_case(s: &str) -> String {
    let mut res = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            res.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            res.push(c);
        }
    }
    res
}

/// Converts a string to snake_case.
pub fn to_snake_case(s: &str) -> String {
    let mut res = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                res.push('_');
            }
            res.push(c.to_ascii_lowercase());
        } else {
            res.push(c);
        }
    }
    res.replace(['-', ' '], "_")
}

/// Converts a string to kebab-case.
pub fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}
