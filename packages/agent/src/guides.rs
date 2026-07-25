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

pub const ARCHITECTURE_GUIDE: &str = r#"
# MontRS Architecture Guide
MontRS is a trait-driven, deterministic web framework built on Leptos 0.8.

## Core Concepts
- **AppSpec**: The single source of truth for the application blueprint.
- **Plate**: A unit of composition (Auth, Blog, etc.) that registers unified routes.
- **Unified Route**: A single struct implementing `Route` that unifies Params, Loader, Action, and View.
- **Loaders**: Read-only, idempotent data fetching.
- **Actions**: State-changing mutations.

## How to build a plate
1. Implement the `Plate` trait.
2. Define explicit dependencies using the `dependencies()` method if your plate requires other plates to be initialized first.
3. Define your `Route` implementation (which includes its `Loader`, `Action`, and `View`).
4. Register the route in `register_routes` using `router.register(MyRoute)`.
"#;

pub const DEBUGGING_GUIDE: &str = r#"
# Debugging MontRS
MontRS provides an `errorfile.json` in the `.agent/errorfiles` folder when a build or test fails.
Agents should use these files to understand the context of the error and propose fixes.
"#;
