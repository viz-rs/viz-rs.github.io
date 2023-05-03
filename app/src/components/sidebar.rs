use leptos::*;
use leptos_router::*;

use crate::api::{fetch_toc, Section};

#[component]
pub fn Sidebar(cx: Scope, version: ReadSignal<String>) -> impl IntoView {
    let sections = create_resource(cx, version, fetch_toc);

    view! { cx,
        <aside class="fixed z-35 flex flex-col p-5 gap-4 sidebar top-4.375rem bottom-0">
            <Suspense
                fallback=move || view! {
                    cx,
                    <div class="i-lucide-loader w-6 h-6 animate-spin absolute" />
                }
            >
            {
                move || sections.read(cx)
                    .and_then(|sections| sections)
                    .map(move |sections| {
                        view! {
                            cx,
                            <For
                                each=move || sections.clone()
                                key=|section| section.text.clone()
                                view=move |cx, Section { text, prefix, items }| view! {
                                    cx,
                                    <section>
                                        <h3 class="py-1 text-4 font-medium">{text}</h3>
                                        <ul class="text-3.5">
                                        <For
                                            each=move || items.clone()
                                            key=|item| item.0.clone()
                                            view=move |cx, (text, path)| {
                                                view! {
                                                    cx,
                                                    <li>
                                                        <A
                                                            href=format!("/{}/{}/{}", version(), prefix, path)
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
