mod components;
mod pages;
mod utils;

pub mod api;
use components::*;
use leptos::*;
use leptos_router::*;
use pages::*;
use web_sys::MediaQueryListEvent;

pub const LANGS: [[&str; 2]; 2] = [["en", "English"], ["zh-cn", "简体中文"]];
pub const VERSIONS: [&str; 2] = ["0.4.x", "0.5.x"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppState {
    dark: bool,
    sidebar: bool,
    lang: String,
    version: String,
}

#[component]
pub fn MyRouter(cx: Scope) -> impl IntoView {
    log::debug!("rendering <MyRouter/>");

    let (dark_matches, set_dark_matches) = create_signal(cx, false);

    let dark_media = utils::media_query(
        "(prefers-color-scheme: dark)",
        move |e: MediaQueryListEvent| {
            set_dark_matches(e.matches());
        },
    )
    .unwrap();

    let mode = utils::get_color_scheme();

    let dark = if dark_media.matches() {
        mode != "light"
    } else {
        mode == "dark"
    };

    utils::toggle_dark(dark);

    let state = create_rw_signal(
        cx,
        AppState {
            dark,
            sidebar: false,
            lang: LANGS[0][0].to_string(),
            version: VERSIONS[0].to_string(),
        },
    );

    provide_context(cx, state);

    let dark_part = create_slice(
        cx,
        state,
        move |state| state.dark,
        move |state, dark| {
            if state.dark != dark {
                log::info!("change dark: {}", &dark);
                state.dark = dark;
                utils::toggle_dark(state.dark);
                utils::local_storage_set(
                    "color-scheme",
                    if state.dark == dark_matches() {
                        "auto"
                    } else if state.dark {
                        "dark"
                    } else {
                        "light"
                    },
                );
            }
        },
    );
    let sidebar_part = create_slice(
        cx,
        state,
        |state| state.sidebar,
        |state, sidebar| {
            log::info!("change sidebar: {}", &sidebar);
            state.sidebar = sidebar;
        },
    );
    let lang_part = create_slice(
        cx,
        state,
        |state| state.lang.clone(),
        |state, lang| {
            log::info!("change lang: {}", &lang);
            state.lang = lang
        },
    );
    let version_part = create_slice(
        cx,
        state,
        |state| state.version.clone(),
        |state, version| {
            log::info!("change version: {}", &version);
            state.version = version
        },
    );

    view! { cx,
        <Router>
            <div id="app" class="tracking-0.2px">
                <Navbar dark_part=dark_part sidebar_part=sidebar_part lang_part=lang_part version_part=version_part />
                <div class="page-container flex-row pt-4.375rem" class:opened=move || sidebar_part.0()>
                    <div id="backdrop" on:click=move |_| sidebar_part.1(false) />

                    <Sidebar version_part=version_part />

                    <main id="page" class="flex flex-row flex-1 py-5">
                        <Routes>
                            <Route
                                path=""
                                view=move |cx| view! { cx,  <Home version=version_part.0 /> }
                            />
                            <Route
                                path=":version/*path"
                                view=move |cx| view! { cx, <Doc /> }
                            />
                            <Route
                                path="redirect-home"
                                view=move |cx| view! { cx, <Redirect path="/"/> }
                            />
                        </Routes>
                    </main>
                </div>

                <Footer />
            </div>
        </Router>
    }
}
