use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

use crate::blocks::*;

#[component]
pub fn Blocks() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-6xl px-6 py-12 lg:px-8">
            <div class="mb-12">
                <h1 class="text-3xl font-bold">"Blocks"</h1>
                <p class="mt-2 text-muted-foreground">"Pre-built UI sections. Copy, paste, and customize."</p>
            </div>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"FAQ"</h2>
                <div class="space-y-8">
                    <Faq01 />
                    <Faq02 />
                    <Faq03 />
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Footers"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Footer01 />
                    <Footer02 />
                    <Footer03 />
                    <Footer04 />
                    <Footer05 />
                    <FooterLogos />
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Headers"</h2>
                <Header01 />
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Integrations"</h2>
                <div class="space-y-8">
                    <Integration01 />
                    <Integration02 />
                    <Integration03 />
                    <Integration04 />
                    <Integration05 />
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                        <Integration06 />
                        <Integration07 />
                    </div>
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Login"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Login01 />
                    <Login02 />
                    <Login03 />
                    <Login04 />
                </div>
            </section>

            <section class="mb-16">
                <h2 class="text-2xl font-semibold mb-6">"Sidenav"</h2>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
                    <Sidenav01 />
                    <Sidenav02 />
                    <Sidenav03 />
                    <Sidenav04 />
                    <Sidenav05 />
                    <Sidenav06 />
                    <Sidenav07 />
                    <Sidenav08 />
                    <Sidenav09 />
                    <Sidenav10 />
                    <Sidenav11 />
                    <SidenavInsetRight />
                    <SidenavRoutes />
                    <SidenavRoutesSelector />
                    <SidenavRoutesSimplified />
                </div>
            </section>
        </div>
    }
}