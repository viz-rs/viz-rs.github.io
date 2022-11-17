# 中间件

在 Viz 中同时也内置了一些方便的中间件，开启相应的特性，就可以使用它们。

| 特性                             | 说明                  |
| -------------------------------- | --------------------- |
| [cookie][m:cookie]               | 提供 Cooke 管理功能   |
| [cors][m:cors]                   | 跨资源共享处理        |
| [csrf][m:csrf]                   | 跨站请求伪造处理      |
| [limits][m:limits]               | 请求主体大小限制      |
| [session][m:session]             | 会话处理              |
| [compression][m:compression]     | 响应主体压缩          |
| [otel::tracing][m:otel::tracing] | OpenTelemetry Tracing |
| [otel::metrics][m:otel::metrics] | OpenTelemetry Metrics |

## Cookie

```rust
use viz::middleware::cookie;

let app = Router::new()
    .with(cookie::Config::new());
```

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/session)。

## CORS

```rust
use viz::middleware::cors;

let app = Router::new()
    .with(cors::Config::default());
```

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/cors)。

## CSRF

```rust
use viz::{
    middleware::{
        csrf::{self, CsrfToken},
        helper::CookieOptions,
    },
};

let app = Router::new()
    .with(csrf::Config::new(
        csrf::Store::Cookie,
        [Method::GET, Method::HEAD, Method::OPTIONS, Method::TRACE].into(),
        CookieOptions::new("_csrf").max_age(Duration::from_secs(3600 * 24)),
        csrf::secret,
        csrf::generate,
        csrf::verify,
    ));
```

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/csrf)。

## Limits

```rust
use viz::middleware::limits;

let app = Router::new()
    .with(limits::Config::default());
```

## Session

```rust
use sessions::MemoryStorage;
use viz::middleware::{helper::CookieOptions, session::{self, Store}};

let app = Router::new()
    .with(session::Config::new(
        Store::new(MemoryStorage::new(), nano_id::base64::<32>, |sid: &str| {
            sid.len() == 32
        }),
        CookieOptions::default(),
    ));
```

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/session)。

## Compression

```rust
use viz::middleware::compression;

let app = Router::new()
    .with(compression::Config::default());
```

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/compression)。

## OpenTelemetry Tracing

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/otel/tracing)。

## OpenTelemetry Metrics

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/otel/metrics)。

[m:cookie]: https://docs.rs/viz-core/latest/viz_core/middleware/cookie
[m:cors]: https://docs.rs/viz-core/latest/viz_core/middleware/cors
[m:csrf]: https://docs.rs/viz-core/latest/viz_core/middleware/csrf
[m:limits]: https://docs.rs/viz-core/latest/viz_core/middleware/limits
[m:session]: https://docs.rs/viz-core/latest/viz_core/middleware/session
[m:compression]: https://docs.rs/viz-core/latest/viz_core/middleware/compression
[m:otel::tracing]: https://docs.rs/viz-core/latest/viz_core/middleware/otel/tracing
[m:otel::metrics]: https://docs.rs/viz-core/latest/viz_core/middleware/otel/metrics
