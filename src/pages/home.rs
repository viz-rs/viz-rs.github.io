use sycamore::prelude::*;

#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        section(class="hero text-center px-8 py-8 sm:py-24"){
            h1(class="text-8 sm:text-10") {
                "Fast, robust, flexible, lightweight web framework for Rust"
            }
            p(class="text-4 sm:text-5 mt-4.5 mb-7.5 sm:mt-6 sm:mb-8 op-61.8") {
                "Viz is built on top of hyper and tokio."
            }
            a(id="get-started", class="inline-block bg-neutral-900 text-neutral-100 dark:bg-neutral-100 dark:text-neutral-900 shadow py-2 px-4.5 border-rounded font-medium text-4", href="/docs/guide/introduction") {
                "Get Started"
            }
        }
    }
}
