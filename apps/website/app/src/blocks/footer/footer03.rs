use leptos::prelude::*;
use montrs_ui::prelude::*;

#[component]
pub fn Footer03() -> impl IntoView {
    let clicked = RwSignal::new(false);
    let email = RwSignal::new(String::new());
    let subscribed = RwSignal::new(false);
    let subscribe = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !email.get().trim().is_empty() {
            subscribed.set(true);
        }
    };

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="bg-primary/10 p-8 text-center">
                <h3 class="text-lg font-semibold">"Ready to get started?"</h3>
                <p class="mt-2 text-sm text-muted-foreground">"Join thousands of developers building with MontRS."</p>
                <button
                    on:click=move |_| clicked.set(true)
                    class="mt-4 rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 active:scale-95 transition-all"
                >
                    "Get Started Free"
                </button>
                <Show when=move || clicked.get()>
                    <p class="mt-2 text-xs text-green-600 dark:text-green-400">"Welcome! Check your email for next steps."</p>
                </Show>
            </div>
            <div class="p-8">
                <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                    <div>
                        <h4 class="text-sm font-semibold mb-3">"Product"</h4>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Features"</a></li>
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Pricing"</a></li>
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Docs"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-sm font-semibold mb-3">"Company"</h4>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"About"</a></li>
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Blog"</a></li>
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Careers"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-sm font-semibold mb-3">"Resources"</h4>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Community"</a></li>
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Support"</a></li>
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"API"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-sm font-semibold mb-3">"Legal"</h4>
                        <ul class="space-y-2 text-sm text-muted-foreground">
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Privacy"</a></li>
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Terms"</a></li>
                            <li><a href="#" class="hover:text-foreground hover:translate-x-0.5 inline-block transition-all">"Cookies"</a></li>
                        </ul>
                    </div>
                </div>
                <form on:submit=subscribe class="mt-8 flex items-center gap-2 max-w-md mx-auto">
                    <input
                        type="email"
                        placeholder="Your email"
                        on:input=move |ev| email.set(event_target_value(&ev))
                        prop:value=email
                        class="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                    <button type="submit" class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors shrink-0">
                        "Subscribe"
                    </button>
                </form>
                <Show when=move || subscribed.get()>
                    <p class="mt-2 text-center text-xs text-green-600 dark:text-green-400">"Subscribed!"</p>
                </Show>
                <div class="mt-6 pt-6 border-t border-border text-center text-xs text-muted-foreground">
                    "© 2026 MontRS. All rights reserved."
                </div>
            </div>
        </div>
    }
}
