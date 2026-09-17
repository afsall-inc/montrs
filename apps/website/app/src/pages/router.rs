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

use crate::highlight::highlight_rust;
use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

const ROUTER_SNIPPET: &str = r#"use montrs_core::{AppConfig, Plate, Router, view_route};

view_route! { HomeRoute, "/", HomeView }
view_route! { UserRoute, "/users/:id", UserView }

pub struct AppPlate;

impl<C: AppConfig + 'static> Plate<C> for AppPlate {
    fn register_routes(&self, router: &mut Router<C>) {
        router.register(HomeRoute);
        router.register(UserRoute);
    }
}

// In the app shell:
// <RouterOutlet::<MyConfig>() />
"#;

#[component]
pub fn RouterDocs() -> impl IntoView {
    let features = [
        (
            Glyph::Route,
            "Typed routes",
            "Routes are Rust types. Params, loaders, and actions are checked \
             at compile time — no stringly-typed surprises.",
        ),
        (
            Glyph::GitBranch,
            "Nested layouts",
            "Compose layouts from Plates. A nested route reuses the parent \
             shell instead of re-rendering the whole tree.",
        ),
        (
            Glyph::Network,
            "RouterOutlet",
            "A single component renders the active route. It matches the \
             address bar immediately, without waiting on a client tree.",
        ),
        (
            Glyph::ScrollText,
            "Route data",
            "Loaders fetch before render; actions mutate and revalidate. Both \
             are tracked in deterministic test runtimes.",
        ),
        (
            Glyph::Shapes,
            "Lazy routes",
            "Split heavy pages into their own bundles and stream them in on \
             navigation.",
        ),
        (
            Glyph::ShieldCheck,
            "Route guards",
            "Typed guards run before a route resolves — redirect, or render \
             an auth boundary.",
        ),
    ];

    view! {
        <div class="page-container py-12">
            <div class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight">"Router"</h1>
                <p class="mt-2 max-w-2xl text-muted-foreground">
                    "The MontRS router is a typed, file-free router built on
                    Leptos — routes are types, layouts compose, and data
                    loading happens before paint."
                </p>
            </div>

            <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
                {features.into_iter().map(|(icon, title, desc)| view! {
                    <div class="showcase-card reveal p-6">
                        <Icon glyph=icon class="h-6 w-6 text-primary" />
                        <h3 class="mt-4 font-semibold">{title}</h3>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">{desc}</p>
                    </div>
                }).collect::<Vec<_>>()}
            </div>

            <div class="mt-10">
                <div class="code-window max-w-2xl">
                    <div class="code-window-bar">
                        <span class="traffic-light traffic-light-red"></span>
                        <span class="traffic-light traffic-light-yellow"></span>
                        <span class="traffic-light traffic-light-green"></span>
                        <span class="code-window-tab">"routes.rs"</span>
                    </div>
                    <pre class="code-window-body text-left" inner_html=highlight_rust(ROUTER_SNIPPET)></pre>
                </div>
            </div>
        </div>
    }
}
