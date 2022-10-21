use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::Duration;

use gloo_net::http::Request;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    Element, HtmlAnchorElement, HtmlElement, IntersectionObserver, IntersectionObserverEntry,
    IntersectionObserverInit,
};
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, Suspense};

use crate::utils::{self, document};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub path: String,
}

#[function_component(Content)]
fn content(props: &Props) -> HtmlResult {
    let node = use_node_ref();
    let path = use_state_eq(|| None);
    let toc: Rc<RefCell<BTreeMap<String, bool>>> = use_mut_ref(|| BTreeMap::new());

    let onclick = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
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

    let ob = {
        let toc = toc.clone();

        use_memo(
            move |_| {
                let cb: Closure<dyn Fn(Vec<IntersectionObserverEntry>)> =
                    Closure::new(move |es: Vec<IntersectionObserverEntry>| {
                        for e in es {
                            let id = e.target().id();
                            let is_intersecting = e.is_intersecting();
                            toc.borrow_mut().insert(id, is_intersecting);
                        }

                        if let Some(k) = toc
                            .as_ref()
                            .borrow()
                            .iter()
                            .filter_map(|(k, v)| if *v { Some(k) } else { None })
                            .next()
                        {
                            if let Ok(nodes) = document().query_selector_all("#page nav a") {
                                for index in 0..nodes.length() {
                                    nodes
                                        .get(index)
                                        .as_ref()
                                        .and_then(|node| node.dyn_ref::<HtmlAnchorElement>())
                                        .map(|node| {
                                            log::info!(
                                                "{} {}",
                                                node.href().trim_start_matches('#'),
                                                &k
                                            );
                                            if node
                                                .get_attribute("href")
                                                .unwrap()
                                                .trim_start_matches('#')
                                                == k
                                            {
                                                if !node.class_list().contains("active") {
                                                    let _ = node.class_list().add_1("active");
                                                }
                                            } else {
                                                if node.class_list().contains("active") {
                                                    let _ = node.class_list().remove_1("active");
                                                }
                                            }
                                        });
                                }
                            }
                        }
                    });

                log::info!("233");
                let mut options = IntersectionObserverInit::new();

                let root = document().query_selector("#page article").unwrap();
                log::info!("377 {:?}", &root);
                options.root(root.as_ref());

                let ob =
                    IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &options)
                        .unwrap();

                cb.forget();

                ob
            },
            (), // path.clone(),
        )
    };

    {
        let node = node.clone();
        let toc = toc.clone();
        let ob = ob.clone();
        let _ = use_future_with_deps(
            |path| async move {
                if let Some(p) = path.as_deref() {
                    let mut url = String::new();
                    url.push_str("/assets/");
                    url.push_str(p);
                    url.push_str(".html");
                    let res = Request::new(&url).send().await?.text().await?;

                    if let Some(div) = node.cast::<HtmlElement>() {
                        if let Ok(nodes) = div.query_selector_all("h2") {
                            log::info!("0 {}", nodes.length());
                            for index in 0..nodes.length() {
                                nodes
                                    .get(index)
                                    .as_ref()
                                    .and_then(|node| node.dyn_ref::<Element>())
                                    .map(|node| {
                                        toc.borrow_mut().remove(&node.id());
                                        ob.unobserve(node);
                                    });
                            }
                        }
                        div.set_inner_html(&res);
                        if let Ok(nodes) = div.query_selector_all("h2") {
                            log::info!("1 {}", nodes.length() as usize);
                            for index in 0..nodes.length() {
                                nodes
                                    .get(index)
                                    .as_ref()
                                    .and_then(|node| node.dyn_ref::<HtmlElement>())
                                    .map(|node| {
                                        log::info!("1 node {:?}", &node);
                                        toc.borrow_mut().insert(node.id(), false);
                                        ob.observe(node);
                                    });
                            }
                        }
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
            <div class="i-lucide-loader w-6 h-6 animate-spin" />
        </div>
    };

    html! {
        <Suspense {fallback}>
            <Content path={props.path.to_owned()} />
        </Suspense>
    }
}
