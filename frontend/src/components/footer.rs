use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <footer class="footer text-center text-neutral-400 text-sm p-5">
                <p>
                    {"Built with "}<a href="https://yew.rs/" target="_blank" class="text-neutral-500">{"Yew"}</a>
                    {" · Deploys on "}<a href="https://www.cloudflare.com/" target="_blank" class="text-neutral-500">{"Cloudflare"}</a>
                </p>
                <p>
                    {"MIT Licensed | Copyright © 2022 Fangdun Tsai"}
                </p>
            </footer>
        }
    }
}
