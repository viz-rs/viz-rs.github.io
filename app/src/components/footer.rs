use leptos::*;

// use crate::i18n::*;

#[component]
pub fn Footer() -> impl IntoView {
    // let i18n = use_i18n();

    view! {
        <footer class="footer text-center text-neutral-400 text-sm p-5">
            <p>
            {"builds_with"}" "<a href="https://github.com/leptos-rs/leptos" target="_blank" class="text-neutral-500">"Leptos"</a>
                " · "{"deploys_in"}" "<a href="https://www.cloudflare.com/" target="_blank" class="text-neutral-500">"Cloudflare"</a>
            </p>
            <p>"MIT Licensed | Copyright © 2023 Fangdun Tsai"</p>
        </footer>
    }
}
