use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn FooterLogos() -> impl IntoView {
    let paused = RwSignal::new(false);
    let logos = vec!["Rust", "WASM", "Tailwind", "Docker", "Postgres", "Redis"];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <p class="text-center text-xs text-muted-foreground mb-4 uppercase tracking-wider">"Trusted by teams using"</p>
            <div
                class="flex items-center justify-center gap-8 overflow-hidden"
                on:mouseenter=move |_| paused.set(true)
                on:mouseleave=move |_| paused.set(false)
            >
                <div class=move || {
                    let base = "flex items-center gap-8 transition-transform duration-500";
                    if paused.get() { format!("{} animate-none", base) }
                    else { format!("{} animate-marquee", base) }
                }>
                    {logos.iter().chain(logos.iter()).map(|name| {
                        view! {
                            <div class="flex items-center gap-2 text-sm text-muted-foreground shrink-0">
                                <Icon glyph=Glyph::Blocks class="w-4 h-4" />
                                <span class="font-medium">{*name}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}