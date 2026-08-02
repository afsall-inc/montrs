use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Sidenav08() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="w-56 p-4 space-y-1">
                <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::LayoutDashboard class="w-4 h-4" />"Dashboard"
                </a>
                <button on:click=move |_| open.update(|v| *v = !*v)
                    class="flex w-full items-center justify-between rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                    <div class="flex items-center gap-3">
                        <Icon glyph=Glyph::Folder class="w-4 h-4" />"Projects"
                    </div>
                    <Icon glyph=Glyph::ChevronDown class=move || {
                        if open.get() { "w-4 h-4 transition-transform rotate-180" } else { "w-4 h-4 transition-transform" }
                    } />
                </button>
                <Show when=move || open.get()>
                    <div class="ml-6 space-y-1 border-l border-border pl-3">
                        <a href="#" class="block rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">"Active"</a>
                        <a href="#" class="block rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">"Archived"</a>
                        <a href="#" class="block rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">"Templates"</a>
                    </div>
                </Show>
                <a href="#" class="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors">
                    <Icon glyph=Glyph::Settings class="w-4 h-4" />"Settings"
                </a>
            </div>
        </div>
    }
}