use app::*;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    });
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
