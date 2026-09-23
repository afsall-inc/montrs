// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Procedural macros for the MontRS deterministic test fabric.
//!
//! - [`macro@test`] (`#[montrs::test]`) — a test that gets a [`TestHarness`] for
//!   free and runs async or sync.
//! - [`macro@suite`] (`montrs::suite!`) — generate smoke tests for an app.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Turn a function into a test with a [`TestHarness`] in scope.
///
/// The function body gets a `harness` binding built from the app's
/// `build_spec()`. Parameters are removed; use `harness` instead.
///
/// ```rust,ignore
/// #[montrs::test]
/// async fn health(harness: &TestHarness<MyConfig>) {
/// ```
///
/// runs `build_spec()`. To use a different factory:
///
/// ```rust,ignore
/// #[montrs::test(website::build_spec)]
/// async fn health() {
///     let res = harness.load("/health").await.unwrap();
/// }
/// ```
///
/// `harness` is the `TestHarness`; `spec` is a reference to its `AppSpec`.
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let spec_path: syn::ExprPath = if attr.is_empty() {
        syn::parse_quote!(crate::build_spec)
    } else {
        parse_macro_input!(attr as syn::ExprPath)
    };

    let input = parse_macro_input!(item as ItemFn);
    let attrs = input.attrs;
    let vis = input.vis;
    let ident = input.sig.ident;
    let is_async = input.sig.asyncness.is_some();
    let stmts = input.block.stmts;

    let body = quote! {
        let harness = montrs_test::TestHarness::new(#spec_path());
        let spec = &harness.spec;
        #(#stmts)*
    };

    let wrapped = if is_async {
        quote! {
            let outcome = montrs_test::block_on(async move { #body });
            montrs_test::assert_passed(outcome);
        }
    } else {
        body
    };

    quote! {
        #(#attrs)*
        #[test]
        #vis fn #ident() {
            #wrapped
        }
    }
    .into()
}

/// Generate smoke tests for an app.
///
/// `montrs::suite!(website::build_spec)` generates a `montrs_generated_suite`
/// module that asserts the spec builds and every registered route resolves.
#[proc_macro]
pub fn suite(input: TokenStream) -> TokenStream {
    let spec_path: syn::ExprPath = parse_macro_input!(input as syn::ExprPath);
    quote! {
        #[cfg(test)]
        mod montrs_generated_suite {
            use super::*;

            #[test]
            fn app_spec_builds_and_routes_resolve() {
                let harness = montrs_test::TestHarness::new(#spec_path());
                let routes = harness.spec.router.spec().routes;
                assert!(
                    !routes.is_empty(),
                    "the app registered no routes"
                );
                for path in routes.keys() {
                    assert!(
                        harness.spec.router.route(path).is_some(),
                        "route {:?} does not resolve",
                        path
                    );
                }
            }
        }
    }
    .into()
}
