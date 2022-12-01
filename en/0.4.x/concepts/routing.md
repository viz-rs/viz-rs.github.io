# Routing

Routing is the traffic hub for the entire web application, acting as a backbone
for connecting views, controls and data.

In Viz's routing system, the [`path-tree`] module is used to store the
relationship between `path` and `handler`.

At its core, [`path-tree`] is a [trie] implementation that provides an efficient
search engine and some friendly helper methods.

## Structures and concepts

- [`Route`]: basic structure, which is the set of [`Method`] - [`Handler`]
  pairs.

- [`Resources`]: resource structure, which performs CRUD operations by mapping
  between `Action` - [`Method`].

- [`Router`]: It is the management and maintenance of [`Route`], [`Resources`],
  nested routes, middleware.

## Path parameters

Thanks to the support provided by [`path-tree`], it is possible to name
parameters in a path by `:` and `?` `\+` `\*` sets the type of the parameter.

| Pattern                             | Rule                                                                           |
| ----------------------------------- | ------------------------------------------------------------------------------ |
| `:name`                             | Matches a path piece, excludes `/`                                             |
| `:name?`                            | Matches an optional path piece, excludes `/`                                   |
| `/:name?/` `/:name?`                | Matches an optional path segment, excludes `/`, prefix or suffix should be `/` |
| `\+` `:name\+`                      | Matches a path piece, includes `/`                                             |
| `\*` `:name\*`                      | Matches an optional path piece, includes `/`                                   |
| `/\*` `/\*/` `/:name\*/` `/:name\*` | Matches zero or more path segments, prefix or suffix should be `/`             |

The parameters can be extracted later with the [`Params\<T\>`] extractor.

> Also supported are **continuous parameters** and special characters
> interspersed between parameters for segmentation.

## Method routing

Add routes using the HTTP method. The following methods are supported:

- [`get`][router.get]
- [`post`][router.post]
- [`put`][router.put]
- [`delete`][router.delete]
- [`head`][router.head]
- [`options`][router.options]
- [`connect`][router.connect]
- [`patch`][router.patch]
- [`trace`][router.trace]

```rust
let routes = Router::new()
  .get("/about", |_: Request| async { Ok(Response::text("About")) });
```

## Resources routing

Add routes using the agreed actions. The following actions are supported:

| HTTP Method | Rule                  | Action                                             | Used for                                        |
| ----------- | --------------------- | -------------------------------------------------- | ----------------------------------------------- |
| GET         | `/resources`          | [`index`][resources.index]                         | display a list of all resource                  |
| GET         | `/resources/new`      | [`new`][resources.new]                             | return an HTML form for creating a new resource |
| POST        | `/resources`          | [`create`][resources.create]                       | create a new resource                           |
| GET         | `/resources/:id`      | [`show`][resources.show]                           | display a specific resource                     |
| GET         | `/resources/:id/edit` | [`edit`][resources.edit]                           | return an HTML form for editing a resource      |
| PUT         | `/resources/:id`      | [`update`][resources.update]                       | update a specific resource                      |
| PATCH       | `/resources/:id`      | [`update_with_patch`][resources.update_with_patch] | update a specific resource with patch           |
| DELETE      | `/resources/:id`      | [`destroy`][resources.destroy]                     | delete a specific resource                      |

```rust
let resources = Resources::default()
  .named("post")
  .index(|_: Request| async { Ok(Response::text("posts index")) });
```

## Nested routing

Nested [`Route`], [`Resources`], [`Router`] into a larger route.

Here are the three nesting methods:

- [`route`][router.route]

- [`resources`][router.resources]

- [`nest`][router.nest]

```rust
let search = Route::new().get(|_: Request| async { Ok(Response::text("search")) });

let orgs = Resources::default()
    .index(|_: Request| async { Ok(Response::text("list posts")) })
    .create(|_: Request| async { Ok(Response::text("create post")) })
    .show(|_: Request| async { Ok(Response::text("show post")) });

let settings = Router::new()
    .get("/", |_: Request| async { Ok(Response::text("settings")) })
    .get("/:page", |_: Request| async {
        Ok(Response::text("setting page"))
    });

let api = Router::new().route("/search", search.clone());

let app = Router::new()
    .get("/", |_: Request| async { Ok(Response::text("index")) })
    .route("search", search)
    .resources(":org", orgs)
    .nest("settings", settings)
    .nest("api", api);
```

