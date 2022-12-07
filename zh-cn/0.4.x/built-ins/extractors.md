# 提取器

以下是已经内置的提取器，开启响应特性就可以使用它们。

| 特性      | 提取器      | 说明                           |
| --------- | ----------- | ------------------------------ |
|           | [Header]    | 提取 `header` 信息             |
| cookie    | [Cookies]   | 提取 `cookies` 信息            |
| limites   | [Limits]    | 提取 `limits` 设置             |
| form      | [Form]      | 提取 `from-data` 数据          |
| json      | [Json]      | 提取 `JSON` 数据               |
| multipart | [Multipart] | 提取 `multipart`               |
| params    | [Params]    | 从请求路径中提取参数 `params`  |
| query     | [Query]     | 从请求 `query string` 提取参数 |
| session   | [Session]   | 提取会话信息                   |
| state     | [State]     | 提取 `state` 状态数据          |
| websocket | [Websocket] | 提取 `WebSocket` 连接          |

[query]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.Query.html
[params]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.Params.html
[header]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.Header.html
[cookies]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.Cookies.html
[form]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.Form.html
[json]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.Json.html
[multipart]: https://docs.rs/viz-core/0.4.x/viz_core/types/type.Multipart.html
[session]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.Session.html
[state]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.State.html
[websocket]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.WebSocket.html
[limits]: https://docs.rs/viz-core/0.4.x/viz_core/types/struct.Limits.html
