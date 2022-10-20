# 快速上手

## 创建一个 Cargo 项目

```bash
> cargo new app
```

## 安装 Viz 和 Tokio

```bash
> cd app
> cargo add viz
> cargo add tokio
```

## 编写 main

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

## 运行测试

```bash
> cargo run -- --nocaputre
```
