use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer04() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm">
            <div class="flex flex-col md:flex-row items-center justify-between gap-6 pb-8 border-b border-border">
                <div>
                    <h3 class="text-lg font-semibold">"Stay up to date"</h3>
                    <p class="text-sm text-muted-foreground mt-1">"Get the latest MontRS news and updates."</p>
                </div>
                <div class="flex gap-2 w-full md:w-auto">
                    <input
                        type="email"
                        placeholder="Enter your email"
                        class="flex-1 md:w-64 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                    <button class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors shrink-0">
                        "Subscribe"
                    </button>
                </div>
            </div>
            <div class="mt-8 grid grid-cols-2 md:grid-cols-4 gap-8">
                <div>
                    <h4 class="text-sm font-semibold mb-3">"Product"</h4>
                    <ul class="space-y-2 text-sm text-muted-foreground">
                        <li><a href="#" class="hover:text-foreground">"Features"</a></li>
                        <li><a href="#" class="hover:text-foreground">"Pricing"</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="text-sm font-semibold mb-3">"Company"</h4>
                    <ul class="space-y-2 text-sm text-muted-foreground">
                        <li><a href="#" class="hover:text-foreground">"About"</a></li>
                        <li><a href="#" class="hover:text-foreground">"Blog"</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="text-sm font-semibold mb-3">"Resources"</h4>
                    <ul class="space-y-2 text-sm text-muted-foreground">
                        <li><a href="#" class="hover:text-foreground">"Docs"</a></li>
                        <li><a href="#" class="hover:text-foreground">"API"</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="text-sm font-semibold mb-3">"Legal"</h4>
                    <ul class="space-y-2 text-sm text-muted-foreground">
                        <li><a href="#" class="hover:text-foreground">"Privacy"</a></li>
                        <li><a href="#" class="hover:text-foreground">"Terms"</a></li>
                    </ul>
                </div>
            </div>
            <div class="mt-8 pt-6 border-t border-border flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <Icon glyph=Glyph::Blocks class="w-5 h-5 text-primary" />
                    <span class="text-sm font-semibold">"MontRS"</span>
                </div>
                <p class="text-xs text-muted-foreground">"© 2026 MontRS"</p>
            </div>
        </div>
    }
}