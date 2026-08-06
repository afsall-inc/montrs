//! Invariant tests for montrs-prdoc.

use montrs_prdoc::{types::*, *};

#[test]
fn test_parse_prdoc() {
    let content = r#"---
title: Add new feature
pr: 1
doc:
  - audience: "App Dev"
    description: "A description of the feature."
crates:
  - name: montrs-core
    bump: minor
---"#;
    let prdoc = parse_prdoc(content).expect("failed to parse prdoc");
    assert_eq!(prdoc.title, "Add new feature");
    assert_eq!(prdoc.pr, Some(1));
    assert_eq!(prdoc.doc.len(), 1);
    assert_eq!(prdoc.crates.len(), 1);
    assert_eq!(prdoc.crates[0].name, "montrs-core");
    assert_eq!(prdoc.crates[0].bump, BumpLevel::Minor);
}

#[test]
fn test_validate_prdoc_valid() {
    let prdoc = PrDoc {
        title: "Fix bug".to_string(),
        author: None,
        pr: Some(42),
        doc: vec![DocSection {
            audience: Audience::AppDev,
            description: "Fixes a critical bug".to_string(),
            title: None,
        }],
        crates: vec![CrateChange {
            name: "montrs-core".to_string(),
            bump: BumpLevel::Patch,
            validate: true,
            note: None,
        }],
        migrations: None,
        host_functions: None,
    };
    let issues = validate_prdoc(&prdoc);
    assert!(issues.is_empty());
}

#[test]
fn test_validate_prdoc_missing_title() {
    let prdoc = PrDoc {
        title: "".to_string(),
        author: None,
        pr: None,
        doc: vec![DocSection {
            audience: Audience::AppDev,
            description: "desc".to_string(),
            title: None,
        }],
        crates: vec![CrateChange {
            name: "montrs-core".to_string(),
            bump: BumpLevel::Patch,
            validate: true,
            note: None,
        }],
        migrations: None,
        host_functions: None,
    };
    let issues = validate_prdoc(&prdoc);
    assert!(!issues.is_empty());
    assert!(issues.iter().any(|i| i.contains("title")));
}

#[test]
fn test_bump_level_as_str() {
    assert_eq!(BumpLevel::Major.as_str(), "major");
    assert_eq!(BumpLevel::Minor.as_str(), "minor");
    assert_eq!(BumpLevel::Patch.as_str(), "patch");
    assert_eq!(BumpLevel::None.as_str(), "none");
}

#[test]
fn test_bump_level_from_str_lossy() {
    assert_eq!(BumpLevel::from_str_lossy("major"), BumpLevel::Major);
    assert_eq!(BumpLevel::from_str_lossy("minor"), BumpLevel::Minor);
    assert_eq!(BumpLevel::from_str_lossy("patch"), BumpLevel::Patch);
    assert_eq!(BumpLevel::from_str_lossy("unknown"), BumpLevel::None);
}

#[test]
fn test_bump_level_dominates() {
    assert!(BumpLevel::Major.dominates(&BumpLevel::Minor));
    assert!(BumpLevel::Minor.dominates(&BumpLevel::Patch));
    assert!(BumpLevel::Patch.dominates(&BumpLevel::None));
    assert!(!BumpLevel::Patch.dominates(&BumpLevel::Major));
}

#[test]
fn test_audience_from_str_lossy() {
    assert_eq!(
        Audience::from_str_lossy("FrameworkDev"),
        Audience::FrameworkDev
    );
    assert_eq!(
        Audience::from_str_lossy("frameworkdev"),
        Audience::FrameworkDev
    );
    assert_eq!(Audience::from_str_lossy("AppDev"), Audience::AppDev);
    assert_eq!(Audience::from_str_lossy("appdev"), Audience::AppDev);
    assert_eq!(Audience::from_str_lossy("Operator"), Audience::Operator);
}

#[test]
fn test_audience_as_str() {
    assert_eq!(Audience::FrameworkDev.as_str(), "Framework Dev");
    assert_eq!(Audience::AppDev.as_str(), "App Dev");
}

#[test]
fn test_audience_all() {
    let all = Audience::all();
    assert_eq!(all.len(), 4);
}

#[test]
fn test_prdoc_config_default() {
    let config = config::PrdocConfig::default();
    assert_eq!(config.generate.default_output, "");
}

#[test]
fn test_load_config_missing() {
    let dir = tempfile::tempdir().unwrap();
    let config = config::load_config(dir.path());
    assert_eq!(config.generate.default_output, "");
}
