use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Login02() -> impl IntoView {
    let first = RwSignal::new(String::new());
    let last = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if first.get().trim().is_empty() || last.get().trim().is_empty() || email.get().trim().is_empty() {
            error.set("All fields are required.".to_string());
            return;
        }
        if !email.get().contains('@') {
            error.set("Invalid email.".to_string());
            return;
        }
        error.set(String::new());
        submitted.set(true);
    };

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="grid grid-cols-1 md:grid-cols-2">
                <div class="p-8">
                    <h3 class="text-xl font-semibold">"Create account"</h3>
                    <p class="mt-2 text-sm text-muted-foreground">"Enter your details below"</p>
                    <form on:submit=submit class="mt-6 space-y-4">
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label for="first" class="block text-sm font-medium mb-1">"First"</label>
                                <input id="first" on:input=move |ev| first.set(event_target_value(&ev)) prop:value=first class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                            </div>
                            <div>
                                <label for="last" class="block text-sm font-medium mb-1">"Last"</label>
                                <input id="last" on:input=move |ev| last.set(event_target_value(&ev)) prop:value=last class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                            </div>
                        </div>
                        <div>
                            <label for="email2" class="block text-sm font-medium mb-1">"Email"</label>
                            <input id="email2" type="email" on:input=move |ev| email.set(event_target_value(&ev)) prop:value=email class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                        </div>
                        <Show when=move || !error.get().is_empty()>
                            <p class="text-xs text-red-500">{error}</p>
                        </Show>
                        <button type="submit" class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                            "Sign Up"
                        </button>
                    </form>
                    <Show when=move || submitted.get()>
                        <p class="mt-4 text-sm text-green-600 dark:text-green-400">"Account created!"</p>
                    </Show>
                    <div class="mt-6 space-y-3">
                        <p class="text-xs text-center text-muted-foreground">"Or sign up with"</p>
                        <button class="w-full flex items-center justify-center gap-2 rounded-md border border-border bg-background px-4 py-2 text-sm font-medium hover:bg-muted active:scale-[0.98] transition-all">
                            <Icon glyph=Glyph::GitBranch class="w-4 h-4" />"GitHub"
                        </button>
                        <button class="w-full flex items-center justify-center gap-2 rounded-md border border-border bg-background px-4 py-2 text-sm font-medium hover:bg-muted active:scale-[0.98] transition-all">
                            <Icon glyph=Glyph::Globe class="w-4 h-4" />"Google"
                        </button>
                    </div>
                </div>
                <div class="hidden md:flex flex-col items-center justify-center bg-muted p-8">
                    <Icon glyph=Glyph::Rocket class="w-16 h-16 text-primary mb-4" />
                    <h3 class="text-lg font-semibold">"Get started free"</h3>
                    <p class="mt-2 text-sm text-muted-foreground text-center">"No credit card required. Start building today."</p>
                </div>
            </div>
        </div>
    }
}