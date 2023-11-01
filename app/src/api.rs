use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub text: String,
    pub prefix: String,
    pub items: Vec<(String, String)>,
}

pub async fn fetch_toc((lang, version): (String, String)) -> Option<Vec<Section>> {
    let mut url = String::new();
    url.push_str("/docs/");
    url.push_str(&lang);
    url.push('/');
    url.push_str(&version);
    url.push_str("/toc.json");
    Request::get(&url).send().await.ok()?.json().await.ok()
}

pub async fn fetch_doc(lang: &str, version: &str, tail: &str) -> Option<String> {
    let mut url = String::new();
    url.push_str("/docs/");
    url.push_str(lang);
    url.push('/');
    url.push_str(version);
    url.push('/');
    url.push_str(tail);
    url.push_str(".html");
    let req = Request::get(&url).send().await.ok()?;

    if !req.ok() {
        return None;
    }

    req.text()
        .await
        .ok()
        .filter(|body| body.starts_with("<article"))
}
