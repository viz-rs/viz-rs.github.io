use leptos::*;
use leptos_router::*;

use crate::api::{fetch_toc, Section};

#[component]
pub fn Sidebar(
    cx: Scope,
    lang_part: (Signal<String>, SignalSetter<String>),
    version_part: (Signal<String>, SignalSetter<String>),
) -> impl IntoView {
    let location = use_location(cx);
    let (version, _) = version_part;
    // let sections = create_resource(cx, version_part.0, api::fetch_toc);
    let sections = create_resource(cx, version, move |version| async move {
        // log::info!("{} {}", version_part.0(), version);
        fetch_toc(version).await
    });

    view! { cx,
        <aside class="fixed z-35 flex flex-col p-5 gap-4 sidebar top-4.375rem bottom-0">
        {
            move || sections.read(cx)
                .and_then(|sections| sections)
                .map(|sections| {
                    sections.into_iter()
                        .map(|Section {
                            text, prefix, items
                        }| view! {
                            cx,
                            <section>
                                <h3 class="py-1 text-4 font-medium">{text}</h3>
                                <ul class="text-3.5">
                                {
                                    items
                                        .into_iter()
                                        .map(|(text, path)| {
                                            let full_path = format!("/{}/{}/{}", version(), prefix, path);
                                            let class = format!(
                                                "inline-block py-1 font-normal transition-colors hover:op100 {}",
                                                if (location.pathname)().ends_with(&full_path) {
                                                    "op100 text-yellow-600"
                                                } else {
                                                    "op61.8"

                                                }
                                            );
                                            view! {
                                                cx,
                                                <li>
                                                    <A href={full_path} class={class}>{text}</A>
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }
                                </ul>
                            </section>
                        })
                        .collect::<Vec<_>>()
                })
        }
        </aside>
    }
}
