use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, HtmlElement};

use crate::{AppState, LANGS, VERSIONS};

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>(cx).unwrap();

    let (lang, set_lang) = create_slice(
        cx,
        state,
        |state| state.lang.clone(),
        |state, lang| {
            log::info!("{}", lang);
            state.lang = lang
        },
    );
    let (version, set_version) = create_slice(
        cx,
        state,
        |state| state.version.clone(),
        |state, version| {
            log::info!("{}", version);
            state.version = version
        },
    );

    let change_version = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        set_version(value);
    };

    let change_lang = move |ev: ev::MouseEvent| {
        let element = ev.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        if let Some(el) = JsCast::dyn_ref::<HtmlElement>(&element) {
            if let Some(value) = el.get_attribute("data-lang") {
                set_lang(value);
            }
        }
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
            <div class="flex-row items-center gap-5 font-medium text-15px">
                <a href="/docs/0.4.x/guide/introduction" class="transition-colors op75 hover:op100">
                    <span class="i-lucide-book-open block"></span>
                </a>
                <a rel="noreferrer" target="_blank" href="https://docs.rs/viz/"{version()} class="transition-colors op75 hover:op100">
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
                                                href={format!("/{}/{}", l[0], version())}
                                                class="flex hover:text-yellow-600" class=("text-yellow-600", {move || l[0] == lang()} )
                                                on:click=change_lang.clone()
                                            >{l[1]}</a>
                                        </li>
                                    }
                                )
                                .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
                <button class="transition-colors op75 hover:op100">
                    <span aria-hidden="true" class="dark:i-lucide-moon i-lucide-sun block"></span>
                </button>
            </div>
            <button id="toggle-sidebar" class="absolute w-8 h-8 items-center justify-center left-0 bottom--8 transition-colors op75 hover:op100">
                <span class="block i-lucide-sidebar-close"></span>
            </button>
        </header>
    }
}
