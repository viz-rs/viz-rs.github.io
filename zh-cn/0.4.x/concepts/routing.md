# 路由

路由是整个 Web 应用的交通枢纽，对连接视图、控制、数据起到承上启下的作用。

在 Viz 的路由系统中，使用 [`path-tree`] 模块来存储 `path` 和 `handler` 之间的关系。

[`path-tree`] 核心是一个[前缀树][trie]实现，提供一个高效的搜索引擎和一些友好的辅助方法。

## 结构及概念

- [`Route`]：基础结构，是 [`Method`] - [`Handler`] 对的合集。

- [`Resources`]：资源结构，通过对 `Action` - [`Method`] 之间的映射，来进行 CRUD 操作。

- [`Router`]：是对 [`Route`]、[`Resources`]、嵌套路由，中间件的管理和维护。

## 路径参数

得益于 [`path-tree`] 提供的支持，在路径中，可以通过 `:` 对参数命名，`?` `\+` `\*` 对参数设置类型。

| 模式                                | 规则                                             |
| ----------------------------------- | ------------------------------------------------ |
| `:name`                             | 匹配除 `/` 以外的字符                            |
| `:name?`                            | 匹配除 `/` 以外的字符，可选                      |
| `/:name?/` `/:name?`                | 匹配以 `/` 开头或结尾，但除 `/` 以外的字符，可选 |
| `\+` `:name\+`                      | 匹配长度 > 0 的字符串                            |
| `\*` `:name\*`                      | 匹配长度 >= 0 的字符串                           |
| `/\*` `/\*/` `/:name\*/` `/:name\*` | 匹配以 `/` 开头或结尾，且长度 >= 0 的字符串      |

后续可以通过 [`Params\<T\>`] 提取器对参数进行提取。

> 同时也支持**连续参数**和参数之间穿插特殊字符进行分割。

## 方法路由

使用 HTTP 方法添加路由。支持以下方法：

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

## 资源路由

使用约定的动作添加路由。支持以下动作：

| HTTP 方法 | 路径                  | 动作                                               | 用途                         |
| --------- | --------------------- | -------------------------------------------------- | ---------------------------- |
| GET       | `/resources`          | [`index`][resources.index]                         | 显示资源列表                 |
| GET       | `/resources/new`      | [`new`][resources.new]                             | 返回用于新建资源的 HTML 表单 |
| POST      | `/resources`          | [`create`][resources.create]                       | 新建资源                     |
| GET       | `/resources/:id`      | [`show`][resources.show]                           | 显示指定资源                 |
| GET       | `/resources/:id/edit` | [`edit`][resources.edit]                           | 返回用于修改资源的 HTML 表单 |
| PUT       | `/resources/:id`      | [`update`][resources.update]                       | 更新指定资源                 |
| PATCH     | `/resources/:id`      | [`update_with_patch`][resources.update_with_patch] | 更新指定资源                 |
| DELETE    | `/resources/:id`      | [`destroy`][resources.destroy]                     | 删除指定资源                 |

```rust
let resources = Resources::default()
  .named("post")
  .index(|_: Request| async { Ok(Response::text("posts index")) });
```

## 嵌套路由

可以把 [`Route`]、[`Resources`]、[`Router`] 嵌入更大的路由当中。

以下是三种嵌套方法：

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

## 中间件

可以对单个 `handler` 添加中间件，也可以对所有路由添加中间件。

### handler

[`HandlerExt`] 扩展特征提供了很多友好简便的方法，可以轻松地对 `handler` 进行包裹封装。

可以通过 [`around`][handlerext.around] 和 [`with`][handlerext.with] 对 `handler`
添加中间件。

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

### 所有路由

[`Route`]、[`Resources`]、[`Router`] 都提供了两个方法来添加中间件 `with` 和 `with_handler`。

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

## 例子

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
