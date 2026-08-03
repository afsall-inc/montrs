use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration05() -> impl IntoView {
    let selected = RwSignal::new(Option::<&str>::None);
    let plans = vec![
        ("Starter", "Free", vec!["1 project", "Basic features", "Community support"], false),
        ("Pro", "$29/mo", vec!["Unlimited projects", "Advanced features", "Priority support", "Team access"], true),
        ("Enterprise", "Custom", vec!["Everything in Pro", "SSO", "Dedicated support", "SLA"], false),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                {plans.into_iter().map(|(name, price, features, popular)| {
                    let n = name;
                    let is_selected = move || selected.get() == Some(n);
                    let click = move |_| selected.set(Some(n));
                    view! {
                        <div class=move || {
                            let base = "relative rounded-lg border p-6 flex flex-col transition-all";
                            if is_selected() {
                                format!("{} border-primary bg-primary/10 ring-2 ring-primary", base)
                            } else if popular {
                                format!("{} border-primary bg-primary/5", base)
                            } else {
                                format!("{} border-border hover:border-primary/30", base)
                            }
                        }>
                            {if popular {
                                Some(view! {
                                    <span class="absolute -top-3 left-1/2 -translate-x-1/2 rounded-full bg-primary px-3 py-1 text-xs font-medium text-primary-foreground">
                                        "Popular"
                                    </span>
                                })
                            } else { None }}
                            <h3 class="text-lg font-semibold">{name}</h3>
                            <p class="mt-2 text-3xl font-bold">{price}</p>
                            <ul class="mt-4 space-y-2 text-sm text-muted-foreground flex-1">
                                {features.into_iter().map(|f| {
                                    view! {
                                        <li class="flex items-center gap-2">
                                            <Icon glyph=Glyph::Check class="w-4 h-4 text-green-500" />
                                            {f}
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                            <button on:click=click class=move || {
                                let base = "mt-6 w-full rounded-md px-4 py-2 text-sm font-medium transition-all";
                                if is_selected() || popular {
                                    format!("{} bg-primary text-primary-foreground hover:bg-primary/90", base)
                                } else {
                                    format!("{} border border-border bg-background hover:bg-muted", base)
                                }
                            }>
                                {if is_selected() { "Selected" } else { "Get Started" }}
                            </button>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}