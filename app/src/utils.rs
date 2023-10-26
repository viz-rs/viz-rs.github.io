use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{MediaQueryList, MediaQueryListEvent};

pub fn window() -> web_sys::Window {
    web_sys::window().expect_throw("Can't find the global Window")
}

pub fn document() -> web_sys::Document {
    window().document().expect_throw("Can't find document")
}

pub fn document_element() -> web_sys::Element {
    document()
        .document_element()
        .expect_throw("Can't find document element")
}

pub fn media_query<F>(key: &str, f: F) -> Option<MediaQueryList>
where
    F: Fn(MediaQueryListEvent) + 'static,
{
    let media = window()
        .match_media(key)
        .expect_throw("Can't find media query");

    if let Some(ref media) = media {
        let cb: Closure<dyn Fn(MediaQueryListEvent)> =
            Closure::new(move |e: MediaQueryListEvent| f(e));
        media.set_onchange(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    media
}

pub async fn copy(text: &str) {
    if let Some(c) = window().navigator().clipboard() {
        let _ = wasm_bindgen_futures::JsFuture::from(c.write_text(text)).await;
    }
}

pub fn local_storage_get(key: &str) -> Option<String> {
    window().local_storage().ok()??.get(key).ok()?
}

pub fn local_storage_set(key: &str, val: &str) -> Option<()> {
    window().local_storage().ok()??.set(key, val).ok()
}

pub fn get_color_scheme() -> String {
    local_storage_get("color-scheme").unwrap_or("auto".to_string())
}

pub fn toggle_dark(dark: bool) {
    let _ = document_element()
        .class_list()
        .toggle_with_force("dark", dark);
}

pub fn set_timeout<F>(f: F, number: i32)
where
    F: Fn() + 'static,
{
    let cb: Closure<dyn Fn()> = Closure::new(f);
    let window = web_sys::window().unwrap();
    let _ = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), number);
    cb.forget();
}
