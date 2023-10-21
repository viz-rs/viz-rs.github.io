leptos_i18n::load_locales!();

mod components;
mod pages;
mod utils;

pub mod api;
use components::*;
use leptos::*;
use leptos_router::*;
use pages::*;

#[cfg(feature = "github")]
pub const DOMAIN: &str = "viz-rs.github.io";
#[cfg(not(feature = "github"))]
pub const DOMAIN: &str = "viz.rs";
pub const LANGS: [[&str; 2]; 2] = [["en", "English"], ["zh-cn", "简体中文"]];
pub const VERSIONS: [&str; 1] = ["0.4.x"];
// pub const VERSIONS: [&str; 2] = ["0.4.x", "0.5.x"];

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();
    crate::i18n::provide_i18n_context();

    let (dark, set_dark) = create_signal(false);
    let (sidebar, set_sidebar) = create_signal(false);
    let (lang, set_lang) = create_signal(LANGS[0][0].to_string());
    let (version, set_version) = create_signal(VERSIONS[0].to_string());

    view! {
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
                <div class="page-container pt-4.375rem" class:opened=move || sidebar.get()>
                    <div id="backdrop" on:pointerdown=move |_| set_sidebar.set(false) />

                    <Sidebar lang=lang version=version />

                    <main id="page" class="flex flex-row flex-1 py-5">
                        <Routes>
                            <Route
                                path=""
                                view=move || view! { <Home version=version.get() /> }
                            />
                            <Route
                                path=":lang/:version/*path"
                                view=move || view! { <Doc /> }
                            />
                            <Route
                                path="redirect-home"
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
