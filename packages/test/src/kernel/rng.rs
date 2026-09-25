// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Deterministic randomness for tests.
//!
//! [`Rng`] is a small injectable source of pseudo-randomness. [`TestRng`] is
//! seeded, so a test that seeds it produces the same sequence on every run and
//! every machine.

use std::sync::{Arc, Mutex};

/// A source of pseudo-random values.
pub trait Rng: Send + Sync + 'static {
    /// The next raw `u64`.
    fn next_u64(&self) -> u64;

    /// A value in `[0, bound)`. Panics if `bound == 0`.
    fn gen_range(&self, bound: u64) -> u64 {
        assert!(bound > 0, "bound must be non-zero");
        self.next_u64() % bound
    }

    /// A value in `[0.0, 1.0)`.
    fn gen_f64(&self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// A deterministic, seeded [`Rng`] using SplitMix64.
#[derive(Debug, Clone)]
pub struct TestRng {
    state: Arc<Mutex<u64>>,
}

impl TestRng {
    /// Create a generator from a seed.
    pub fn seed(seed: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new(seed)),
        }
    }
}

impl Default for TestRng {
    fn default() -> Self {
        Self::seed(0x9E37_79B9_7F4A_7C15)
    }
}

impl Rng for TestRng {
    fn next_u64(&self) -> u64 {
        let mut state = self.state.lock().unwrap();
        *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let a = TestRng::seed(42);
        let b = TestRng::seed(42);
        for _ in 0..8 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seeds_diverge() {
        let a = TestRng::seed(1);
        let b = TestRng::seed(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn floats_are_in_unit_interval() {
        let r = TestRng::seed(7);
        for _ in 0..1000 {
            let f = r.gen_f64();
            assert!((0.0..1.0).contains(&f));
        }
    }

    #[test]
    fn ranges_stay_in_bounds() {
        let r = TestRng::seed(3);
        for _ in 0..1000 {
            assert!(r.gen_range(10) < 10);
        }
    }
}
