use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{Route, METADATA};

#[cfg(all(feature = "en", not(feature = "zh-cn")))]
fn sub_desc() -> Html {
    html! {
        <p class="text-4 sm:text-5 mt-4.5 mb-7.5 sm:mt-6 sm:mb-8 op-61.8 font-light">
            <strong class="font-normal">{"Viz"}</strong>
            {" builts on top of "}
            <a href="https://tokio.rs/" target="_bank" class="text-yellow-600 font-normal">{"Tokio"}</a>
            {" and "}
            <a href="https://hyper.rs/" target="_bank" class="text-yellow-600 font-normal">{"Hyper"}</a>
            {"."}
        </p>
    }
}

#[cfg(all(feature = "zh-cn", not(feature = "en")))]
fn sub_desc() -> Html {
    html! {
        <p class="text-4 sm:text-5 mt-4.5 mb-7.5 sm:mt-6 sm:mb-8 op-61.8 font-light">
            <strong class="font-normal">{"Viz"}</strong>
            {" 构建在 "}
            <a href="https://tokio.rs/" target="_bank" class="text-yellow-600 font-normal">{"Tokio"}</a>
            {" 和 "}
            <a href="https://hyper.rs/" target="_bank" class="text-yellow-600 font-normal">{"Hyper"}</a>
            {" 之上。"}
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub version: Rc<String>,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    let mut path = String::new();
    path.push_str(props.version.as_str());
    path.push_str("/guide/introduction");

    html! {
        <section class="w-full hero text-center p-5 sm:py-19">
            <h1 class="text-8 sm:text-10 font-medium">
                {METADATA.description}
            </h1>
            {sub_desc()}
            <Link<Route>
                classes="inline-block bg-neutral-900 text-neutral-100 dark:bg-neutral-100 dark:text-neutral-900 shadow py-2 px-4.5 border-rounded font-medium text-4 get-started"
                to={Route::Document { path }}
            >
                {METADATA.get_started}
            </Link<Route>>
        </section>
    }
}
