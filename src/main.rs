use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

mod pages;
mod components;

#[derive(Clone)]
pub struct DarkMode(RcSignal<bool>);

#[derive(Debug, Route)]
enum Routes {
    #[to("/")]
    Home,
    #[to("/docs/<_..>")]
    Docs(Vec<String>),
    #[not_found]
    NotFound,
}

fn switch<'a, G: Html>(cx: Scope<'a>, route: &'a ReadSignal<Routes>) -> View<G> {
    let view = create_memo(
        cx,
        on([route], move || match route.get().as_ref() {
            Routes::Home => pages::Home(cx),
            Routes::Docs(_a) => pages::Doc(cx),
            Routes::NotFound => view! { cx,
                "404 Not Found"
                a(href="/") { "Home" }
            },
        }),
    );

    view! {
        cx,
        (*view.get())
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap();
    let dark_media = window
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches();
    let dark_value = local_storage
        .as_ref()
        .and_then(|s| s.get_item("color-scheme").ok())
        .map(|val| val.as_deref() == Some("dark"))
        .unwrap_or(dark_media);

    let dark_mode = DarkMode(create_rc_signal(dark_value));
    provide_context(cx, dark_mode);

    let DarkMode(dark_mode) = use_context::<DarkMode>(cx);

    create_effect(cx, move || {
        let value = *dark_mode.get();
        if let Some(local_storage) = local_storage.as_ref() {
            local_storage
                .set_item("color-scheme", if value { "dark" } else { "auto" })
                .unwrap();
        }
        web_sys::window()
            .unwrap()
            .document()
            .and_then(|doc| doc.document_element())
            .and_then(|html| {
                if value {
                    html.class_list().add_1("dark")
                } else {
                    html.class_list().remove_1("dark")
                }
                .ok()
            });
    });

    view! {
        cx,
        div(class="w-screen fixed top-0") {
            (components::Header(cx, dark_mode))
            Router(
                integration=HistoryIntegration::new(),
                view=switch,
            )
        }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            App()
        }
    });
}
