use leptos::*;
use leptos_router::{use_location, use_navigate, A};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, HtmlElement};

use crate::{LANGS, VERSIONS};

#[component]
pub fn Navbar(
    cx: Scope,
    dark_part: (Signal<bool>, SignalSetter<bool>),
    lang_part: (Signal<String>, SignalSetter<String>),
    version_part: (Signal<String>, SignalSetter<String>),
) -> impl IntoView {
    let navigate = use_navigate(cx);
    let location = use_location(cx);

    let (dark, set_dark) = dark_part;
    let (lang, set_lang) = lang_part;
    let (version, set_version) = version_part;

    let toggle_color_scheme = move |_| {
        set_dark(!dark());
    };

    let doc_path = move || {
        let pathname = (location.pathname)();
        pathname
            .trim_start_matches(&format!("/{}", version()))
            .to_string()
    };

    let is_home = move || doc_path() == "/";

    let change_version = move |ev: ev::Event| {
        let doc_path = doc_path();
        let value = event_target_value(&ev);
        set_version(value.clone());
        let _ = navigate(&format!("/{}{}", value, doc_path), Default::default());
    };

    let change_lang = move |ev: ev::MouseEvent| {
        let element = ev.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-lang"))
            .map(set_lang);
    };

    view! { cx,
        <header class="w-full fixed top-0 z-36 flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-900 b-b-op-5 dark:b-b-neutral-100 dark:b-b-op-5 navbar">
            <div class="flex flex-row">
                <a href="/" class="flex flex-row items-center transition-colors op75 hover:op100">
                    <img alt="Viz" src="/logo.svg" class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1 mr-3" />
                    <span class="font-semibold">"V"</span><span>"iz"</span>
                </a>
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
                <A href={move || format!("/{}/guide/introduction", version())} class="transition-colors op75 hover:op100">
                    <span class="block" class=("i-lucide-book", move || is_home()) class=("i-lucide-book-open", move || !is_home())></span>
                </A>
                <a rel="noreferrer" target="_blank" href={move || format!("https://docs.rs/viz/{}", version())} class="transition-colors op75 hover:op100">
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
                                                href={move || format!("https://{}viz.rs/{}{}", if l[0] == "en" { "".to_string() } else { l[0].to_string() + "." }, version(), doc_path())}
                                                class="flex hover:text-yellow-600" class=("text-yellow-600", move || l[0] == lang() )
                                                on:click=change_lang.clone()
                                            >{l[1]}</a>
                                        </li>
                                    }
                                )
                                .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
                <button class="transition-colors op75 hover:op100" on:click=toggle_color_scheme>
                    <span aria-hidden="true" class="dark:i-lucide-moon i-lucide-sun block"></span>
                </button>
            </div>
            <button id="toggle-sidebar" class="absolute w-8 h-8 items-center justify-center left-0 bottom--8 transition-colors op75 hover:op100">
                <span class="block i-lucide-sidebar-close"></span>
            </button>
        </header>
    }
}
