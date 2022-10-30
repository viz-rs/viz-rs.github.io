# 提取器

提取器的主要用途是提取请求中的数据。

## 接口定义

```rust
#[async_trait]
pub trait FromRequest: Sized {
    /// 当提取失败时，返回错误信息
    type Error: IntoResponse;

    /// 从 HTTP [`Request`] 提取数据，返回该类型
    #[must_use]
    async fn extract(req: &mut Request) -> Result<Self, Self::Error>;
}
```

## 提取函数

通过 [`extract`] 函数提取信息

```rust
async fn show_user(mut req: Request) -> Result<Resposne> {
  let id = req.extract::<Params<u32>>().await?;
  Ok(Resposne::text(format!("id: {id}")))
}
```

## 自定义类型

```rust
#[derive(Debug)]
struct RequestInfo {
    pub method: String,
    pub path: String,
}

#[async_trait]
impl<T> FromRequest for RequestInfo
{
    type Error = Infallible;

    async fn extract(req: &mut Request) -> Result<Self, Self::Error> {
        Ok(RequestInfo {
            method: req.method().as_str().to_string(),
            path: req.path().to_string(),
        })
    }
}

async fn show_info(mut req: Request) -> Result<Resposne> {
  let req_info = req.extract::<RequestInfo>().await?;
  Ok(Resposne::text(format!("info: {req_info:#?}")))
}
```

[`extract`]: https://docs.rs/viz/latest/viz/trait.RequestExt.html#tymethod.extract
