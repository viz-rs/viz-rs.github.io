use leptos::*;
use leptos_dom::{helpers::location_hash, html::Div};
use leptos_router::use_params;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, HtmlAnchorElement, HtmlElement, NodeList};

use crate::{
    api::{fetch_page, DocParams},
    utils,
};

fn update_ul_style(container: NodeRef<Div>, a: Option<HtmlAnchorElement>, id: Option<String>) {
    let ul = container
        .get_untracked()
        .unwrap_throw()
        .query_selector("article + nav ul")
        .ok()
        .flatten()
        .and_then(|node| node.dyn_into::<HtmlElement>().ok())
        .unwrap_throw();

    if let Some(a) = a.or_else(|| {
        id.and_then(|id| {
            let mut selector = String::new();
            selector.push_str("a[href='#");
            selector.push_str(&id);
            selector.push_str("']");

            ul.query_selector(&selector)
                .unwrap_throw()
                .and_then(|node| node.dyn_into::<HtmlAnchorElement>().ok())
        })
    }) {
        let (top, height) = (a.offset_top(), a.offset_height());
        let _ = ul.style().set_property("--top", &format!("{}px", top));
        let _ = ul
            .style()
            .set_property("--height", &format!("{}px", height - 4));
    }
}

#[component]
pub fn Doc(cx: Scope) -> impl IntoView {
    let (params, set_params) = create_signal(cx, DocParams::default());
    let (anchors, set_anchors) = create_signal(cx, Option::<NodeList>::None);
    let (disabled, set_disabled) = create_signal(cx, false);
    let (loading, set_loading) = create_signal(cx, false);
    let current_params = use_params::<DocParams>(cx);
    let page = create_resource(
        cx,
        move || current_params.get().ok().filter(|p| p != &params()),
        move |input| async move {
            set_loading(true);
            let params = input?;
            set_params(params.clone());
            let DocParams { version, path } = params;
            log::info!("version: {}, path: {}", &version, &path);
            let result = fetch_page(version, path).await;
            set_loading(false);
            result
        },
    );
    let container = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        let page = page.read(cx)??;
        let root = container.get()?;
        root.set_inner_html(&page);

        let hash = location_hash();
        let hashtag = hash.as_ref();

        let article = root.query_selector("article").ok()??;
        let nodes = article.query_selector_all("h2").ok()?;
        let mut found = None;

        for idx in 0..nodes.length() {
            if found.is_some() {
                break;
            }
            nodes
                .get(idx)
                .as_ref()
                .and_then(JsCast::dyn_ref::<HtmlElement>)
                .map(|node| found = hashtag.filter(|h| **h == node.id()).map(|_| idx));
        }

        let idx = found.unwrap_or(0);

        nodes
            .get(idx)
            .as_ref()
            .and_then(JsCast::dyn_ref::<HtmlElement>)
            .map(|node| {
                if idx == 0 {
                    utils::document_element().set_scroll_top(0);
                } else {
                    node.scroll_into_view();
                }

                update_ul_style(container, None, Some(node.id()));
            });

        set_disabled(true);
        set_anchors(Some(nodes));
        Some(())
    });

    let click = move |e: ev::MouseEvent| {
        if let Some(target) = e
            .target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            if target
                .matches("button.i-lucide-copy:not(.text-lime-500)")
                .unwrap_or(false)
            {
                e.stop_immediate_propagation();

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
            } else if target.matches("a.toc-link").unwrap_or(false) {
                e.stop_immediate_propagation();

                update_ul_style(container, target.dyn_into::<HtmlAnchorElement>().ok(), None);

                set_disabled(true);
            }
        }
    };

    let listener = gloo_events::EventListener::new(&utils::document(), "scroll", move |_| {
        if disabled.get_untracked() {
            set_disabled(false);
            return;
        }

        if let Some(nodes) = anchors.get_untracked() {
            let mut id = None;

            for idx in 0..nodes.length() {
                if let Some(e) = nodes
                    .get(idx)
                    .and_then(|node| node.dyn_into::<Element>().ok())
                {
                    let rect = e.get_bounding_client_rect();
                    if rect.top() - 106. > 0. {
                        break;
                    }
                    id.replace(e.id());
                }
            }

            if id.is_some() {
                let _ = utils::window()
                    .location()
                    .set_hash(id.clone().unwrap_or_default().as_ref());
            } else if utils::document_element().scroll_top() <= 0 {
                let _ = utils::window().location().set_hash("");
            }

            update_ul_style(container, None, id);
        }
    });

    on_cleanup(cx, move || drop(listener));

    view! {
        cx,
        <div class="flex flex-row flex-1">
            <div id="loader" class="i-lucide-loader w-6 h-6 animate-spin absolute" class:hidden=move || !loading() />
            <div
                class="flex flex-row flex-1"
                _ref=container
                on:click=click
            >
            </div>
        </div>
    }
}
