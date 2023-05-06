# viz.rs

Built with [Leptos] · Deploys on [Cloudflare].

* English: https://viz.rs

* 简体中文: https://zh-cn.viz.rs

## Development

1. Install [`cargo-make`]

```bash
$ cargo install --force cargo-make
```

2. Install [`trunk`]

> **Note**: The latest **master** branch includes the **copy-dir** feature.

```bash
$ cargo install --git https://github.com/thedodd/trunk
```

3. Serve `en`, `zh-cn` or other language

```bash
$ cargo make dev-en
```

4. Build

```bash
$ cargo make build-en
```

[Leptos]: https://github.com/leptos-rs/leptos
[Cloudflare]: https://www.cloudflare.com
[`cargo-make`]: https://github.com/sagiegurari/cargo-make
[`trunk`]: https://github.com/thedodd/trunk
