use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section class="relative overflow-hidden">
            <div class="mx-auto max-w-6xl px-6 py-24 sm:py-32 lg:px-8">
                <div class="text-center">
                    <div class="flex justify-center mb-6">
                        <Icon glyph=Glyph::Blocks class="w-16 h-16 text-primary" size="64" />
                    </div>
                    <h1 class="text-4xl font-bold tracking-tight sm:text-6xl">
                        "MontRS"
                        <span class="text-primary block mt-2">"A full-stack Rust framework"</span>
                    </h1>
                    <p class="mt-6 text-lg leading-8 text-muted-foreground max-w-2xl mx-auto">
                        "Build web applications with compile-time correctness, explicit boundaries, and deterministic execution. Designed for humans and agents alike."
                    </p>
                    <div class="mt-10 flex items-center justify-center gap-4">
                        <a href="/components"
                            class="inline-flex items-center rounded-md bg-primary px-6 py-3 text-sm font-semibold text-primary-foreground shadow-sm hover:bg-primary/90 transition-colors"
                        >
                            "Browse Components"
                            <Icon glyph=Glyph::ArrowRight class="ml-2 w-4 h-4" />
                        </a>
                        <a href="/icons"
                            class="inline-flex items-center rounded-md border border-border px-6 py-3 text-sm font-semibold hover:bg-accent transition-colors"
                        >
                            "Browse Icons"
                            <Icon glyph=Glyph::Image class="ml-2 w-4 h-4" />
                        </a>
                    </div>
                </div>
            </div>
        </section>

        <section class="border-t border-border py-20">
            <div class="mx-auto max-w-6xl px-6 lg:px-8">
                <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                    <FeatureCard
                        icon=Glyph::Shield
                        title="Type-Safe"
                        description="Compile-time correctness with Rust's type system. No runtime surprises."
                    />
                    <FeatureCard
                        icon=Glyph::Puzzle
                        title="Modular Plates"
                        description="Compose your app from independent, reusable plates with clear boundaries."
                    />
                    <FeatureCard
                        icon=Glyph::Bot
                        title="Agent-First"
                        description="Machine-readable metadata, snapshots, and error tracking for AI coding partners."
                    />
                    <FeatureCard
                        icon=Glyph::Paintbrush
                        title="Tailwind CSS"
                        description="Beautiful UIs with Tailwind CSS and shadcn-inspired theming system."
                    />
                    <FeatureCard
                        icon=Glyph::Rocket
                        title="Fast Compilation"
                        description="Incremental compilation, WASM targets, and optimized build pipelines."
                    />
                    <FeatureCard
                        icon=Glyph::Heart
                        title="Open Source"
                        description="MIT licensed. Community-driven. Built for the future of web development."
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    icon: Glyph,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-6 hover:shadow-md transition-shadow">
            <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
                <Icon glyph=icon class="w-6 h-6 text-primary" />
            </div>
            <h3 class="text-lg font-semibold text-card-foreground">{title}</h3>
            <p class="mt-2 text-sm text-muted-foreground">{description}</p>
        </div>
    }
}