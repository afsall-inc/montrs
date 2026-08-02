use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav05() -> impl IntoView {
    let items = vec![
        ("Dashboard", Glyph::LayoutDashboard),
        ("Projects", Glyph::Folder),
        ("Team", Glyph::Users),
        ("Settings", Glyph::Settings),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                <div class="flex items-center gap-3 px-3 py-3 mb-4 rounded-lg bg-muted">
                    <div class="flex h-9 w-9 items-center justify-center rounded-full bg-primary/10 text-sm font-bold text-primary">"JD"</div>
                    <div class="flex-1 min-w-0">
                        <p class="text-sm font-medium truncate">"Jane Doe"</p>
                        <p class="text-xs text-muted-foreground truncate">"jane@example.com"</p>
                    </div>
                </div>
                {items.into_iter().map(|(label, icon)| {
                    view! {
                        <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                            <Icon glyph=icon class="w-4 h-4" />
                            {label}
                        </a>
                    }
                }).collect::<Vec<_>>()}
                <div class="mt-4 pt-4 border-t border-border">
                    <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                        <Icon glyph=Glyph::DoorOpen class="w-4 h-4" />
                        "Log out"
                    </a>
                </div>
            </div>
        </div>
    }
}