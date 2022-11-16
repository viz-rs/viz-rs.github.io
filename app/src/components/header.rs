use std::rc::Rc;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlAnchorElement, HtmlSelectElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{utils, Msg, Route, METADATA};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub toggle_dark: Callback<MouseEvent>,
    pub toggle_sidebar: Callback<Msg>,
    pub version: Rc<String>,
    pub change: Callback<String>,
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
        let location = utils::location();
        let hostname = location.hostname().unwrap();
        let pathname = location.pathname().unwrap();
        let parts = hostname.split('.').collect::<Vec<&str>>();
        let lang = if parts.len() == 3 { parts[0] } else { "" };
        let version = ctx.props().version.clone();
        let change_version = ctx.props().change.clone();
        let change = ctx.link().callback(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                change_version.emit(target.value());
            }
        });

        html! {
            <header class="w-full fixed top-0 z-36 flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-900 b-b-op-5 dark:b-b-neutral-100 dark:b-b-op-5 navbar">
                <div class="flex flex-row">
                    <Link<Route> classes="flex flex-row items-center transition-colors op75 hover:op100" to={Route::Home}>
                        <img class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1 mr-3" alt="Viz" src="/logo.svg" />
                        <span class="font-semibold">{"V"}</span>
                        {"iz"}
                    </Link<Route>>
                    <select id="versions" onchange={change} class={classes!("text-right","font-bold","select-none","text-3","font-light", (pathname == "/").then(|| Some("hidden")) )}>
                        // <option value="0.5.0" selected={*version == "0.5.0"}>{ "v0.5.0" }</option>
                        <option value="0.4.x" selected={*version == "0.4.x"}>{ "v0.4.x" }</option>
                    </select>
                </div>
                <div class="flex-row items-center gap-5 font-medium text-15px">
                    <Link<Route> classes="transition-colors op75 hover:op100" to={Route::Document { path: "guide/introduction".to_string() }}>
                        {METADATA.docs}
                    </Link<Route>>
                    <a class="transition-colors op75 hover:op100" href={format!("https://docs.rs/viz/{}", version)} target="_blank" rel="noreferrer">
                        {"API"}
                    </a>
                    <a class="transition-colors op75 hover:op100 i-lucide-github" href="https://github.com/viz-rs/viz" target="_blank" rel="noreferrer" />
                    <div class="dropdown-menu cursor-pointer h-7.5 flex justify-center items-center relative">
                        <button class="flex items-center button">
                            <span class="inline-block transition-colors i-lucide-languages" />
                            <span class="i-lucide-chevron-down" />
                        </button>
                        <ul class="dropdown-list absolute text-3.5">
                            <li>
                                <a class={classes!(
                                        "flex",
                                        "hover:text-yellow-600",
                                        (lang == "").then(|| Some("text-yellow-600"))
                                    )}
                                    data-lang="en"
                                    href={format!("https://viz.rs{}", pathname)}
                                >
                                    {"English"}
                                </a>
                            </li>
                            <li>
                                <a class={classes!(
                                        "flex",
                                        "hover:text-yellow-600",
                                        (lang == "zh-cn").then(|| Some("text-yellow-600"))
                                    )}
                                    data-lang="zh-cn"
                                    href={format!("https://zh-cn.viz.rs{}", pathname)}
                                >
                                    {"简体中文"}
                                </a>
                            </li>
                        </ul>
                    </div>
                    <button class="hover:bg-gray5:2 hover:op100" onclick={ctx.props().toggle_dark.clone()}>
                        <span class="dark:i-lucide-moon i-lucide-sun block" />
                    </button>
                </div>
            </header>
        }
    }
}
