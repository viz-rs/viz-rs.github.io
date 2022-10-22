# 请求处理

## 基础特征

在 Viz 中，定义了一个特征 [`Handler`] 来处理请求，它的接口非常简单，只有一个输入和一个输出，
内部也对异步函数实现了该特征，所以可以轻松地构建异步处理。

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

其中输入 [`Request`]，输出 Result<[`Response`]>。

```rust
async fn index(_: Request) -> Result<Response> {
    Ok(Response::text("Hello, World!"))
}
```

### 支持自定义类型

通过自定义类型，实现 [`Handler`] 特征来构造处理请求。

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
        Ok(format!("method = {}, path = {}, count = {}", method, path, count).into_response())
    }
}
```

## 扩展特征

[`HandlerExt`] 是对 [`Handler`] 的扩展，它提供了方便的适配器，通过链式和组合的方式去构建请求函数。

[`Handler`]: https://docs.rs/viz/latest/viz/trait.Handler.html
[`HandlerExt`]: https://docs.rs/viz/latest/viz/trait.HandlerExt.html
[`Request`]: https://docs.rs/viz/latest/viz/type.Request.html
[`Response`]: https://docs.rs/viz/latest/viz/type.Response.html
