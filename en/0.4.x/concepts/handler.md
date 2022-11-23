# Handler

## Basic Trait

In Viz, we defined a basic trait [`Handler`] to handle the request, it has a
very simple interface, with only one input and one output. It has been
implemented internally for asynchronous functions, so asynchronous processing
can be easily constructed.

Here is [`Handler`]'s definition:

```rust
#[async_trait]
pub trait Handler<Input>: dyn_clone::DynClone + Send + Sync + 'static {
    type Output;

    #[must_use]
    async fn call(&self, input: Input) -> Self::Output;
}
```

### A simple asynchronous processing function

Where input is [`Request`], output is
`Result\<[Response](https://docs.rs/viz/0.4.x/viz/type.Response.html)\>`.

```rust
async fn index(_: Request) -> Result<Response> {
    Ok(Response::text("Hello, World!"))
}
```

### Custom Types

Construct request processing by implementing the [`Handler`] trait with a custom
type.

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

## Extension Trait

An extension trait [`HandlerExt`] for [`Handler`], which provides adapters for
chaining and composing handlers.

The following is a list of extension methods:

| Method                                          | Description                                                            |
| ----------------------------------------------- | ---------------------------------------------------------------------- |
| [`boxed`][method.boxed]                         | Wrap the `handler` in a box                                            |
| [`before`][method.before]                       | Process the input before executing the `handler`                       |
| [`after`][method.after]                         | Process the output after the execution of `handler`                    |
| [`around`][method.around]                       | Customize before and after processing when executing `handler`         |
| [`map`][method.map]                             | After executing `handler`, the correct result is processed             |
| [`map_into_response`][method.map_into_response] | After executing `handler`, convert the correct result to [`Response`]  |
| [`and_then`][method.and_then]                   | After executing `handler`, add an action to process the correct result |
| [`map_err`][method.map_err]                     | After executing `handler`, the error result is processed               |
| [`or_err`][method.or_err]                       | After executing `handler`, add an action to handle the error result    |
| [`catch_error`][method.catch_error]             | When executing `handler`, error messages are caught and processed      |
| [`catch_unwind`][method.catch_unwind]           | Catch error messages and block `panic` when executing `handler`        |
| [`with`][method.with]                           | Wrap `handler` with a type that implements the [`Transform`] trait     |
| [`with_fn`][method.with_fn]                     | Add an action to process or convert `handler`                          |

> Here is a brief introduction, which will be expanded in detail in the routing
> section.

[`handler`]: https://docs.rs/viz/0.4.x/viz/trait.Handler.html
[`handlerext`]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html
[`transform`]: https://docs.rs/viz/0.4.x/viz/trait.Transform.html
[`request`]: https://docs.rs/viz/0.4.x/viz/type.Request.html
[`response`]: https://docs.rs/viz/0.4.x/viz/type.Response.html
[method.boxed]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.boxed
[method.before]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.before
[method.after]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.after
[method.around]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.around
[method.map]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.map
[method.map_into_response]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.map_into_response
[method.and_then]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.and_then
[method.map_err]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.map_err
[method.or_err]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.or_err
[method.catch_error]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.catch_error
[method.catch_unwind]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.catch_unwind
[method.with]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.with
[method.with_fn]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.with_fn
