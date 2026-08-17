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

use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-border">
            <div class="mx-auto max-w-6xl px-6 py-12 lg:px-8">
                <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">
                    <div>
                        <div class="flex items-center gap-2 font-bold text-lg mb-4">
                            <Icon glyph=Glyph::Blocks class="w-5 h-5 text-primary" />
                            "MontRS"
                        </div>
                        <p class="text-sm text-muted-foreground">
                            "A full-stack Rust framework for humans and agents."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-sm font-semibold mb-3">"Framework"</h3>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="/components">"Components"</a></li>
                            <li><a href="/icons">"Icons"</a></li>
                            <li><a href="/blocks">"Blocks"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h3 class="text-sm font-semibold mb-3">"Community"</h3>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="https://github.com/montrs/montrs" target="_blank">"GitHub"</a></li>
                            <li><a href="https://docs.montrs.com" target="_blank">"Documentation"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h3 class="text-sm font-semibold mb-3">"Legal"</h3>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li>"MIT License"</li>
                        </ul>
                    </div>
                </div>
                <div class="mt-8 border-t border-border pt-8 text-center text-sm text-muted-foreground">
                    "© 2026 MontRS. All rights reserved."
                </div>
            </div>
        </footer>
    }
}
