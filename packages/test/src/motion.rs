// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Deterministic animation testing.
//!
//! Motion values in MontRS are pure functions of time, so a test can step a
//! virtual timeline frame by frame and assert exact values, settle times, and
//! whole-timeline snapshots — no real clock, no `requestAnimationFrame`.

use montrs_motion::{Keyframes, Spring, Tween};

/// A scalar animation sampled at a point in time (seconds).
pub trait ScalarAnimation {
    /// The animated value at time `t` seconds.
    fn sample_scalar(&self, t: f64) -> f64;

    /// The value the animation converges to (sampled far in the future).
    fn settled_value(&self) -> f64 {
        self.sample_scalar(1.0e6)
    }
}

impl ScalarAnimation for Spring {
    fn sample_scalar(&self, t: f64) -> f64 {
        self.solve(t)
    }
}

impl ScalarAnimation for Tween {
    fn sample_scalar(&self, t: f64) -> f64 {
        self.sample(t)
    }
}

impl ScalarAnimation for Keyframes {
    fn sample_scalar(&self, t: f64) -> f64 {
        self.sample(t)
    }
}

/// Steps an animation over a virtual timeline and records each frame.
pub struct MotionTest<A: ScalarAnimation> {
    animation: A,
    elapsed_ms: f64,
    timeline: Vec<(u64, f64)>,
}

impl<A: ScalarAnimation> MotionTest<A> {
    /// Start at time zero.
    pub fn new(animation: A) -> Self {
        let value = animation.sample_scalar(0.0);
        Self {
            animation,
            elapsed_ms: 0.0,
            timeline: vec![(0, value)],
        }
    }

    /// Start at a given elapsed time (milliseconds).
    pub fn at(animation: A, elapsed_ms: u64) -> Self {
        let t = elapsed_ms as f64 / 1000.0;
        let value = animation.sample_scalar(t);
        Self {
            animation,
            elapsed_ms: elapsed_ms as f64,
            timeline: vec![(elapsed_ms, value)],
        }
    }

    /// Advance the timeline by `ms` and return the value at the new time.
    pub fn step_ms(&mut self, ms: u64) -> f64 {
        self.elapsed_ms += ms as f64;
        let value = self.animation.sample_scalar(self.elapsed_ms / 1000.0);
        self.timeline.push((self.elapsed_ms as u64, value));
        value
    }

    /// The current value.
    pub fn value(&self) -> f64 {
        self.animation.sample_scalar(self.elapsed_ms / 1000.0)
    }

    /// The value at an absolute time (seconds), without advancing.
    pub fn value_at(&self, secs: f64) -> f64 {
        self.animation.sample_scalar(secs)
    }

    /// Elapsed virtual time in milliseconds.
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed_ms as u64
    }

    /// The recorded `(ms, value)` frames.
    pub fn timeline(&self) -> &[(u64, f64)] {
        &self.timeline
    }

    /// Record frames from the current time up to `end_ms`, in `step_ms` steps.
    pub fn record_until(
        &mut self,
        end_ms: u64,
        step_ms: u64,
    ) -> Vec<(u64, f64)> {
        assert!(step_ms > 0, "step must be non-zero");
        while self.elapsed_ms() < end_ms {
            self.step_ms(step_ms);
        }
        self.timeline.clone()
    }

    /// Assert the animation has converged to within `threshold` of its settled
    /// value by `ms` milliseconds.
    #[track_caller]
    pub fn assert_settles_within_ms(&self, ms: u64, threshold: f64) {
        let target = self.animation.settled_value();
        let value = self.animation.sample_scalar(ms as f64 / 1000.0);
        assert!(
            (value - target).abs() <= threshold,
            "expected settle within {ms}ms (|{value} - {target}| <= \
             {threshold})"
        );
    }

    /// Assert the current value is within `epsilon` of `expected`.
    #[track_caller]
    pub fn assert_value_close(&self, expected: f64, epsilon: f64) {
        let value = self.value();
        assert!(
            (value - expected).abs() <= epsilon,
            "expected {value} ≈ {expected} (±{epsilon})"
        );
    }

    /// A normalized string form of the recorded timeline, for snapshots.
    pub fn timeline_string(&self, decimals: usize) -> String {
        self.timeline
            .iter()
            .map(|(ms, v)| format!("{ms}ms={v:.decimals$}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spring_settles_deterministically() {
        let spring = Spring::new(100.0, 10.0, 1.0).with_range(0.0, 1.0);
        let mut motion = MotionTest::new(spring);

        let first = motion.step_ms(16);
        assert!(first > 0.0 && first < 1.0);

        motion.assert_settles_within_ms(2000, 0.01);
    }

    #[test]
    fn timeline_is_reproducible() {
        let spring = Spring::new(120.0, 12.0, 1.0).with_range(0.0, 1.0);
        let mut a = MotionTest::new(spring);
        let mut b = MotionTest::new(spring);
        assert_eq!(a.record_until(320, 16), b.record_until(320, 16));
    }

    #[test]
    fn tween_hits_endpoints() {
        let tween = Tween::new(0.0, 10.0, 1.0);
        let motion = MotionTest::new(tween);
        motion.assert_value_close(0.0, 1e-9);
        motion.assert_value_close(0.0, 1e-9);
        assert!((motion.value_at(1.0) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn keyframes_sample_between_points() {
        let kf = Keyframes::new(vec![0.0, 0.5, 1.0], vec![0.0, 100.0, 0.0]);
        let motion = MotionTest::new(kf);
        assert!((motion.value_at(0.5) - 100.0).abs() < 1e-6);
    }

    #[test]
    fn timeline_string_is_stable() {
        let spring = Spring::default();
        let mut motion = MotionTest::new(spring);
        motion.record_until(48, 16);
        let rendered = motion.timeline_string(3);
        assert!(rendered.starts_with("0ms="));
        assert!(rendered.contains("48ms="));
    }
}
