use leptos::*;
use leptos_i18n::{use_i18n_context, Locale};
use leptos_router::{use_location, use_navigate, A};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, HtmlElement, MediaQueryListEvent};

use crate::i18n::{self, *};
use crate::{utils, DOMAIN, LANGS, VERSIONS};

#[component]
pub fn Navbar(
    dark: ReadSignal<bool>,
    set_dark: WriteSignal<bool>,
    sidebar: ReadSignal<bool>,
    set_sidebar: WriteSignal<bool>,
    lang: ReadSignal<String>,
    set_lang: WriteSignal<String>,
    version: ReadSignal<String>,
    set_version: WriteSignal<String>,
) -> impl IntoView {
    let i18n = use_i18n_context::<i18n::Locale>();

    let on_switch = move |e: ev::MouseEvent| {
        let element = e.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-lang"))
            .filter(|lang| lang != i18n.get_locale().as_str())
            .as_ref()
            .and_then(|lang| i18n::Locale::from_str(lang))
            .map(|lang| i18n.set_locale(lang));
    };

    let navigate = use_navigate();
    let location = use_location();
    let (dark_matches, set_dark_matches) = create_signal(false);
    let (path, set_path) = create_signal(String::new());

    let path_part = create_memo(move |_| {
        let path = path.get();
        let empty = path.is_empty();
        (empty, path)
    });

    let pad_path = create_memo(move |_| {
        let (home, mut path) = path_part.get();
        if home {
            path.push_str("guide/introduction");
        }
        path
    });

    {
        let dark_media = utils::media_query(
            "(prefers-color-scheme: dark)",
            move |e: MediaQueryListEvent| set_dark_matches.set(e.matches()),
        )
        .unwrap();

        let mode = utils::get_color_scheme();

        let dark = if dark_media.matches() {
            mode != "light"
        } else {
            mode == "dark"
        };

        utils::toggle_dark(dark);
        set_dark.set(dark);

        let sidebar_media =
            utils::media_query("(min-width: 960px)", move |e: MediaQueryListEvent| {
                if path_part.get_untracked().0 {
                    set_sidebar.update(|val| {
                        if !*val {
                            return;
                        }
                        *val = false
                    })
                } else {
                    set_sidebar.update(move |val| {
                        if *val == e.matches() {
                            return;
                        }
                        *val = true
                    })
                }
            })
            .unwrap();

        set_sidebar.set(sidebar_media.matches());
    }

    create_effect(move |_| {
        let dark = dark.get();
        // log::info!("change dark: {}", &dark);
        utils::toggle_dark(dark);
        utils::local_storage_set(
            "color-scheme",
            if dark == dark_matches.get() {
                "auto"
            } else if dark {
                "dark"
            } else {
                "light"
            },
        );
    });

    create_effect(move |_| {
        let path = location
            .pathname
            .get()
            .trim_start_matches("/")
            .trim_start_matches(&version.get())
            .trim_start_matches("/")
            .to_string();
        // log::info!("path: {} - {}", !path.is_empty(), path);
        set_sidebar.set(!path.is_empty());
        set_path.set(path);
    });

    let change_version = move |e: ev::Event| {
        let path = pad_path.get();
        let value = event_target_value(&e);
        let current = value.clone();
        if version.get() != value {
            set_version.set(value);
        }
        let _ = navigate(&format!("/{}/{}", current, path), Default::default());
    };

    let change_lang = move |e: ev::MouseEvent| {
        let element = e.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-lang"))
            .map(|value| set_lang.set(value));
    };

    let toggle_color_scheme = move |e: ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        // log::info!("toggle {}", dark());
        set_dark.update(move |val| *val = !*val);
    };

    view! {
        <header class="w-full fixed top-0 z-36 flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-900 b-b-op-5 dark:b-b-neutral-100 dark:b-b-op-5 navbar">
            <div class="flex flex-row">
                <A href="/" class="flex flex-row items-center transition-colors op75 hover:op100">
                    <img alt="Viz" src="/logo.svg" class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1 mr-3" />
                    <span class="font-semibold">"V"</span><span>"iz"</span>
                </A>
                <select id="versions" class="text-right font-bold select-none text-3 font-light" on:change=change_version>
                    {
                        VERSIONS.into_iter()
                            .map(|v| view! {
                                <option value=v selected={move || v == version.get()}>"v"{v}</option>
                            })
                            .collect::<Vec<_>>()
                    }
                </select>
            </div>
            <div class="flex flex-row items-center gap-5 font-medium text-15px">
                <A class="transition-colors op75 hover:op100" href=move || format!("/{}/{}/guide/introduction", i18n.get_locale().as_str(), version.get())>
                    <span class=move || if path_part.get().0 { "i-lucide-book block" } else { "i-lucide-book-open block" } />
                </A>
                <a rel="noreferrer" target="_blank" class="transition-colors op75 hover:op100" href=move || format!("https://docs.rs/viz/{}", version.get())>
                    <span class="i-lucide-boxes block" />
                </a>
                <a target="_blank" rel="noreferrer" href="https://github.com/viz-rs/viz" class="transition-colors op75 hover:op100">
                    <span class="i-lucide-github block" />
                </a>
                <div class="dropdown-menu cursor-pointer h-7.5 flex justify-center items-center relative transition-colors op75 hover:op100">
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
                                                data-lang={l[0]}
                                                class="flex hover:text-yellow-600"
                                                class=("text-yellow-600", move || l[0] == lang.get())
                                                on:click=on_switch.clone()
                                            >{l[1]}</a>
                                        </li>
                                    }
                                )
                                .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
                <button class="transition-colors op75 hover:op100" on:click=toggle_color_scheme>
                    <span aria-hidden="true" class="dark:i-lucide-moon i-lucide-sun block" />
                </button>
            </div>
            <button
                id="toggle-sidebar"
                class="absolute w-8 h-8 items-center justify-center left-0 bottom--8 transition-colors op75 hover:op100"
                class=("!hidden", move || path_part.get().0)
                on:click=move |_| set_sidebar.update(|val| *val = !*val)>
                <span
                    class="block"
                    class=move || if sidebar.get() { "i-lucide-sidebar-open" } else { "i-lucide-sidebar-close" }
                />
            </button>
        </header>
    }
}
