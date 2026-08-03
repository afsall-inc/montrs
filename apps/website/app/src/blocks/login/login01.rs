use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Login01() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let e = email.get().trim().to_string();
        let p = password.get().trim().to_string();
        if e.is_empty() || p.is_empty() {
            error.set("Please fill in all fields.".to_string());
            return;
        }
        if !e.contains('@') {
            error.set("Please enter a valid email.".to_string());
            return;
        }
        error.set(String::new());
        submitted.set(true);
    };

    view! {
        <div class="rounded-lg border border-border bg-card p-8 shadow-sm max-w-sm mx-auto">
            <div class="mb-6 text-center">
                <Icon glyph=Glyph::Blocks class="mx-auto w-10 h-10 text-primary" />
                <h3 class="mt-4 text-xl font-semibold">"Welcome back"</h3>
                <p class="mt-2 text-sm text-muted-foreground">"Sign in to your account"</p>
            </div>
            <form on:submit=submit class="space-y-4">
                <div>
                    <label for="email" class="block text-sm font-medium mb-1">"Email"</label>
                    <input
                        id="email"
                        type="email"
                        placeholder="m@example.com"
                        on:input=move |ev| email.set(event_target_value(&ev))
                        prop:value=email
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                </div>
                <div>
                    <label for="password" class="block text-sm font-medium mb-1">"Password"</label>
                    <input
                        id="password"
                        type="password"
                        on:input=move |ev| password.set(event_target_value(&ev))
                        prop:value=password
                        class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    />
                </div>
                <Show when=move || !error.get().is_empty()>
                    <p class="text-xs text-red-500">{error}</p>
                </Show>
                <button type="submit"
                    class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                    "Sign In"
                </button>
            </form>
            <Show when=move || submitted.get()>
                <p class="mt-4 text-center text-sm text-green-600 dark:text-green-400">"Signed in successfully!"</p>
            </Show>
            <p class="mt-4 text-center text-sm text-muted-foreground">
                <a href="#" class="text-primary hover:underline">"Forgot password?"</a>
            </p>
        </div>
    }
}