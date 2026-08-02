use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-border">
            <div class="mx-auto max-w-6xl px-6 py-12 lg:px-8">
                <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">
                    <div>
                        <div class="flex items-center gap-2 font-bold text-lg mb-4">
                            <Icon glyph=Glyph::Blocks class="w-5 h-5 text-primary" />
                            "MontRS"
                        </div>
                        <p class="text-sm text-muted-foreground">
                            "A full-stack Rust framework for humans and agents."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-sm font-semibold mb-3">"Framework"</h3>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="/components">"Components"</a></li>
                            <li><a href="/icons">"Icons"</a></li>
                            <li><a href="/blocks">"Blocks"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h3 class="text-sm font-semibold mb-3">"Community"</h3>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="https://github.com/montrs/montrs" target="_blank">"GitHub"</a></li>
                            <li><a href="https://docs.montrs.com" target="_blank">"Documentation"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h3 class="text-sm font-semibold mb-3">"Legal"</h3>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li>"MIT License"</li>
                        </ul>
                    </div>
                </div>
                <div class="mt-8 border-t border-border pt-8 text-center text-sm text-muted-foreground">
                    "© 2026 MontRS. All rights reserved."
                </div>
            </div>
        </footer>
    }
}