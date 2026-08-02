use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav06() -> impl IntoView {
    let items = vec![
        ("Inbox", "3", Glyph::Mail),
        ("Tasks", "12", Glyph::SquareCheck),
        ("Notifications", "5", Glyph::Bell),
        ("Analytics", "", Glyph::ChartColumn),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                {items.into_iter().map(|(label, badge, icon)| {
                    view! {
                        <a href="#" class="flex items-center justify-between rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                            <div class="flex items-center gap-3">
                                <Icon glyph=icon class="w-4 h-4" />
                                {label}
                            </div>
                            {if !badge.is_empty() {
                                Some(view! {
                                    <span class="rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">{badge}</span>
                                })
                            } else { None }}
                        </a>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}