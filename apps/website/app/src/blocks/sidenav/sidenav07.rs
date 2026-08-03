use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav07() -> impl IntoView {
    let active = RwSignal::new("Home");
    let items = vec![
        ("Home", "Navigate to home", Glyph::LayoutDashboard),
        ("Search", "Search the workspace", Glyph::Search),
        ("Settings", "Manage preferences", Glyph::Settings),
        ("Messages", "View messages", Glyph::Mail),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                {items.into_iter().map(|(label, tooltip, icon)| {
                    let l = label;
                    let is_active = move || active.get() == l;
                    let click = move |_| active.set(l);
                    view! {
                        <button on:click=click title=tooltip class=move || {
                            let base = "group relative flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors";
                            if is_active() {
                                format!("{} bg-primary/10 text-primary font-medium", base)
                            } else {
                                format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                            }
                        }>
                            <Icon glyph=icon class="w-4 h-4" />
                            {label}
                            <span class="absolute left-full ml-2 hidden group-hover:inline-flex rounded-md bg-foreground px-2 py-1 text-xs text-background whitespace-nowrap z-10">
                                {tooltip}
                            </span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}