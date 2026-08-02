use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav02() -> impl IntoView {
    let sections = vec![
        ("Main", vec![("Home", Glyph::LayoutDashboard), ("Dashboard", Glyph::LayoutDashboard), ("Analytics", Glyph::ChartColumn)]),
        ("Workspace", vec![("Files", Glyph::Files), ("Projects", Glyph::Folder), ("Team", Glyph::Users)]),
        ("Settings", vec![("Account", Glyph::User), ("Preferences", Glyph::Settings)]),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-6">
                {sections.into_iter().map(|(title, items)| {
                    view! {
                        <div>
                            <h4 class="px-3 mb-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground">{title}</h4>
                            <div class="space-y-1">
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
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}