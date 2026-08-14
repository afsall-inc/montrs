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

use crate::blocks::*;
use leptos::prelude::*;
use montrs_ui::prelude::*;

#[component]
pub fn Blocks() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-6xl px-6 py-12 lg:px-8">
            <div class="mb-12">
                <h1 class="text-3xl font-bold">"Blocks"</h1>
                <p class="mt-2 text-muted-foreground">"Pre-built UI sections. Copy, paste, and customize."</p>
            </div>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"FAQ"</h2>
                <div class="space-y-8">
                    <Faq01 />.into_any()
                    <Faq02 />.into_any()
                    <Faq03 />.into_any()
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Footers"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Footer01 />.into_any()
                    <Footer02 />.into_any()
                    <Footer03 />.into_any()
                    <Footer04 />.into_any()
                    <Footer05 />.into_any()
                    <FooterLogos />.into_any()
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Headers"</h2>
                <Header01 />.into_any()
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Integrations"</h2>
                <div class="space-y-8">
                    <Integration01 />.into_any()
                    <Integration02 />.into_any()
                    <Integration03 />.into_any()
                    <Integration04 />.into_any()
                    <Integration05 />.into_any()
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                        <Integration06 />.into_any()
                        <Integration07 />.into_any()
                    </div>
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Login"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Login01 />.into_any()
                    <Login02 />.into_any()
                    <Login03 />.into_any()
                    <Login04 />.into_any()
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Sidenav"</h2>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
                    <Sidenav01 />.into_any()
                    <Sidenav02 />.into_any()
                    <Sidenav03 />.into_any()
                    <Sidenav04 />.into_any()
                    <Sidenav05 />.into_any()
                    <Sidenav06 />.into_any()
                    <Sidenav07 />.into_any()
                    <Sidenav08 />.into_any()
                    <Sidenav09 />.into_any()
                    <Sidenav10 />.into_any()
                    <Sidenav11 />.into_any()
                    <SidenavInsetRight />.into_any()
                    <SidenavRoutes />.into_any()
                    <SidenavRoutesSelector />.into_any()
                    <SidenavRoutesSimplified />.into_any()
                </div>
            </section>
        </div>
        .into_any()
    }
}
