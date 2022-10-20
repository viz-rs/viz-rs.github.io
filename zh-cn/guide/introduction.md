# 简介

## 什么是 Viz？

**Viz** 是一款用于构建 Web 服务的 [Rust] 框架。它搭建在 [Tokio] 和 [Hyper] 之上，
并提供了一套灵活的路由、简洁的请求处理、健壮的中间件模型，帮助你高效地开发 Web 服务。

下面是一个最基本的示例：

```rust
use std::net::SocketAddr;
use viz::{Request, Result, Router, Server, ServiceMaker};

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello, World!")
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

```bash
> curl 127.0.0.1:3000
# Hello, World!
```

[Rust]: https://rust-lang.org/
[Tokio]: https://tokio.rs/
[Hyper]: https://hyper.rs/
