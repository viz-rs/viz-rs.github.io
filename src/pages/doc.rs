// use std::ops::Deref;

use sycamore::prelude::*;
// use sycamore_router::{HistoryIntegration, Router};

use crate::{DocRoutes, Link, Section, Sections};

// #[derive(Clone, Debug)]
// pub struct State {
//     pub dark: RcSignal<bool>,
//     pub sidebar: RcSignal<Vec<RcSignal<Link>>>,
// }

// #[component(inline_props)]
// fn Ul<'a, G: Html>(cx: Scope<'a>, links: &'a Signal<Vec<Link>>) -> View<G> {
//     view! { cx,
//         ul(class="text-3.5") {
//             Indexed(
//                 iterable=links,
//                 view=|cx, Link { text, href  }| view! { cx,
//                     li() {
//                         a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href=format!("/docs/${}", href.as_ref().unwrap())) {
//                             (text)
//                         }
//                     }
//                 }
//             )
//         }
//     }
// }

fn switch< G: Html>(cx: Scope, route: &DocRoutes) -> View<G> {
    match route {
        DocRoutes::Doc(a) => Doc(cx, a.to_vec()),
        DocRoutes::NotFound => view! { cx,
            "404 Not Found"
            a(href="/") { "Home" }
        },
    }
}

#[component]
pub fn Doc<G: Html>(cx: Scope, path: Vec<String>) -> View<G> {
    log::info!("{:?}", path);
    view! {
        cx,
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

#[component]
pub fn Docs<G: Html>(cx: Scope, route: &DocRoutes) -> View<G> {
    let Sections(sections) = use_context::<Sections>(cx);

    log::info!("{:?}", route);
    log::info!("{:?}", sections);
    view! {
        cx,
        div(class="flex-row") {
            aside(class="sticky flex flex-col flex-[0_0_15rem] p-5 gap-4 sidebar") {
                Indexed(
                    iterable=sections,
                    view=|cx, Section { title, links }| {
                        let links = links.into_iter().map(|Link {text, href}| view! { cx,
                            li() {
                                a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href=format!("/docs{}", href.as_ref().unwrap())) {
                                    (text)
                                }
                            }
                        }).collect();

                        let views = View::new_fragment(links);

                        view! { cx,
                        section() {
                            h3(class="py-1 text-4 font-medium") {
                                (title.text)
                            }
                            ul(class="text-3.5") {
                                (views)
                            }
                        }
                    }
                    }
                )
                // section() {
                //     h3(class="py-1 text-4 font-medium") {
                //         "Get Started"
                //     }
                //     ul(class="text-3.5") {
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/guide/introduction") {
                //                 "Introduction"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/guide/quick-start") {
                //                 "Quick Start"
                //             }
                //         }
                //     }
                // }
                // section() {
                //     h3(class="py-1 text-4 font-medium") {
                //         "Concepts"
                //     }
                //     ul() {
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/handler") {
                //                 "Handler"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/middleware") {
                //                 "Middleware"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/routing") {
                //                 "Routing"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/extractors") {
                //                 "Extractors"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/server") {
                //                 "Server"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/error-handling") {
                //                 "Error Handling"
                //             }
                //         }
                //     }
                // }
                // section() {
                //     h3(class="py-1 text-4 font-medium") {
                //         "Built-in"
                //     }
                //     ul() {
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/built-in/handlers") {
                //                 "Handlers"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/built-in/middleware") {
                //                 "Middleware"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/built-in/extractors") {
                //                 "Extractors"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/built-in/tls") {
                //                 "TLS"
                //             }
                //         }
                //     }
                // }
                // section() {
                //     h3(class="py-1 text-4 font-medium") {
                //         "Extra Topics"
                //     }
                //     ul() {
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/extra-topics/benchmarks") {
                //                 "Benchmarks"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/extra-topics/examples") {
                //                 "Examples"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/extra-topics/showcase") {
                //                 "Showcase"
                //             }
                //         }
                //     }
                // }
            }
            main(class="flex flex-row flex-1") {
                (switch(cx, route))
                // article(class="flex flex-1") {
                // }
                // nav(class="sticky flex-col flex-[0_0_15rem] p-5 gap-5 hidden lg:flex") {
                //     ul(class="text-3") {
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/introduction") {
                //                 "Defining attributes"
                //             }
                //         }
                //         li() {
                //             a(class="block py-1 font-normal transition-colors op75 hover:op100", href="/docs/quick-start") {
                //                 "Create a custom attribute"
                //             }
                //         }
                //     }
                // }
            }
        }
    }
}
