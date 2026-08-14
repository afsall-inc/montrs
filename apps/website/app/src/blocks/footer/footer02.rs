use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer02() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let subscribed = RwSignal::new(false);
    let subscribe = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !email.get().trim().is_empty() {
            subscribed.set(true);
        }
    };
    let columns = vec![
        ("Product", vec!["Features", "Pricing", "Docs", "Changelog"]),
        ("Company", vec!["About", "Blog", "Careers", "Press"]),
        ("Resources", vec!["Community", "Support", "Status", "API"]),
        ("Legal", vec!["Privacy", "Terms", "Security", "Cookies"]),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                {columns.into_iter().map(|(title, links)| {
                    view! {
                        <div>
                            <h4 class="text-sm font-semibold mb-3">{title}</h4>
                            <ul class="space-y-2 text-sm text-muted-foreground">
                                {links.into_iter().map(|link| {
                                    view! { <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">{link}</a></li> }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div class="mt-8 pt-6 border-t border-border">
                <form on:submit=subscribe class="flex items-center gap-2 mb-4">
                    <input
                        type="email"
                        placeholder="Enter your email"
                        on:input=move |ev| email.set(event_target_value(&ev))
                        prop:value=email
                        class="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                    <button type="submit" class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors shrink-0">
                        "Subscribe"
                    </button>
                </form>
                <Show when=move || subscribed.get()>
                    <p class="text-xs text-green-600 dark:text-green-400 mb-4">"Thanks for subscribing!"</p>
                </Show>
            </div>
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <Icon glyph=Glyph::Blocks class="w-5 h-5 text-primary" />
                    <span class="text-sm font-semibold">"MontRS"</span>
                </div>
                <p class="text-xs text-muted-foreground">"© 2026 MontRS. All rights reserved."</p>
            </div>
        </div>
    }
}
