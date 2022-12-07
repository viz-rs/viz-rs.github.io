# 服务

By configuring the [`Server`], you can then quickly start a service.

Before it can be fully started, the [`Router`] instance needs to be converted to
a [`ServiceMaker`] instance that implements the [`Service`] trait.

```rust
async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}

let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

// Create Router instances
let app = Router::new().get("/", index);

// Create a Server instance and bind the port
if let Err(err) = Server::bind(&addr)
    // Converting to a ServiceMaker Instance
    .serve(ServiceMaker::from(app))
    .await
{
    println!("{err}");
}
```

## TCP/HTTP

Enable the `http1` or `http2` feature, the default `http1` is already enabled.

## Unix Sock Domain

Enabling the `unix-socket` feature usually requires an inverse proxy (e.g.,
Nginx Proxy).

[`server`]: https://docs.rs/viz/0.4.x/viz/struct.Server.html
[`router`]: https://docs.rs/viz/0.4.x/viz/struct.Router.html
[`servicemaker`]: https://docs.rs/viz/0.4.x/viz/struct.ServiceMaker.html
[`service`]: https://docs.rs/hyper/0.14/hyper/service/index.html
