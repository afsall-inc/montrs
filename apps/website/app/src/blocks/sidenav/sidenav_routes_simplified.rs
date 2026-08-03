use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn SidenavRoutesSimplified() -> impl IntoView {
    let active = RwSignal::new("Home");
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
                    let l = label;
                    let is_active = move || active.get() == l;
                    let click = move |_| active.set(l);
                    view! {
                        <button on:click=click class=move || {
                            let base = "flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs transition-colors";
                            if is_active() {
                                format!("{} bg-primary/10 text-primary font-medium", base)
                            } else {
                                format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                            }
                        }>
                            <Icon glyph=icon class="w-3.5 h-3.5" />
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}