use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav06() -> impl IntoView {
    let active = RwSignal::new("Inbox");
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
                    let l = label;
                    let is_active = move || active.get() == l;
                    let click = move |_| active.set(l);
                    view! {
                        <button on:click=click class=move || {
                            let base = "flex w-full items-center justify-between rounded-md px-3 py-2 text-sm transition-colors";
                            if is_active() {
                                format!("{} bg-primary/10 text-primary font-medium", base)
                            } else {
                                format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                            }
                        }>
                            <div class="flex items-center gap-3">
                                <Icon glyph=icon class="w-4 h-4" />
                                {label}
                            </div>
                            {if !badge.is_empty() {
                                Some(view! {
                                    <span class="rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">{badge}</span>
                                })
                            } else { None }}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
