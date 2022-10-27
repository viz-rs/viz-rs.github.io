# 路由

路由是整个 Web 应用的交通枢纽，对连接视图、控制、数据起到承上启下的作用。

在 Viz 的路由系统中，使用 [`path-tree`] 模块来存储 `path` 和 `handler` 之间的关系。

[`path-tree`] 核心是一个[前缀树]实现，提供一个高效的搜索引擎和一些友好的辅助方法。

## 概念及结构

* [`Route`]：最基础的路由结构，是 [`Method`] - [`Handler`] 对的合集。

* [`Resource`]：定义资源的路由结构，通过对 `Action` - [`Method`] 之间的映射，来进行 CRUD 操作。

* [`Router`]：是对 [`Route`]、[`Resource`]、嵌套路由，中间件的管理和维护。 

## 路径

## 方法路由

## 资源路由

## 嵌套路由

## 中间件

[`path-tree`]: https://github.com/viz-rs/path-tree
[前缀树]: https://en.wikipedia.org/wiki/Trie
[`Route`]: https://docs.rs/viz/latest/viz/struct.Route.html
[`Resource`]: https://docs.rs/viz/latest/viz/struct.Resource.html
[`Router`]: https://docs.rs/viz/latest/viz/struct.Router.html
[`method`]: https://docs.rs/viz/latest/viz/struct.Method.html
[`handler`]: https://docs.rs/viz/latest/viz/handler/trait.Handler.html
