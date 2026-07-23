// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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

