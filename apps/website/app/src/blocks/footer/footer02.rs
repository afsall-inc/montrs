use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer02() -> impl IntoView {
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
                                    view! { <li><a href="#" class="hover:text-foreground transition-colors">{link}</a></li> }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div class="mt-8 pt-6 border-t border-border flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <Icon glyph=Glyph::Blocks class="w-5 h-5 text-primary" />
                    <span class="text-sm font-semibold">"MontRS"</span>
                </div>
                <p class="text-xs text-muted-foreground">"© 2026 MontRS. All rights reserved."</p>
            </div>
        </div>
    }
}