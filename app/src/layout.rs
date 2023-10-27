use leptos::*;
use leptos_router::{Route, Router, Routes};
use web_sys::MediaQueryListEvent;

use crate::components::{Footer, Navbar, Sidebar};
use crate::pages::Home;
use crate::utils;
use crate::GlobalState;

#[component]
pub fn Layout() -> impl IntoView {
    let state = expect_context::<GlobalState>();
    let dark_matched = RwSignal::new(false);

    {
        let color_scheme_media_query = utils::media_query(
            "(prefers-color-scheme: dark)",
            move |e: MediaQueryListEvent| {
                let dark = e.matches();
                log::debug!("prefers-color-scheme dark: {}", dark);
                dark_matched.set(dark);
            },
        )
        .unwrap();

        let current_mode = utils::local_storage::get_color_scheme();
        let current_dark_matched = color_scheme_media_query.matches();

        log::debug!(
            "curent color scheme: {}",
            color_scheme_media_query.matches()
        );

        let dark = match current_mode {
            Some(mode) => {
                if current_dark_matched {
                    mode != "light"
                } else {
                    mode == "dark"
                }
            }
            None => current_dark_matched,
        };

        state.dark.set(dark);
    }

    create_effect(move |_| {
        let dark = state.dark.get();
        log::info!("color {}", dark);
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
                        </Routes>
                    </main>
                </div>
                <Footer />
            </div>
        </Router>
    }
}
