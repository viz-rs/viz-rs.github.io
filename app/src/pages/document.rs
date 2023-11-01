use leptos::*;
use leptos_dom::html::Div;
use leptos_i18n::Locale;
use leptos_router::use_params;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, HtmlAnchorElement, HtmlElement};

use crate::api::fetch_doc;
use crate::i18n::{self, use_i18n};
use crate::pages::{ComingSoon, NotFound};
use crate::{
    langs_contains,
    utils::{copy, document, set_timeout},
    versions_contains, UNPUBLISHED, VERSIONS,
};
use crate::{DocumentParams, GlobalState};

#[component]
pub fn Document() -> impl IntoView {
    let GlobalState { version, .. } = expect_context();
    let current_params = use_params::<DocumentParams>();
    let container = create_node_ref::<Div>();
    let disable = RwSignal::new(false);
    let i18n = use_i18n();

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
                        copy(&next.inner_text()).await;
                        let _ = target.class_list().add_1("text-lime-500");
                        let _ = target.class_list().remove_1("op-20");
                        set_timeout(
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

                disable.set(true);
            }
        }
    };

    let listener = gloo_events::EventListener::new(&document(), "scroll", move |_| {
        if disable.get_untracked() {
            return disable.set(false);
        }

        let id = scroll(container);
        let _ = update_ul_style(container, None, id);
    });

    let resource = create_resource(
        move || current_params.get().ok(),
        move |input| async move {
            let DocumentParams {
                lang,
                tail,
                version: ver,
            } = input?;
            let l = lang.filter(|v| langs_contains(&v.as_str()))?;
            let v = ver.filter(|v| versions_contains(&v.as_str()))?;
            let t = tail.filter(|v| !v.is_empty())?;

            log::debug!("fetch resource");
            log::debug!("lang: {}, version: {}, tail: {}", l, v, t);

            i18n.set_locale(i18n::Locale::from_str(&l)?);
            version.update(|n| *n = v.clone());

            fetch_doc(&l, &v, &t).await
        },
    );

    on_cleanup(move || drop(listener));

    view! {
        <div class="flex flex-row flex-1">
            <Suspense
                fallback=|| view! {
                    <div id="loader" class="i-lucide-loader w-6 h-6 animate-spin absolute" />
                }
            >
            <div
                class="flex flex-row flex-1"
                _ref=container
                on:click=click
            >
                {move || resource.get().map(|resource| match resource {
                    None => if VERSIONS[UNPUBLISHED] == version.get() {
                        view! { <ComingSoon />  }.into_view()
                    } else {
                        view! { <NotFound /> }.into_view()
                    },
                    Some(content) => view! { <div class="flex flex-row flex-1" inner_html=content /> }.into_view(),
                })}
            </div>
            </Suspense>

        </div>
    }
}

#[inline]
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

    let style = ul.style();
    style.set_property("--top", &format!("{}px", a.offset_top()))?;
    style.set_property("--height", &format!("{}px", a.offset_height() - 4))
}

#[inline]
fn scroll(container: NodeRef<Div>) -> Option<String> {
    let root = container.get_untracked()?;
    let article = root.query_selector("article").ok()??;
    let nodes = article.query_selector_all("h2").ok()?;

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

    id
}
