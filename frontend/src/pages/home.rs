use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

pub struct Home {}
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="w-full hero text-center px-3 py-3 sm:py-19">
                <h1 class="text-8 sm:text-10">
                    {"Fast, robust, flexible, lightweight web framework for Rust"}
                </h1>
                <p class="text-4 sm:text-5 mt-4.5 mb-7.5 sm:mt-6 sm:mb-8 op-61.8">
                    {"Viz is built on top of hyper and tokio."}
                </p>
                <Link<Route> classes="inline-block bg-neutral-900 text-neutral-100 dark:bg-neutral-100 dark:text-neutral-900 shadow py-2 px-4.5 border-rounded font-medium text-4 get-started" to={Route::Document { path: "guide/introduction".to_string()  }}>{ "Get Started" }</Link<Route>>
            </section>
        }
    }
}
