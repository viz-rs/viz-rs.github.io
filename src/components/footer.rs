use leptos::*;

#[component]
pub fn Footer(
    cx: Scope,
) -> impl IntoView {
    view! { cx,
        <footer class="footer text-center text-neutral-400 text-sm p-5">
            <p>
                <a href="https://github.com/leptos-rs/leptos" target="_blank" class="text-neutral-500">"Leptos"</a>
                " · "<a href="https://www.cloudflare.com/" target="_blank" class="text-neutral-500">"Cloudflare"</a>
            </p>
            <p>
                "MIT Licensed | Copyright © 2023 Fangdun Tsai"
            </p>
        </footer>
    }
}
