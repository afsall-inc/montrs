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

use crate::BenchCase;
use async_trait::async_trait;
use std::{future::Future, ops::RangeInclusive};

/// Represents a benchmark parameter with a range of values.
///
/// Inspired by Substrate's parameter-based benchmarking, this allows
/// measuring how performance scales with input size.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub range: RangeInclusive<u32>,
    pub step: u32,
}

impl Parameter {
    pub fn new(name: impl Into<String>, range: RangeInclusive<u32>) -> Self {
        Self {
            name: name.into(),
            range,
            step: 1,
        }
    }

    pub fn with_step(mut self, step: u32) -> Self {
        self.step = step;
        self
    }

    pub fn values(&self) -> Vec<u32> {
        let mut vals = Vec::new();
        let mut curr = *self.range.start();
        while curr <= *self.range.end() {
            vals.push(curr);
            if self.step == 0 {
                break;
            }
            curr += self.step;
        }
        vals
    }
}

/// A benchmark that varies based on a parameter.
pub struct ParametricBench<F, Fut>
where
    F: Fn(u32) -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    name: String,
    parameter: Parameter,
    func: F,
    current_param: std::sync::atomic::AtomicU32,
}

impl<F, Fut> ParametricBench<F, Fut>
where
    F: Fn(u32) -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    pub fn new(name: impl Into<String>, parameter: Parameter, func: F) -> Self {
        let start = *parameter.range.start();
        Self {
            name: name.into(),
            parameter,
            func,
            current_param: std::sync::atomic::AtomicU32::new(start),
        }
    }

    /// Sets the current parameter value for the next run.
    pub fn set_param(&self, val: u32) {
        self.current_param
            .store(val, std::sync::atomic::Ordering::SeqCst);
    }
}

#[async_trait]
impl<F, Fut> BenchCase for ParametricBench<F, Fut>
where
    F: Fn(u32) -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn parameter(&self) -> Option<Parameter> {
        Some(self.parameter.clone())
    }

    fn set_parameter(&self, val: u32) {
        self.set_param(val);
    }

    async fn run(&self) -> anyhow::Result<()> {
        let p = self.current_param.load(std::sync::atomic::Ordering::SeqCst);
        (self.func)(p).await
    }
}

