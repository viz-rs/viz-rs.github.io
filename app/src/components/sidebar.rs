use leptos::*;
use leptos_router::*;

use crate::api::{fetch_toc, Section};

#[component]
pub fn Sidebar(#[prop(into)] version: ReadSignal<String>) -> impl IntoView {
    let sections = create_resource(move || version.get(), fetch_toc);

    view! {
        <aside class="fixed z-35 flex flex-col p-5 gap-4 sidebar top-4.375rem bottom-0">
            <Suspense
                fallback=move || view! {
                    <div class="i-lucide-loader w-6 h-6 animate-spin absolute" />
                }
            >
            {
                move || sections.read()
                    .flatten()
                    .map(move |sections| {
                        view! {
                            <For
                                each=move || sections.clone()
                                key=|section| section.text.clone()
                                view=move |Section { text, prefix, items }| view! {
                                    <section>
                                        <h3 class="py-1 text-4 font-medium">{text}</h3>
                                        <ul class="text-3.5">
                                        <For
                                            each=move || items.clone()
                                            key=|item| item.0.clone()
                                            view=move |(text, path)| {
                                                let prefix = prefix.clone();
                                                view! {
                                                    <li>
                                                        <A
                                                            href=move || format!("/{}/{}/{}", version.get(), prefix, path)
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
