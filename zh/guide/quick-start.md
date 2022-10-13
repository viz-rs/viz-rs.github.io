# 快速上手

## Cargo

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
viz = { version = "0.4.2" }

tokio = { version = "1.0", features = [
  "rt-multi-thread",
  "macros",
] }
```

## Main

```rust
use std::net::SocketAddr;
use viz::{Request, Result, Router, Server, ServiceMaker};

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello Viz")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let app = Router::new().get("/", index);

    if let Err(err) = Server::bind(&addr)
        .serve(ServiceMaker::from(app))
        .await
    {
        println!("{}", err);
    }

    Ok(())
}
```
