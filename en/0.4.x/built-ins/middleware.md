# Middleware

There is also some convenient middleware built into Viz, and you can use it by
enabling the appropriate features.

| Feature                          | Description                             |
| -------------------------------- | --------------------------------------- |
| [cookie][m:cookie]               | Provides cookie management capabilities |
| [cors][m:cors]                   | Cross-resource sharing processing       |
| [csrf][m:csrf]                   | Cross-site request forgery processing   |
| [limits][m:limits]               | Request body size Limit                 |
| [session][m:session]             | Session processing                      |
| [compression][m:compression]     | Response body compression               |
| [otel::tracing][m:otel::tracing] | OpenTelemetry Tracing                   |
| [otel::metrics][m:otel::metrics] | OpenTelemetry Metrics                   |

## Cookie

```rust
use viz::middleware::cookie;

let app = Router::new()
    .with(cookie::Config::new());
```

Complete [example](https://github.com/viz-rs/viz/tree/0.4.x/examples/session).

## CORS

```rust
use viz::middleware::cors;

let app = Router::new()
    .with(cors::Config::default());
```

Complete [example](https://github.com/viz-rs/viz/tree/0.4.x/examples/cors).

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

Complete [example](https://github.com/viz-rs/viz/tree/0.4.x/examples/csrf).

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

Complete [example](https://github.com/viz-rs/viz/tree/0.4.x/examples/session).

## Compression

```rust
use viz::middleware::compression;

let app = Router::new()
    .with(compression::Config::default());
```

Complete
[example](https://github.com/viz-rs/viz/tree/0.4.x/examples/compression).

## OpenTelemetry Tracing

Complete
[example](https://github.com/viz-rs/viz/tree/0.4.x/examples/otel/tracing).

## OpenTelemetry Metrics

Complete
[example](https://github.com/viz-rs/viz/tree/0.4.x/examples/otel/metrics).

[m:cookie]: https://docs.rs/viz-core/0.4.x/viz_core/middleware/cookie
[m:cors]: https://docs.rs/viz-core/0.4.x/viz_core/middleware/cors
[m:csrf]: https://docs.rs/viz-core/0.4.x/viz_core/middleware/csrf
[m:limits]: https://docs.rs/viz-core/0.4.x/viz_core/middleware/limits
[m:session]: https://docs.rs/viz-core/0.4.x/viz_core/middleware/session
[m:compression]: https://docs.rs/viz-core/0.4.x/viz_core/middleware/compression
[m:otel::tracing]: https://docs.rs/viz-core/0.4.x/viz_core/middleware/otel/tracing
[m:otel::metrics]: https://docs.rs/viz-core/0.4.x/viz_core/middleware/otel/metrics
