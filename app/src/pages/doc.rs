use leptos::*;
use leptos_dom::{helpers::location_hash, html::Div};
// use leptos_router::{use_navigate, use_params, NavigateOptions, State};
use leptos_router::use_params;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, HtmlAnchorElement, HtmlElement, NodeList};

use crate::{
    api::{fetch_page, DocParams},
    utils,
};

fn update_ul_style(
    container: NodeRef<Div>,
    a: Option<HtmlAnchorElement>,
    id: Option<String>,
) -> Result<(), JsValue> {
    let ul = container
        .get_untracked()
        .ok_or(JsValue::NULL)?
        .query_selector("article + nav ul")?
        .and_then(|node| node.dyn_into::<HtmlElement>().ok())
        .ok_or(JsValue::NULL)?;

    let a = a
        .or_else(|| {
            id.and_then(|id| {
                ul.query_selector(&format!("a[href='#{}']", id))
                    .ok()
                    .flatten()
                    .and_then(|node| node.dyn_into::<HtmlAnchorElement>().ok())
            })
        })
        .ok_or(JsValue::NULL)?;

    let (top, height) = (a.offset_top(), a.offset_height());
    ul.style().set_property("--top", &format!("{}px", top))?;
    ul.style()
        .set_property("--height", &format!("{}px", height - 4))
}

#[component]
pub fn Doc(cx: Scope) -> impl IntoView {
    // let navigate = use_navigate(cx);
    let (anchors, set_anchors) = create_signal(cx, Option::<NodeList>::None);
    let (disabled, set_disabled) = create_signal(cx, false);
    let (loading, set_loading) = create_signal(cx, false);
    let current_params = use_params::<DocParams>(cx);
    let container = create_node_ref::<Div>(cx);
    let page = create_resource(
        cx,
        move || current_params.get().ok(),
        move |input| async move {
        set_loading.set(true);
            let params = input?;
            let DocParams { version, path } = params;
            let v = version.unwrap_or_default();
            let p = path.unwrap_or_default();
            fetch_page(v, p).await
        },
    );

    create_effect(cx, move |_| {
        set_loading.set(false);
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

        set_disabled.set(true);

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

                let _ = update_ul_style(container, None, Some(node.id()));
            });

        set_anchors.set(Some(nodes));

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

                let _ =
                    update_ul_style(container, target.dyn_into::<HtmlAnchorElement>().ok(), None);

                set_disabled.set(true);
            }
        }
    };

    let listener = gloo_events::EventListener::new(&utils::document(), "scroll", move |_| {
        if disabled.get_untracked() {
            set_disabled.set(false);
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

            let _ = update_ul_style(container, None, id);

            /*
            // https://stackoverflow.com/questions/3870057/how-can-i-update-window-location-hash-without-jumping-the-document
            let hash = utils::window().location().hash().ok().unwrap_or_default();
            let prev = hash.trim_start_matches('#').to_string();
            let anchor = id.clone().unwrap_or_default();

            (prev != anchor)
                .then(|| current_params.get_untracked().ok())
                .flatten()
                .map(|DocParams { version, path }| {
                    {
                        if utils::document_element().scroll_top() <= 106 / 2 {
                            Some(format!("/{}/{}", version, path))
                        } else if !anchor.is_empty() {
                            Some(format!("/{}/{}#{}", version, path, anchor))
                        } else {
                            None
                        }
                    }
                    .and_then(|to| {
                        set_disabled(true);
                        utils::window().history()
                            .and_then(|history| history.replace_state_with_url(&JsValue::NULL, "", Some(&to)))
                            .ok()
                        // navigate(
                        //     &to,
                        //     NavigateOptions {
                        //         resolve: false,
                        //         replace: true,
                        //         scroll: false,
                        //         state: State(None),
                        //     },
                        // )
                        // .ok()
                    })
                });
            */
        }
    });

    on_cleanup(cx, move || drop(listener));

    view! {
        cx,
        <div class="flex flex-row flex-1">
            <div id="loader" class="i-lucide-loader w-6 h-6 animate-spin absolute" class:hidden=move || !loading.get() />
            <div
                class="flex flex-row flex-1"
                _ref=container
                on:click=click
            >
            </div>
        </div>
    }
}
