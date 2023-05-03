mod components;
mod pages;
mod utils;

pub mod api;
use components::*;
use leptos::*;
use leptos_router::*;
use pages::*;

pub const LANGS: [[&str; 2]; 2] = [["en", "English"], ["zh-cn", "简体中文"]];
pub const VERSIONS: [&str; 2] = ["0.4.x", "0.5.x"];

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    log::debug!("rendering <App />");

    let (dark, set_dark) = create_signal(cx, false);
    let (sidebar, set_sidebar) = create_signal(cx, false);
    let (lang, set_lang) = create_signal(cx, LANGS[0][0].to_string());
    let (version, set_version) = create_signal(cx, VERSIONS[0].to_string());

    view! { cx,
        <Router>
            <div id="app" class="tracking-0.2px">
                <Navbar
                    dark=dark
                    set_dark=set_dark
                    sidebar=sidebar
                    set_sidebar=set_sidebar
                    lang=lang
                    set_lang=set_lang
                    version=version
                    set_version=set_version
                />
                <div class="page-container pt-4.375rem" class:opened=sidebar>
                    <div id="backdrop" on:click=move |_| set_sidebar(false) />

                    <Sidebar version=version />

                    <main id="page" class="flex flex-row flex-1 py-5">
                        <Routes>
                            <Route
                                path=""
                                view=move |cx| view! { cx,  <Home version=version /> }
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
