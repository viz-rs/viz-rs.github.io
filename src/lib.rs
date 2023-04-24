mod api;
mod components;
mod pages;

use crate::api::*;
use leptos::*;
use leptos_router::*;
use components::*;
use pages::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Context(i32);

#[component]
pub fn MyRouter(cx: Scope) -> impl IntoView {
    log::debug!("rendering <MyRouter/>");

    // contexts are passed down through the route tree
    provide_context(cx, Context(0));

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
                                path="docs/*path"
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
