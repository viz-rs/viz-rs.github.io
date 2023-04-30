mod components;
mod pages;

pub mod api;
use components::*;
use leptos::*;
use leptos_router::*;
use pages::*;

pub const LANGS: [[&str; 2]; 2] = [["en", "English"], ["zh-cn", "简体中文"]];
pub const VERSIONS: [&str; 2] = ["0.4.x", "0.5.x"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppState {
    dark: bool,
    home: bool,
    lang: String,
    version: String,
}

#[component]
pub fn MyRouter(cx: Scope) -> impl IntoView {
    log::debug!("rendering <MyRouter/>");

    // contexts are passed down through the route tree
    let state = create_rw_signal(
        cx,
        AppState {
            dark: false,
            home: true,
            lang: LANGS[0][0].to_string(),
            version: VERSIONS[0].to_string(),
        },
    );

    let dark_part = create_slice(
        cx,
        state,
        |state| state.dark,
        |state, dark| {
            log::info!("change dark: {}", &dark);
            state.dark = dark;
        },
    );
    let home_part = create_slice(
        cx,
        state,
        |state| state.home,
        |state, home| {
            log::info!("change home: {}", &home);
            state.home = home;
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

    provide_context(cx, state);

    view! { cx,
        <Router>
            <div id="app" class="tracking-0.2px">
                <Navbar dark_part=dark_part home_part=home_part lang_part=lang_part version_part=version_part />
                <div class="page-container flex-row pt-4.375rem" class:opened={move || !home_part.0()}>
                    <div id="backdrop" />

                    <Sidebar lang_part=lang_part version_part=version_part />

                    <main id="page" class="flex flex-row flex-1 py-5">
                        <Routes>
                            <Route
                                path=""
                                view=move |cx| view! { cx,  <Home version=version_part.0 /> }
                            />
                            <Route
                                path=":version/*path"
                                view=move |cx| view! { cx, <Doc version_part=version_part /> }
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
