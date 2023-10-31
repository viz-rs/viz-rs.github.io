use leptos::*;
use leptos_dom::helpers::location_pathname;
use leptos_router::{use_navigate, NavigateOptions};

use crate::{i18n::*, GlobalState, LATEST, VERSIONS};

#[component]
pub fn ComingSoon() -> impl IntoView {
    let state = expect_context::<GlobalState>();
    let navigate = use_navigate();
    let i18n = use_i18n();

    let click = move |_| {
        location_pathname()
            .map(|path| {
                navigate(
                    &path.replace(&state.version.get(), VERSIONS[LATEST]),
                    NavigateOptions {
                        resolve: false,
                        replace: true,
                        ..Default::default()
                    },
                )
            })
            .unwrap();
    };

    view! {
        <section class="w-full hero text-center p-5 sm:py-19">
            <h1 class="text-8 sm:text-10 font-medium">{t!(i18n, coming_soon)}</h1>
            <button
                class="inline-block bg-neutral-900 text-neutral-100 dark:bg-neutral-100 dark:text-neutral-900 shadow py-2 px-4.5 border-rounded font-medium text-4 cursor-pointer"
                on:click=click
            >
                {t!(i18n, go_latest)}" - v"{VERSIONS[LATEST]}
            </button>
        </section>
    }
}
