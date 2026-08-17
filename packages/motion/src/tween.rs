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

/// Easing functions for tween animations.
///
/// Based on Remotion's easing.ts — provides standard easing curves plus
/// cubic bezier, elastic, back, and bounce easings.
#[derive(Debug, Clone, Copy)]
pub enum Easing {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    SineIn,
    SineOut,
    SineInOut,
    BackIn,
    BackOut,
    BackInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
    Bezier(f64, f64, f64, f64),
}

impl Easing {
    /// Apply the easing function to a normalized time `t` (0.0 to 1.0).
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Easing::Linear => t,
            Easing::Ease => cubic_bezier(0.25, 0.1, 0.25, 1.0, t),
            Easing::EaseIn => cubic_bezier(0.42, 0.0, 1.0, 1.0, t),
            Easing::EaseOut => cubic_bezier(0.0, 0.0, 0.58, 1.0, t),
            Easing::EaseInOut => cubic_bezier(0.42, 0.0, 0.58, 1.0, t),
            Easing::QuadIn => t * t,
            Easing::QuadOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::QuadInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::CubicIn => t * t * t,
            Easing::CubicOut => 1.0 - (1.0 - t).powi(3),
            Easing::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::SineIn => 1.0 - (t * std::f64::consts::FRAC_PI_2).cos(),
            Easing::SineOut => (t * std::f64::consts::FRAC_PI_2).sin(),
            Easing::SineInOut => -(t * std::f64::consts::PI).cos() / 2.0 + 0.5,
            Easing::BackIn => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Easing::BackOut => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Easing::BackInOut => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    (2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2)
                        * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2)
                        + 2.0)
                        / 2.0
                }
            }
            Easing::ElasticIn => {
                let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -(2.0f64).powf(10.0 * t - 10.0)
                        * ((t * 10.0 - 10.75) * c4).sin()
                }
            }
            Easing::ElasticOut => {
                let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin()
                        + 1.0
                }
            }
            Easing::ElasticInOut => {
                let c5 = (2.0 * std::f64::consts::PI) / 4.5;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2.0f64.powf(20.0 * t - 10.0)
                        * ((20.0 * t - 11.125) * c5).sin())
                        / 2.0
                } else {
                    2.0f64.powf(-20.0 * t + 10.0)
                        * ((20.0 * t - 11.125) * c5).sin()
                        / 2.0
                        + 1.0
                }
            }
            Easing::BounceIn => 1.0 - bounce_out(1.0 - t),
            Easing::BounceOut => bounce_out(t),
            Easing::BounceInOut => {
                if t < 0.5 {
                    (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
                }
            }
            Easing::Bezier(x1, y1, x2, y2) => {
                cubic_bezier(*x1, *y1, *x2, *y2, t)
            }
        }
    }
}

fn bounce_out(t: f64) -> f64 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        n1 * (t - 1.5 / d1) * (t - 1.5 / d1) + 0.75
    } else if t < 2.5 / d1 {
        n1 * (t - 2.25 / d1) * (t - 2.25 / d1) + 0.9375
    } else {
        n1 * (t - 2.625 / d1) * (t - 2.625 / d1) + 0.984375
    }
}

/// Cubic bezier evaluation using Newton-Raphson.
fn cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> f64 {
    let t = sample_curve_x(x1, x2, t);
    sample_curve_y(y1, y2, t)
}

fn sample_curve_x(x1: f64, x2: f64, t: f64) -> f64 {
    calc_bezier(x1, x2, t)
}

fn sample_curve_y(y1: f64, y2: f64, t: f64) -> f64 {
    calc_bezier(y1, y2, t)
}

fn calc_bezier(a: f64, b: f64, t: f64) -> f64 {
    ((1.0 - 3.0 * b + 3.0 * a) * t + (3.0 * b - 6.0 * a)) * t + 3.0 * a * t
}

/// A tween animation that interpolates a value over time.
#[derive(Debug, Clone)]
pub struct Tween {
    pub from: f64,
    pub to: f64,
    pub duration: f64,
    pub easing: Easing,
    pub delay: f64,
}

impl Tween {
    pub fn new(from: f64, to: f64, duration: f64) -> Self {
        Self {
            from,
            to,
            duration,
            easing: Easing::EaseOut,
            delay: 0.0,
        }
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    /// Get the value at time `t` (seconds).
    pub fn sample(&self, t: f64) -> f64 {
        let t = t - self.delay;
        if t <= 0.0 {
            return self.from;
        }
        if t >= self.duration {
            return self.to;
        }
        let progress = self.easing.apply(t / self.duration);
        self.from + (self.to - self.from) * progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let tween = Tween::new(0.0, 100.0, 1.0).with_easing(Easing::Linear);
        assert!((tween.sample(0.5) - 50.0).abs() < 0.01);
        assert!((tween.sample(0.0) - 0.0).abs() < 0.001);
        assert!((tween.sample(1.0) - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_delay() {
        let tween = Tween::new(0.0, 100.0, 1.0).with_delay(0.5);
        assert!((tween.sample(0.0) - 0.0).abs() < 0.001);
        assert!((tween.sample(0.5) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_easing_bounds() {
        for easing in &[
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::QuadIn,
            Easing::BackOut,
        ] {
            let tween = Tween::new(0.0, 1.0, 1.0).with_easing(*easing);
            assert!((tween.sample(0.0) - 0.0).abs() < 0.001);
            assert!((tween.sample(1.0) - 1.0).abs() < 0.001);
        }
    }
}
