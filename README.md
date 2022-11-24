# viz.rs

* English: https://viz.rs

* 简体中文: https://zh-cn.viz.rs

## Development

1. Install [`just`]

```bash
$ cargo install --locked just
```

2. Install [`trunk`]

```bash
$ cargo install --locked trunk
```

3. List just cmds

```bash
$ just -l
Available recipes:
    build-en
    build-zh-cn
    en
    serve-en
    serve-zh-cn
    zh-cn
```

4. Serve `en`, `zh-cn` or other language

```bash
$ just serve-en
```

5. Build

```bash
$ just build-en
```

[`just`]: https://github.com/casey/just
[`trunk`]: https://github.com/thedodd/trunk
