# 请求处理

## 基础特征

在 Viz 中，定义了一个基础特征 [`Handler`] 来处理请求，它的接口非常简单，只有一个输入和一个输出，
内部已对异步函数实现了该特征，所以可以轻松地构建异步处理。

下面是 [`Handler`] 的定义：

```rust
#[async_trait]
pub trait Handler<Input>: dyn_clone::DynClone + Send + Sync + 'static {
    type Output;

    #[must_use]
    async fn call(&self, input: Input) -> Self::Output;
}
```

### 一个简单的异步处理函数

其中输入 [`Request`]，输出 `Result\<[Response](https://docs.rs/viz/latest/viz/type.Response.html)\>`。

```rust
async fn index(_: Request) -> Result<Response> {
    Ok(Response::text("Hello, World!"))
}
```

### 支持自定义类型

通过自定义类型，实现 [`Handler`] 特征来构造请求处理。

```rust
#[derive(Clone)]
struct MyHandler {
  count: Arc<AtomicUsize>,
}

impl Handler<Request> for MyHandler {
    type Output = Result<Response>;

    async fn call(&self, req: Request) -> Self::Output {
        let path = req.path().clone();
        let method = req.method().clone();
        let count = self.count.fetch_add(1, Ordering::SeqCst);
        Ok(format!("method = {method}, path = {path}, count = {count}").into_response())
    }
}
```

## 扩展特征

[`HandlerExt`] 是对 [`Handler`] 的扩展，提供了各种方便的适配器，通过链式和组合的方式去构造请求处理。

以下是扩展方法列表：

| 方法                                            | 说明                                               |
| ----------------------------------------------- | -------------------------------------------------- |
| [`boxed`][method.boxed]                         | 包裹 `handler`，将它存储在堆上                     |
| [`before`][method.before]                       | 在执行 `handler` 前，对输入进行处理                |
| [`after`][method.after]                         | 在执行 `handler` 后，对输出进行处理                |
| [`around`][method.around]                       | 在执行 `handler` 时，自定义前置、后置处理          |
| [`map`][method.map]                             | 在执行 `handler` 后，对正确结果进行处理            |
| [`map_into_response`][method.map_into_response] | 在执行 `handler` 后，把正确结果转换成 [`Response`] |
| [`and_then`][method.and_then]                   | 在执行 `handler` 后，添加一个操作来处理正确结果    |
| [`map_err`][method.map_err]                     | 在执行 `handler` 后，对错误结果进行处理            |
| [`or_err`][method.or_err]                       | 在执行 `handler` 后，添加一个操作来处理错误结果    |
| [`catch_error`][method.catch_error]             | 在执行 `handler` 时，捕获错误信息并处理            |
| [`catch_unwind`][method.catch_unwind]           | 在执行 `handler` 时，捕获错误信息并阻止 `panic`    |
| [`with`][method.with]                           | 用一个实现 [`Transform`] 特征的类型包裹 `handler`  |
| [`with_fn`][method.with_fn]                     | 添加一个操作，对 `handler` 进行处理或者转换        |

> 这里先简单介绍下，在路由章节会详细展开

[`handler`]: https://docs.rs/viz/latest/viz/trait.Handler.html
[`handlerext`]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html
[`transform`]: https://docs.rs/viz/latest/viz/trait.Transform.html
[`request`]: https://docs.rs/viz/latest/viz/type.Request.html
[`response`]: https://docs.rs/viz/latest/viz/type.Response.html
[method.boxed]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.boxed
[method.before]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.before
[method.after]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.after
[method.around]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.around
[method.map]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.map
[method.map_into_response]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.map_into_response
[method.and_then]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.and_then
[method.map_err]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.map_err
[method.or_err]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.or_err
[method.catch_error]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.catch_error
[method.catch_unwind]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.catch_unwind
[method.with]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.with
[method.with_fn]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html#method.with_fn
