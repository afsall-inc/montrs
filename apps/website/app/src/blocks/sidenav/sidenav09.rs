use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav09() -> impl IntoView {
    let items = vec![Glyph::LayoutDashboard, Glyph::Search, Glyph::Settings, Glyph::Mail, Glyph::Bell, Glyph::User];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-16 p-3 flex flex-col items-center gap-2">
                {items.into_iter().map(|icon| {
                    view! {
                        <a href="#" class="flex items-center justify-center rounded-md p-2 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                            <Icon glyph=icon class="w-5 h-5" />
                        </a>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}