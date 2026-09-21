// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Deterministic kernel for the MontRS test fabric.
//!
//! Everything here is hermetic: a controllable [`clock::Clock`], a seeded
//! [`rng::Rng`], and a [`harness::TestHarness`] that ties them to an app spec.

pub mod clock;
pub mod harness;
pub mod rng;

pub use clock::{Clock, SystemClock, TestClock};
pub use harness::TestHarness;
pub use rng::{Rng, TestRng};
