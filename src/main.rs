use viz::prelude::*;
use viz::utils::{log, pretty_env_logger};

const NOT_FOUND: &str = "404 - This is not the web page you are looking for.";
const INDEX: &str = include_str!("../public/index.html");

async fn logger(cx: &mut Context) -> Result<Response> {
    log::info!("IN  Mid: {} {}", cx.method(), cx.path());

    let fut = cx.next().await;

    log::info!("OUT Mid: {} {}", cx.method(), cx.path());

    fut.map(|mut res| {
        if res.status() == http::StatusCode::NOT_FOUND {
            *res.body_mut() = NOT_FOUND.into();
        }

        res.headers_mut().insert(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("text/html; charset=utf-8"),
        );

        res
    })
}

async fn not_found() -> http::StatusCode {
    http::StatusCode::NOT_FOUND
}

async fn hello_world() -> &'static str {
    INDEX
}

#[tokio::main]
async fn main() -> Result {
    pretty_env_logger::init();

    let mut app = viz::new();

    app.routes(
        router()
            .mid(logger)
            .at("/", route().all(hello_world))
            .at("/*", route().all(not_found)),
    );

    app.listen("127.0.0.1:8000").await
}
