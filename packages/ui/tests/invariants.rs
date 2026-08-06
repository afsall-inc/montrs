//! Invariant tests for montrs-ui.

#[test]
fn test_prelude_imports() {
    let _ = montrs_ui::prelude::ThemeMode::Light;
    let _ = montrs_ui::prelude::ThemeMode::Dark;
}

#[test]
fn test_cn_module_exists() {
    let _ = montrs_ui::cn;
}

#[test]
fn test_clx_module_exists() {
    let _ = montrs_ui::clx;
}

#[test]
fn test_theme_module_exists() {
    let _ = montrs_ui::theme;
}

#[test]
fn test_variants_module_exists() {
    let _ = montrs_ui::variants;
}
