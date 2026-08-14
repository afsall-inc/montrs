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
use montrs_motion::*;
use montrs_ui::{components::slider::Slider, prelude::*};

#[component]
pub fn Motion() -> impl IntoView {
    let stiffness = RwSignal::new(100.0);
    let damping = RwSignal::new(10.0);
    let mass = RwSignal::new(1.0);
    let spring_value = RwSignal::new(0.0);
    let spring_playing = RwSignal::new(false);

    let easing_idx = RwSignal::new(0usize);
    let tween_progress = RwSignal::new(0.0);
    let tween_playing = RwSignal::new(false);

    let path_progress = RwSignal::new(0.0);
    let path_playing = RwSignal::new(false);

    let easings = [
        ("Linear", Easing::Linear),
        ("Ease", Easing::Ease),
        ("EaseIn", Easing::EaseIn),
        ("EaseOut", Easing::EaseOut),
        ("EaseInOut", Easing::EaseInOut),
        ("QuadIn", Easing::QuadIn),
        ("QuadOut", Easing::QuadOut),
        ("CubicIn", Easing::CubicIn),
        ("CubicOut", Easing::CubicOut),
        ("SineIn", Easing::SineIn),
        ("SineOut", Easing::SineOut),
        ("BackOut", Easing::BackOut),
        ("ElasticOut", Easing::ElasticOut),
        ("BounceOut", Easing::BounceOut),
    ];

    let play_spring = move |_| {
        spring_playing.set(true);
        spring_value.set(0.0);
        let spring = Spring::new(stiffness.get(), damping.get(), mass.get())
            .with_range(0.0, 1.0);
        let start = FrameLoop::now();
        FrameLoop::on_frame(move || {
            let elapsed = FrameLoop::now() - start;
            let v = spring.solve(elapsed);
            spring_value.set(v);
            if elapsed > 2.0 {
                spring_playing.set(false);
                false
            } else {
                true
            }
        });
    };

    let play_tween = move |_| {
        tween_playing.set(true);
        tween_progress.set(0.0);
        let (_, easing) = easings[easing_idx.get()];
        let tween = Tween::new(0.0, 1.0, 1.0).with_easing(easing);
        let start = FrameLoop::now();
        FrameLoop::on_frame(move || {
            let elapsed = FrameLoop::now() - start;
            let v = tween.sample(elapsed);
            tween_progress.set(v);
            if elapsed > 1.0 {
                tween_playing.set(false);
                false
            } else {
                true
            }
        });
    };

    let play_path = move |_| {
        path_playing.set(true);
        path_progress.set(0.0);
        let start = FrameLoop::now();
        FrameLoop::on_frame(move || {
            let elapsed = FrameLoop::now() - start;
            let p = (elapsed / 2.0).min(1.0);
            path_progress.set(p);
            if elapsed > 2.0 {
                path_playing.set(false);
                false
            } else {
                true
            }
        });
    };

    view! {
        <div class="mx-auto max-w-6xl px-6 py-12 lg:px-8">
            <div class="mb-8">
                <h1 class="text-3xl font-bold">"Motion & Animation"</h1>
                <p class="mt-2 text-muted-foreground">
                    "Interactive demos of spring physics, tween easing, and SVG path animation."
                </p>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <div class="rounded-lg border border-border bg-card p-6">
                    <h2 class="text-xl font-semibold flex items-center gap-2">
                        <Icon glyph=Glyph::Activity class="w-5 h-5 text-primary" />
                        "Spring Physics"
                    </h2>
                    <p class="mt-1 text-sm text-muted-foreground">
                        "Adjust spring parameters and watch the mass bounce."
                    </p>

                    <div class="mt-6 space-y-4">
                        <div>
                            <label class="text-sm font-medium">"Stiffness:" {move || format!("{:.0}", stiffness.get())}</label>
                            <Slider min=10.0 max=500.0 step=1.0 value=stiffness />
                        </div>
                        <div>
                            <label class="text-sm font-medium">"Damping:" {move || format!("{:.1}", damping.get())}</label>
                            <Slider min=1.0 max=50.0 step=0.5 value=damping />
                        </div>
                        <div>
                            <label class="text-sm font-medium">"Mass:" {move || format!("{:.1}", mass.get())}</label>
                            <Slider min=0.1 max=10.0 step=0.1 value=mass />
                        </div>
                        <button
                            class="inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors"
                            on:click=play_spring
                            disabled=move || spring_playing.get()
                        >
                            <Icon glyph=Glyph::Play class="mr-2 w-4 h-4" />
                            "Play Spring"
                        </button>
                    </div>

                    <div class="mt-6 flex items-end gap-1" style="height: 120px;">
                        {move || {
                            let pct = (spring_value.get() * 100.0).min(100.0);
                            view! {
                                <div
                                    class="w-full bg-primary rounded-t transition-all duration-16"
                                    style=format!("height: {}%;", pct)
                                />
                            }
                        }}
                    </div>
                </div>

                <div class="rounded-lg border border-border bg-card p-6">
                    <h2 class="text-xl font-semibold flex items-center gap-2">
                        <Icon glyph=Glyph::ChartSpline class="w-5 h-5 text-primary" />
                        "Tween Easing Visualizer"
                    </h2>
                    <p class="mt-1 text-sm text-muted-foreground">
                        "Pick an easing function and see its curve."
                    </p>

                    <div class="mt-6 space-y-4">
                        <select
                            class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                            prop:value=move || easing_idx.get().to_string()
                            on:change=move |e| {
                                let val = event_target_value(&e);
                                if let Ok(idx) = val.parse::<usize>() {
                                    easing_idx.set(idx);
                                }
                            }
                        >
                            {easings.iter().enumerate().map(|(i, (name, _))| {
                                view! {
                                    <option value=i.to_string()>{*name}</option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                        <button
                            class="inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors"
                            on:click=play_tween
                            disabled=move || tween_playing.get()
                        >
                            <Icon glyph=Glyph::Play class="mr-2 w-4 h-4" />
                            "Play Tween"
                        </button>
                    </div>

                    <div class="mt-6">
                        <svg viewBox="0 0 200 120" class="w-full h-32 border border-border rounded-md bg-background">
                            <line x1="0" y1="120" x2="200" y2="120" stroke="currentColor" stroke-width="1" opacity="0.3" />
                            <line x1="0" y1="0" x2="0" y2="120" stroke="currentColor" stroke-width="1" opacity="0.3" />
                            <rect x="-1" y="-1" width="202" height="122" fill="none" stroke="currentColor" stroke-width="1" opacity="0.15" />
                            {move || {
                                let pts: Vec<_> = (0..=50).map(|i| {
                                    let t = i as f64 / 50.0;
                                    let (_, easing) = easings[easing_idx.get()];
                                    let y = 1.0 - easing.apply(t);
                                    let px = 4.0 + t * 192.0;
                                    let py = 4.0 + y * 112.0;
                                    format!("{:.1},{:.1}", px, py)
                                }).collect::<Vec<_>>();
                                let d = format!("M{}", pts.join(" L"));
                                view! {
                                    <path d=d stroke="hsl(var(--primary))" stroke-width="2" fill="none" />
                                }
                            }}
                            {move || {
                                let p = tween_progress.get();
                                let (_, easing) = easings[easing_idx.get()];
                                let y = 1.0 - easing.apply(p);
                                let cx = 4.0 + p * 192.0;
                                let cy = 4.0 + y * 112.0;
                                view! {
                                    <circle cx=cx.to_string() cy=cy.to_string() r="5" fill="hsl(var(--primary))" />
                                }
                            }}
                        </svg>
                    </div>
                </div>
            </div>

            <div class="mt-8 rounded-lg border border-border bg-card p-6">
                <h2 class="text-xl font-semibold flex items-center gap-2">
                    <Icon glyph=Glyph::PenLine class="w-5 h-5 text-primary" />
                    "SVG Path Animation"
                </h2>
                <p class="mt-1 text-sm text-muted-foreground">
                    "Animated SVG path drawing — stroke-dasharray / stroke-dashoffset technique."
                </p>

                <div class="mt-4 flex items-center gap-4">
                    <button
                        class="inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors"
                        on:click=play_path
                        disabled=move || path_playing.get()
                    >
                        <Icon glyph=Glyph::Play class="mr-2 w-4 h-4" />
                        "Animate Path"
                    </button>
                    <span class="text-sm text-muted-foreground">
                        {move || format!("Progress: {:.0}%", path_progress.get() * 100.0)}
                    </span>
                </div>

                <div class="mt-6 flex justify-center">
                    <svg viewBox="0 0 200 120" class="w-64 h-40">
                        {move || {
                            let length = 280.0;
                            let offset = length * (1.0 - path_progress.get());
                            view! {
                                <path
                                    d="M20 100 Q50 10 100 60 T180 40"
                                    stroke="hsl(var(--primary))"
                                    stroke-width="3"
                                    fill="none"
                                    stroke-linecap="round"
                                    stroke-dasharray=format!("{} {}", length, length)
                                    stroke-dashoffset=offset.to_string()
                                />
                            }
                        }}
                    </svg>
                </div>
            </div>
        </div>
    }
}
