use sycamore::prelude::*;

#[component]
pub fn Doc<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        div(class="flex-row") {
            aside(class="sticky flex flex-col flex-[0_0_15rem] p-5 gap-4 sidebar") {
                section() {
                    h3(class="py-1 text-4 font-medium") {
                        "Get Started"
                    }
                    ul(class="text-3.5") {
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/guide/introduction") {
                                "Introduction"
                            }
                        }
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/guide/quick-start") {
                                "Quick Start"
                            }
                        }
                    }
                }
                section() {
                    h3(class="py-1 text-4 font-medium") {
                        "Concepts"
                    }
                    ul() {
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/handler") {
                                "Handler"
                            }
                        }
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/middleware") {
                                "Middleware"
                            }
                        }
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/routing") {
                                "Routing"
                            }
                        }
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/handler") {
                                "Extractors"
                            }
                        }
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/concepts/handler") {
                                "Error Handling"
                            }
                        }
                    }
                }
                section() {
                    h3(class="py-1 text-4 font-medium") {
                        "Built-in"
                    }
                    ul() {
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/built-in/handler") {
                                "Extractors"
                            }
                        }
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/built-in/handler") {
                                "Middleware"
                            }
                        }
                    }
                }
                section() {
                    h3(class="py-1 text-4 font-medium") {
                        "Extra Topics"
                    }
                    ul() {
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/extra-topics/benchmarks") {
                                "Benchmarks"
                            }
                        }
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/extra-topics/examples") {
                                "Examples"
                            }
                        }
                        li() {
                            a(class="block py-1 font-normal transition-colors op61.8 hover:op100", href="/docs/extra-topics/showcase") {
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
