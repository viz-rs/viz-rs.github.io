use std::rc::Rc;

use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{Route, Section};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub sections: Rc<Vec<Section>>,
}

pub struct Sidebar {}

impl Component for Sidebar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let location = ctx.link().location().expect_throw("Can't find location");

        html! {
            <aside class="fixed flex flex-col p-5 gap-4 sidebar top-4.375rem bottom-0">
                { self.view_sections(ctx.props().sections.to_vec(), location.path()) }
            </aside>
        }
    }
}

impl Sidebar {
    fn view_sections(&self, sections: Vec<Section>, path: &str) -> Html {
        html! {
            {
                sections.into_iter()
                    .map(|section| html! {
                        <section key={section.text.to_string()}>
                            <h3 class="py-1 text-4 font-medium">
                                {section.text}
                            </h3>
                            <ul class="text-3.5">
                                {self.view_list(section.items, section.prefix, path)}
                            </ul>
                        </section>
                    })
                    .collect::<Html>()
            }
        }
    }

    fn view_list(&self, list: Vec<(String, String)>, prefix: String, path: &str) -> Html {
        let cs = "inline-block py-1 font-normal transition-colors hover:op100";

        html! {
            {
                list.into_iter()
                    .map(|item| {
                        let p = format!("{}{}", prefix.clone(), item.1);
                        let a = if path.ends_with(p.as_str()) { "op100 text-yellow-600" } else { "op61.8" };
                        html! {
                            <li key={p.to_string()}>
                                <Link<Route>
                                    classes={classes!(
                                        cs.to_string(),
                                        a
                                    )}
                                    to={Route::Document { path: p.to_string()}}
                                >
                                    {item.0}
                                </Link<Route>>
                            </li>
                        }
                    })
                    .collect::<Html>()
            }
        }
    }
}
