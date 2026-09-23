// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! End-to-end checks for the `montrs::test` / `montrs::suite!` macros.

#![cfg(feature = "macros")]

use montrs_core::{AppConfig, AppSpec, EnvConfig, env::EnvError};

#[derive(Clone)]
struct Env;

impl EnvConfig for Env {
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        Err(EnvError::MissingKey(key.to_string()))
    }
}

#[derive(Clone)]
struct Cfg;

impl AppConfig for Cfg {
    type Error = std::io::Error;
    type Env = Env;
}

fn build_spec() -> AppSpec<Cfg> {
    let mut spec = AppSpec::new(Cfg, Env);
    spec.router.register(DemoRoute);
    spec
}

struct DemoView;
impl montrs_core::RouteView for DemoView {
    fn render(&self) -> impl montrs_core::IntoView {
        {}
    }
}

struct DemoRoute;
impl montrs_core::Route<Cfg> for DemoRoute {
    type Params = montrs_core::NoParams;
    type Loader = montrs_core::NoopLoader;
    type Action = montrs_core::NoopAction;
    type View = DemoView;
    fn path() -> &'static str {
        "/demo"
    }
    fn loader(&self) -> Self::Loader {
        montrs_core::NoopLoader
    }
    fn action(&self) -> Self::Action {
        montrs_core::NoopAction
    }
    fn view(&self) -> Self::View {
        DemoView
    }
}

// `harness` is injected and runnable from an async test body.
#[montrs_test::test(build_spec)]
async fn harness_is_injected() {
    let _ = harness.rng.next_u64();
    assert!(spec.router.route("/demo").is_some());
}

// Sync test bodies work too.
#[montrs_test::test(build_spec)]
fn sync_tests_are_supported() {
    harness.advance_ms(100);
}

// `?` in an async body is reported by `assert_passed`.
#[montrs_test::test(build_spec)]
async fn fallible_bodies_propagate_errors() {
    Ok::<(), std::io::Error>(())
}

montrs_test::suite!(build_spec);
