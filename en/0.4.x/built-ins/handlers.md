# Request Handling

There are some handy `handler` built into Viz, and you can use them by turning
on the corresponding features.

| Feature        | Description                       |
| -------------- | --------------------------------- |
| [`serve`]      | Serve static file and directory   |
| [`embed`]      | Embedded resource files           |
| [`prometheus`] | OpenTelemetry Prometheus Exporter |

## serve

Flexible servicing of individual files or directories is possible.

- [`serve::File`]：Serve specific file

- [`serve::Dir`]：Serve specific directory

Complete
[example](https://github.com/viz-rs/viz/tree/0.4.x/examples/static-files/serve).

## embed

Resource files can be packaged into executable files to generate a single file
for easy management of deployment.

- [`embed::File`]：Embed specific file

- [`embed::Dir`]：Embed specific directory

Complete
[example](https://github.com/viz-rs/viz/tree/0.4.x/examples/static-files/embed).

## prometheus

Current `metrics` information can be exported for collection by `Prometheus`.

```rust
use opentelemetry::{
    global,
    sdk::{
        export::metrics::aggregation,
        metrics::{controllers, processors, selectors},
    },
};

use viz::{
    handlers::prometheus::{ExporterBuilder, Prometheus},
};

let exporter = {
    let controller = controllers::basic(
        processors::factory(
            selectors::simple::histogram([1.0, 2.0, 5.0, 10.0, 20.0, 50.0]),
            aggregation::cumulative_temporality_selector(),
        )
        .with_memory(true),
    )
    .build();
    ExporterBuilder::new(controller).init()
};

let handler = Prometheus::new(exporter);

let app = Router::new()
    .get("/metrics", handler);
```

Complete
[example](https://github.com/viz-rs/viz/tree/0.4.x/examples/otel/metrics).

[`serve`]: https://docs.rs/viz/0.4.x/viz/handlers/serve/index.html
[`serve::file`]: https://docs.rs/viz/0.4.x/viz/handlers/serve/struct.File.html
[`serve::dir`]: https://docs.rs/viz/0.4.x/viz/handlers/serve/struct.Dir.html
[`embed`]: https://docs.rs/viz/0.4.x/viz/handlers/embed/index.html
[`embed::file`]: https://docs.rs/viz/0.4.x/viz/handlers/embed/struct.File.html
[`embed::dir`]: https://docs.rs/viz/0.4.x/viz/handlers/embed/struct.Dir.html
[`prometheus`]: https://docs.rs/viz/0.4.x/viz/handlers/prometheus/index.html