## Middleware

We can add a middleware to single `handler`, all add it to all routes.

### Handler

[`HandlerExt`] is an extension trait which provides a number of friendly and easy
ways to easily wrap `handler`.

Adds the middleware to `handler` with [`around`][handlerext.around] or [`with`][handlerext.with] methods.

- [`around`][handlerext.around]

```rust
async fn middle_fn<H>((req, h): Next<Request, H>) -> Result<Response>
where
    H: Handler<Request, Output = Result<Response>> + Clone,
{
    h.call(req).await
}

let search = (|_: Request| async { Ok(Response::text("search")) })
  .around(middle_fn);
```

- [`with`][handlerext.with]

```rust
struct Timeout {
    delay: Duration,
}

impl Timeout {
    pub fn new(secs: u64) -> Self {
        Self { delay: Duration::from_secs(secs) }
    }
}

impl<H: Clone> Transform<H> for Timeout {
    type Output = TimeoutMiddleware<H>;

    fn transform(&self, h: H) -> Self::Output {
        TimeoutMiddleware(h, self.delay)
    }
}

#[derive(Clone)]
struct TimeoutMiddleware<H>(H, Duration);

#[async_trait]
impl<H> Handler<Request> for TimeoutMiddleware<H>
where
    H: Handler<Request> + Clone,
{
    type Output = H::Output;

    async fn call(&self, req: Request) -> Self::Output {
        self.0.call(req).await
    }
}

let search = (|_: Request| async { Ok(Response::text("search")) })
  .with(timeHMS::new());
```

### All routes

[`Route`], [`Resources`], [`Router`] all support `with` and `with_handler`
methods for adding middleware.

```rust
let routes = Router::new()
  .get("/", |_: Request| async { Ok(Response::text("index")) }
    // handler middleware
    .with(Timeout::new())
    .with_handler(middle_fn)
  )
  .route("/search", Route::new()
    .get(|_: Request| async { Ok(Response::text("search")) })
    // route middle_fn
    .with(Timeout::new())
    .with_handler(middle_fn)
  )
  .resources("/posts", Resources::default()
    .named("post")
    .index(|_: Request| async { Ok(Response::text("posts index")) })
    // resources middleware
    .with(Timeout::new())
    .with_handler(middle_fn)
  )
  .nest("/users", Router::new()
    // router middleware
    .with(Timeout::new())
    .with_handler(middle_fn)
  )
  // router middleware
  .with(Timeout::new())
  .with_handler(middle_fn);
```

## Examples

- [Todos]

[`path-tree`]: https://github.com/viz-rs/path-tree
[trie]: https://en.wikipedia.org/wiki/Trie
[`route`]: https://docs.rs/viz/0.4.x/viz/struct.Route.html
[`resources`]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html
[`params\<t\>`]: https://docs.rs/viz/0.4.x/viz/types/struct.Params.html
[`router`]: https://docs.rs/viz/0.4.x/viz/struct.Router.html
[`method`]: https://docs.rs/viz/0.4.x/viz/struct.Method.html
[`handler`]: https://docs.rs/viz/0.4.x/viz/handler/trait.Handler.html
[`handlerext`]: https://docs.rs/viz/0.4.x/viz/handler/trait.HandlerExt.html
[handlerext.around]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.around
[handlerext.with]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.with
[handlerext.with_fn]: https://docs.rs/viz/0.4.x/viz/trait.HandlerExt.html#method.with_fn
[router.get]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.get
[router.post]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.post
[router.put]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.put
[router.delete]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.delete
[router.head]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.head
[router.options]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.options
[router.connect]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.connect
[router.patch]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.path
[router.trace]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.trace
[router.route]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.route
[router.resources]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.resources
[router.nest]: https://docs.rs/viz/0.4.x/viz/struct.Router.html#method.nest
[resources.index]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html#method.index
[resources.new]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html#method.new
[resources.create]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html#method.create
[resources.show]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html#method.show
[resources.edit]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html#method.edit
[resources.update]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html#method.update
[resources.update_with_patch]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html#method.update_with_patch
[resources.destroy]: https://docs.rs/viz/0.4.x/viz/struct.Resources.html#method.destroy
[todos]: https://github.com/viz-rs/viz/tree/0.4.x/examples/routing/todos
