use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn SidenavRoutes() -> impl IntoView {
    let active = RwSignal::new("/");
    let routes = vec![
        ("/", "Home", Glyph::LayoutDashboard),
        ("/docs", "Docs", Glyph::BookOpen),
        ("/components", "Components", Glyph::Blocks),
        ("/pricing", "Pricing", Glyph::CreditCard),
        ("/blog", "Blog", Glyph::FileText),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                <h4 class="px-3 mb-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">"Routes"</h4>
                {routes.into_iter().map(|(path, label, icon)| {
                    let p = path;
                    let is_active = move || active.get() == p;
                    let click = move |_| active.set(p);
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
                            <span class="ml-auto text-xs text-muted-foreground/60">{path}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}