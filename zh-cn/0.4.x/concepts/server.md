# 服务

通过对 [`Server`] 进行配置，然后就可以快速地启动一个服务。

在完全启动前，需要把 [`Router`] 实例转换成实现 [`Service`] 特性的 [`ServiceMaker`] 实例。

```rust
async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}

let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

// 创建 Router 实例
let app = Router::new().get("/", index);

// 创建 Server 实例，绑定端口
if let Err(err) = Server::bind(&addr)
    // 转换成 ServiceMaker 实例
    .serve(ServiceMaker::from(app))
    .await
{
    println!("{err}");
}
```

## TCP/HTTP 

开启 `http1` 或 `http2` 特性，默认 `http1` 已经开启。

## Unix Sock Domain

开启 `unix-socket` 特性，一般需要配合一个反代（例如：Nginx Proxy）。

[`Server`]: https://docs.rs/viz/latest/viz/struct.Server.html
[`Router`]: https://docs.rs/viz/latest/viz/struct.Router.html
[`ServiceMaker`]: https://docs.rs/viz/latest/viz/struct.ServiceMaker.html
[`Service`]: https://docs.rs/hyper/0.14/hyper/service/index.html
