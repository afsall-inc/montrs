use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer01() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm text-center">
            <div class="flex items-center justify-center gap-2 mb-4">
                <Icon glyph=Glyph::Blocks class="w-6 h-6 text-primary" />
                <span class="font-bold text-lg">"MontRS"</span>
            </div>
            <p class="text-sm text-muted-foreground mb-6">"Building the future of full-stack Rust web development."</p>
            <div class="flex justify-center gap-4 text-sm text-muted-foreground">
                <a href="#" class="hover:text-foreground transition-colors">"Twitter"</a>
                <a href="#" class="hover:text-foreground transition-colors">"GitHub"</a>
                <a href="#" class="hover:text-foreground transition-colors">"Discord"</a>
            </div>
            <p class="mt-6 text-xs text-muted-foreground">"© 2026 MontRS. All rights reserved."</p>
        </div>
    }
}