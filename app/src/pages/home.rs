use leptos::*;
use leptos_i18n::Locale;
use leptos_router::A;

use crate::i18n::*;

#[component]
pub fn Home(version: RwSignal<String>) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="w-full hero text-center p-5 sm:py-19">
            <h1 class="text-8 sm:text-10 font-medium">{t!(i18n, description)}</h1>
            <p class="text-4 sm:text-5 mt-4.5 mb-7.5 sm:mt-6 sm:mb-8 op-61.8 font-light" inner_html={t!(i18n, builts_on)}></p>
            <A href=move || format!("/{}/{}/guide/introduction", i18n.get_locale().as_str(), version.get()) class="inline-block bg-neutral-900 text-neutral-100 dark:bg-neutral-100 dark:text-neutral-900 shadow py-2 px-4.5 border-rounded font-medium text-4 cursor-pointer">{t!(i18n, get_started)}</A>
        </section>
    }
}
