# 请求及响应

## 请求

[`Request`]：类型，定义了 HTTP 请求结构。

[`RequestExt`]：特征，提供了一些友好、便捷的方法，帮助我们从 [`Request`] 中提取数据。

## 响应

[`Response`]：类型，定义了 HTTP 响应结构。

[`ResponseExt`]：特征，提供了一些友好、便捷的方法来构造 [`Response`]。

[`IntoResponse`]：特征，使其他类型转换成 [`Response`] 类型。

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
[`RequestExt`]: https://docs.rs/viz/latest/viz/trait.RequestExt.html
[`response`]: https://docs.rs/http/0.2.8/http/request/struct.Response.html
[`responseext`]: https://docs.rs/viz/latest/viz/trait.ResponseExt.html
[`intoresponse`]: https://docs.rs/viz/latest/viz/trait.IntoResponse.html
