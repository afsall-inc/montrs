use montrs_fmt::{FormatterSettings, format_source};

#[test]
fn test_integration_full_formatting() {
    let source = r#"
fn main() {
    // Top level comment
    let x = 1;
    view! {
        <div class="container">
            // Nested comment
            <span>"Hello MontRS"</span>
        </div>
    };
}
"#;

    let settings = FormatterSettings::default();
    let result = format_source(source, &settings).expect("Formatting failed");

    // Check for preservation of essential elements
    assert!(result.contains("fn main()"), "Function signature lost");
    assert!(
        result.contains("// Top level comment"),
        "Top level comment lost"
    );
    assert!(result.contains("// Nested comment"), "Nested comment lost");
    assert!(
        result.contains(r#"class="container""#),
        "Macro attribute lost"
    );
    assert!(result.contains("Hello MontRS"), "Macro text content lost");
}

#[test]
fn test_integration_no_macros() {
    let source = "fn add(a: i32, b: i32) -> i32 { a + b }";
    let settings = FormatterSettings::default();
    let result = format_source(source, &settings).expect("Formatting failed");

    assert!(result.contains("fn add(a: i32, b: i32) -> i32 {"));
    assert!(result.contains("a + b"));
}
