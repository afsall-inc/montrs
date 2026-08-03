use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration06() -> impl IntoView {
    let expanded = RwSignal::new(Option::<usize>::None);
    let testimonials = vec![
        ("Alice Chen", "CTO at TechCorp", "MontRS transformed our development workflow. The compile-time guarantees are a game-changer."),
        ("Bob Martinez", "Lead Engineer at StartupX", "The agent system is incredible. We built our entire API layer in days, not weeks."),
        ("Carol Williams", "Founder of WebForge", "Finally, a Rust framework that makes full-stack development feel natural and productive."),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                {testimonials.into_iter().enumerate().map(|(i, (name, role, quote))| {
                    let is_expanded = move || expanded.get() == Some(i);
                    let toggle = move |_| expanded.set(if is_expanded() { None } else { Some(i) });
                    view! {
                        <button on:click=toggle class=move || {
                            let base = "rounded-lg border p-6 text-left transition-all";
                            if is_expanded() {
                                format!("{} border-primary bg-primary/5 ring-1 ring-primary", base)
                            } else {
                                format!("{} border-border bg-muted/30 hover:bg-muted/50 hover:border-primary/30", base)
                            }
                        }>
                            <div class="flex items-center gap-2 mb-4">
                                <div class="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-sm font-bold text-primary">
                                    {name.chars().next().unwrap()}
                                </div>
                                <div>
                                    <h4 class="text-sm font-medium">{name}</h4>
                                    <p class="text-xs text-muted-foreground">{role}</p>
                                </div>
                            </div>
                            <p class="text-sm text-muted-foreground italic">{format!("\u{201c}{}\u{201d}", quote)}</p>
                            <div class="mt-4 flex gap-1">
                                {(0..5).map(|_| {
                                    view! { <Icon glyph=Glyph::Star class="w-4 h-4 fill-yellow-400 text-yellow-400" /> }
                                }).collect::<Vec<_>>()}
                            </div>
                            <Show when=is_expanded>
                                <p class="mt-3 text-xs text-primary">"Click to collapse"</p>
                            </Show>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}