use gloo_net::http::Request;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Params, PartialEq, Clone, Debug)]
pub struct DocParams {
    pub version: Option<String>,
    pub path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub text: String,
    pub prefix: String,
    pub items: Vec<(String, String)>,
}

pub async fn fetch_toc(version: String) -> Option<Vec<Section>> {
    let mut url = String::new();
    url.push_str("/docs/");
    url.push_str(&version);
    url.push_str("/toc.json");
    Request::get(&url).send().await.ok()?.json().await.ok()
}

pub async fn fetch_page(version: String, path: String) -> Option<String> {
    let mut url = String::new();
    url.push_str("/docs/");
    url.push_str(&version);
    url.push('/');
    url.push_str(&path);
    url.push_str(".html");
    Request::get(&url).send().await.ok()?.text().await.ok()
}
