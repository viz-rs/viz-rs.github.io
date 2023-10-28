use leptos::{create_rw_signal, RwSignal};

pub const LANGS: [[&str; 2]; 3] = [
    ["en", "English"],
    ["zh-CN", "简体中文"],
    ["zh-TW", "繁體中文"],
];
pub const VERSIONS: [&str; 2] = ["0.4.x", "0.5.x"];

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub dark: RwSignal<bool>,
    pub home: RwSignal<bool>,
    pub sidebar: RwSignal<bool>,
    // pub lang: RwSignal<String>,
    pub version: RwSignal<String>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            dark: create_rw_signal(false),
            home: create_rw_signal(true),
            sidebar: create_rw_signal(false),
            // lang: create_rw_signal(LANGS[0][0].to_string()),
            version: create_rw_signal(VERSIONS[0].to_string()),
        }
    }
}
