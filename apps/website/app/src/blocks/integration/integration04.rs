use leptos::prelude::*;
use montrs_icons::*;
use montrs_ui::prelude::*;

#[component]
pub fn Integration04() -> impl IntoView {
    let features = vec![
        ("Compile-time safety", "Yes", "Yes", "No"),
        ("WASM support", "Yes", "Partial", "No"),
        ("Built-in ORM", "Yes", "No", "No"),
        ("Agent framework", "Yes", "No", "No"),
        ("Hot reload", "Yes", "Yes", "Yes"),
        ("TypeScript support", "N/A (Rust)", "Yes", "Yes"),
    ];

    view! {
        <div class="rounded-lg border border-border bg-card shadow-sm overflow-hidden">
            <div class="overflow-x-auto">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="border-b border-border bg-muted/50">
                            <th class="text-left px-4 py-3 font-medium">"Feature"</th>
                            <th class="text-left px-4 py-3 font-medium">"MontRS"</th>
                            <th class="text-left px-4 py-3 font-medium">"Framework A"</th>
                            <th class="text-left px-4 py-3 font-medium">"Framework B"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {features.into_iter().map(|(feature, a, b, c)| {
                            view! {
                                <tr class="border-b border-border last:border-0">
                                    <td class="px-4 py-3 font-medium">{feature}</td>
                                    <td class="px-4 py-3">
                                        <span class="text-green-600 dark:text-green-400">{a}</span>
                                    </td>
                                    <td class="px-4 py-3 text-muted-foreground">{b}</td>
                                    <td class="px-4 py-3 text-muted-foreground">{c}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}