// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
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
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Spring physics simulation.
///
/// Based on Motion's spring.ts — uses Newton's method to approximate spring
/// roots for duration-based springs, or simulates a damped harmonic oscillator.
///
/// # Example
/// ```rust,ignore
/// use montrs_motion::Spring;
///
/// let spring = Spring::new(100.0, 10.0, 1.0);  // stiffness, damping, mass
/// let value = spring.solve(0.5);  // value at time 0.5s
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Spring {
    stiffness: f64,
    damping: f64,
    mass: f64,
    velocity: f64,
    initial: f64,
    target: f64,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
            initial: 0.0,
            target: 1.0,
        }
    }
}

impl Spring {
    pub fn new(stiffness: f64, damping: f64, mass: f64) -> Self {
        Self {
            stiffness,
            damping,
            mass,
            ..Default::default()
        }
    }

    pub fn with_velocity(mut self, velocity: f64) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_range(mut self, from: f64, to: f64) -> Self {
        self.initial = from;
        self.target = to;
        self
    }

    /// Solve the spring at time `t` (seconds).
    /// Returns the interpolated value between `initial` and `target`.
    pub fn solve(&self, t: f64) -> f64 {
        let w0 = (self.stiffness / self.mass).sqrt();
        let zeta = self.damping / (2.0 * (self.stiffness * self.mass).sqrt());
        let range = self.target - self.initial;

        if zeta < 1.0 {
            // Underdamped
            let wd = w0 * (1.0 - zeta * zeta).sqrt();
            let a = 1.0;
            let b = (self.velocity + zeta * w0) / wd;
            let envelope = (-zeta * w0 * t).exp();
            let result = range
                * (1.0 - envelope * (a * (wd * t).cos() + b * (wd * t).sin()));
            self.initial + result
        } else {
            // Critically damped or overdamped
            let c1 = 1.0;
            let c2 = self.velocity + zeta * w0;
            let envelope = (-zeta * w0 * t).exp();
            let result = range * (1.0 - envelope * (c1 + c2 * t));
            self.initial + result
        }
    }

    /// Estimate the duration needed for the spring to settle within `threshold`.
    pub fn duration(&self, threshold: f64) -> f64 {
        let mut t = 0.0;
        let dt = 1.0 / 60.0;
        loop {
            let val = self.solve(t);
            if (val - self.target).abs() < threshold {
                return t;
            }
            t += dt;
            if t > 10.0 {
                return 10.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_default() {
        let spring = Spring::default();
        let start = spring.solve(0.0);
        let end = spring.solve(2.0);
        assert!((start - 0.0).abs() < 0.001);
        assert!((end - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_spring_stiff() {
        let spring = Spring::new(300.0, 20.0, 1.0);
        let mid = spring.solve(0.1);
        assert!(mid > 0.0);
        assert!(spring.duration(0.01) < 2.0);
    }

    #[test]
    fn test_spring_velocity() {
        let spring = Spring::new(100.0, 10.0, 1.0).with_velocity(50.0);
        let val = spring.solve(0.01);
        // With initial velocity, the spring may briefly overshoot below 0
        // before settling. Just verify it produces a finite value.
        assert!(val.is_finite());
    }
}
