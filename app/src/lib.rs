leptos_i18n::load_locales!();

mod app;
mod layout;
mod state;

pub use app::App;
pub use layout::Layout;
pub use state::*;

pub mod components;
pub mod pages;
