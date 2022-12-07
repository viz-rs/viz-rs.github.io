# TLS

Generally, when `TLS` encryption is enabled, there are 2 ways to achieve it.

1. Configure on the reverse proxy server

2. Integration in the program

There are currently 2 `features` to choose from:

| Feature                    | Description                       |
| -------------------------- | --------------------------------- |
| [rustls][v:rustls]         | Based on the [rustls] library     |
| [native-tls][v:native-tls] | Based on the [native-tls] library |

[v:rustls]: https://docs.rs/viz/0.4.x/viz/tls/rustls/index.html
[v:native-tls]: https://docs.rs/viz/0.4.x/viz/tls/rustls/index.html
[rustls]: https://docs.rs/rustls
[native-tls]: https://docs.rs/native-tls

Complete [example](https://github.com/viz-rs/viz/tree/0.4.x/examples/rustls).
