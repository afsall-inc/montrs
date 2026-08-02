use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn FooterSimple() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm text-center">
            <div class="flex items-center justify-center gap-2 mb-4">
                <Icon glyph=Glyph::Blocks class="w-6 h-6 text-primary" />
                <span class="font-bold text-lg">"Brand"</span>
            </div>
            <p class="text-sm text-muted-foreground mb-6">
                "Building the future of web development."
            </p>
            <div class="flex justify-center gap-4 text-sm text-muted-foreground">
                <a href="#" class="hover:text-foreground transition-colors">"Twitter"</a>
                <a href="#" class="hover:text-foreground transition-colors">"GitHub"</a>
                <a href="#" class="hover:text-foreground transition-colors">"Discord"</a>
            </div>
            <p class="mt-6 text-xs text-muted-foreground">
                "© 2026 Brand. All rights reserved."
            </p>
        </div>
    }
}

#[component]
pub fn FooterGrid() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm">
            <div class="grid grid-cols-2 gap-8">
                <div>
                    <h3 class="text-sm font-semibold mb-3">"Product"</h3>
                    <ul class="space-y-2 text-sm text-muted-foreground">
                        <li><a href="#" class="hover:text-foreground">"Features"</a></li>
                        <li><a href="#" class="hover:text-foreground">"Pricing"</a></li>
                        <li><a href="#" class="hover:text-foreground">"Docs"</a></li>
                    </ul>
                </div>
                <div>
                    <h3 class="text-sm font-semibold mb-3">"Company"</h3>
                    <ul class="space-y-2 text-sm text-muted-foreground">
                        <li><a href="#" class="hover:text-foreground">"About"</a></li>
                        <li><a href="#" class="hover:text-foreground">"Blog"</a></li>
                        <li><a href="#" class="hover:text-foreground">"Careers"</a></li>
                    </ul>
                </div>
            </div>
            <div class="mt-8 pt-6 border-t border-border text-center text-xs text-muted-foreground">
                "© 2026 Brand. All rights reserved."
            </div>
        </div>
    }
}