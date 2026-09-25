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

//! Persistent sub-navigation for the `/ui` section: breadcrumbs + one-click
//! links to every UI subpage, so you never need the header dropdown.

use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use montrs_ui::components::floating_tab_bar::FloatingTabBar;

const LINKS: &[(&str, &str)] = &[
    ("/ui", "MontRS UI"),
    ("/ui/components", "Components"),
    ("/ui/blocks", "Blocks"),
    ("/ui/icons", "Icons"),
    ("/ui/motion", "Motion"),
    ("/ui/themes", "Themes"),
    ("/ui/backgrounds", "Backgrounds"),
];

#[component]
pub fn UiSubNav() -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    let path = Signal::derive(move || location.pathname.get());

    // Only render inside the /ui section.
    let visible = move || path.get().starts_with("/ui");

    let items = LINKS
        .iter()
        .map(|(href, label)| (label.to_string(), href.to_string()))
        .collect::<Vec<_>>();

    let on_select = Callback::new({
        let nav = navigate.clone();
        move |href: String| {
            nav(&href, Default::default());
        }
    });

    view! {
        <Show when=move || visible()>
            <div class="sticky top-14 z-40 w-full border-b border-border/40 bg-background/80 backdrop-blur supports-[backdrop-filter]:bg-background/60">
                <nav class="page-container py-2" aria-label="UI sections">
                    <FloatingTabBar sticky=false items=items.clone() active=path on_select=on_select />
                </nav>
            </div>
        </Show>
    }
}
