//! Weight utilities for resource accounting and performance modeling.
//!
//! Inspired by Substrate's weight system, this plate provides tools to turn
//! benchmark results into actionable application logic.

/// Represents the computational cost of an operation.
///
/// A weight is modeled as a linear function: `cost = base + (slope * n)`
/// where `n` is a parameter representing the complexity of the input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weight {
    /// Fixed overhead in nanoseconds.
    pub base_ns: u64,
    /// Cost per unit of the parameter in nanoseconds.
    pub slope_ns: u64,
}

impl Weight {
    /// Creates a new `Weight` from nanosecond values.
    pub const fn from_ns(base_ns: u64, slope_ns: u64) -> Self {
        Self { base_ns, slope_ns }
    }

    /// Calculates the total cost for a given parameter value.
    pub fn calc(&self, n: u32) -> u64 {
        self.base_ns
            .saturating_add(self.slope_ns.saturating_mul(n as u64))
    }

    /// Returns the base overhead in nanoseconds.
    pub fn base(&self) -> u64 {
        self.base_ns
    }

    /// Returns the slope (cost per unit) in nanoseconds.
    pub fn slope(&self) -> u64 {
        self.slope_ns
    }
}
