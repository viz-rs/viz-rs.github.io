# 请求处理

在 Viz 中内置了一些方便的 `handler`，开启相应的特性，就可以使用它们。

| 特性           | 说明                              |
| -------------- | --------------------------------- |
| [`serve`]      | 静态文件和目录服务                |
| [`embed`]      | 嵌入资源文件                      |
| [`prometheus`] | OpenTelemetry Prometheus Exporter |

## serve

可以灵活的对单个文件或者目录进行服务。

- [`serve::File`]：服务指定文件

- [`serve::Dir`]：服务指定目录

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/static-files/serve)。

## embed

可以把资源文件打包进执行文件中，生成单一文件，方便管理部署。

- [`embed::File`]：嵌入指定文件

- [`embed::Dir`]：嵌入指定目录

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/static-files/embed)。

## prometheus

可以输出当前 `metrics` 信息，以供 `Prometheus` 收集。

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

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/otel/metrics)。

[`serve`]: https://docs.rs/viz/0.4.x/viz/handlers/serve/index.html
[`serve::file`]: https://docs.rs/viz/0.4.x/viz/handlers/serve/struct.File.html
[`serve::dir`]: https://docs.rs/viz/0.4.x/viz/handlers/serve/struct.Dir.html
[`embed`]: https://docs.rs/viz/0.4.x/viz/handlers/embed/index.html
[`embed::file`]: https://docs.rs/viz/0.4.x/viz/handlers/embed/struct.File.html
[`embed::dir`]: https://docs.rs/viz/0.4.x/viz/handlers/embed/struct.Dir.html
[`prometheus`]: https://docs.rs/viz/0.4.x/viz/handlers/prometheus/index.html
