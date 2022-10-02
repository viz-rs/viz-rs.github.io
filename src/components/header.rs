use sycamore::prelude::*;

#[component]
pub fn Header<'a, G: Html>(cx: Scope<'a>, dark_mode: &'a RcSignal<bool>) -> View<G> {
    let window = web_sys::window().unwrap();
    // let DarkMode(dark_mode) = use_context::<DarkMode>(cx);
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
        header(class="flex flex-row px-5 py-3.75 items-center justify-between text-5 b-b b-b-neutral-900 b-b-op-5 dark:b-b-neutral-100 dark:b-b-op-5") {
            div() {
                a(class="flex flex-row items-center gap-3 transition-colors op75 hover:op100", href="/") {
                    img(class="h-10 block b-neutral-100 dark:b-neutral-500 b mr-1", src="/public/logo.svg")
                    "Viz"
                }
            }
            div(class="flex-row items-center gap-5") {
                a(class="transition-colors op75 hover:op100", href="/docs/") {
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
    }
}
