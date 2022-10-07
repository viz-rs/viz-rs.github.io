use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub path: String,
}

pub struct Document {
}
impl Component for Document {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <article class="flex flex-1">
                    {ctx.props().path.to_string()}
                </article>
                <nav class="sticky flex-col flex-[0_0_15rem] p-5 gap-5 hidden lg:flex">
                    <ul class="text-3">
                        <li>
                            <a class="block py-1 font-normal transition-colors op75 hover:op100" href="/docs/introduction">
                                {"Defining attributes"}
                            </a>
                        </li>
                        <li>
                            <a class="block py-1 font-normal transition-colors op75 hover:op100" href="/docs/quick-start">
                                {"Create a custom attribute"}
                            </a>
                        </li>
                    </ul>
                </nav>
            </>
        }
    }
}
