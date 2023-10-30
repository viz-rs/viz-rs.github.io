use leptos::*;
use leptos_dom::html::Div;
use leptos_i18n::Locale;
use leptos_router::{use_params, use_route, A};

use crate::api::fetch_doc;
use crate::i18n::*;
use crate::DocumentParams;

#[component]
pub fn Document() -> impl IntoView {
    let i18n = use_i18n();
    let loading = RwSignal::new(false);
    let container = create_node_ref::<Div>();
    let current_params = use_params::<DocumentParams>();

    let click = move |_| {};

    let doc = create_resource(
        move || current_params.get().ok(),
        move |input| async move {
            let DocumentParams {
                lang,
                version,
                tail,
            } = input?;
            let l = lang?;
            let v = version?;
            let t = tail?;

            log::debug!("lang: {}, version: {}, tail: {}", l, v, t);

            loading.set(true);
            fetch_doc(&l, &v, &t).await
        },
    );

    create_effect(move |_| {
        loading.set(false);

        let context = doc.get()??;
        let div = container.get()?;

        div.set_inner_html(&context);

        Some(())
    });

    view! {
        <div class="flex flex-row flex-1">
            <div id="loader" class="i-lucide-loader w-6 h-6 animate-spin absolute" class:hidden=move || !loading.get() />
            <div
                class="flex flex-row flex-1"
                _ref=container
                on:click=click
            >
            </div>
        </div>
    }
}
