use leptos::*;
// use leptos_i18n::Locale;
use leptos_router::{use_navigate, use_query_map, NavigateOptions, A};

use crate::{
    // i18n::*,
    GlobalState,
};

#[component]
pub fn Home() -> impl IntoView {
    let GlobalState { version, .. } = expect_context();
    let navigate = use_navigate();
    let query = use_query_map();
    // let i18n = use_i18n();

    create_effect(move |_| {
        let query = query.get();
        if let Some(r) = query.get("r") {
            log::debug!("r: {:?}", r);
            navigate(r, NavigateOptions::default());
        }
    });

    view! {
        <section class="w-full hero text-center p-5 sm:py-19">
            <h1 class="text-8 sm:text-10 font-medium">{"description"}</h1>
            <p class="text-4 sm:text-5 mt-4.5 mb-7.5 sm:mt-6 sm:mb-8 op-61.8 font-light" inner_html={"built_on"}></p>
            <A
                class="inline-block bg-neutral-900 text-neutral-100 dark:bg-neutral-100 dark:text-neutral-900 shadow py-2 px-4.5 border-rounded font-medium text-4 cursor-pointer"
                href=move || format!("/{}/{}/guide/introduction", "zh-cn", version.get())
            >{"get_started"}</A>
        </section>
    }
}
