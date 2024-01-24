use leptos::*;
use leptos_meta::provide_meta_context;

// use crate::i18n::provide_i18n_context;
use crate::GlobalState;
use crate::Layout;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    // provide_i18n_context();
    provide_context(GlobalState::new());

    view! {
        <Layout />
    }
}
