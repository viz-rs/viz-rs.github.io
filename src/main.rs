use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        div(class="w-screen fixed top-0 text-neutral-500") {
            header(class="flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-100") {
                div() {
                    a(class="flex flex-row items-center gap-3 transition-colors hover-text-neutral-900", href="/") {
                        img(class="h-10 block b-neutral-100 b mr-1", src="/public/logo.svg")
                        "Viz"
                    }
                }
                div(class="flex-row gap-5") {
                    a(class="transition-colors hover-text-neutral-900", href="/docs") {
                        "Docs"
                    }
                    a(class="transition-colors hover-text-neutral-900", href="https://docs.rs/viz/latest/viz", target="_blank", rel="noreferrer") {
                        "API"
                    }
                    a(class="transition-colors hover-text-neutral-900", href="https://github.com/viz-rs/viz", target="_blank", rel="noreferrer") {
                        "GitHub"
                    }
                }
            }
            div(class="flex-row") {
                aside(class="sticky flex flex-col flex-[0_0_15rem] p-5 gap-5 sidebar") {
                    section() {
                        h3(class="py-1 text-4 font-medium") {
                            "Get started"
                        }
                        ul(class="text-3.5") {
                            li() {
                                a(class="block py-1 font-normal transition-colors hover-text-neutral-900", href="/docs/introduction") {
                                    "Introduction"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors hover-text-neutral-900", href="/docs/quick-start") {
                                    "Quick Start"
                                }
                            }
                        }
                    }
                    section() {
                        h3(class="py-1 text-4 font-medium") {
                            "Core concepts"
                        }
                        ul() {
                        }
                    }
                    section() {
                        h3(class="py-1 text-4 font-medium") {
                            "Advanced concepts"
                        }
                        ul() {
                        }
                    }
                }
                main(class="flex flex-row flex-1") {
                    article(class="flex flex-1") {
                    }
                    nav(class="sticky flex-col flex-[0_0_15rem] p-5 gap-5 hidden lg:flex") {
                        ul(class="text-3") {
                            li() {
                                a(class="block py-1 font-normal transition-colors hover-text-neutral-900", href="/docs/introduction") {
                                    "Defining attributes"
                                }
                            }
                            li() {
                                a(class="block py-1 font-normal transition-colors hover-text-neutral-900", href="/docs/quick-start") {
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
