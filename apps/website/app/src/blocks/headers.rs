use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn HeaderBlock() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <header class="flex items-center justify-between px-6 py-4 border-b border-border">
                <div class="flex items-center gap-2">
                    <Icon glyph=Glyph::Blocks class="w-6 h-6 text-primary" />
                    <span class="font-bold">"Brand"</span>
                </div>
                <nav class="hidden md:flex items-center gap-6 text-sm">
                    <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Features"</a>
                    <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Pricing"</a>
                    <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"About"</a>
                </nav>
                <div class="flex items-center gap-3">
                    <button class="text-sm text-muted-foreground hover:text-foreground">"Sign In"</button>
                    <button class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                        "Get Started"
                    </button>
                </div>
            </header>
            <div class="p-12 text-center">
                <h2 class="text-3xl font-bold">"Beautiful header with animation"</h2>
                <p class="mt-4 text-muted-foreground max-w-md mx-auto">
                    "Scroll down to see the sticky header effect with backdrop blur."
                </p>
            </div>
        </div>
    }
}