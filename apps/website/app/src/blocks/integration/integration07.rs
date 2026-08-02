use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration07() -> impl IntoView {
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
                    view! {
                        <div class="text-center">
                            <p class="text-3xl font-bold text-primary">{value}</p>
                            <p class="mt-1 text-sm text-muted-foreground">{label}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}