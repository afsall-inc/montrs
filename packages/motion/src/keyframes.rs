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

/// Multi-keyframe interpolation with per-segment easing.
///
/// Maps input times to output values using the specified easing per segment.
/// Inspired by Remotion's interpolate() and Motion's keyframes.ts.
#[derive(Debug, Clone)]
pub struct Keyframes {
    input: Vec<f64>,
    output: Vec<f64>,
    easings: Vec<crate::tween::Easing>,
    extrapolate_left: Extrapolate,
    extrapolate_right: Extrapolate,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Extrapolate {
    Extend,
    Clamp,
    Identity,
}

impl Keyframes {
    pub fn new(input: Vec<f64>, output: Vec<f64>) -> Self {
        let count = input.len().min(output.len());
        let easings = vec![crate::tween::Easing::EaseInOut; count - 1];
        Self {
            input: input.into_iter().take(count).collect(),
            output: output.into_iter().take(count).collect(),
            easings,
            extrapolate_left: Extrapolate::Extend,
            extrapolate_right: Extrapolate::Extend,
        }
    }

    pub fn with_easings(mut self, easings: Vec<crate::tween::Easing>) -> Self {
        self.easings = easings;
        self
    }

    pub fn with_extrapolate(
        mut self,
        left: Extrapolate,
        right: Extrapolate,
    ) -> Self {
        self.extrapolate_left = left;
        self.extrapolate_right = right;
        self
    }

    /// Sample the keyframe at input value `t`.
    pub fn sample(&self, t: f64) -> f64 {
        let len = self.input.len();
        if len == 0 {
            return 0.0;
        }
        if len == 1 {
            return self.output[0];
        }

        if t <= self.input[0] {
            return match self.extrapolate_left {
                Extrapolate::Clamp => self.output[0],
                Extrapolate::Identity => t,
                Extrapolate::Extend => {
                    let slope = (self.output[1] - self.output[0])
                        / (self.input[1] - self.input[0]);
                    self.output[0] + slope * (t - self.input[0])
                }
            };
        }
        if t >= self.input[len - 1] {
            return match self.extrapolate_right {
                Extrapolate::Clamp => self.output[len - 1],
                Extrapolate::Identity => t,
                Extrapolate::Extend => {
                    let slope = (self.output[len - 1] - self.output[len - 2])
                        / (self.input[len - 1] - self.input[len - 2]);
                    self.output[len - 1] + slope * (t - self.input[len - 1])
                }
            };
        }

        for i in 0..len - 1 {
            if t >= self.input[i] && t <= self.input[i + 1] {
                let segment_t =
                    (t - self.input[i]) / (self.input[i + 1] - self.input[i]);
                let eased = self.easings[i.min(self.easings.len() - 1)]
                    .apply(segment_t);
                return self.output[i]
                    + (self.output[i + 1] - self.output[i]) * eased;
            }
        }

        self.output[len - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframes_basic() {
        let kf = Keyframes::new(vec![0.0, 0.5, 1.0], vec![0.0, 50.0, 100.0]);
        assert!((kf.sample(0.0) - 0.0).abs() < 0.001);
        assert!((kf.sample(1.0) - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_clamp() {
        let kf = Keyframes::new(vec![0.0, 1.0], vec![0.0, 100.0])
            .with_extrapolate(Extrapolate::Clamp, Extrapolate::Clamp);
        assert!((kf.sample(-1.0) - 0.0).abs() < 0.001);
        assert!((kf.sample(2.0) - 100.0).abs() < 0.001);
    }
}
