use leptos::*;
use leptos_i18n::Locale;
use leptos_router::A;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, HtmlElement, MediaQueryListEvent};

use crate::i18n::{self, use_i18n};
use crate::GlobalState;
use crate::{LANGS, VERSIONS};

#[component]
pub fn Navbar() -> impl IntoView {
    let i18n = use_i18n();
    let state = expect_context::<GlobalState>();

    let on_switch_version = move |e: ev::Event| {
        let value = event_target_value(&e);
        log::debug!("version: {}", &value);
        state.version.update(|v| *v = value);
    };

    let on_switch_lang = move |e: ev::MouseEvent| {
        let element = e.target().unwrap().unchecked_into::<HtmlAnchorElement>();
        JsCast::dyn_ref::<HtmlElement>(&element)
            .and_then(|el| el.get_attribute("data-lang"))
            .filter(|lang| lang != i18n.get_locale().as_str())
            .as_ref()
            .and_then(|lang| i18n::Locale::from_str(lang))
            .map(|lang| {
                log::debug!("lang: {:?}", &lang);
                i18n.set_locale(lang);
            });
    };

    let on_switch_color_scheme = move |e: ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        log::debug!("color scheme: {}", !state.dark.get());
        state.dark.update(|v| *v = !*v);
    };

    view! {
        <header class="w-full fixed top-0 z-36 flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-900 b-b-op-5 dark:b-b-neutral-100 dark:b-b-op-5 navbar">
            <div class="flex flex-row">
                <A href="/" class="flex flex-row items-center transition-colors op75 hover:op100">
                    <img alt="Viz" src="/logo.svg" class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1 mr-3" />
                    <span class="font-semibold">"V"</span><span>"iz"</span>
                </A>
                <select id="versions" class="text-right select-none text-2 font-light" on:change=on_switch_version>
                    {
                        VERSIONS.into_iter()
                            .map(|v| view! {
                                <option value=v selected=move || v == state.version.get()>"v"{v}</option>
                            })
                            .collect::<Vec<_>>()
                    }
                </select>
            </div>
            <div class="flex flex-row items-center gap-5 font-medium text-15px">
                <A class="transition-colors op75 hover:op100" href=move || format!("/{}/{}/guide/introduction", "", "")>
                    <span class=move || if true { "i-lucide-book block" } else { "i-lucide-book-open block" } />
                </A>
                <a rel="noreferrer" target="_blank" class="transition-colors op75 hover:op100" href=move || format!("https://docs.rs/viz/{}", "")>
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
                class=("!hidden", move || false)
                on:click=move |_| {}>
                <span
                    class="block"
                    class=move || if true { "i-lucide-sidebar-open" } else { "i-lucide-sidebar-close" }
                />
            </button>
        </header>
    }
}
