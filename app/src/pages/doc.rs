use leptos::*;
use leptos_router::use_params;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;

use crate::{
    api::{fetch_page, DocParams},
    utils,
};

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

    let click = |e: ev::MouseEvent| {
        if let Some(target) = e
            .target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            log::info!("{:?}", target);
            if target
                .matches("button.i-lucide-copy:not(.text-lime-500)")
                .unwrap_or(false)
            {
                log::info!("{}", 1);
                if let Some(next) = target
                    .next_element_sibling()
                    .and_then(|node| node.dyn_into::<HtmlElement>().ok())
                {
                    log::info!("{}", 2);
                    wasm_bindgen_futures::spawn_local(async move {
                        utils::copy(&next.inner_text()).await;
                        let _ = target.class_list().add_1("text-lime-500");
                        let _ = target.class_list().remove_1("op-20");
                        utils::set_timeout(1_000);
                        let _ = target.class_list().add_1("op-20");
                        let _ = target.class_list().remove_1("text-lime-500");
                    });
                }
            }
        }
    };

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
                    <div class="flex flex-row flex-1" on:click=click inner_html={page} />
                })
        }
        </Suspense>
    }
}
