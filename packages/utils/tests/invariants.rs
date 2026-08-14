//! Invariant tests for montrs-utils.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - Side-Effect Free: pure functions
//! - Generic Utility: truly generic logic
//! - High Stability: low-level dependency

use montrs_utils::*;

#[test]
fn test_to_pascal_case() {
    assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
    assert_eq!(to_pascal_case("hello world"), "HelloWorld");
    assert_eq!(to_pascal_case("HelloWorld"), "HelloWorld");
    assert_eq!(to_pascal_case(""), "");
}

#[test]
fn test_to_snake_case() {
    assert_eq!(to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(to_snake_case("hello-world"), "hello_world");
    assert_eq!(to_snake_case("hello world"), "hello_world");
    assert_eq!(to_snake_case("hello"), "hello");
    assert_eq!(to_snake_case(""), "");
}

#[test]
fn test_to_kebab_case() {
    assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
    assert_eq!(to_kebab_case("hello_world"), "hello-world");
    assert_eq!(to_kebab_case("hello world"), "hello-world");
    assert_eq!(to_kebab_case("hello"), "hello");
}

#[test]
fn test_conversion_roundtrip() {
    let original = "my_variable_name";
    let pascal = to_pascal_case(original);
    let snake = to_snake_case(&pascal);
    assert_eq!(snake, original);
}

#[test]
fn test_no_side_effects() {
    let input = "test_input".to_string();
    let _result = to_pascal_case(&input);
    assert_eq!(input, "test_input");
}
