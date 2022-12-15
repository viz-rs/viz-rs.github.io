use std::rc::Rc;

use gloo_net::http::Request;
use once_cell::sync::Lazy;
use serde::Deserialize;
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
    pub mode: &'static str,
    pub color_scheme: &'static str,
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

fn switch((routes, version, sections): (Route, Rc<String>, Rc<Vec<Section>>)) -> Html {
    match routes {
        Route::Home => {
            html! { <pages::Home version={version} /> }
        }
        Route::Document { path } => {
            html! { <pages::Document path={path} sections={sections} /> }
        }
        Route::NotFound => {
            html! { <div>{"Not Found!"}</div> }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Section {
    text: String,
    prefix: String,
    items: Vec<(String, String)>,
}

pub enum Msg {
    ToggleDark,
    ChangedDark(bool),
    UpdateSidebar(Vec<Section>),
    OpenOrCloseSidebar(bool),
    ChangedVersion(String),
    UpdatePath(String),
}

#[allow(dead_code)]
struct App {
    dark: bool,
    home: bool,
    sidebar: bool,
    version: Rc<String>,
    sections: Rc<Vec<Section>>,
    mqls: (MediaQueryList, MediaQueryList),
    update: Callback<Msg>,
    update_sidebar: Callback<Vec<Section>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let update = link.callback(|m: Msg| m);
        let update_sidebar = link.callback(|sections| Msg::UpdateSidebar(sections));

        let m0 = utils::media_query("(prefers-color-scheme: dark)").unwrap();
        {
            let update = update.clone();
            let cb: Closure<dyn Fn(MediaQueryListEvent)> =
                Closure::new(move |e: MediaQueryListEvent| {
                    update.emit(Msg::ChangedDark(e.matches()))
                });
            m0.set_onchange(Some(cb.as_ref().unchecked_ref()));
            cb.forget();
        }

        let mode = utils::get_color_scheme();
        let dark = if m0.matches() {
            mode != "light"
        } else {
            mode == "dark"
        };

        if dark {
            let _ = utils::document_element()
                .class_list()
                .toggle_with_force("dark", dark);
        }

        let m1 = utils::media_query("(min-width: 960px)").unwrap();
        {
            let update = update.clone();
            let cb: Closure<dyn Fn(MediaQueryListEvent)> =
                Closure::new(move |e: MediaQueryListEvent| {
                    update.emit(Msg::OpenOrCloseSidebar(e.matches()))
                });
            m1.set_onchange(Some(cb.as_ref().unchecked_ref()));
            cb.forget();
        }

        let sidebar = m1.matches();

        Self {
            dark,
            sidebar,
            home: false,
            mqls: (m0, m1),
            version: Rc::new("0.4.x".to_string()),
            sections: Rc::new(vec![]),
            update,
            update_sidebar,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangedDark(dark) => {
                if self.dark == dark {
                    return false;
                }

                let mode = utils::get_color_scheme();
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
                    if self.dark == self.mqls.0.matches() {
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
            Msg::OpenOrCloseSidebar(status) => {
                if self.home {
                    if self.sidebar {
                        self.sidebar = false;
                        true
                    } else {
                        false
                    }
                } else {
                    if self.sidebar == status {
                        return false;
                    }
                    self.sidebar = status;
                    return true;
                }
            }
            Msg::ChangedVersion(version) => {
                if *self.version == version {
                    false
                } else {
                    self.version = version.clone().into();
                    let update_sidebar = self.update_sidebar.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        let mut url = String::new();
                        url.push_str("/assets/");
                        url.push_str(&version);
                        url.push_str("/toc.json");
                        if let Ok(res) = Request::new(&url).send().await {
                            if let Ok(body) = res.json::<Vec<Section>>().await {
                                update_sidebar.emit(body);
                            }
                        }
                    });

                    true
                }
            }
            Msg::UpdateSidebar(sections) => {
                self.sections = sections.into();
                true
            }
            Msg::UpdatePath(path) => {
                let home = path.len() == 1;
                let mut changed = self.home != home;

                if self.home == home {
                    false
                } else {
                    self.home = home;
                    true
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let updater = ctx.link().callback(|m| m);
        let change_version = ctx.link().callback(|v| Msg::ChangedVersion(v));
        let version = self.version.clone();

        html! {
            <BrowserRouter>
                <div id="app" class="tracking-0.2px">
                    <components::Header
                        updater={updater.clone()}
                        version={version.clone()}
                        change={change_version}
                        sidebar={self.sidebar}
                        home={self.home}
                    />

                    <div class={classes!(
                            "page-container", "flex-row", "pt-4.375rem",
                            self.sidebar.then_some("opened")
                        )}
                    >
                        <div id="backdrop" onclick={move |_| updater.emit(Msg::OpenOrCloseSidebar(false))} />

                        <components::Sidebar
                            version={version.clone()}
                            sections={self.sections.clone()}
                        />

                        <main id="page" class="flex flex-row flex-1 py-5">
                            <components::Switch<Route>
                                render={switch}
                                version={version}
                                sections={self.sections.clone()}
                        />
                        </main>
                    </div>

                    <components::Footer />
                </div>
            </BrowserRouter>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let version = self.version.clone();
            let update_sidebar = self.update_sidebar.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut url = String::new();
                url.push_str("/assets/");
                url.push_str(&version);
                url.push_str("/toc.json");
                if let Ok(res) = Request::new(&url).send().await {
                    if let Ok(body) = res.json::<Vec<Section>>().await {
                        update_sidebar.emit(body);
                    }
                }
            });
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    yew::Renderer::<App>::new().render();
}
