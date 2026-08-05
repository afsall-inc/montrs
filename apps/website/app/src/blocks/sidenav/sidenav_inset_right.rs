use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn SidenavInsetRight() -> impl IntoView {
    let active = RwSignal::new("Properties");
    let items = vec![
        ("Properties", Glyph::Settings),
        ("Comments", Glyph::MessageSquare),
        ("Activity", Glyph::Activity),
        ("History", Glyph::Clock),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-48 p-4 space-y-1">
                <h4 class="px-3 mb-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">"Details"</h4>
                {items.into_iter().map(|(label, icon)| {
                    let l = label;
                    let is_active = move || active.get() == l;
                    let click = move |_| active.set(l);
                    view! {
                        <button on:click=click class=move || {
                            let base = "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors";
                            if is_active() {
                                format!("{} bg-primary/10 text-primary font-medium", base)
                            } else {
                                format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                            }
                        }>
                            <Icon glyph=icon class="w-4 h-4" />
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}
                <div class="mt-4 pt-4 border-t border-border px-3">
                    <p class="text-xs text-muted-foreground">"Status: Active"</p>
                    <p class="text-xs text-muted-foreground mt-1">"Last edited: 2m ago"</p>
                </div>
            </div>
        </div>
    }
}
