use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer03() -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="bg-primary/10 p-8 text-center">
                <h3 class="text-lg font-semibold">"Ready to get started?"</h3>
                <p class="mt-2 text-sm text-muted-foreground">"Join thousands of developers building with MontRS."</p>
                <button class="mt-4 rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                    "Get Started Free"
                </button>
            </div>
            <div class="p-8">
                <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                    <div>
                        <h4 class="text-sm font-semibold mb-3">"Product"</h4>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="#" class="hover:text-foreground">"Features"</a></li>
                            <li><a href="#" class="hover:text-foreground">"Pricing"</a></li>
                            <li><a href="#" class="hover:text-foreground">"Docs"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-sm font-semibold mb-3">"Company"</h4>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="#" class="hover:text-foreground">"About"</a></li>
                            <li><a href="#" class="hover:text-foreground">"Blog"</a></li>
                            <li><a href="#" class="hover:text-foreground">"Careers"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-sm font-semibold mb-3">"Resources"</h4>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="#" class="hover:text-foreground">"Community"</a></li>
                            <li><a href="#" class="hover:text-foreground">"Support"</a></li>
                            <li><a href="#" class="hover:text-foreground">"API"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-sm font-semibold mb-3">"Legal"</h4>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="#" class="hover:text-foreground">"Privacy"</a></li>
                            <li><a href="#" class="hover:text-foreground">"Terms"</a></li>
                            <li><a href="#" class="hover:text-foreground">"Cookies"</a></li>
                        </ul>
                    </div>
                </div>
                <div class="mt-8 pt-6 border-t border-border text-center text-xs text-muted-foreground">
                    "© 2026 MontRS. All rights reserved."
                </div>
            </div>
        </div>
    }
}