use leptos::*;
use leptos_router::{use_location, use_navigate, A};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, HtmlElement, MediaQueryListEvent};

use crate::{utils, LANGS, VERSIONS};

#[component]
pub fn Navbar(
    cx: Scope,
    dark: ReadSignal<bool>,
    set_dark: WriteSignal<bool>,
    sidebar: ReadSignal<bool>,
    set_sidebar: WriteSignal<bool>,
    lang: ReadSignal<String>,
    set_lang: WriteSignal<String>,
    version: ReadSignal<String>,
    set_version: WriteSignal<String>,
) -> impl IntoView {
    let navigate = use_navigate(cx);
    let location = use_location(cx);
    let (dark_matches, set_dark_matches) = create_signal(cx, false);
    let (path, set_path) = create_signal(cx, String::new());

    let path_part = create_memo(cx, move |_| {
        let path = path();
        let empty = path.is_empty();
        (empty, path)
    });

    let pad_path = create_memo(cx, move |_| {
        let (home, mut path) = path_part();
        if home {
            path.push_str("guide/introduction");
        }
        path
    });

    {
        let dark_media = utils::media_query(
            "(prefers-color-scheme: dark)",
            move |e: MediaQueryListEvent| set_dark_matches(e.matches()),
        )
        .unwrap();

        let mode = utils::get_color_scheme();

        let dark = if dark_media.matches() {
            mode != "light"
        } else {
            mode == "dark"
        };

        utils::toggle_dark(dark);
        set_dark(dark);

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

        set_sidebar(sidebar_media.matches());
    }

    create_effect(cx, move |_| {
        let dark = dark();
        // log::info!("change dark: {}", &dark);
        utils::toggle_dark(dark);
        utils::local_storage_set(
            "color-scheme",
            if dark == dark_matches() {
                "auto"
            } else if dark {
                "dark"
            } else {
                "light"
            },
        );
    });

    create_effect(cx, move |_| {
        let path = location
            .pathname
            .get()
            .trim_start_matches("/")
            .trim_start_matches(&version())
            .trim_start_matches("/")
            .to_string();
        // log::info!("path: {} - {}", !path.is_empty(), path);
        let opened = !path.is_empty();
        set_sidebar.update(move |val| *val = opened);
        set_path.update(move |val| {
            val.clear();
            val.push_str(&path)
        });
    });

    let change_version = move |e: ev::Event| {
        let path = pad_path();
        let value = event_target_value(&e);
        let current = value.clone();
        if version() != value {
            set_version.update(move |val| {
                val.clear();
                val.push_str(&value)
            });
        }
        let _ = navigate(&format!("/{}/{}", current, path), Default::default());
    };

    let change_lang = move |e: ev::MouseEvent| {
        let element = e.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-lang"))
            .map(set_lang);
    };

    let toggle_color_scheme = move |e: ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        // log::info!("toggle {}", dark());
        set_dark.update(move |val| *val = !*val);
    };

    view! { cx,
        <header class="w-full fixed top-0 z-36 flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-900 b-b-op-5 dark:b-b-neutral-100 dark:b-b-op-5 navbar">
            <div class="flex flex-row">
                <A href="/" class="flex flex-row items-center transition-colors op75 hover:op100">
                    <img alt="Viz" src="/logo.svg" class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1 mr-3" />
                    <span class="font-semibold">"V"</span><span>"iz"</span>
                </A>
                <select id="versions" class="text-right font-bold select-none text-3 font-light" on:change=change_version>
                    {
                        VERSIONS.into_iter()
                            .map(|v| view! { cx,
                                <option value=v selected={move || v == version()}>"v"{v}</option>
                            })
                            .collect::<Vec<_>>()
                    }
                </select>
            </div>
            <div class="flex flex-row items-center gap-5 font-medium text-15px">
                <A class="transition-colors op75 hover:op100" href=move || format!("/{}/guide/introduction", version())>
                    <span
                        class="block"
                        class=("i-lucide-book", move || path_part().0)
                        class=("i-lucide-book-open", move || !path_part().0)
                    ></span>
                </A>
                <a rel="noreferrer" target="_blank" class="transition-colors op75 hover:op100" href=move || format!("https://docs.rs/viz/{}", version())>
                    <span class="i-lucide-boxes block"></span>
                </a>
                <a target="_blank" rel="noreferrer" href="https://github.com/viz-rs/viz" class="transition-colors op75 hover:op100">
                    <span class="i-lucide-github block"></span>
                </a>
                <div class="dropdown-menu cursor-pointer h-7.5 flex justify-center items-center relative transition-colors op75 hover:op100">
                    <button title="" class="flex items-center button">
                        <span class="inline-block i-lucide-languages"></span>
                        <span class="i-lucide-chevron-down w-4 h-4"></span>
                    </button>
                    <ul class="dropdown-list absolute text-3.5">
                        {
                            LANGS.into_iter()
                                .map(|l|
                                    view! { cx,
                                        <li>
                                            <a
                                                data-lang={l[0]}
                                                class="flex hover:text-yellow-600"
                                                class=("text-yellow-600", move || l[0] == lang())
                                                on:click=change_lang.clone()
                                                href=move || format!("https://{}viz.rs/{}/{}", if l[0] == "en" { "".to_string() } else { l[0].to_string() + "." }, version(), pad_path())
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
                class=("!hidden", move || path_part().0)
                on:click=move |_| set_sidebar.update(|val| *val = !*val)>
                <span
                    class="block"
                    class=("i-lucide-sidebar-open", move || sidebar())
                    class=("i-lucide-sidebar-close", move || !sidebar())
                />
            </button>
        </header>
    }
}
