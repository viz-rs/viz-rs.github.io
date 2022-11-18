# TLS

一般启用 `TLS` 加密，会有 2 种方式实现：

1. 在反向代理服务器上进行配置

2. 在程序中集成

目前提供 2 个 `feature` 供选择：

| 特性                       | 说明                 |
| -------------------------- | -------------------- |
| [rustls][v:rustls]         | 基于 [rustls] 库     |
| [native-tls][v:native-tls] | 基于 [native-tls] 库 |

[v:rustls]: https://docs.rs/viz/0.4.x/viz/tls/rustls/index.html
[v:native-tls]: https://docs.rs/viz/0.4.x/viz/tls/rustls/index.html
[rustls]: https://docs.rs/rustls
[native-tls]: https://docs.rs/native-tls

完整[示例](https://github.com/viz-rs/viz/tree/0.4.x/examples/rustls)。
