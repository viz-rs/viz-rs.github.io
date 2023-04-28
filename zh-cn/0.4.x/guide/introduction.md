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

## 为什么取名 Viz？

我们的愿景是用可视化的方式去构建应用。

## 关于 Logo

取自[围棋]棋盘的一角，点和线纵横交错，无线延伸；视觉和逻辑交织，充满想象；也像一条轻灵的鱼， 助我们的开发如鱼得水，体验流畅。

> 目前的设计还很朴素，不够醒目。

[rust]: https://rust-lang.org/
[tokio]: https://tokio.rs/
[hyper]: https://hyper.rs/
[围棋]: https://zh.wikipedia.org/wiki/%E5%9B%B4%E6%A3%8B 
