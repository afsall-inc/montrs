use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer05() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-6 shadow-sm">
            <div class="flex flex-col md:flex-row items-center justify-between gap-4">
                <div class="flex items-center gap-2">
                    <Icon glyph=Glyph::Blocks class="w-5 h-5 text-primary" />
                    <span class="text-sm font-semibold">"MontRS"</span>
                </div>
                <div class="flex items-center gap-4 text-sm text-muted-foreground">
                    <a href="#" class="hover:text-foreground transition-colors">"Privacy"</a>
                    <a href="#" class="hover:text-foreground transition-colors">"Terms"</a>
                    <a href="#" class="hover:text-foreground transition-colors">"Contact"</a>
                </div>
                <p class="text-xs text-muted-foreground">"© 2026 MontRS"</p>
            </div>
        </div>
    }
}