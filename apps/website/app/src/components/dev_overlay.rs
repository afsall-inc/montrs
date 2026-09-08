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

//! Next.js-style dev overlay: a floating MontRS logo (bottom-right) that only
//! appears in dev mode (detected by connecting to the live-reload WebSocket)
//! and surfaces runtime errors / hydration issues without opening the console.

use leptos::prelude::*;

#[component]
pub fn DevOverlay() -> impl IntoView {
    let dev = RwSignal::new(false);
    let open = RwSignal::new(false);
    let errors = RwSignal::new(Vec::<(String, String)>::new());

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::{JsCast, prelude::Closure};
        let Some(window) = web_sys::window() else {
            return;
        };

        // Capture uncaught errors and unhandled rejections.
        let errs = errors;
        let cb = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |ev: web_sys::ErrorEvent| {
                let msg = ev.message();
                errs.update(|v| {
                    v.push(("error".to_string(), msg));
                    v.truncate(50);
                });
            },
        ));
        let _ = window.add_event_listener_with_callback(
            "error",
            cb.as_ref().unchecked_ref(),
        );
        cb.forget();

        let errs2 = errors;
        let cb2 = Closure::<dyn FnMut(web_sys::PromiseRejectionEvent)>::wrap(
            Box::new(move |ev: web_sys::PromiseRejectionEvent| {
                let reason = ev.reason();
                let text = reason.as_string().unwrap_or_else(|| {
                    "Unhandled promise rejection".to_string()
                });
                errs2.update(|v| {
                    v.push(("rejection".to_string(), text));
                    v.truncate(50);
                });
            }),
        );
        let _ = window.add_event_listener_with_callback(
            "unhandledrejection",
            cb2.as_ref().unchecked_ref(),
        );
        cb2.forget();

        // Dev-mode detection: the live-reload WebSocket only exists when
        // `montrs serve` / `montrs watch` is running.
        let host = window.location().host().unwrap_or_default();
        let url = format!("ws://{host}:3001");
        if let Ok(ws) = web_sys::WebSocket::new(&url) {
            let dev = dev;
            let on_open = Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(
                move |_: web_sys::Event| {
                    dev.set(true);
                },
            ));
            let _ = ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            on_open.forget();
        }
    });

    let toggle = move |_| open.update(|o| *o = !*o);

    view! {
        <Show when=move || dev.get()>
            <div class="fixed bottom-4 right-4 z-[60] flex flex-col items-end gap-2">
                <Show when=move || open.get() && !errors.get().is_empty()>
                    <div class="w-80 rounded-lg border border-border bg-background shadow-xl">
                        <div class="flex items-center justify-between border-b border-border px-3 py-2">
                            <p class="text-xs font-semibold">"Dev console"</p>
                            <button
                                type="button"
                                class="text-[10px] text-muted-foreground underline-offset-2 hover:text-foreground hover:underline"
                                on:click=move |_| errors.set(Vec::new())
                            >
                                "clear"
                            </button>
                        </div>
                        <div class="max-h-56 overflow-y-auto p-2 font-mono text-[11px]">
                            {move || errors.get().iter().map(|(kind, msg)| {
                                let color = if kind == "error" {
                                    "text-red-500"
                                } else {
                                    "text-amber-500"
                                };
                                view! {
                                    <p class={format!("whitespace-pre-wrap border-b border-border/50 py-1 {color}")}>{msg.clone()}</p>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </Show>
                <button
                    type="button"
                    class="flex h-10 w-10 items-center justify-center rounded-full border border-border bg-background shadow-lg transition-transform hover:scale-105"
                    on:click=toggle
                    aria-label="Toggle dev console"
                    title="MontRS dev console"
                >
                    <img src="/logo-64.png" alt="" class="h-6 w-6 rounded" />
                </button>
            </div>
        </Show>
    }
}
