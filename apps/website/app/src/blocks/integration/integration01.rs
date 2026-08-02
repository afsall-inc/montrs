use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration01() -> impl IntoView {
    let icons = vec![
        Glyph::Search, Glyph::Settings, Glyph::User, Glyph::Bell,
        Glyph::LayoutDashboard, Glyph::Mail, Glyph::Calendar, Glyph::Clock,
    ];

    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <h3 class="text-sm font-semibold mb-4">"Icon Library"</h3>
            <div class="grid grid-cols-4 gap-4">
                {icons.into_iter().map(|g| {
                    view! {
                        <div class="flex flex-col items-center gap-2 rounded-lg border border-border bg-muted/50 p-4 hover:bg-muted transition-colors">
                            <Icon glyph=g class="w-6 h-6 text-foreground" />
                            <span class="text-xs text-muted-foreground">{format!("{:?}", g)}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}