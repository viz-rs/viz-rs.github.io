use leptos::*;
use leptos_router::use_params;

use crate::api::{fetch_page, DocParams};

#[component]
pub fn Doc(cx: Scope) -> impl IntoView {
    let params = use_params::<DocParams>(cx);

    let page = create_resource(
        cx,
        move || params.get(),
        move |input| async move {
            let DocParams { version, path } = input.ok()?;
            fetch_page(version, path).await
        },
    );

    view! {
        cx,
        <Suspense
            fallback=move || view! {
                cx,
                <div id="loader" class="i-lucide-loader w-6 h-6 animate-spin absolute" />
            }
        >
        {
            move || page.read(cx)
                .and_then(|page| page)
                .map(|page| view! {
                    cx,
                    <div class="flex flex-row flex-1" inner_html={page} />
                })
        }
        </Suspense>
    }
}
