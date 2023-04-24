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
                        <AnimatedRoutes
                            outro="slideOut"
                            intro="slideIn"
                            outro_back="slideOutBack"
                            intro_back="slideInBack"
                        >
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
                        </AnimatedRoutes>
                    </main>
                </div>

                <Footer />
            </div>
        </Router>
    }
}

#[component]
pub fn ContactList(cx: Scope) -> impl IntoView {
    log::debug!("rendering <ContactList/>");

    // contexts are passed down through the route tree
    provide_context(cx, Context(42));

    on_cleanup(cx, || {
        log!("cleaning up <ContactList/>");
    });

    let location = use_location(cx);
    let contacts =
        create_resource(cx, move || location.search.get(), get_contacts);
    let contacts = move || {
        contacts.read(cx).map(|contacts| {
            // this data doesn't change frequently so we can use .map().collect() instead of a keyed <For/>
            contacts
                .into_iter()
                .map(|contact| {
                    view! { cx,
                        <li><A href=contact.id.to_string()><span>{&contact.first_name} " " {&contact.last_name}</span></A></li>
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    view! { cx,
        <div class="contact-list">
            <h1>"Contacts"</h1>
            <Suspense fallback=move || view! { cx,  <p>"Loading contacts..."</p> }>
                {move || view! { cx, <ul>{contacts}</ul>}}
            </Suspense>
            <AnimatedOutlet 
                class="outlet"
                outro="fadeOut"
                intro="fadeIn"
            />
        </div>
    }
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ContactParams {
    id: usize,
}

#[component]
pub fn Contact(cx: Scope) -> impl IntoView {
    log::debug!("rendering <Contact/>");

    log::debug!(
        "ExampleContext should be Some(42). It is {:?}",
        use_context::<Context>(cx)
    );

    on_cleanup(cx, || {
        log!("cleaning up <Contact/>");
    });

    let params = use_params::<ContactParams>(cx);
    let contact = create_resource(
        cx,
        move || params().map(|params| params.id).ok(),
        // any of the following would work (they're identical)
        // move |id| async move { get_contact(id).await }
        // move |id| get_contact(id),
        // get_contact
        get_contact,
    );

    create_effect(cx, move |_| {
        log!("params = {:#?}", params.get());
    });

    let contact_display = move || match contact.read(cx) {
        // None => loading, but will be caught by Suspense fallback
        // I'm only doing this explicitly for the example
        None => None,
        // Some(None) => has loaded and found no contact
        Some(None) => Some(
            view! { cx, <p>"No contact with this ID was found."</p> }
                .into_any(),
        ),
        // Some(Some) => has loaded and found a contact
        Some(Some(contact)) => Some(
            view! { cx,
                <section class="card">
                    <h1>{contact.first_name} " " {contact.last_name}</h1>
                    <p>{contact.address_1}<br/>{contact.address_2}</p>
                </section>
            }
            .into_any(),
        ),
    };

    view! { cx,
        <div class="contact">
            <Transition fallback=move || view! { cx,  <p>"Loading..."</p> }>
                {contact_display}
            </Transition>
        </div>
    }
}
