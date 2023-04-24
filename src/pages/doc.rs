use leptos::*;
use leptos_router::{Params, use_params, IntoParam};

#[derive(Params, PartialEq, Clone, Debug)]
pub struct DocParams {
    path: String,
}

#[component]
pub fn Doc(
    cx: Scope,
) -> impl IntoView {
    let params = use_params::<DocParams>(cx);
    let path = move || {
        params.with(|params| {
            params.as_ref().map(|params| params.path.clone())
                .unwrap_or_default()
        })
    };

    view! { cx,
        <div>
        {path}
        </div>
    }
}
