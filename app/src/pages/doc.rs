use leptos::*;
use leptos_dom::html::Div;
use leptos_router::use_params;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    HtmlAnchorElement, HtmlDivElement, HtmlElement, IntersectionObserver,
    IntersectionObserverEntry, IntersectionObserverInit,
};

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
    let container = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        log::info!("doc");
        if let Some(node) = container.get() {
            let cb: Closure<dyn Fn(Vec<IntersectionObserverEntry>)> =
                Closure::new(move |es: Vec<IntersectionObserverEntry>| {
                    log::info!("inter section observer");
                });

            let mut options = IntersectionObserverInit::new();

            let root = node.query_selector("article").unwrap_throw();
            options.root(root.as_ref());

            let ob = IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &options)
                .unwrap();

            cb.forget();
        }
    });

    let click = |e: ev::MouseEvent| {
        if let Some(target) = e
            .target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            if target
                .matches("button.i-lucide-copy:not(.text-lime-500)")
                .unwrap_or(false)
            {
                if let Some(next) = target
                    .next_element_sibling()
                    .and_then(|node| node.dyn_into::<HtmlElement>().ok())
                {
                    wasm_bindgen_futures::spawn_local(async move {
                        utils::copy(&next.inner_text()).await;
                        let _ = target.class_list().add_1("text-lime-500");
                        let _ = target.class_list().remove_1("op-20");
                        utils::set_timeout(
                            move || {
                                let _ = target.class_list().add_1("op-20");
                                let _ = target.class_list().remove_1("text-lime-500");
                            },
                            610,
                        );
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
                    <div
                        _ref=container
                        class="flex flex-row flex-1"
                        inner_html=page
                        on:click=click
                    />
                })
        }
        </Suspense>
    }
}
