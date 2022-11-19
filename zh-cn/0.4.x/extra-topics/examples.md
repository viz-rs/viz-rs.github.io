# 示例

## 列表

* [Hello world][hello-world]
* [Unix socket domain][unix-socket]
* [Static file serving and directory listing][static-files/serve]
* [Static files serving and embedding][static-files/embed]
* [Extract body from Form][forms/form]
* [Extract body from Multipart][forms/multipart]
* [Websockt Chat][websocket-chat]
* [Server-Sent Events][sse]
* [Session][session]
* [CSRF][csrf]
* [CORS][cors]
* [Compression response body][compression]
* [HTTPS/TLS - rustls][rustls]
* [Defined a static router][static-routes]
* [Todos][routing/todos]
* [Integration Opentelemetry(OTEL)](https://github.com/open-telemetry/opentelemetry-rust)
  * [Tracing][otel/tracing]
  * [Metrics & Prometheus][otel/metrics]

## 运行与测试

### 运行

```bash
$ cargo run --bin hello-world -- --nocapture
```

### 测试

```bash
$ curl http://127.0.0.1:3000
```

[hello-world]: https://github.com/viz-rs/viz/blob/0.4.x/examples/hello-world
[unix-socket]: https://github.com/viz-rs/viz/blob/0.4.x/examples/unix-socket
[static-files/serve]: https://github.com/viz-rs/viz/blob/0.4.x/examples/static-files/serve
[static-files/embed]: https://github.com/viz-rs/viz/blob/0.4.x/examples/static-files/embed
[forms/form]: https://github.com/viz-rs/viz/blob/0.4.x/examples/forms/form
[forms/multipart]: https://github.com/viz-rs/viz/blob/0.4.x/examples/forms/multipart
[websocket-chat]: https://github.com/viz-rs/viz/blob/0.4.x/examples/websocket-chat
[sse]: https://github.com/viz-rs/viz/blob/0.4.x/examples/sse
[session]: https://github.com/viz-rs/viz/blob/0.4.x/examples/session
[csrf]: https://github.com/viz-rs/viz/blob/0.4.x/examples/csrf
[cors]: https://github.com/viz-rs/viz/blob/0.4.x/examples/cors
[compression]: https://github.com/viz-rs/viz/blob/0.4.x/examples/compresssion 
[rustls]: https://github.com/viz-rs/viz/blob/0.4.x/examples/rustls
[static-routes]: https://github.com/viz-rs/viz/blob/0.4.x/examples/static-routes
[routing/todos]: https://github.com/viz-rs/viz/blob/0.4.x/examples/routing/todos
[otel/tracing]: https://github.com/viz-rs/viz/blob/0.4.x/examples/otel/tracing
[otel/metrics]: https://github.com/viz-rs/viz/blob/0.4.x/examples/otel/metrics
