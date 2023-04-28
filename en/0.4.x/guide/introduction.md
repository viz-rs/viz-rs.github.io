# Introduction

## What is Viz?

**Viz** is a Rust framework for building Web services. It builts on top of
[Tokio] and [Hyper], provides a set of flexible routing, clean request
processing, and a robust middleware model to help you develop Web services
efficiently.

Here is a minimal example:

```rust
use std::net::SocketAddr;
use viz::{Request, Result, Router, Server, ServiceMaker};

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello, World!")
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

```bash
> curl 127.0.0.1:3000
# Hello, World!
```

## Why name it Viz?

Our vision is to build applications in a visual way.

## About logo

Take the corner of a [Go] board.

Dots and lines are intertwined and extend wirelessly,
Visual and logical interweaving, full of imagination.

It is also like a light fish, helping us to develop like a fish in water and experience smoothly.

> The current design is still very plain and not eye-catching enough.

[rust]: https://rust-lang.org/
[tokio]: https://tokio.rs/
[hyper]: https://hyper.rs/
[go]: https://en.wikipedia.org/wiki/Go_(game)
