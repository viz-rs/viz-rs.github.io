use sycamore::prelude::*;

#[derive(Clone)]
struct DarkMode(RcSignal<bool>);

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap();
    let dark_media = window
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches();
    let dark_value = local_storage
        .as_ref()
        .and_then(|s| s.get_item("color-scheme").ok())
        .map(|val| val.as_deref() == Some("dark"))
        .unwrap_or(dark_media);

    let dark_mode = DarkMode(create_rc_signal(dark_value));
    provide_context(cx, dark_mode);

    let DarkMode(dark_mode) = use_context::<DarkMode>(cx);

    create_effect(cx, move || {
        let value = *dark_mode.get();
        if let Some(local_storage) = local_storage.as_ref() {
            local_storage
                .set_item("color-scheme", if value { "dark" } else { "auto" })
                .unwrap();
        }
        web_sys::window()
            .unwrap()
            .document()
            .and_then(|doc| doc.document_element())
            .and_then(|html| {
                if value {
                    html.class_list().add_1("dark")
                } else {
                    html.class_list().remove_1("dark")
                }
                .ok()
            });
    });

    let toggle = move |_| {
        let value = !*dark_mode.get();
        dark_mode.set(value);
        window
            .document()
            .and_then(|doc| doc.document_element())
            .map(|html| html.class_list().toggle_with_force("dark", value));
    };

    view! {
        cx,
        div(class="w-screen fixed top-0 text-neutral-500") {
            header(class="flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-100 dark:b-b-neutral-900") {
                div() {
                    a(class="flex flex-row items-center gap-3 transition-colors op75 hover:op100", href="/") {
                        img(class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1", src="/public/logo.svg")
                        "Viz"
                    }
                }
                div(class="flex-row items-center gap-5") {
                    a(class="transition-colors op75 hover:op100", href="/docs") {
                        "Docs"
                    }
                    a(class="transition-colors op75 hover:op100", href="https://docs.rs/viz/latest/viz", target="_blank", rel="noreferrer") {
                        "API"
                    }
                    a(class="transition-colors op75 hover:op100 i-carbon-logo-github", href="https://github.com/viz-rs/viz", target="_blank", rel="noreferrer") {
                    }
                    button(
                        class="hover:bg-gray5:2 hover:op100",
                        on:click=toggle,
                    ) {
                        span(class="dark:i-carbon-moon i-carbon-sun block")
                    }
                }
            }
            div(class="flex-row") {
                aside(class="sticky flex flex-col flex-[0_0_15rem] p-5 gap-4 sidebar") {
                    section() {
                        h3(class="py-1 text-neutral-800 dark:text-neutral-500 text-4 font-medium") {
                            "Get Started"
                        }
                        ul(class="text-3.5") {
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/guide/introduction") {
                                    "Introduction"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/guide/quick-start") {
                                    "Quick Start"
                                }
                            }
                        }
                    }
                    section() {
                        h3(class="py-1 text-neutral-800 text-4 font-medium") {
                            "Core Concepts"
                        }
                        ul() {
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/concepts/handler") {
                                    "Handler"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/concepts/middleware") {
                                    "Middleware"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/concepts/routing") {
                                    "Routing"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/concepts/handler") {
                                    "Extractors"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/concepts/handler") {
                                    "Error Handling"
                                }
                            }
                        }
                    }
                    section() {
                        h3(class="py-1 text-neutral-800 text-4 font-medium") {
                            "Advanced Concepts"
                        }
                        ul() {
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/concepts/handler") {
                                    "Built-in Extractors"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/concepts/handler") {
                                    "Built-in Middleware"
                                }
                            }
                        }
                    }
                    section() {
                        h3(class="py-1 text-neutral-800 text-4 font-medium") {
                            "Extra Topics"
                        }
                        ul() {
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/extra-topics/benchmarks") {
                                    "Benchmarks"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/extra-topics/examples") {
                                    "Examples"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/extra-topics/showcase") {
                                    "Showcase"
                                }
                            }
                        }
                    }
                }
                main(class="flex flex-row flex-1") {
                    article(class="flex flex-1") {
                    }
                    nav(class="sticky flex-col flex-[0_0_15rem] p-5 gap-5 hidden lg:flex") {
                        ul(class="text-3") {
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/introduction") {
                                    "Defining attributes"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/quick-start") {
                                    "Create a custom attribute"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            App()
        }
    });
}
