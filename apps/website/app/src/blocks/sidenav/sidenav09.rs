use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav09() -> impl IntoView {
    let active = RwSignal::new(Glyph::LayoutDashboard);
    let items = vec![
        Glyph::LayoutDashboard,
        Glyph::Search,
        Glyph::Settings,
        Glyph::Mail,
        Glyph::Bell,
        Glyph::User,
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-16 p-3 flex flex-col items-center gap-2">
                {items.into_iter().map(|icon| {
                    let is_active = move || active.get() == icon;
                    let click = move |_| active.set(icon);
                    view! {
                        <button on:click=click class=move || {
                            let base = "flex items-center justify-center rounded-md p-2 transition-colors";
                            if is_active() {
                                format!("{} bg-primary/10 text-primary", base)
                            } else {
                                format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base)
                            }
                        }>
                            <Icon glyph=icon class="w-5 h-5" />
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
