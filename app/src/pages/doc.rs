use leptos::*;
use leptos_dom::{helpers::location_hash, html::Div};
use leptos_router::use_params;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    HtmlAnchorElement, HtmlElement, IntersectionObserver, IntersectionObserverEntry,
    IntersectionObserverInit,
};

use crate::{
    api::{fetch_page, DocParams},
    utils,
};

#[component]
pub fn Doc(cx: Scope) -> impl IntoView {
    let (ancestors, set_ancestors) = create_signal(cx, Vec::<(String, bool)>::new());
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
    let observer = create_memo(cx, move |_| {
        let cb: Closure<dyn Fn(Vec<IntersectionObserverEntry>)> =
            Closure::new(move |es: Vec<IntersectionObserverEntry>| {
                let mut ancestors = ancestors.get_untracked();

                for e in es {
                    let id = e.target().id();
                    let is_intersecting = e.is_intersecting();
                    ancestors
                        .iter_mut()
                        .find(|a| a.0 == id)
                        .map(|a| a.1 = is_intersecting);
                }

                let nav = container
                    .get_untracked()
                    .unwrap_throw()
                    .query_selector("article + nav")
                    .ok()
                    .flatten()
                    .unwrap_throw();

                ancestors
                    .iter()
                    .rev()
                    .filter_map(|(k, v)| if *v { Some(k) } else { None })
                    .next()
                    .map(|id| {
                        let mut selector = String::new();
                        selector.push_str("a[href='#");
                        selector.push_str(id);
                        selector.push_str("']");

                        if let Some(node) = nav
                            .query_selector(&selector)
                            .unwrap_throw()
                            .and_then(|node| node.dyn_into::<HtmlAnchorElement>().ok())
                        {
                            let top = node.offset_top();
                            let height = node.offset_height();

                            if let Some(node) = nav
                                .query_selector("ul")
                                .unwrap_throw()
                                .and_then(|node| node.dyn_into::<HtmlElement>().ok())
                            {
                                let _ = node.style().set_property("--top", &format!("{}px", top));
                                let _ = node
                                    .style()
                                    .set_property("--height", &format!("{}px", height - 4));
                            }
                        }
                    });

                set_ancestors(ancestors);
            });

        let mut options = IntersectionObserverInit::new();
        options.root_margin("-71px 0px 0px 0px");

        let observer =
            IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &options).unwrap();

        cb.forget();

        observer
    });

    create_effect(cx, move |_| {
        let page = page.read(cx).and_then(|page| page)?;

        let root = container.get()?;
        root.set_inner_html(&page);

        let observer = observer();
        observer.disconnect();

        let hash = location_hash();
        let hashtag = hash.as_ref();

        let article = root.query_selector("article").ok()??;
        let nodes = article.query_selector_all("h2").ok()?;
        let mut found = false;

        let mut temp = Vec::new();

        for idx in 0..nodes.length() {
            nodes
                .get(idx)
                .as_ref()
                .and_then(JsCast::dyn_ref::<HtmlElement>)
                .map(|node| {
                    observer.observe(node);
                    let id = node.id();
                    if !found {
                        found = hashtag.filter(|h| **h == id).is_some();
                        if found {
                            node.scroll_into_view();
                            return (id, true);
                        }
                    }
                    (id, false)
                })
                .map(|anchor| temp.push(anchor));
        }

        if !found {
            utils::document_element().set_scroll_top(0);
            nodes
                .get(0)
                .as_ref()
                .and_then(JsCast::dyn_ref::<HtmlElement>)
                .map(|node| temp[0] = (node.id(), true));
        }

        set_ancestors(temp);

        Some(())
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
        <div
            class="flex flex-row flex-1"
            _ref=container
            on:click=click
        >
            // <Suspense
            //     fallback=move || view! {
            //         cx,
            //         <div id="loader" class="i-lucide-loader w-6 h-6 animate-spin absolute" />
            //     }
            // >
            // {
            //     move || page.read(cx)
            //         .and_then(|page| page)
            //         .map(|page| view! {
            //             cx,
            //             <div
            //                 class="flex flex-row flex-1"
            //                 inner_html=page
            //             />
            //         })
            // }
            // </Suspense>
        </div>
    }
}
