# Error Handling

Recall first the definition of the [`Handler`] trait:

```rust
#[async_trait]
pub trait Handler<Input>: dyn_clone::DynClone + Send + Sync + 'static {
    type Output;

    #[must_use]
    async fn call(&self, input: Input) -> Self::Output;
}
```

And the implementation of the [`Handler`] trait by asynchronous functions:

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

A real asynchronous function can be declared according to the above:

```rust
async fn index(_: Request) -> Result<Response> {
    Ok(Response::text("Hello, World!"))
}
```

Or (closure function):

```rust
|req: Request| async { Ok("Hello".into_response()) }
```

Which `Input = Request`，`Output = Result\<Response\>`.

Completing `Result\<Response\>` would be
`Result\<Response, [Error](https://docs.rs/viz/0.4.x/viz/enum.Error.html)\>`

1. Why not just return [`Response`]?

Since in a real-world scenario, you need to determine regular errors in files,
IO, DB, etc., returning `Result\<Response\>` is the most appropriate. It is also
possible to use `?` operator to return an error early and respond to the client.

2. Why not just return
   `impl [IntoResponse](https://docs.rs/viz/0.4.x/viz/trait.IntoResponse.html)`?

Although the [`IntoResponse`] feature has been implemented for `Result\<T\>`,
there is a special case if `T = Result\<R\>`, in which [`fatten`] is not stable,
the result cannot be tied yet, so it cannot be returned properly.

3. How to customize the error and response?

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

[`handler`]: https://docs.rs/viz/0.4.x/viz/trait.Handler.html
[`intoresponse`]: https://docs.rs/viz/0.4.x/viz/trait.IntoResponse.html
[`response`]: https://docs.rs/viz/0.4.x/viz/struct.Response.html
[`fatten`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.flatten
