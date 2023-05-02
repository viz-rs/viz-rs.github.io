use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    cfg_if::cfg_if! {
        if #[cfg(feature = "en")] {
            view! { cx,
                <footer class="footer text-center text-neutral-400 text-sm p-5">
                    <p>
                        "Built with "<a href="https://github.com/leptos-rs/leptos" target="_blank" class="text-neutral-500">"Leptos"</a>
                        " · Deploys on "<a href="https://www.cloudflare.com/" target="_blank" class="text-neutral-500">"Cloudflare"</a>
                    </p>
                    <p>"MIT Licensed | Copyright © 2023 Fangdun Tsai"</p>
                </footer>
            }
        } else {
            view! { cx,
                <footer class="footer text-center text-neutral-400 text-sm p-5">
                    <p>
                        "构建于 "<a href="https://github.com/leptos-rs/leptos" target="_blank" class="text-neutral-500">"Leptos"</a>
                        " · 部署在 "<a href="https://www.cloudflare.com/" target="_blank" class="text-neutral-500">"Cloudflare"</a>
                    </p>
                    <p>"MIT Licensed | Copyright © 2023 Fangdun Tsai"</p>
                </footer>
            }
        }
    }
}
