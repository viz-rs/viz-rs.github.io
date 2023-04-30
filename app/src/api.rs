use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

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
