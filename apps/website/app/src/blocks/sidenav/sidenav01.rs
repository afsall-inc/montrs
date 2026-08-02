use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav01() -> impl IntoView {
    let items = vec![
        ("Home", Glyph::LayoutDashboard),
        ("Search", Glyph::Search),
        ("Settings", Glyph::Settings),
        ("Messages", Glyph::Mail),
        ("Notifications", Glyph::Bell),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                <div class="flex items-center gap-2 px-3 py-2 mb-4">
                    <Icon glyph=Glyph::Blocks class="w-5 h-5 text-primary" />
                    <span class="font-semibold text-sm">"MontRS"</span>
                </div>
                {items.into_iter().map(|(label, icon)| {
                    view! {
                        <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                            <Icon glyph=icon class="w-4 h-4" />
                            {label}
                        </a>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}