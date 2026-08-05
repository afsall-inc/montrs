use leptos::prelude::*;
use montrs_icons::{Glyph, Icon, strum::IntoEnumIterator};

#[test]
fn find_returns_results() {
    let results = Glyph::find("arrow");
    assert!(!results.is_empty());
}

#[test]
fn find_empty_returns_all() {
    let all = Glyph::find("");
    assert_eq!(all.len(), Glyph::count());
}

#[test]
fn find_case_insensitive() {
    let lower = Glyph::find("arrow");
    let upper = Glyph::find("Arrow");
    let mixed = Glyph::find("ARROW");
    assert_eq!(lower.len(), upper.len());
    assert_eq!(lower.len(), mixed.len());
}

#[test]
fn by_name_known_icon() {
    assert!(Glyph::by_name("Search").is_some());
}

#[test]
fn by_name_accepts_kebab_case() {
    assert_eq!(Glyph::by_name("arrow-right"), Glyph::by_name("ArrowRight"));
}

#[test]
fn by_name_unknown_returns_none() {
    assert!(Glyph::by_name("NotAnIcon").is_none());
}

#[test]
fn name_is_pascal_case() {
    for g in Glyph::iter() {
        let n = g.name();
        assert!(!n.contains('-'), "name contains hyphen: {n}");
        assert!(!n.contains(' '), "name contains space: {n}");
        assert!(
            n.chars().next().is_some_and(|c| c.is_uppercase()),
            "name does not start uppercase: {n}"
        );
    }
}

#[test]
fn kebab_name_format() {
    let icon = Glyph::by_name("ArrowRight").unwrap();
    assert_eq!(icon.kebab_name(), "arrow-right");
}

#[test]
fn count_matches_iter() {
    assert_eq!(Glyph::count(), Glyph::iter().count());
}

#[test]
fn count_is_positive() {
    assert!(Glyph::count() > 600);
}

#[test]
fn all_icons_have_svg() {
    for g in Glyph::iter() {
        let svg = g.svg();
        assert!(!svg.is_empty(), "empty svg for {}", g.name());
        assert!(svg.contains('<'), "svg has no elements for {}", g.name());
    }
}

#[test]
fn all_icons_have_at_least_one_category() {
    for g in Glyph::iter() {
        assert!(g.categories().count() > 0, "no category for {}", g.name());
    }
}

#[test]
fn glyph_trait_returns_svg() {
    let icon = Glyph::by_name("Search").unwrap();
    let svg = icon.svg();
    assert!(svg.contains("<"));
    assert!(svg.contains("path"));
}
