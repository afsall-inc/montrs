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
//! let content = std::fs::read_to_string("prdoc/pr_1.prdoc").unwrap();
//! let prdoc = parse_prdoc(&content).unwrap();
//! let issues = validate_prdoc(&prdoc);
//! ```
//!
//! ## Auto-Generation
//!
//! ```rust,no_run
//! use montrs_prdoc::{
//!     GenerateOptions, generate_prdoc, render_prdoc,
//!     types::{Audience, BumpLevel},
//! };
//!
//! let opts = GenerateOptions {
//!     pr_number: 42,
//!     bump: BumpLevel::Minor,
//!     audience: Audience::AppDev,
//!     force: false,
//! };
//! let prdoc = generate_prdoc(&opts).unwrap();
//! let rendered = render_prdoc(&prdoc);
//! std::fs::write("prdoc/pr_42.prdoc", rendered).unwrap();
//! ```

pub mod analyzer;
pub mod changelog;
pub mod config;
pub mod generator;
pub mod types;

pub use analyzer::*;
pub use changelog::*;
pub use config::*;
pub use generator::*;
pub use types::*;
