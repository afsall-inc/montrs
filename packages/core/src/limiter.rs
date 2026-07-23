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

//! montrs-core/src/limiter.rs: Rate limiting primitives.
//! This file provides a generic Limiter trait and a concrete implementation
//! using the governor crate for sophisticated rate limiting strategies.

use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use nonzero_ext::nonzero;
use std::num::NonZeroU32;

/// Trait for components that can perform rate limiting checks.
pub trait Limiter: Send + Sync + 'static {
    /// Returns true if the request is allowed, false otherwise.
    fn check(&self) -> bool;

    /// Returns a description of the rate limiting strategy, useful for agents.
    fn description(&self) -> &'static str {
        "A generic rate limiter."
    }
}

/// A rate limiter implementation backed by the governor crate.
/// Uses an in-memory state and a simple per-second quota.
pub struct GovernorLimiter {
    limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl GovernorLimiter {
    /// Creates a new GovernorLimiter with the specified allows requests per second.
    pub fn new(per_second: u32) -> Self {
        let quota = Quota::per_second(
            NonZeroU32::new(per_second).unwrap_or(nonzero!(1u32)),
        );
        Self {
            limiter: RateLimiter::direct(quota),
        }
    }
}

impl Limiter for GovernorLimiter {
    fn check(&self) -> bool {
        self.limiter.check().is_ok()
    }
}

