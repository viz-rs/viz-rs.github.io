# Extractors

The purpose of the extractor is to extract data from HTTP requests.

## Interface definition

```rust
#[async_trait]
pub trait FromRequest: Sized {
    /// Return error message when extraction fails
    type Error: IntoResponse;

    /// Extracts data from an HTTP request and returns that type
    #[must_use]
    async fn extract(req: &mut Request) -> Result<Self, Self::Error>;
}
```

## Extraction functions

Information is extracted via the [`extract`] function.

```rust
async fn show_user(mut req: Request) -> Result<Resposne> {
  let id = req.extract::<Params<u32>>().await?;
  Ok(Resposne::text(format!("id: {id}")))
}
```

## Custom Types

The [`FromRequest`] feature is implemented to extract information by defining
new types.

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

[`extract`]: https://docs.rs/viz/0.4.x/viz/trait.RequestExt.html#tymethod.extract
[`fromrequest`]: https://docs.rs/viz/0.4.x/viz/trait.FromRequest.html
