use leptos::{create_rw_signal, RwSignal};

pub const LANGS: [&str; 2] = ["en", "zh-CN"];
pub const VERSIONS: [&str; 1] = ["0.4.x"];

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub dark: RwSignal<bool>,
    pub sidebar: RwSignal<bool>,
    pub lang: RwSignal<String>,
    pub version: RwSignal<String>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            dark: create_rw_signal(false),
            sidebar: create_rw_signal(false),
            lang: create_rw_signal(LANGS[0].to_string()),
            version: create_rw_signal(VERSIONS[0].to_string()),
        }
    }
}
