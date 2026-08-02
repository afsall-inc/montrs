use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn FooterLogos() -> impl IntoView {
    let logos = vec!["Rust", "WASM", "Tailwind", "Docker", "Postgres", "Redis"];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <p class="text-center text-xs text-muted-foreground mb-4 uppercase tracking-wider">"Trusted by teams using"</p>
            <div class="flex flex-wrap items-center justify-center gap-8">
                {logos.into_iter().map(|name| {
                    view! {
                        <div class="flex items-center gap-2 text-sm text-muted-foreground">
                            <Icon glyph=Glyph::Blocks class="w-4 h-4" />
                            <span class="font-medium">{name}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}