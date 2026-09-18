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

use crate::copy::CopyButton;
use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Cli() -> impl IntoView {
    let commands: &[(&str, &str)] = &[
        ("montrs new <name>", "Scaffold a new app from a template."),
        (
            "montrs install",
            "Install the toolchain the project needs (Tailwind, wasm-bindgen, \
             wasm target).",
        ),
        (
            "montrs serve",
            "Dev server with hot reload and a live error overlay.",
        ),
        (
            "montrs watch",
            "Alias for `montrs serve` — rebuild + reload on change.",
        ),
        (
            "montrs build",
            "Production build for web, desktop, or mobile.",
        ),
        (
            "montrs add <component>",
            "Copy a component, icon, or theme into your app.",
        ),
        ("montrs fmt", "Format Rust and view! macros."),
        ("montrs test", "Run the workspace test suite."),
        (
            "montrs agent check",
            "Agent-level diagnostics over your project.",
        ),
        (
            "montrs mcp serve",
            "Start the MCP server for agent tool calls.",
        ),
    ];

    let terminals: &[&str] = &[
        "montrs new my-app",
        "cd my-app && montrs install",
        "montrs serve",
    ];

    view! {
        <div class="page-container py-12">
            <div class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight">
                    "CLI"
                </h1>
                <p class="mt-2 max-w-2xl text-muted-foreground">
                    "One binary wires the whole toolchain: scaffold, install,
                    serve, build, and run agent tooling — no Node, no package
                    manager, no config sprawl."
                </p>
            </div>
            <div class="mx-auto max-w-3xl">
                <div class="code-window">
                    <div class="code-window-bar">
                        <span class="traffic-light traffic-light-red" />
                        <span class="traffic-light traffic-light-yellow" />
                        <span class="traffic-light traffic-light-green" />
                        <span class="code-window-tab">
                            "terminal"
                        </span>
                    </div>
                    <div class="code-window-body space-y-2">
                        {terminals.iter().map(|cmd| view! {
                            <div class="flex items-center justify-between gap-3">
                                <span>
                                    <span class="terminal-prompt">"$"</span>
                                    {format!(" {cmd}")}
                                </span>
                                <CopyButton text=*cmd label="Copy" />
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
            <div class="mx-auto mt-10 max-w-3xl">
                <div class="overflow-hidden rounded-xl border border-border">
                    {commands.iter().map(|(cmd, desc)| view! {
                        <div class="flex flex-col gap-1 border-b border-border px-4 py-3 last:border-b-0 sm:flex-row sm:items-center sm:gap-4">
                            <code class="shrink-0 font-mono text-sm text-foreground">{*cmd}</code>
                            <span class="text-sm text-muted-foreground">{*desc}</span>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="mt-10 flex flex-wrap gap-3">
                <a href="/docs" class="inline-flex items-center rounded-md border border-border px-4 py-2 text-sm font-medium transition-colors hover:bg-accent">
                    <Icon glyph=Glyph::BookOpen class="mr-2 h-4 w-4" />
                    "Read the docs"
                </a>
                <a href="/router" class="inline-flex items-center rounded-md border border-border px-4 py-2 text-sm font-medium transition-colors hover:bg-accent">
                    <Icon glyph=Glyph::Route class="mr-2 h-4 w-4" />
                    "Router guide"
                </a>
            </div>
        </div>
    }
}
