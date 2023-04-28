# Middleware

In Viz, the middleware model and request processing share a common trait
[`Handler`], which simplifies the model and reduces development effort.

## Basic functions

The following is a minimal middleware function:

```rust
async fn middleware<H>((req, handler): Next<Request, H>) -> Result<Response>
where
    H: Handler<Request, Output = Result<Response>> + Clone,
{
    // before executing the handler, you can do some processing on the request
    let resp = handler.call(req).await;
    // after the handler is executed, something can be done with the response
    resp
}
```

Where the input parameter is a tuple type containing two elements
`[Next<Request, H>](https://docs.rs/viz/0.4.x/viz/type.Next.html)`：

```rust
pub type Next<I, H> = (I, H);
```

The output is then returned
`Result<[Response](https://docs.rs/viz/0.4.x/viz/type.Response.html)>`.

## Custom Types

Of course it is also possible to construct middleware by defining new types.

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
        // before executing the handler, you can do some processing on the request
        let resp = handler.call(req).await;
        // after the handler is executed, something can be done with the response
        resp
    }
}
```

> You can open "handler" for comparison, they are very similar, just with
> different parameters, `Handler` saves us a lot of brain power. 🦀️

[`handler`]: https://docs.rs/viz/0.4.x/viz/trait.Handler.html
