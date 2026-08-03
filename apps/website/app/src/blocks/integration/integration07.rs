use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration07() -> impl IntoView {
    let hovered = RwSignal::new(Option::<&str>::None);
    let stats = vec![
        ("10K+", "GitHub Stars"),
        ("500+", "Contributors"),
        ("50K+", "Apps Built"),
        ("99.9%", "Uptime"),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                {stats.into_iter().map(|(value, label)| {
                    let l = label;
                    let is_hovered = move || hovered.get() == Some(l);
                    let mouseenter = move |_| hovered.set(Some(l));
                    let mouseleave = move |_| hovered.set(None);
                    view! {
                        <div
                            on:mouseenter=mouseenter
                            on:mouseleave=mouseleave
                            class=move || {
                                let base = "text-center rounded-lg p-4 transition-all cursor-default";
                                if is_hovered() {
                                    format!("{} bg-primary/5 scale-105", base)
                                } else {
                                    format!("{} hover:bg-muted/50", base)
                                }
                            }
                        >
                            <p class="text-3xl font-bold text-primary">{value}</p>
                            <p class="mt-1 text-sm text-muted-foreground">{label}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}