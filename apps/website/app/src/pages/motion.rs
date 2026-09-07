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
    let demos = [
        ("spring", "Spring Physics"),
        ("tween", "Tween Easing"),
        ("path", "SVG Path Draw"),
        ("keyframes", "Keyframes"),
        ("morph", "Shape Morphing"),
        ("gesture", "Gesture Primitives"),
        ("pentagon", "Pentagon Balls"),
    ];

    view! {
        <div class="page-container py-12">
            <div class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight">"Motion & Animation"</h1>
                <p class="mt-2 max-w-2xl text-muted-foreground">
                    "Interactive demos of spring physics, tween easing, keyframe
                    interpolation, shape morphing, and gesture-driven motion — all
                    built on the montrs-motion FrameLoop."
                </p>
            </div>

            <div class="grid grid-cols-1 gap-10 lg:grid-cols-[180px_1fr]">
                <nav class="hidden lg:block">
                    <div class="sticky top-20 space-y-1 border-l border-border pl-4 text-sm">
                        <p class="font-mono text-[10px] uppercase tracking-wide text-muted-foreground">"Demos"</p>
                        {demos.iter().map(|(id, label)| {
                            let on_click = scroll_to(id);
                            view! {
                                <a
                                    href="#"
                                    class="block rounded-md px-3 py-1.5 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                                    on:click=on_click
                                >{*label}</a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </nav>

                <div class="min-w-0">
                    <section id="spring" class="scroll-mt-24">
                        <SpringDemo />
                    </section>
                    <section id="tween" class="mt-10 scroll-mt-24">
                        <TweenDemo />
                    </section>
                    <section id="path" class="mt-10 scroll-mt-24">
                        <PathDemo />
                    </section>
                    <section id="keyframes" class="mt-10 scroll-mt-24">
                        <KeyframesDemo />
                    </section>
                    <section id="morph" class="mt-10 scroll-mt-24">
                        <MorphDemo />
                    </section>
                    <section id="gesture" class="mt-10 scroll-mt-24">
                        <GestureDemo />
                    </section>
                    <section id="pentagon" class="mt-10 scroll-mt-24">
                        <PentagonBallsDemo />
                    </section>
                </div>
            </div>
        </div>
    }
}

/// Smooth-scroll to an element id without touching the URL.
fn scroll_to(id: &str) -> impl Fn(leptos::ev::MouseEvent) {
    let id = id.to_string();
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &id;
    move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            if let Some(doc) = web_sys::window().and_then(|w| w.document())
                && let Some(el) = doc.get_element_by_id(&id)
            {
                if let Some(html) = el.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = html.scroll_into_view();
                }
            }
        }
    }
}

