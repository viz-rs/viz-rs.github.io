use leptos::*;
use leptos_router::{Route, Router, Routes};

use crate::components::{Footer, Navbar, Sidebar};
use crate::pages::Home;
use crate::GlobalState;

#[component]
pub fn Layout() -> impl IntoView {
    let state = expect_context::<GlobalState>();

    create_effect(move |_| {
        let dark = state.dark.get();
        log::info!("color {}", dark);
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
