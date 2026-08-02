use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav11() -> impl IntoView {
    let workspace = RwSignal::new("Personal");

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                <div class="relative mb-4">
                    <button class="flex w-full items-center justify-between rounded-md border border-border bg-background px-3 py-2 text-sm hover:bg-muted transition-colors">
                        <span>{workspace.get()}</span>
                        <Icon glyph=Glyph::ChevronDown class="w-4 h-4 text-muted-foreground" />
                    </button>
                </div>
                <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::LayoutDashboard class="w-4 h-4" />"Overview"
                </a>
                <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::Folder class="w-4 h-4" />"Projects"
                </a>
                <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::Users class="w-4 h-4" />"Members"
                </a>
                <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::Settings class="w-4 h-4" />"Settings"
                </a>
            </div>
        </div>
    }
}