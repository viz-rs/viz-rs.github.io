use leptos::*;
use leptos_router::{Redirect, Route, Router, Routes};
use web_sys::MediaQueryListEvent;

use crate::components::{Footer, Navbar, Sidebar};
use crate::pages::{Document, Home};
use crate::utils;
use crate::GlobalState;

#[component]
pub fn Layout() -> impl IntoView {
    let state = expect_context::<GlobalState>();
    let dark_matched = RwSignal::new(false);

    {
        let min_width_media_query =
            utils::media_query("(min-width: 960px)", move |e: MediaQueryListEvent| {
                state.sidebar.set(e.matches())
            })
            .unwrap();

        let color_scheme_media_query = utils::media_query(
            "(prefers-color-scheme: dark)",
            move |e: MediaQueryListEvent| dark_matched.set(e.matches()),
        )
        .unwrap();
        let current_dark_matched = color_scheme_media_query.matches();
        let dark = utils::local_storage::get_color_scheme().map_or(current_dark_matched, |mode| {
            if current_dark_matched {
                mode != "light"
            } else {
                mode == "dark"
            }
        });

        state.dark.set(dark);
        state.sidebar.set(min_width_media_query.matches());
    }

    create_effect(move |_| {
        let dark = state.dark.get();
        utils::toggle_dark(dark);
        utils::local_storage::set_color_scheme(if dark == dark_matched.get() {
            "auto"
        } else if dark {
            "dark"
        } else {
            "light"
        });
    });

    view! {
        <Router>
            <div id="app" class="tracking-0.2px">
                <Navbar />
                <div class="page-container pt-4.375rem" class:opened=state.sidebar>
                    <div id="backdrop" on:pointerdown=move |_| state.sidebar.update(|v| *v = false) />

                    <Sidebar />

                    <main id="page" class="flex flex-row flex-1 py-5">
                        <Routes>
                            <Route
                                path=""
                                view=move || view! { <Home version=state.version /> }
                            />
                            <Route
                                path=":lang/:version/*path"
                                view=move || view! { <Document /> }
                            />
                            <Route
                                path="*"
                                view=move || view! { <Redirect path="/"/> }
                            />
                        </Routes>
                    </main>
                </div>
                <Footer />
            </div>
        </Router>
    }
}
