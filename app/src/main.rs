use std::rc::Rc;

use once_cell::sync::Lazy;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, MediaQueryList, MediaQueryListEvent};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod i18n;
mod pages;

pub mod utils;

#[derive(Debug)]
pub struct Metadata {
    pub title: &'static str,
    pub description: &'static str,
    pub note: &'static str,
    pub docs: &'static str,
    pub build_with: &'static str,
    pub deploys_on: &'static str,
    pub get_started: &'static str,
}

pub static METADATA: Lazy<Metadata> = Lazy::new(i18n::metadata);

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/docs/*path")]
    Document { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <pages::Home /> }
        }
        Route::Document { path } => {
            html! { <pages::Document path={path} /> }
        }
        Route::NotFound => {
            html! { <div>{"Not Found!"}</div> }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Section {
    text: String,
    prefix: String,
    items: Vec<(String, String)>,
}

pub enum Msg {
    ToggleDark,
    ChangedDark(bool),
    OpenSidebar,
    CloseSidebar,
}

#[allow(dead_code)]
struct App {
    dark: bool,
    sidebar: bool,
    mql: MediaQueryList,
    sections: Rc<Vec<Section>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let changed_dark = ctx.link().callback(|m: Msg| m);
        let mql = utils::media_query("(prefers-color-scheme: dark)").unwrap();
        let cb: Closure<dyn Fn(MediaQueryListEvent)> =
            Closure::new(move |e: MediaQueryListEvent| {
                changed_dark.emit(Msg::ChangedDark(e.matches()));
            });
        mql.set_onchange(Some(cb.as_ref().unchecked_ref()));
        cb.forget();

        let mode = utils::local_storage_get("color-scheme").unwrap_or("auto".to_string());
        let dark = if mql.matches() {
            mode != "light"
        } else {
            mode == "dark"
        };

        if dark {
            let _ = utils::document_element()
                .class_list()
                .toggle_with_force("dark", dark);
        }

        Self {
            dark,
            sidebar: false,
            mql,
            #[cfg(all(feature = "en", not(feature = "zh-cn")))]
            sections: Rc::new(vec![
                Section {
                    text: "Get Started".to_string(),
                    prefix: "guide/".to_string(),
                    items: vec![
                        ("Introduction".to_string(), "introduction".to_string()),
                        ("Quick Start".to_string(), "quick-start".to_string()),
                    ],
                },
                Section {
                    text: "Concepts".to_string(),
                    prefix: "concepts/".to_string(),
                    items: vec![
                        ("Request & Response".to_string(), "requests-and-responses".to_string()),
                        ("Handler".to_string(), "handler".to_string()),
                        ("Middleware".to_string(), "middleware".to_string()),
                        ("Routing".to_string(), "routing".to_string()),
                        ("Extractors".to_string(), "extractors".to_string()),
                        ("Server".to_string(), "server".to_string()),
                        ("Error Handling".to_string(), "error-handling".to_string()),
                    ],
                },
                Section {
                    text: "Built-in".to_string(),
                    prefix: "built-ins/".to_string(),
                    items: vec![
                        ("Handlers".to_string(), "handlers".to_string()),
                        ("Middleware".to_string(), "middleware".to_string()),
                        ("Extractors".to_string(), "extractors".to_string()),
                        ("TLS".to_string(), "tls".to_string()),
                    ],
                },
                Section {
                    text: "Extra Topics".to_string(),
                    prefix: "extra-topics/".to_string(),
                    items: vec![
                        ("Benchmarks".to_string(), "benchmarks".to_string()),
                        ("Examples".to_string(), "examples".to_string()),
                        ("Extractors".to_string(), "extractors".to_string()),
                        ("Showcase".to_string(), "showcase".to_string()),
                    ],
                },
                Section {
                    text: "Others".to_string(),
                    prefix: "others/".to_string(),
                    items: vec![
                        ("Sponsor".to_string(), "sponsor".to_string()),
                    ],
                },
            ]),
            #[cfg(all(feature = "zh-cn", not(feature = "en")))]
            sections: Rc::new(vec![
                Section {
                    text: "开始".to_string(),
                    prefix: "guide/".to_string(),
                    items: vec![
                        ("介绍".to_string(), "introduction".to_string()),
                        ("快速上手".to_string(), "quick-start".to_string()),
                    ],
                },
                Section {
                    text: "概念".to_string(),
                    prefix: "concepts/".to_string(),
                    items: vec![
                        ("请求及响应".to_string(), "requests-and-responses".to_string()),
                        ("请求处理".to_string(), "handler".to_string()),
                        ("中间件".to_string(), "middleware".to_string()),
                        ("路由".to_string(), "routing".to_string()),
                        ("提取器".to_string(), "extractors".to_string()),
                        ("服务".to_string(), "server".to_string()),
                        ("错误处理".to_string(), "error-handling".to_string()),
                    ],
                },
                Section {
                    text: "内建组件".to_string(),
                    prefix: "built-ins/".to_string(),
                    items: vec![
                        ("处理函数".to_string(), "handlers".to_string()),
                        ("中间件".to_string(), "middleware".to_string()),
                        ("提取器".to_string(), "extractors".to_string()),
                        ("TLS".to_string(), "tls".to_string()),
                    ],
                },
                Section {
                    text: "进阶主题".to_string(),
                    prefix: "extra-topics/".to_string(),
                    items: vec![
                        ("性能测试".to_string(), "benchmarks".to_string()),
                        ("例子".to_string(), "examples".to_string()),
                        ("提取器".to_string(), "extractors".to_string()),
                        ("产品示例".to_string(), "showcase".to_string()),
                    ],
                },
                Section {
                    text: "其他".to_string(),
                    prefix: "others/".to_string(),
                    items: vec![
                        ("捐助".to_string(), "sponsor".to_string()),
                    ],
                },
            ]),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangedDark(dark) => {
                if self.dark == dark {
                    return false;
                }

                let mode = utils::local_storage_get("color-scheme").unwrap_or("auto".to_string());
                if mode != "auto" {
                    return false;
                }

                self.dark = dark;

                utils::document_element()
                    .class_list()
                    .toggle_with_force("dark", self.dark)
                    .is_ok()
            }
            Msg::ToggleDark => {
                self.dark = !self.dark;
                utils::local_storage_set(
                    "color-scheme",
                    if self.dark == self.mql.matches() {
                        "auto"
                    } else if self.dark {
                        "dark"
                    } else {
                        "light"
                    },
                );

                let root = utils::document_element();

                if let Ok(nodes) = root.query_selector_all("article > .code > pre") {
                    for index in 0..nodes.length() {
                        nodes
                            .get(index)
                            .as_ref()
                            .and_then(|node| node.dyn_ref::<HtmlElement>())
                            .and_then(|node| {
                                if self.dark {
                                    let _ = node.class_list().remove_1("latte");
                                    node.class_list().add_1("macchiato").ok()
                                } else {
                                    let _ = node.class_list().remove_1("macchiato");
                                    node.class_list().add_1("latte").ok()
                                }
                            });
                    }
                }

                root.class_list()
                    .toggle_with_force("dark", self.dark)
                    .is_ok()
            }
            Msg::OpenSidebar => {
                if self.sidebar {
                    false
                } else {
                    self.sidebar = true;
                    true
                }
            }
            Msg::CloseSidebar => {
                if self.sidebar {
                    self.sidebar = false;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let toggle_dark = ctx.link().callback(|_| Msg::ToggleDark);
        let toggle_sidebar = ctx.link().callback(|m| m);

        html! {
            <BrowserRouter>
                <div id="app" class="tracking-0.2px">
                    <components::Header toggle_dark={toggle_dark} toggle_sidebar={toggle_sidebar} />

                    <div class="page-container flex-row pt-4.375rem">
                        if self.sidebar {
                            <components::Sidebar sections={self.sections.clone()} />
                        }

                        <main id="page" class="flex flex-row flex-1 py-5">
                            <Switch<Route> render={switch} />
                        </main>
                    </div>

                    <components::Footer />
                </div>
            </BrowserRouter>
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    yew::Renderer::<App>::new().render();
}
