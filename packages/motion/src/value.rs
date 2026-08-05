use leptos::prelude::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

/// A reactive animation value with velocity tracking.
///
/// Like Framer Motion's `useMotionValue` or Motion's `MotionValue`.
/// Provides a reactive signal that smoothly transitions between values,
/// with built-in velocity tracking for gesture-based animations.
#[derive(Debug, Clone)]
pub struct MotionValue {
    value: RwSignal<f64>,
    velocity: Arc<AtomicU64>, // stored as f64 bits for atomic access
    target: RwSignal<f64>,
    is_animating: Arc<AtomicBool>,
}

impl MotionValue {
    pub fn new(initial: f64) -> Self {
        Self {
            value: RwSignal::new(initial),
            velocity: Arc::new(AtomicU64::new(initial.to_bits())),
            target: RwSignal::new(initial),
            is_animating: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get the current value.
    pub fn get(&self) -> f64 {
        self.value.get()
    }

    /// Get a read signal for reactive use.
    pub fn read_signal(&self) -> ReadSignal<f64> {
        self.value.read_only()
    }

    /// Set the value immediately (no animation).
    pub fn jump(&self, value: f64) {
        self.value.set(value);
        self.target.set(value);
        self.velocity.store(value.to_bits(), Ordering::SeqCst);
        self.is_animating.store(false, Ordering::SeqCst);
    }

    /// Start a spring animation toward a target.
    pub fn animate_to(
        &self,
        target: f64,
        stiffness: f64,
        damping: f64,
        mass: f64,
    ) {
        self.target.set(target);
        self.is_animating.store(true, Ordering::SeqCst);

        let value = self.value;
        let velocity = self.velocity.clone();
        let is_animating = self.is_animating.clone();
        let start = value.get();
        let current_velocity = f64::from_bits(velocity.load(Ordering::SeqCst));

        let spring = crate::Spring::new(stiffness, damping, mass)
            .with_range(start, target)
            .with_velocity(current_velocity);

        let start_time = crate::FrameLoop::now();
        let is_animating_clone = is_animating.clone();

        crate::FrameLoop::on_frame(move || {
            if !is_animating_clone.load(Ordering::SeqCst) {
                return false; // stop
            }
            let elapsed = crate::FrameLoop::now() - start_time;
            let v = spring.solve(elapsed);
            value.set(v);
            // estimate velocity
            let prev = f64::from_bits(velocity.load(Ordering::SeqCst));
            let vel = (v - prev) * 60.0; // rough fps
            velocity.store(v.to_bits(), Ordering::SeqCst);

            if (v - target).abs() < 0.001 && vel.abs() < 0.01 {
                value.set(target);
                is_animating_clone.store(false, Ordering::SeqCst);
                false // stop
            } else {
                true // continue
            }
        });
    }

    /// Get the current velocity estimate.
    pub fn velocity(&self) -> f64 {
        f64::from_bits(self.velocity.load(Ordering::SeqCst))
    }
}

/// A convenience wrapper for creating spring-animated signal values.
pub fn use_motion_value(initial: f64) -> MotionValue {
    MotionValue::new(initial)
}
