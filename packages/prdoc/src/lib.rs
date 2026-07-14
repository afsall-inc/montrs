//! # montrs-prdoc
//!
//! Structured PR documentation, auto-generation, changelog, and SemVer
//! bumping for Rust projects. Usable standalone — no MontRS framework
//! dependency required.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use montrs_prdoc::{PrDoc, parse_prdoc, validate_prdoc};
//!
//! let content = std::fs::read_to_string("prdoc.md").unwrap();
//! let prdoc = parse_prdoc(&content).unwrap();
//! let issues = validate_prdoc(&prdoc);
//! ```
//!
//! ## Auto-Generation
//!
//! ```rust,no_run
//! use montrs_prdoc::{analyze_diff, generate_prdoc, render_prdoc};
//!
//! let diff = std::fs::read_to_string("changes.diff").unwrap();
//! let analysis = analyze_diff(&diff);
//! let prdoc = generate_prdoc(&analysis, None);
//! let rendered = render_prdoc(&prdoc, &analysis);
//! std::fs::write("prdoc.md", rendered).unwrap();
//! ```
//!
//! ## Changelog
//!
//! ```rust,no_run
//! use montrs_prdoc::{Changelog, load_prdoc};
//!
//! let prdoc = load_prdoc(&std::path::Path::new("prdoc.md")).unwrap();
//! let mut changelog = Changelog::new();
//! changelog.add_prdoc(&prdoc);
//! let rendered = changelog.render();
//! std::fs::write("CHANGELOG.md", rendered).unwrap();
//! ```

pub mod analyzer;
pub mod changelog;
pub mod config;
pub mod embed;
pub mod generator;
#[cfg(feature = "llm")]
pub mod llm;
#[cfg(feature = "local-llm")]
pub mod local_llm;
pub mod summary;
pub mod types;

pub use analyzer::*;
pub use changelog::*;
pub use config::*;
pub use embed::*;
pub use generator::*;
pub use summary::*;
pub use types::*;
