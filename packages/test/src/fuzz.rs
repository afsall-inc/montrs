// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Property-based testing and fuzzing for MontRS applications.
//!
//! Exposes proptest strategies for routes, inputs, and payloads to verify
//! that handlers, actions, and loaders gracefully handle edge-cases and
//! never crash unexpectedly.

pub use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};

/// Run a property test runner over generated inputs.
pub fn run_fuzz<
    T: std::fmt::Debug,
    S: Strategy<Value = T>,
    F: Fn(T) -> Result<(), TestCaseError>,
>(
    strategy: S,
    cases: u32,
    f: F,
) -> Result<(), String> {
    let config = Config {
        cases,
        ..Default::default()
    };
    let mut runner = TestRunner::new(config);
    runner
        .run(&strategy, f)
        .map_err(|e| format!("Fuzz test failed: {:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_fuzz_property() {
        let res = run_fuzz(any::<i32>(), 100, |val| {
            let double = val.wrapping_mul(2);
            if val != 0 && double == val {
                return Err(TestCaseError::fail("unexpected fixed point"));
            }
            Ok(())
        });
        assert!(res.is_ok());
    }
}