#[component]
fn SpringDemo() -> impl IntoView {
    let stiffness = RwSignal::new(100.0);
    let damping = RwSignal::new(10.0);
    let mass = RwSignal::new(1.0);
    let value = RwSignal::new(0.0);
    let playing = RwSignal::new(false);

    let play_spring = move |_| {
        if playing.get() {
            return;
        }
        playing.set(true);
        value.set(0.0);
        let spring = Spring::new(stiffness.get(), damping.get(), mass.get())
            .with_range(0.0, 1.0);
        let start = FrameLoop::now();
        FrameLoop::on_frame(move || {
            let elapsed = FrameLoop::now() - start;
            value.set(spring.solve(elapsed));
            if elapsed > 2.0 {
                playing.set(false);
                false
            } else {
                true
            }
        });
    };

    view! {
        <div class="showcase-card p-6">
            <div class="flex items-center gap-2">
                <Icon glyph=Glyph::Activity class="h-5 w-5 text-primary" />
                <h2 class="text-xl font-semibold">"Spring Physics"</h2>
            </div>
            <p class="mt-1 text-sm text-muted-foreground">
                "Tune the spring and watch the mass settle."
            </p>

            <div class="mt-6 space-y-5">
                <div>
                    <label class="flex justify-between text-sm font-medium">
                        "Stiffness"
                        <span class="font-mono text-muted-foreground">{move || format!("{:.0}", stiffness.get())}</span>
                    </label>
                    <Slider min=10.0 max=500.0 step=1.0 value=stiffness class="mt-2" />
                </div>
                <div>
                    <label class="flex justify-between text-sm font-medium">
                        "Damping"
                        <span class="font-mono text-muted-foreground">{move || format!("{:.1}", damping.get())}</span>
                    </label>
                    <Slider min=1.0 max=50.0 step=0.5 value=damping class="mt-2" />
                </div>
                <div>
                    <label class="flex justify-between text-sm font-medium">
                        "Mass"
                        <span class="font-mono text-muted-foreground">{move || format!("{:.1}", mass.get())}</span>
                    </label>
                    <Slider min=0.1 max=10.0 step=0.1 value=mass class="mt-2" />
                </div>

                <button
                    type="button"
                    class="inline-flex h-10 items-center gap-2 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 disabled:opacity-50"
                    on:click=play_spring
                    disabled=move || playing.get()
                >
                    <Icon glyph=Glyph::Play class="h-4 w-4" />
                    {move || if playing.get() { "Playing…" } else { "Play Spring" }}
                </button>
            </div>

            <div class="mt-6 flex items-end gap-1 rounded-md border border-border bg-background p-4" style="height: 140px;">
                {move || {
                    let pct = (value.get() * 100.0).min(100.0);
                    view! {
                        <div class="w-full rounded-t bg-primary" style=format!("height: {}%;", pct)></div>
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn TweenDemo() -> impl IntoView {
    let easing_idx = RwSignal::new(0usize);
    let progress = RwSignal::new(0.0);
    let playing = RwSignal::new(false);

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

    let play_tween = move |_| {
        if playing.get() {
            return;
        }
        playing.set(true);
        progress.set(0.0);
        let (_, easing) = easings[easing_idx.get()];
        let tween = Tween::new(0.0, 1.0, 1.0).with_easing(easing);
        let start = FrameLoop::now();
        FrameLoop::on_frame(move || {
            let elapsed = FrameLoop::now() - start;
            progress.set(tween.sample(elapsed));
            if elapsed > 1.0 {
                playing.set(false);
                false
            } else {
                true
            }
        });
    };

    view! {
        <div class="showcase-card p-6">
            <div class="flex items-center gap-2">
                <Icon glyph=Glyph::ChartSpline class="h-5 w-5 text-primary" />
                <h2 class="text-xl font-semibold">"Tween Easing Visualizer"</h2>
            </div>
            <p class="mt-1 text-sm text-muted-foreground">
                "Pick an easing function and watch the curve."
            </p>

            <div class="mt-6 space-y-5">
                <div>
                    <label class="mb-2 block text-sm font-medium">"Easing"</label>
                    <select
                        class="h-10 w-full rounded-md border border-input bg-background px-3 text-sm"
                        prop:value=move || easing_idx.get().to_string()
                        on:change=move |e| {
                            let val = event_target_value(&e);
                            if let Ok(idx) = val.parse::<usize>() {
                                easing_idx.set(idx);
                            }
                        }
                    >
                        {easings.iter().enumerate().map(|(i, (name, _))| {
                            view! { <option value=i.to_string()>{*name}</option> }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>

                <button
                    type="button"
                    class="inline-flex h-10 items-center gap-2 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 disabled:opacity-50"
                    on:click=play_tween
                    disabled=move || playing.get()
                >
                    <Icon glyph=Glyph::Play class="h-4 w-4" />
                    {move || if playing.get() { "Playing…" } else { "Play Tween" }}
                </button>
            </div>

            <div class="mt-6">
                <svg viewBox="0 0 200 120" class="h-32 w-full rounded-md border border-border bg-background">
                    <line x1="0" y1="120" x2="200" y2="120" stroke="currentColor" stroke-width="1" opacity="0.25" />
                    <line x1="0" y1="0" x2="0" y2="120" stroke="currentColor" stroke-width="1" opacity="0.25" />
                    {move || {
                        let pts: Vec<String> = (0..=50).map(|i| {
                            let t = i as f64 / 50.0;
                            let (_, easing) = easings[easing_idx.get()];
                            let y = 1.0 - easing.apply(t);
                            let px = 4.0 + t * 192.0;
                            let py = 4.0 + y * 112.0;
                            format!("{:.1},{:.1}", px, py)
                        }).collect();
                        let d = format!("M{}", pts.join(" L"));
                        view! {
                            <path d=d stroke="hsl(var(--primary))" stroke-width="2" fill="none" />
                        }
                    }}
                    {move || {
                        let p = progress.get();
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
    }
}

#[component]
fn PathDemo() -> impl IntoView {
    let progress = RwSignal::new(0.0);
    let playing = RwSignal::new(false);

    let play_path = move |_| {
        if playing.get() {
            return;
        }
        playing.set(true);
        progress.set(0.0);
        let start = FrameLoop::now();
        FrameLoop::on_frame(move || {
            let elapsed = FrameLoop::now() - start;
            progress.set((elapsed / 2.0).min(1.0));
            if elapsed > 2.0 {
                playing.set(false);
                false
            } else {
                true
            }
        });
    };

    view! {
        <div class="showcase-card p-6">
            <div class="flex flex-wrap items-center justify-between gap-4">
                <div>
                    <div class="flex items-center gap-2">
                        <Icon glyph=Glyph::PenLine class="h-5 w-5 text-primary" />
                        <h2 class="text-xl font-semibold">"SVG Path Animation"</h2>
                    </div>
                    <p class="mt-1 text-sm text-muted-foreground">
                        "Stroke-dasharray / stroke-dashoffset path drawing."
                    </p>
                </div>
                <div class="flex items-center gap-4">
                    <button
                        type="button"
                        class="inline-flex h-10 items-center gap-2 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 disabled:opacity-50"
                        on:click=play_path
                        disabled=move || playing.get()
                    >
                        <Icon glyph=Glyph::Play class="h-4 w-4" />
                        {move || if playing.get() { "Drawing…" } else { "Animate Path" }}
                    </button>
                    <span class="font-mono text-sm text-muted-foreground">
                        {move || format!("{:.0}%", progress.get() * 100.0)}
                    </span>
                </div>
            </div>

            <div class="mt-6 flex justify-center rounded-md border border-border bg-background p-6">
                <svg viewBox="0 0 200 120" class="h-40 w-64">
                    {move || {
                        let length = 280.0;
                        let offset = length * (1.0 - progress.get());
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
    }
}

#[component]
fn KeyframesDemo() -> impl IntoView {
    let progress = RwSignal::new(0.0);
    let playing = RwSignal::new(false);

    // Keyframe stops: x travels right while y bounces with per-segment easing.
    let x_stops = vec![0.0, 42.0, 84.0, 126.0, 168.0];
    let y_stops = vec![0.0, -58.0, 0.0, -34.0, 0.0];
    let xs = Keyframes::new(vec![0.0, 0.25, 0.5, 0.75, 1.0], x_stops.clone())
        .with_easings(vec![Easing::EaseInOut; 4]);
    let ys = Keyframes::new(vec![0.0, 0.25, 0.5, 0.75, 1.0], y_stops.clone())
        .with_easings(vec![
            Easing::QuadIn,
            Easing::QuadOut,
            Easing::QuadIn,
            Easing::QuadOut,
        ]);

    // Pre-computed trail so the full interpolated path stays visible.
    let trail: Vec<(f64, f64)> = (0..=64)
        .map(|i| {
            let t = i as f64 / 64.0;
            (20.0 + xs.sample(t), 100.0 + ys.sample(t))
        })
        .collect();
    let trail_d = format!(
        "M{}",
        trail
            .iter()
            .map(|(x, y)| format!("{:.1} {:.1}", x, y))
            .collect::<Vec<_>>()
            .join(" L")
    );

    let play_kf = move |_| {
        if playing.get() {
            return;
        }
        playing.set(true);
        progress.set(0.0);
        let start = FrameLoop::now();
        FrameLoop::on_frame(move || {
            let elapsed = FrameLoop::now() - start;
            let t = (elapsed / 2.4).min(1.0);
            progress.set(t);
            if elapsed > 2.4 {
                playing.set(false);
                false
            } else {
                true
            }
        });
    };

    let xs_ball = xs.clone();
    let ys_ball = ys.clone();

    view! {
        <div class="showcase-card p-6">
            <div class="flex items-center gap-2">
                <Icon glyph=Glyph::Layers class="h-5 w-5 text-primary" />
                <h2 class="text-xl font-semibold">"Keyframe Interpolation"</h2>
            </div>
            <p class="mt-1 text-sm text-muted-foreground">
                "Multi-segment interpolation with per-segment easing between stops."
            </p>

            <div class="mt-6 flex flex-wrap items-center justify-between gap-4">
                <button
                    type="button"
                    class="inline-flex h-10 items-center gap-2 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 disabled:opacity-50"
                    on:click=play_kf
                    disabled=move || playing.get()
                >
                    <Icon glyph=Glyph::Play class="h-4 w-4" />
                    {move || if playing.get() { "Bouncing…" } else { "Play Keyframes" }}
                </button>
                <span class="font-mono text-sm text-muted-foreground">
                    {move || format!("{:.0}%", progress.get() * 100.0)}
                </span>
            </div>

            <div class="mt-6">
                <svg viewBox="0 0 220 120" class="h-40 w-full rounded-md border border-border bg-background">
                    <line
                        x1="20" y1="100" x2="200" y2="100"
                        stroke="hsl(var(--foreground))" stroke-width="1" opacity="0.15"
                    />
                    {x_stops.iter().zip(&y_stops).map(|(x, y)| {
                        let cx = (20.0 + x).to_string();
                        let cy = (100.0 + y).to_string();
                        view! {
                            <circle cx=cx cy=cy r="3.5" fill="hsl(var(--foreground))" opacity="0.35" />
                        }
                    }).collect::<Vec<_>>()}
                    <path
                        d=trail_d
                        stroke="hsl(var(--primary))" stroke-width="1.5" fill="none"
                        opacity="0.4" stroke-dasharray="2 4"
                    />
                    {move || {
                        let t = progress.get();
                        let bx = (20.0 + xs_ball.sample(t)).to_string();
                        let by = (100.0 + ys_ball.sample(t)).to_string();
                        view! {
                            <circle cx=bx cy=by r="7" fill="hsl(var(--primary))" />
                        }
                    }}
                </svg>
            </div>
        </div>
    }
}

#[component]
fn MorphDemo() -> impl IntoView {
    let shape_toggle = MotionValue::new(0.0);
    let shape_name = shape_toggle.clone();
    let shape_pts = shape_toggle.clone();
    let shape_p = shape_toggle.clone();
    let shape_v = shape_toggle.clone();

    let toggle = move |_| {
        let target = if shape_toggle.get() < 0.5 { 1.0 } else { 0.0 };
        shape_toggle.animate_to(target, 240.0, 18.0, 1.0);
    };

    let name = move || {
        if shape_name.get() < 0.5 {
            "Blob".to_string()
        } else {
            "Burst".to_string()
        }
    };

    let pts = move || {
        let p = shape_pts.get();
        (0..48)
            .map(|i| {
                let th = i as f64 / 48.0 * std::f64::consts::TAU;
                let r =
                    30.0 + p * 22.0 * (5.0 * th + std::f64::consts::PI).cos();
                let x = 100.0 + r * th.cos();
                let y = 100.0 + r * th.sin();
                format!("{:.1},{:.1}", x, y)
            })
            .collect::<Vec<_>>()
            .join(" ")
    };

    view! {
        <div class="showcase-card p-6">
            <div class="flex items-center gap-2">
                <Icon glyph=Glyph::Star class="h-5 w-5 text-primary" />
                <h2 class="text-xl font-semibold">"Shape Morphing"</h2>
            </div>
            <p class="mt-1 text-sm text-muted-foreground">
                "A spring-driven MotionValue interpolates every vertex between two shapes."
            </p>

            <div class="mt-6 flex flex-wrap items-center justify-between gap-4">
                <button
                    type="button"
                    class="inline-flex h-10 items-center gap-2 rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
                    on:click=toggle
                >
                    <Icon glyph=Glyph::Repeat class="h-4 w-4" />
                    "Morph"
                </button>
                <div class="flex items-center gap-3 font-mono text-xs text-muted-foreground">
                    <span>{name}</span>
                    <span>{move || format!("p={:.2}", shape_p.get())}</span>
                    <span>{move || format!("v={:+.0}", shape_v.velocity())}</span>
                </div>
            </div>

            <div class="mt-6 flex justify-center rounded-md border border-border bg-background p-6">
                <svg viewBox="0 0 200 200" class="h-48 w-48">
                    <polygon
                        points=pts
                        fill="hsl(var(--primary) / 0.12)"
                        stroke="hsl(var(--primary))"
                        stroke-width="2"
                        stroke-linejoin="round"
                    />
                </svg>
            </div>
        </div>
    }
}

#[component]
fn GestureDemo() -> impl IntoView {
    let (hover_enter, hover_leave, hovered) = use_hover();
    let (press_down, press_up, pressed) = use_press();
    let (pan_down, pan_move, pan_up, delta, dragging) = use_pan();

    let mvx_down = MotionValue::new(0.0);
    let mvy_down = MotionValue::new(0.0);
    let mvx_move = mvx_down.clone();
    let mvy_move = mvy_down.clone();
    let mvx_up = mvx_down.clone();
    let mvy_up = mvy_down.clone();
    let mvx_style = mvx_down.clone();
    let mvy_style = mvy_down.clone();
    let mvx_read = mvx_down.clone();
    let mvy_read = mvy_down.clone();

    let on_pan_down = move |e: leptos::ev::MouseEvent| {
        pan_down(e);
    };
    let on_pan_move = move |e: leptos::ev::MouseEvent| {
        pan_move(e);
        // Only track while actually dragging — a plain hover/mousemove must
        // not re-seat the tile (it would fight the spring-back and appear
        // to drag itself across the screen).
        if dragging.get() {
            let (dx, dy) = delta.get();
            mvx_move.jump(dx);
            mvy_move.jump(dy);
        }
    };
    let on_pan_up = move |e: leptos::ev::MouseEvent| {
        pan_up(e);
        mvx_up.animate_to(0.0, 320.0, 22.0, 1.0);
        mvy_up.animate_to(0.0, 320.0, 22.0, 1.0);
    };

    let hover_scale = move || if hovered.get() { 1.05 } else { 1.0 };
    let press_scale = move || if pressed.get() { 0.94 } else { 1.0 };
    let drag_style = move || {
        format!(
            "transform: {};",
            transform_2d(mvx_style.get(), mvy_style.get(), 1.0, 0.0)
        )
    };

    let press_up_leave = press_up.clone();
    let pan_up_leave = on_pan_up.clone();

    view! {
        <div class="showcase-card p-6">
            <div class="flex items-center gap-2">
                <Icon glyph=Glyph::Move class="h-5 w-5 text-primary" />
                <h2 class="text-xl font-semibold">"Gesture Primitives"</h2>
            </div>
            <p class="mt-1 text-sm text-muted-foreground">
                "Hover, press, and pan helpers — drag the tile and it springs back."
            </p>

            <div class="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-3">
                <button
                    type="button"
                    class="flex h-24 flex-col items-center justify-center gap-1 rounded-xl border border-border bg-background font-mono text-xs text-muted-foreground transition-colors"
                    on:mouseenter=hover_enter
                    on:mouseleave=hover_leave
                    style=move || format!("transform: scale({:.2}); transition: {};", hover_scale(), SCALE)
                >
                    <Icon glyph=Glyph::Mouse class="h-5 w-5" />
                    {move || if hovered.get() { "hovered" } else { "hover" }}
                </button>
                <button
                    type="button"
                    class="flex h-24 flex-col items-center justify-center gap-1 rounded-xl border border-border bg-background font-mono text-xs text-muted-foreground transition-colors"
                    on:mousedown=press_down
                    on:mouseup=press_up
                    on:mouseleave=press_up_leave
                    style=move || format!("transform: scale({:.2}); transition: {};", press_scale(), SCALE)
                >
                    <Icon glyph=Glyph::Hand class="h-5 w-5" />
                    {move || if pressed.get() { "pressed" } else { "press" }}
                </button>
                <div
                    class="flex h-24 flex-col items-center justify-center gap-1 rounded-xl border border-border bg-background font-mono text-xs text-muted-foreground cursor-grab active:cursor-grabbing select-none touch-none"
                    on:mousedown=on_pan_down
                    on:mousemove=on_pan_move
                    on:mouseup=on_pan_up
                    on:mouseleave=pan_up_leave
                    style=drag_style
                >
                    <Icon glyph=Glyph::Grip class="h-5 w-5" />
                    {move || if dragging.get() { "dragging" } else { "drag me" }}
                </div>
            </div>

            <div class="mt-6 space-y-1 font-mono text-xs text-muted-foreground">
                <p>{move || format!("hover: {} · press: {} · drag: {}", hovered.get(), pressed.get(), dragging.get())}</p>
                <p>{move || format!("dx {:+.0}px · dy {:+.0}px", delta.get().0, delta.get().1)}</p>
                <p>{move || format!("vx {:+.1} · vy {:+.1}", mvx_read.velocity(), mvy_read.velocity())}</p>
            </div>
        </div>
    }
}

/// A spinning pentagon with balls bouncing inside it — gravity, circle-wall
/// collisions, and a rotation all driven by a single continuous FrameLoop.
#[component]
fn PentagonBallsDemo() -> impl IntoView {
    let rot = RwSignal::new(0.0);
    // (x, y, vx, vy) per ball.
    let balls: Vec<RwSignal<(f64, f64, f64, f64)>> = (0..9)
        .map(|i| {
            let a = i as f64 / 9.0 * std::f64::consts::TAU;
            RwSignal::new((
                100.0 + 50.0 * a.cos(),
                100.0 + 50.0 * a.sin(),
                (i as f64 - 4.0) * 42.0,
                (i as f64 - 4.0) * 30.0,
            ))
        })
        .collect();

    let start = FrameLoop::now();
    let balls_for_loop = balls.clone();
    FrameLoop::on_frame(move || {
        let t = FrameLoop::now() - start;
        rot.set((t * 34.0) % 360.0);
        let dt = 1.0 / 60.0;
        let mut updated: Vec<(usize, (f64, f64, f64, f64))> = Vec::new();
        for (i, b) in balls_for_loop.iter().enumerate() {
            let (mut x, mut y, mut vx, mut vy) = b.get();
            vy += 300.0 * dt;
            x += vx * dt;
            y += vy * dt;
            let dx = x - 100.0;
            let dy = y - 100.0;
            let r = (dx * dx + dy * dy).sqrt();
            let bound = 60.0;
            if r > bound && r > 0.0 {
                let nx = dx / r;
                let ny = dy / r;
                let vn = vx * nx + vy * ny;
                if vn > 0.0 {
                    vx -= 2.0 * vn * nx;
                    vy -= 2.0 * vn * ny;
                    vx *= 0.96;
                    vy *= 0.96;
                }
                x = 100.0 + nx * bound;
                y = 100.0 + ny * bound;
            }
            updated.push((i, (x, y, vx, vy)));
        }
        for (i, v) in updated {
            balls_for_loop[i].set(v);
        }
        true
    });

    // Pentagon vertices at radius 80 around the center.
    let pent_pts: String = (0..5)
        .map(|i| {
            let a = -std::f64::consts::FRAC_PI_2
                + i as f64 / 5.0 * std::f64::consts::TAU;
            format!(
                "{:.1},{:.1}",
                100.0 + 80.0 * a.cos(),
                100.0 + 80.0 * a.sin()
            )
        })
        .collect::<Vec<_>>()
        .join(" ");

    view! {
        <div class="showcase-card p-6">
            <div class="flex items-center gap-2">
                <Icon glyph=Glyph::Hexagon class="h-5 w-5 text-primary" />
                <h2 class="text-xl font-semibold">"Bouncing Balls in a Spinning Pentagon"</h2>
            </div>
            <p class="mt-1 text-sm text-muted-foreground">
                "Gravity, elastic wall collisions, and continuous rotation — one
                FrameLoop, zero CSS keyframes."
            </p>
            <div class="mt-6 flex justify-center rounded-md border border-border bg-background p-6">
                <svg viewBox="0 0 200 200" class="h-64 w-64">
                    <g transform=move || format!("rotate({} 100 100)", rot.get())>
                        <polygon
                            points=pent_pts
                            fill="hsl(var(--primary) / 0.06)"
                            stroke="hsl(var(--primary))"
                            stroke-width="2"
                            stroke-linejoin="round"
                        />
                    </g>
                    {balls.iter().map(|b| {
                        let ball = *b;
                        let cx = move || format!("{:.1}", ball.get().0);
                        let cy = move || format!("{:.1}", ball.get().1);
                        view! {
                            <circle
                                cx=cx
                                cy=cy
                                r="5"
                                fill="hsl(var(--primary))"
                                style=format!("transition: none;")
                            />
                        }
                    }).collect::<Vec<_>>()}
                </svg>
            </div>
        </div>
    }
}
