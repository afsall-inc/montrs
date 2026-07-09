use montrs_agent::error_parser::parse_rustc_errors;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_code_context_reading() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("main.rs");

    let content = r#"fn main() {
    let x = 5;
    println!("{}", x)
    let y = 10;
}
"#;
    fs::write(&file_path, content).unwrap();

    // Mock rustc output pointing to the temp file
    let rustc_output = format!(
        "error[E0425]: cannot find value `z` in this scope\n  --> {}:3:5",
        file_path.to_str().unwrap()
    );

    let errors = parse_rustc_errors(&rustc_output);

    assert_eq!(errors.len(), 1);
    let error = &errors[0];

    // Check that code context was read and includes the line number and marker
    assert!(
        error
            .code_context
            .contains(">    3 |     println!(\"{}\", x)")
    );
    assert!(error.code_context.contains("     2 |     let x = 5;"));
    assert!(error.code_context.contains("     4 |     let y = 10;"));
}
