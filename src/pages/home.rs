use leptos::*;
use leptos_router::A;

use crate::AppState;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).unwrap();
    let path = app_state.lang + "/" + &app_state.version;

    view! { cx,
        <section class="w-full hero text-center p-5 sm:py-19">
            <h1 class="text-8 sm:text-10 font-medium">"Fast, robust, flexible, lightweight web framework for Rust"</h1>
            <p class="text-4 sm:text-5 mt-4.5 mb-7.5 sm:mt-6 sm:mb-8 op-61.8 font-light">
                <strong class="font-normal">"Viz"</strong>" builts on top of "<a href="https://tokio.rs/" target="_bank" class="text-yellow-600 font-normal">"Tokio"</a>" and "<a href="https://hyper.rs/" target="_bank" class="text-yellow-600 font-normal">"Hyper"</a>"."
            </p>
            <A href={path} class="inline-block bg-neutral-900 text-neutral-100 dark:bg-neutral-100 dark:text-neutral-900 shadow py-2 px-4.5 border-rounded font-medium text-4 cursor-pointer">"Get Started"</A>
        </section>
    }
}
