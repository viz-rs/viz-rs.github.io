use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{MediaQueryList, MediaQueryListEvent};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

pub mod utils;

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
            html! {
                <pages::Document path={path} />
            }
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
    items: Vec<(String, String)>
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
    sections: Rc<Vec<Section>>
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let changed_dark = ctx.link().callback(|m: Msg| m);
        let mql = utils::media_query("(prefers-color-scheme: dark)").unwrap();
        let cb: Closure<dyn FnMut(MediaQueryListEvent)> =
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
            sections: Rc::new(vec![
                Section {
                    text: "Get Started".to_string(),
                    prefix: "guide/".to_string(),
                    items: vec![
                        (
                            "Introduction".to_string(),
                            "introduction".to_string()
                        ),
                        (
                            "Quick Start".to_string(),
                            "quick-start".to_string()
                        )
                    ]
                },
                Section {
                    text: "Concepts".to_string(),
                    prefix: "concepts/".to_string(),
                    items: vec![
                        (
                            "Handler".to_string(),
                            "handler".to_string()
                        ),
                        (
                            "Middleware".to_string(),
                            "middleware".to_string()
                        ),
                        (
                            "Routing".to_string(),
                            "routing".to_string()
                        ),
                        (
                            "Extractors".to_string(),
                            "extractors".to_string()
                        ),
                        (
                            "Server".to_string(),
                            "server".to_string()
                        ),
                        (
                            "Error Handling".to_string(),
                            "error-handling".to_string()
                        ),
                    ]
                },
                Section {
                    text: "Built-in".to_string(),
                    prefix: "built-in/".to_string(),
                    items: vec![
                        (
                            "Handlers".to_string(),
                            "handlers".to_string()
                        ),
                        (
                            "Middleware".to_string(),
                            "middleware".to_string()
                        ),
                        (
                            "Extractors".to_string(),
                            "extractors".to_string()
                        ),
                        (
                            "Server".to_string(),
                            "server".to_string()
                        ),
                        (
                            "TLS".to_string(),
                            "tls".to_string()
                        ),
                    ]
                },
                Section {
                    text: "Extra Topics".to_string(),
                    prefix: "extra-topics/".to_string(),
                    items: vec![
                        (
                            "Benchmarks".to_string(),
                            "benchmarks".to_string()
                        ),
                        (
                            "Examples".to_string(),
                            "examples".to_string()
                        ),
                        (
                            "Extractors".to_string(),
                            "extractors".to_string()
                        ),
                        (
                            "Showcase".to_string(),
                            "showcase".to_string()
                        ),
                    ]
                },
            ])
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
                utils::document_element()
                    .class_list()
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
                <div class="w-screen fixed top-0">
                    <components::Header toggle_dark={toggle_dark} toggle_sidebar={toggle_sidebar} />

                    <div class="flex-row">
                        if self.sidebar {
                            <components::Sidebar sections={self.sections.clone()} />
                        }

                        <main class="flex flex-row flex-1 p-5">
                            <Switch<Route> render={switch} />
                        </main>
                    </div>
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
