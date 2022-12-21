use std::{rc::Rc, time::Duration};

use gloo_net::http::Request;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    HtmlAnchorElement, HtmlElement, IntersectionObserver, IntersectionObserverEntry,
    IntersectionObserverInit,
};
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, Suspense};
use yew_router::prelude::use_navigator;

use crate::utils::{document_element, window};
use crate::{
    utils::{self, document},
    Section,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub path: String,
    pub sections: Rc<Vec<Section>>,
}

#[function_component(Content)]
fn content(props: &Props) -> HtmlResult {
    let node = use_node_ref();
    let path = use_state_eq(|| None);
    let toc = use_mut_ref(|| Vec::<(String, bool)>::new());
    let navigator = use_navigator();

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

    let onclick_nav = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e: Event| {
        e.stop_propagation();
        e.prevent_default();
        if let Some(target) = e.target_dyn_into::<HtmlAnchorElement>() {
            log::info!("{:?}", target);
        }
    }));

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
                            if let Some(t) = toc.borrow_mut().iter_mut().find(|t| t.0 == id) {
                                t.1 = is_intersecting;
                            }
                        }

                        if let Some(k) = toc
                            .as_ref()
                            .borrow()
                            .iter()
                            .filter_map(|(k, v)| if *v { Some(k) } else { None })
                            .next()
                        {
                            let document = document();
                            let mut not_find = true;
                            if let Some(node) = document
                                .query_selector("#page nav a.active")
                                .unwrap_throw()
                                .and_then(|node| node.dyn_into::<HtmlAnchorElement>().ok())
                            {
                                if node.get_attribute("href").unwrap().trim_start_matches('#') != k
                                {
                                    let _ = node.class_list().remove_1("active");
                                } else {
                                    not_find = false;
                                }
                            }

                            if not_find {
                                let mut selector = String::new();
                                selector.push_str("#page nav a[href='#");
                                selector.push_str(k);
                                selector.push_str("']");

                                if let Some(node) = document
                                    .query_selector(&selector)
                                    .unwrap_throw()
                                    .and_then(|node| node.dyn_into::<HtmlAnchorElement>().ok())
                                {
                                    let _ = node.class_list().add_1("active");
                                    let top = node.offset_top();

                                    if let Some(node) = document
                                        .query_selector("#page nav ul")
                                        .unwrap_throw()
                                        .and_then(|node| node.dyn_into::<HtmlElement>().ok())
                                    {
                                        let mut value = String::new();
                                        value.push_str(&top.to_string());
                                        value.push_str("px");
                                        let _ = node.style().set_property("--top", &value);
                                    }
                                }
                            }
                        }
                    });

                let mut options = IntersectionObserverInit::new();

                let root = document().query_selector("#page article").unwrap_throw();
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
        let sections = props.sections.clone();
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
                            for index in 0..nodes.length() {
                                nodes
                                    .get(index)
                                    .as_ref()
                                    .and_then(|node| node.dyn_ref::<HtmlElement>())
                                    .map(|node| {
                                        toc.borrow_mut().retain(|(id, _)| id == &node.id());
                                        ob.unobserve(node);
                                    });
                            }
                        }
                        div.set_inner_html(&res);

                        if let Ok(nodes) = div.query_selector_all(".page-nav > a") {
                            for index in 0..nodes.length() {
                                nodes
                                    .get(index)
                                    .as_ref()
                                    // .and_then(|node| node.dyn_ref::<HtmlAnchorElement>())
                                    .map(|node| {
                                        log::info!("{:?}", &node);
                                        node.add_event_listener_with_callback(
                                            "click",
                                            onclick_nav.as_ref().unchecked_ref(),
                                        )
                                        .unwrap_throw();
                                    });
                            }
                        }

                        if let Ok(nodes) = div.query_selector_all("h2") {
                            for index in 0..nodes.length() {
                                nodes
                                    .get(index)
                                    .as_ref()
                                    .and_then(|node| node.dyn_ref::<HtmlElement>())
                                    .map(|node| {
                                        toc.borrow_mut().push((node.id(), false));
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
        <div id="loader" class="i-lucide-loader w-6 h-6 animate-spin absolute" />
    };

    html! {
        <Suspense {fallback}>
            <Content path={props.path.to_owned()} sections={props.sections.clone()} />
        </Suspense>
    }
}
