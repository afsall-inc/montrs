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
pub fn Sidenav11() -> impl IntoView {
    let workspace = RwSignal::new("Personal".to_string());
    let show_workspaces = RwSignal::new(false);
    let active = RwSignal::new("Overview".to_string());

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                <div class="relative mb-4">
                    <button
                        on:click=move |_| show_workspaces.update(|v| *v = !*v)
                        class="flex w-full items-center justify-between rounded-md border border-border bg-background px-3 py-2 text-sm hover:bg-muted transition-colors"
                    >
                        <span>{workspace.get()}</span>
                        <Icon glyph=Glyph::ChevronDown class="w-4 h-4 text-muted-foreground" />
                    </button>
                    <Show when=move || show_workspaces.get()>
                        <div class="absolute top-full left-0 right-0 mt-1 rounded-md border border-border bg-card shadow-lg z-10">
                            <button
                                on:click=move |_| { workspace.set("Personal".to_string()); show_workspaces.set(false); }
                                class=move || {
                                    let base = "w-full text-left px-3 py-2 text-sm transition-colors";
                                    if workspace.get() == "Personal" { format!("{} bg-primary/10 text-primary font-medium", base) } else { format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base) }
                                }
                            >"Personal"</button>
                            <button
                                on:click=move |_| { workspace.set("Work".to_string()); show_workspaces.set(false); }
                                class=move || {
                                    let base = "w-full text-left px-3 py-2 text-sm transition-colors";
                                    if workspace.get() == "Work" { format!("{} bg-primary/10 text-primary font-medium", base) } else { format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base) }
                                }
                            >"Work"</button>
                            <button
                                on:click=move |_| { workspace.set("Open Source".to_string()); show_workspaces.set(false); }
                                class=move || {
                                    let base = "w-full text-left px-3 py-2 text-sm transition-colors";
                                    if workspace.get() == "Open Source" { format!("{} bg-primary/10 text-primary font-medium", base) } else { format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base) }
                                }
                            >"Open Source"</button>
                        </div>
                    </Show>
                </div>
                <button
                    on:click=move |_| active.set("Overview".to_string())
                    class=move || {
                        let base = "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors";
                        if active.get() == "Overview" { format!("{} bg-primary/10 text-primary font-medium", base) } else { format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base) }
                    }
                >
                    <Icon glyph=Glyph::LayoutDashboard class="w-4 h-4" />"Overview"
                </button>
                <button
                    on:click=move |_| active.set("Projects".to_string())
                    class=move || {
                        let base = "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors";
                        if active.get() == "Projects" { format!("{} bg-primary/10 text-primary font-medium", base) } else { format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base) }
                    }
                >
                    <Icon glyph=Glyph::Folder class="w-4 h-4" />"Projects"
                </button>
                <button
                    on:click=move |_| active.set("Members".to_string())
                    class=move || {
                        let base = "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors";
                        if active.get() == "Members" { format!("{} bg-primary/10 text-primary font-medium", base) } else { format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base) }
                    }
                >
                    <Icon glyph=Glyph::Users class="w-4 h-4" />"Members"
                </button>
                <button
                    on:click=move |_| active.set("Settings".to_string())
                    class=move || {
                        let base = "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors";
                        if active.get() == "Settings" { format!("{} bg-primary/10 text-primary font-medium", base) } else { format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base) }
                    }
                >
                    <Icon glyph=Glyph::Settings class="w-4 h-4" />"Settings"
                </button>
            </div>
        </div>
    }
}
