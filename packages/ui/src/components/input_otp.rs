use crate::cn::*;
use leptos::prelude::*;

#[component]
pub fn InputOtp(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] value: RwSignal<String>,
    #[prop(optional)] length: u8,
) -> impl IntoView {
    let len = length.max(4).min(8) as usize;
    let merged = move || cn!("flex items-center gap-2", class.get());
    let chars = move || {
        let v = value.get();
        let chars: Vec<char> = v.chars().collect();
        (0..len)
            .map(move |i| chars.get(i).copied().unwrap_or(' '))
            .collect::<Vec<_>>()
    };
    let on_input = move |ev: leptos::ev::Event| {
        let val: String = event_target_value(&ev)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .take(len)
            .collect();
        value.set(val);
    };
    view! {
        <div class=merged data-name="InputOtp">
            <input
                type="text"
                inputmode="numeric"
                maxlength=len.to_string()
                class="sr-only"
                value=move || value.get()
                on:input=on_input
            />
            {move || chars().into_iter().enumerate().map(|(_i, c)| {
                view! {
                    <div class="flex h-12 w-10 items-center justify-center rounded-md border border-input text-sm font-mono bg-background">
                        {if c != ' ' { c.to_string() } else { String::new() }}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
