use leptos::*;
use leptos_i18n::Locale;
use leptos_router::{use_location, use_navigate, A};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, HtmlElement};

use crate::i18n::{self, use_i18n};
use crate::GlobalState;
use crate::{LANGS, VERSIONS};

#[component]
pub fn Navbar() -> impl IntoView {
    let state = expect_context::<GlobalState>();
    let navigate = store_value(use_navigate());
    let location = use_location();
    let i18n = use_i18n();

    create_effect(move |_| {
        let path = location.pathname.get();
        let home = path.len() <= 1;
        log::debug!("home: {}", home);

        state.home.update(|v| *v = home);

        if !home {
            if let Some((lang, next)) = path.trim_start_matches("/").split_once("/") {
                if LANGS.map(|l| l[0]).contains(&lang) {
                    if lang != i18n.get_locale().as_str() {
                        i18n.set_locale(i18n::Locale::from_str(lang).expect(""));
                        log::debug!("set lang");
                    }

                    if let Some((version, _)) = next.trim_start_matches("/").split_once("/") {
                        if VERSIONS.contains(&version) {
                            if version != state.version.get() {
                                state.version.update(|v| *v = version.to_string());
                                log::debug!("set version");
                            }
                        }
                    }
                }
            }
        }
    });

    let on_switch_version = move |e: ev::MouseEvent| {
        let current_version = state.version.get();
        let element = e.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-version"))
            .filter(|version| version != &current_version)
            .map(|version| {
                if state.home.get() {
                    log::debug!("version: {}", &version);
                    state.version.update(|v| *v = version.clone());
                } else {
                    let current_lang = i18n.get_locale().as_str();
                    let prefix = format!("/{}", current_lang);
                    let middle = format!("/{}", current_version);
                    location
                        .pathname
                        .get()
                        .strip_prefix(&prefix)
                        .and_then(|path| path.strip_prefix(&middle))
                        .map(|tail| {
                            navigate.with_value(|n| {
                                n(
                                    &format!("{}{}{}", prefix, format!("/{}", version), tail),
                                    Default::default(),
                                )
                            });
                        });
                }
            });
    };

    let on_switch_lang = move |e: ev::MouseEvent| {
        let current_lang = i18n.get_locale().as_str();
        let element = e.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-lang"))
            .filter(|lang| lang != current_lang)
            .as_deref()
            .and_then(i18n::Locale::from_str)
            .map(|lang| {
                if state.home.get() {
                    log::debug!("lang: {:?}", &lang);
                    i18n.set_locale(lang);
                } else {
                    location
                        .pathname
                        .get()
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
        log::debug!("color scheme: {}", !state.dark.get());
        state.dark.update(|v| *v = !*v);
    };

    let on_switch_sidebar = move |_| {
        state.sidebar.update(|v| *v = !*v);
    };

    let toggle_class_sidebar = move || {
        if state.sidebar.get() {
            "i-lucide-sidebar-open"
        } else {
            "i-lucide-sidebar-close"
        }
    };

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
                        <span class="h-4 text-yellow-600">"v"{state.version}</span>
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
                                                class=("text-yellow-600", move || v == state.version.get())
                                                on:click=on_switch_version
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
                <A class="transition-colors op75 hover:op100" href=move || format!("/{}/{}/guide/introduction", i18n.get_locale().as_str(), state.version.get())>
                    <span class=move || if state.home.get() { "i-lucide-book block" } else { "i-lucide-book-open block" } />
                </A>
                <a rel="noreferrer" target="_blank" class="transition-colors op75 hover:op100" href=move || format!("https://docs.rs/viz/{}", state.version.get())>
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
                                                on:click=on_switch_lang
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
                class=("!hidden", move || state.home.get())
                on:click=on_switch_sidebar>
                <span class="block" class=toggle_class_sidebar />
            </button>
        </header>
    }
}
