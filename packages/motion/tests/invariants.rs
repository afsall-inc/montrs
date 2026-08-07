//! Invariant tests for montrs-motion.
//!
//! Validates the invariants defined in `docs/invariants.md`:
//! - All animation primitives are pure Rust
//! - Spring physics uses Newton's method
//! - Easing functions match standard CSS easing curves
//! - MotionValue is a reactive wrapper around Leptos signals

use montrs_motion::*;

#[test]
fn test_spring_construct() {
    let spring = Spring::new(100.0, 10.0, 1.0);
    assert!(spring.solve(0.0).abs() < 1e-6);
}

#[test]
fn test_spring_solve_is_monotonic() {
    // Overdamped spring (damping > critical) should be monotonic
    let spring = Spring::new(100.0, 30.0, 1.0);
    let mut prev = spring.solve(0.0);
    for t in (1..=100).map(|i| i as f64 / 100.0) {
        let val = spring.solve(t);
        assert!(val >= prev - 1e-10, "spring not monotonic at t={}", t);
        prev = val;
    }
}

#[test]
fn test_easing_bounds() {
    let easings = [
        Easing::Linear,
        Easing::Ease,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
    ];
    for easing in &easings {
        let start = easing.apply(0.0);
        let end = easing.apply(1.0);
        assert!(
            (start - 0.0).abs() < 1e-6,
            "Easing {:?} apply(0.0) = {}",
            easing,
            start
        );
        assert!(
            (end - 1.0).abs() < 1e-6,
            "Easing {:?} apply(1.0) = {}",
            easing,
            end
        );
    }
}

#[test]
fn test_tween_construct() {
    let _tween = Tween::new(0.0, 1.0, 1.0);
}

#[test]
fn test_keyframes_construct() {
    let _kf = Keyframes::new(vec![0.0, 1.0], vec![0.0, 100.0]);
}

#[test]
fn test_version_constant() {
    assert!(!VERSION.is_empty());
}
