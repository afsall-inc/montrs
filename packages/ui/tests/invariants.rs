//! Invariant tests for montrs-ui.

#[test]
fn test_prelude_imports() {
    let _ = montrs_ui::prelude::ThemeMode::Light;
    let _ = montrs_ui::prelude::ThemeMode::Dark;
}

#[test]
fn test_cn_macro_usable() {
    let cls = montrs_ui::cn::cn!("px-4", "text-red-500");
    assert!(!cls.is_empty());
}

#[test]
fn test_theme_types_exist() {
    fn _assert_type<T>() {}
    _assert_type::<montrs_ui::theme::provider::ThemeMode>();
}

#[test]
fn test_modules_accessible() {
    // Verify the module paths resolve (compile-time check)
    #[allow(unused_imports)]
    use montrs_ui as _ui;
}
