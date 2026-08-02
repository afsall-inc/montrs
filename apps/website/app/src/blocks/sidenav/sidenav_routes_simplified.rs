use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn SidenavRoutesSimplified() -> impl IntoView {
    let routes = vec![
        ("Home", Glyph::LayoutDashboard),
        ("Docs", Glyph::BookOpen),
        ("Components", Glyph::Blocks),
        ("Pricing", Glyph::CreditCard),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-44 p-3 space-y-0.5">
                {routes.into_iter().map(|(label, icon)| {
                    view! {
                        <a href="#" class="flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                            <Icon glyph=icon class="w-3.5 h-3.5" />
                            {label}
                        </a>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}