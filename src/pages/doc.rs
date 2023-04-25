use leptos::*;
use leptos_router::{use_params, IntoParam, Params};

#[derive(Params, PartialEq, Clone, Debug)]
pub struct DocParams {
    lang: String,
    version: String,
    path: String,
}

#[component]
pub fn Doc(
    cx: Scope,
    lang_part: (Signal<String>, SignalSetter<String>),
    version_part: (Signal<String>, SignalSetter<String>),
) -> impl IntoView {
    let (_, set_lang) = lang_part;
    let (_, set_version) = version_part;
    let (path, set_path) = create_signal(cx, String::new());

    let params = use_params::<DocParams>(cx);

    create_effect(cx, move |_| {
        let _ = params.get()
            .map(|params| {
                log::info!("{} {} {}", params.lang, params.version, params.path);
                set_lang(params.lang);
                set_version(params.version);
                set_path(params.path);
            });
    });

    view! { cx,
        <div>
        {path}
        </div>
    }
}
