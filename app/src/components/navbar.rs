use leptos::*;
use leptos_dom::helpers::location_pathname;
use leptos_i18n::Locale;
use leptos_router::{use_location, use_navigate, A};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, HtmlElement};

use crate::i18n::{self, use_i18n};
use crate::GlobalState;
use crate::{LANGS, VERSIONS};

#[component]
pub fn Navbar() -> impl IntoView {
    let GlobalState {
        version,
        home,
        dark,
        sidebar,
    } = expect_context();
    let navigate = store_value(use_navigate());
    let location = use_location();
    let i18n = use_i18n();

    let on_switch_version = move |e: ev::PointerEvent| {
        e.stop_propagation();
        let current_version = version.get();
        let element = e.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-version"))
            .filter(|ver| ver != &current_version)
            .map(|ver| {
                if home.get() {
                    log::debug!("version: {}", &ver);
                    version.update(|v| *v = ver.clone());
                } else {
                    let current_lang = i18n.get_locale().as_str();
                    let prefix = format!("/{}", current_lang);
                    let middle = format!("/{}", current_version);
                    location_pathname()
                        .unwrap_or("/".to_string())
                        .strip_prefix(&prefix)
                        .and_then(|path| path.strip_prefix(&middle))
                        .map(|tail| {
                            navigate.with_value(|n| {
                                n(
                                    &format!("{}{}{}", prefix, format!("/{}", ver), tail),
                                    Default::default(),
                                )
                            });
                        });
                }
            });
    };

    let on_switch_lang = move |e: ev::PointerEvent| {
        e.stop_propagation();
        let current_lang = i18n.get_locale().as_str();
        let element = e.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-lang"))
            .filter(|lang| lang != current_lang)
            .as_deref()
            .and_then(i18n::Locale::from_str)
            .map(|lang| {
                if home.get() {
                    log::debug!("lang: {:?}", &lang);
                    i18n.set_locale(lang);
                } else {
                    location_pathname()
                        .unwrap_or("/".to_string())
                        .strip_prefix(&format!("/{}", current_lang))
                        .map(|tail| {
                            navigate.with_value(|n| {
                                n(&format!("/{}{}", lang.as_str(), tail), Default::default())
                            });
                        });
                }
            });
    };

    let on_switch_color_scheme = move |e: ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        log::debug!("color scheme: {}", !dark.get());
        dark.update(|v| *v = !*v);
    };

    let on_switch_sidebar = move |_| {
        sidebar.update(|v| *v = !*v);
    };

    let toggle_class_sidebar = move || {
        if sidebar.get() {
            "i-lucide-sidebar-open"
        } else {
            "i-lucide-sidebar-close"
        }
    };

    create_effect(move |_| {
        let path = location.pathname.get();
        let root = path.len() <= 1;
        log::debug!("home: {} {} {:?}", root, &path, location_pathname());

        home.update(move |v| *v = root);
    });

    view! {
        <header class="w-full fixed top-0 z-36 flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-900 b-b-op-5 dark:b-b-neutral-100 dark:b-b-op-5 navbar">
            <div class="flex flex-row">
                <A href="/" class="flex flex-row items-center transition-colors op75 hover:op100">
                    <img alt="Viz" src="/logo.svg" class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1 mr-3" />
                    <span class="font-semibold">"V"</span><span>"iz"</span>
                </A>
                <div id="versions" class="dropdown-menu cursor-pointer h-7.5 flex justify-center items-end relative transition-colors op75 hover:op100">
                    <button title="" class="flex items-center button text-2.5 mb-1">
                        <span class="inline-block i-lucide-milestone" />
                        <span class="h-4 text-yellow-600">"v"{version}</span>
                    </button>
                    <ul class="dropdown-list absolute text-3.5">
                        {
                            VERSIONS.into_iter()
                                .map(|v|
                                    view! {
                                        <li>
                                            <a
                                                data-version=v
                                                class="flex hover:text-yellow-600"
                                                class=("text-yellow-600", move || v == version.get())
                                                on:pointerdown=on_switch_version
                                            >{v}</a>
                                        </li>
                                    }
                                )
                                .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
            </div>
            <div class="flex flex-row items-center gap-5 font-medium text-15px">
                <A class="transition-colors op75 hover:op100" href=move || format!("/{}/{}/guide/introduction", i18n.get_locale().as_str(), version.get())>
                    <span class=move || if home.get() { "i-lucide-book block" } else { "i-lucide-book-open block" } />
                </A>
                <a rel="noreferrer" target="_blank" class="transition-colors op75 hover:op100" href=move || format!("https://docs.rs/viz/{}", version.get())>
                    <span class="i-lucide-boxes block" />
                </a>
                <a rel="noreferrer" target="_blank" href="https://github.com/viz-rs/viz" class="transition-colors op75 hover:op100">
                    <span class="i-lucide-github block" />
                </a>
                <div id="langs" class="dropdown-menu cursor-pointer h-7.5 flex justify-center items-center relative transition-colors op75 hover:op100">
                    <button title="" class="flex items-center button">
                        <span class="inline-block i-lucide-languages" />
                        <span class="i-lucide-chevron-down w-4 h-4" />
                    </button>
                    <ul class="dropdown-list absolute text-3.5">
                        {
                            LANGS.into_iter()
                                .map(|l|
                                    view! {
                                        <li>
                                            <a
                                                data-lang=l[0]
                                                class="flex hover:text-yellow-600"
                                                class=("text-yellow-600", move || l[0] == i18n.get_locale().as_str())
                                                on:pointerdown=on_switch_lang
                                            >{l[1]}</a>
                                        </li>
                                    }
                                )
                                .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
                <button class="transition-colors op75 hover:op100" on:click=on_switch_color_scheme>
                    <span aria-hidden="true" class="dark:i-lucide-moon i-lucide-sun block" />
                </button>
            </div>
            <button
                id="toggle-sidebar"
                class="absolute w-8 h-8 items-center justify-center left-0 bottom--8 transition-colors op75 hover:op100"
                class=("!hidden", move || home.get())
                on:pointerdown=on_switch_sidebar>
                <span class="block" class=toggle_class_sidebar />
            </button>
        </header>
    }
}
