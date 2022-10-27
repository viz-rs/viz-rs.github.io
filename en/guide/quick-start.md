# Quick Start

## Create a cargo project

```bash
> cargo new app
```

## Install Viz and Tokio

```bash
> cd app
> cargo add viz
> cargo add tokio
```

## Edit main

```rust
use std::net::SocketAddr;
use viz::{Request, Result, Router, Server, ServiceMaker};

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello Viz")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");

    let app = Router::new().get("/", index);

    if let Err(err) = Server::bind(&addr)
        .serve(ServiceMaker::from(app))
        .await
    {
        println!("{err}");
    }

    Ok(())
}
```

## Run and test

```bash
> cargo run -- --nocaputre
```
