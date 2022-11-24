# Request & Response

## Request

[`Request`]：Represents an HTTP request.

[`RequestExt`]： Provides some friendly and convenient methods to help us extract
data from [`Request`].

## Response

[`Response`]：Represents an HTTP response.

[`ResponseExt`]：Provides some friendly, convenient methods to construct [`Response`].

[`IntoResponse`]：Converts other types to [`Response`] type.

```rust
struct MyResponse {
    code: u16,
    data: Vec<u8>,
}

impl IntoResponse for MyResponse {
  fn into_response(self) -> Response {
      Response::builder()
          .status(StatusCode::from_u16(self.code).unwrap())
          .body(self.data.into())
          .unwrap()
  }
}
```

[`Request`]: https://docs.rs/http/0.2.8/http/request/struct.Request.html
[`RequestExt`]: https://docs.rs/viz/0.4.x/viz/trait.RequestExt.html
[`Response`]: https://docs.rs/http/0.2.8/http/request/struct.Response.html
[`ResponseExt`]: https://docs.rs/viz/0.4.x/viz/trait.ResponseExt.html
[`IntoResponse`]: https://docs.rs/viz/0.4.x/viz/trait.IntoResponse.html

