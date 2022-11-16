# 中间件

在 Viz 中，中间件模型跟请求处理共用一个特征 [`Handler`]，即简化了模型，也降低了开发难度。

## 基本函数

以下是一个最基本的中间件函数：

```rust
async fn middleware<H>((req, handler): Next<Request, H>) -> Result<Response>
where
    H: Handler<Request, Output = Result<Response>> + Clone,
{
    // 在执行 handler 之前，可以对 request 做些处理
    let resp = handler.call(req).await;
    // 在执行 handler 之后，可以对 response 做些处理
    resp
}
```

其中输入参数是一个包含两个元素的元组类型
`[Next<Request, H>](https://docs.rs/viz/latest/viz/type.Next.html)`：

```rust
pub type Next<I, H> = (I, H);
```

然后输出结果 `Result<[Response](https://docs.rs/viz/latest/viz/type.Response.html)>`
进行返回。

## 自定义类型

当然也可以通过定义新类型来构造中间件。

```rust
#[derive(Clone)]
struct MyMiddleware;

#[async_trait]
impl<H> Handler<Next<Request, H>> for MyMiddleware
where
    H: Handler<Request, Output = Result<Response>> + Clone,
{
    type Output = H::Output;

    async fn call(&self, (req, handler): Next<Request, H>) -> Self::Output {
        // 在执行 handler 之前，可以对 request 做些处理
        let resp = handler.call(req).await;
        // 在执行 handler 之后，可以对 response 做些处理
        resp
    }
}
```

> 可以打开 「请求处理」进行对比，它们非常相似，只是参数不同而已，`Handler` 节省了我们很多脑力。🦀️

[`handler`]: https://docs.rs/viz/latest/viz/trait.Handler.html

