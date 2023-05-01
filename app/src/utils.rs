use wasm_bindgen::UnwrapThrowExt;

pub fn window() -> web_sys::Window {
    web_sys::window().expect_throw("Can't find the global Window")
}

pub fn media_query(key: &str) -> Option<web_sys::MediaQueryList> {
    window()
        .match_media(key)
        .expect_throw("Can't find media query")
}
