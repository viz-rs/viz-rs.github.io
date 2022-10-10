use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlAnchorElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{utils, Msg, Route};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub toggle_dark: Callback<MouseEvent>,
    pub toggle_sidebar: Callback<Msg>,
}

pub struct Header {
    _listener: Option<LocationHandle>,
}

impl Component for Header {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let listener = {
            let props = ctx.props().clone();
            ctx.link()
                .add_location_listener(ctx.link().callback(move |location: Location| {
                    let path = location.path();
                    props.toggle_sidebar.emit(if path.starts_with("/docs") {
                        if let Some(aside) = utils::document()
                            .query_selector("aside")
                            .expect_throw("Can't find .aside")
                        {
                            let mut is_not_self = true;
                            if let Some(e) = aside
                                .query_selector("a.text-yellow-600")
                                .expect_throw("Can't find .text-yellow-600")
                            {
                                let a = e.dyn_ref::<HtmlAnchorElement>().unwrap();
                                if !a.href().ends_with(&path) {
                                    let _ = a.class_list().remove_2("op100", "text-yellow-600");
                                    let _ = a.class_list().add_1("op61.8");
                                } else {
                                    is_not_self = false;
                                }
                            }

                            if is_not_self {
                                if let Some(e) = aside
                                    .query_selector(&format!(r#"a[href$="{}"]"#, path))
                                    .expect_throw("Can't find a tag")
                                {
                                    let a = e.dyn_ref::<HtmlAnchorElement>().unwrap();
                                    let _ = a.class_list().remove_1("op61.8");
                                    let _ = a.class_list().add_2("op100", "text-yellow-600");
                                }
                            }
                        }

                        Msg::OpenSidebar
                    } else {
                        Msg::CloseSidebar
                    });
                }))
        };

        Self {
            _listener: listener,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header class="w-full fixed top-0 z-36 flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-900 b-b-op-5 dark:b-b-neutral-100 dark:b-b-op-5 navbar">
                <div>
                    <Link<Route> classes="flex flex-row items-center gap-3 transition-colors op75 hover:op100" to={Route::Home}>
                        <img class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1" alt="Viz" src="/logo.svg" />
                        {"Viz"}
                    </Link<Route>>
                </div>
                <div class="flex-row items-center gap-5">
                    <Link<Route> classes="transition-colors op75 hover:op100" to={Route::Document { path: "guide/introduction".to_string() }}>
                        {"Docs"}
                    </Link<Route>>
                    <a class="transition-colors op75 hover:op100" href="https://docs.rs/viz/latest/viz" target="_blank" rel="noreferrer">
                        {"API"}
                    </a>
                    <a class="transition-colors op75 hover:op100 i-carbon-logo-github" href="https://github.com/viz-rs/viz" target="_blank" rel="noreferrer" />
                    <butrton class="transition-colors op75 hover:op100 cursor-pointer i-lucide-languages" />
                    <button class="hover:bg-gray5:2 hover:op100" onclick={ctx.props().toggle_dark.clone()}>
                        <span class="dark:i-carbon-moon i-carbon-sun block" />
                    </button>
                </div>
            </header>
        }
    }
}
