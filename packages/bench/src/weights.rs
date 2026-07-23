// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
