use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, Suspense};

use crate::utils;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub path: String,
}

#[function_component(Content)]
fn content(props: &Props) -> HtmlResult {
    let res = use_future_with_deps(
        |path| async move {
            let url = format!("/assets/{}.html", path);
            Request::new(&url).send().await?.text().await
        },
        props.path.to_owned(),
    )?;
    let result_html = match *res {
        Ok(ref res) => {
            if !res.starts_with("<article") {
                return Ok(html! {
                    <div class="flex items-center w-full op61.8">{"Not Found!"}</div>
                });
            }
            let document = utils::document();
            let dark = document
                .document_element()
                .unwrap()
                .class_list()
                .contains("dark");
            let div = document.create_element("div").unwrap();
            div.set_class_name("flex flex-row flex-1");
            div.set_inner_html(res);
            if let Ok(nodes) = div.query_selector_all("pre") {
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
            Html::VRef(div.into())
        }
        Err(ref failure) => failure.to_string().into(),
    };
    Ok(html! {
        {result_html}
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
