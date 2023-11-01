use leptos::*;
use leptos_router::{Redirect, Route, Router, Routes};
use web_sys::MediaQueryListEvent;

use crate::components::{Footer, Navbar, Sidebar};
use crate::pages::{Document, Home};
use crate::utils;
use crate::GlobalState;

#[component]
pub fn Layout() -> impl IntoView {
    let GlobalState {
        dark,
        home,
        sidebar,
        ..
    } = expect_context();
    let dark_matched = RwSignal::new(false);
    let opened = create_memo(move |_| !home.get() && sidebar.get());

    {
        let min_width_media_query =
            utils::media_query("(min-width: 960px)", move |e: MediaQueryListEvent| {
                sidebar.set(e.matches())
            })
            .unwrap();
        let color_scheme_media_query = utils::media_query(
            "(prefers-color-scheme: dark)",
            move |e: MediaQueryListEvent| dark_matched.set(e.matches()),
        )
        .unwrap();
        let current_dark_matched = color_scheme_media_query.matches();
        let current_dark =
            utils::local_storage::get_color_scheme().map_or(current_dark_matched, |mode| {
                if current_dark_matched {
                    mode != "light"
                } else {
                    mode == "dark"
                }
            });

        dark.set(current_dark);
        sidebar.set(min_width_media_query.matches());
    }

    create_effect(move |_| {
        let dark = dark.get();
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

                <div class="page-container pt-4.375rem" class:opened=opened>
                    <div id="backdrop" on:pointerdown=move |_| sidebar.update(|v| *v = false) />

                    <Show when=move || !home.get()>
                        <Sidebar />
                    </Show>

                    <main id="page" class="flex flex-row flex-1 py-5">
                        <Routes>
                            <Route path="/" view=Home />
                            <Route path=":lang/:version/*tail" view=Document />
                            <Route
                                path="*any"
                                view=|| view! { <Redirect path="/"/> }
                            />
                        </Routes>
                    </main>
                </div>

                <Footer />
            </div>
        </Router>
    }
}
