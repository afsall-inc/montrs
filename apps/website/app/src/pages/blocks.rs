use leptos::prelude::*;
use montrs_ui::prelude::*;

use crate::blocks::*;

#[component]
pub fn Blocks() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-6xl px-6 py-12 lg:px-8">
            <div class="mb-12">
                <h1 class="text-3xl font-bold">"Blocks"</h1>
                <p class="mt-2 text-muted-foreground">
                    "Pre-built UI sections. Copy, paste, and customize."
                </p>
            </div>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Login"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <LoginFormCard />
                    <LoginSplit />
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Headers"</h2>
                <HeaderBlock />
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Footers"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <FooterSimple />
                    <FooterGrid />
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"FAQ"</h2>
                <FaqSimple />
            </section>
        </div>
    }
}