# 错误处理

先回顾 [`Handler`] 特征的定义：

```rust
#[async_trait]
pub trait Handler<Input>: dyn_clone::DynClone + Send + Sync + 'static {
    type Output;

    #[must_use]
    async fn call(&self, input: Input) -> Self::Output;
}
```

还有异步函数对 [`Handler`] 特征的实现：

```rust
#[async_trait]
impl<F, I, Fut, O> Handler<I> for F
where
    I: Send + 'static,
    F: Fn(I) -> Fut + ?Sized + Clone + Send + Sync + 'static,
    Fut: Future<Output = O> + Send,
{
    type Output = Fut::Output;

    async fn call(&self, i: I) -> Self::Output {
        (self)(i).await
    }
}
```

根据上面就可以声明一个真实的异步函数：

```rust
async fn index(_: Request) -> Result<Response> {
    Ok(Response::text("Hello, World!"))
}
```

或者（闭包函数）：

```rust
|req: Request| async { Ok("Hello".into_response()) }
```

其中 `Input = Request`，`Output = Result\<Response\>`。

对 `Result\<Response\>` 补全，
将是 `Result\<Response, [Error](https://docs.rs/viz/latest/viz/enum.Error.html)\>`

1. 为什么不直接返回 [`Response`]？

因为在实际场景中，需要判断文件、IO、DB 等的常规错误，所以返回 `Result\<Response\>` 是最合适的。
也能利用 `?` 操作符，尽早返回错误，响应给客户端。

2. 为什么不直接返回 `impl [IntoResponse](https://docs.rs/viz/latest/viz/trait.IntoResponse.htm)`？

虽然已经对 `Result\<T\>` 实现了 [`IntoResponse`] 特征，但有个特殊情况，如果 `T = Result\<R\>`，
在 [`fatten`] 未稳定的情况下，暂时还无法打平结果，因此无法正常返回。

3. 如何自定义错误及响应？

```rust
#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("Not Found")]
    NotFound,
}

impl From<MyError> for Error {
    fn from(e: MyError) -> Self {
        // Error::Responder(e.into_response())
        Error::Report(Box::new(e), MyError::NotFound.into_response())
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(self.to_string().into())
            .unwrap()
    }
}
```


[`Handler`]: https://docs.rs/viz/latest/viz/trait.Handler.html
[`IntoResponse`]: https://docs.rs/viz/latest/viz/trait.IntoResponse.html
[`Response`]: https://docs.rs/viz/latest/viz/struct.Response.html
[`fatten`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.flatten
