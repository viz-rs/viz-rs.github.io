use gloo_net::http::Request;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, Suspense};

use crate::utils;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub path: String,
}

#[function_component(Content)]
fn content(props: &Props) -> HtmlResult {
    let node = use_node_ref();
    let path = use_state_eq(|| None);

    let onclick = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            if target
                .matches("button.i-carbon-copy:not(.text-lime-500)")
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
                        sleep(Duration::from_secs(1)).await;
                        let _ = target.class_list().add_1("op-20");
                        let _ = target.class_list().remove_1("text-lime-500");
                    });
                }
            }
        }
    });

    {
        let node = node.clone();
        let path = path.clone();
        use_effect_with_deps(
            move |p| {
                if node.get().is_some() {
                    path.set(Some(p.to_string()));
                }
            },
            props.path.to_owned(),
        );
    }

    {
        let node = node.clone();
        let _ = use_future_with_deps(
            |path| async move {
                if let Some(p) = path.as_deref() {
                    let mut url = String::new();
                    url.push_str("/assets/");
                    url.push_str(p);
                    url.push_str(".html");
                    let res = Request::new(&url).send().await?.text().await?;

                    if let Some(div) = node.cast::<HtmlElement>() {
                        div.set_inner_html(&res);
                        if let Ok(nodes) = div.query_selector_all("pre") {
                            let dark = utils::document_element().class_list().contains("dark");
                            for index in 0..nodes.length() {
                                nodes
                                    .get(index)
                                    .as_ref()
                                    .and_then(|node| node.dyn_ref::<HtmlElement>())
                                    .and_then(|node| {
                                        node.class_list()
                                            .add_1(if dark { "macchiato" } else { "latte" })
                                            .ok()
                                    });
                            }
                        }
                    }
                }

                Ok::<(), gloo_net::Error>(())
            },
            path,
        )?;
    }

    Ok(html! {
        <div class="flex flex-row flex-1" ref={node} {onclick}></div>
    })
}

#[function_component(Document)]
pub fn doc(props: &Props) -> Html {
    let fallback = html! {
        <div class="flex items-center w-full op61.8">
            <div class="i-carbon-circle-dash w-6 h-6 animate-spin" />
        </div>
    };

    html! {
        <Suspense {fallback}>
            <Content path={props.path.to_owned()} />
        </Suspense>
    }
}
