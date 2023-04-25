// mod api;
mod components;
mod pages;

// use crate::api::*;
use components::*;
use leptos::*;
use leptos_router::*;
use pages::*;

pub const LANGS: [[&str; 2]; 2] = [["en", "English"], ["zh-cn", "简体中文"]];
pub const VERSIONS: [&str; 2] = ["0.4.x", "0.5.x"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppState {
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
            lang: LANGS[0][0].to_string(),
            version: VERSIONS[0].to_string(),
        },
    );
    provide_context(cx, state);

    view! { cx,
        <Router>
            <div id="app" class="tracking-0.2px">
                <Navbar />
                <div class="page-container flex-row pt-4.375rem" >
                    <div id="backdrop" />

                    <Sidebar />

                    <main id="page" class="flex flex-row flex-1 py-5">
                        <Routes>
                            <Route
                                path=""
                                view=move |cx| view! { cx,  <Home /> }
                            />
                            <Route
                                path=":lang/:version/*path"
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
