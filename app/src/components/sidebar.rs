use leptos::*;
use leptos_i18n::Locale;
use leptos_router::A;

use crate::api::{fetch_toc, Section};
use crate::{i18n::*, GlobalState};

#[component]
pub fn Sidebar() -> impl IntoView {
    let state = expect_context::<GlobalState>();
    let i18n = use_i18n();

    let sections = create_resource(
        move || (i18n.get_locale().as_str().to_string(), state.version.get()),
        fetch_toc,
    );

    view! {
        <aside class="fixed z-35 flex flex-col p-5 gap-4 sidebar top-4.375rem bottom-0">
            <Suspense
                fallback=|| view! {
                    <div class="i-lucide-loader w-6 h-6 animate-spin absolute" />
                }
            >
            {
                move || sections.get()
                    .flatten()
                    .map(move |sections| {
                        view! {
                            <For
                                each=move || sections.clone()
                                key=|section| section.text.clone()
                                children=move |Section { text, prefix, items }| view! {
                                    <section>
                                        <h3 class="py-1 text-4 font-medium">{text}</h3>
                                        <ul class="text-3.5">
                                        <For
                                            each=move || items.clone()
                                            key=|item| item.0.clone()
                                            children=move |(text, path)| {
                                                let prefix = prefix.clone();
                                                view! {
                                                    <li>
                                                        <A
                                                            href=move || format!("/{}/{}/{}/{}", i18n.get_locale().as_str(), state.version.get(), prefix, path)
                                                            class="inline-block py-1 font-normal transition-colors hover:op100 op61.8"
                                                        >
                                                            {text}
                                                        </A>
                                                    </li>
                                                }
                                            }
                                        />
                                        </ul>
                                    </section>
                                }
                            />
                        }
                    })
            }
            </Suspense>
        </aside>
    }
}
