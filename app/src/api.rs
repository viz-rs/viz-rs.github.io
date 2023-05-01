use gloo_net::http::Request;
use leptos::Params;
use leptos_router::{IntoParam, Params};
use serde::{Deserialize, Serialize};

#[derive(Params, PartialEq, Clone, Debug)]
pub struct DocParams {
    pub version: String,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub text: String,
    pub prefix: String,
    pub items: Vec<(String, String)>,
}

pub async fn fetch_toc(version: String) -> Option<Vec<Section>> {
    let mut url = String::new();
    url.push_str("/assets/");
    url.push_str(&version);
    url.push_str("/toc.json");
    Request::new(&url).send().await.ok()?.json().await.ok()
}

pub async fn fetch_page(version: String, path: String) -> Option<String> {
    let mut url = String::new();
    url.push_str("/assets/");
    url.push_str(&version);
    url.push('/');
    url.push_str(&path);
    url.push_str(".html");
    Request::new(&url).send().await.ok()?.text().await.ok()
}
