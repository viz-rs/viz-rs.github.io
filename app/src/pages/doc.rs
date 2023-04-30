use leptos::*;
use leptos_router::{use_params, IntoParam, Params};

#[derive(Params, PartialEq, Clone, Debug)]
pub struct DocParams {
    version: String,
    path: String,
}

#[component]
pub fn Doc(cx: Scope, version_part: (Signal<String>, SignalSetter<String>)) -> impl IntoView {
    // let (version, set_version) = version_part;
    let (path, set_path) = create_signal(cx, String::new());

    let params = use_params::<DocParams>(cx);

    create_effect(cx, move |_| {
        let _ = params.get().map(|params| {
            log::info!("{} {}", params.version, params.path);
            // if params.version != version() {
            //     set_version(params.version);
            // }
            // set_path(params.path);
        });
    });

    view! { cx,
        <div>
        {path}
        </div>
    }
}
