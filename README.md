# axum-tutorial

一个基于 [axum](https://github.com/tokio-rs/axum) 的 Rust Web 服务学习项目，演示了路由、Handler、进程内测试（oneshot）与端到端（e2e）测试等基础用法。

## 技术栈

| 依赖 | 版本 | 说明 |
| --- | --- | --- |
| axum | 0.8 | Web 框架 |
| tokio | 1.53（`full` features） | 异步运行时 |
| serde / serde_json | 1.0 | 序列化与反序列化 |
| uuid | 1.27（`serde`、`v4` features） | 用户 ID 生成与序列化 |
| utoipa | 5 | OpenAPI 注解与文档生成 |
| utoipa-swagger-ui | 9（`axum` feature） | Swagger UI 文档页面 |
| utoipa-redoc | 6（`axum` feature） | Redoc 阅读式文档页面 |
| utoipa-rapidoc | 6（`axum` feature） | RapiDoc 文档页面 |
| utoipa-scalar | 0.3（`axum` feature） | Scalar 文档页面 |
| reqwest（dev） | 0.13（`json` features） | e2e 测试用 HTTP 客户端 |
| tower（dev） | 0.5（`util` features） | 提供 `oneshot` 进程内测试能力 |
| anyhow（dev） | 1.0 | 测试中的错误处理 |

Rust edition：`2024`

## 项目结构

```
axum-tutorial/
├── src/
│   ├── main.rs          # 程序入口：绑定 127.0.0.1:8080 并启动服务，含进程内单元测试
│   ├── lib.rs           # app() 组装路由，挂载 AppState 与 fallback
│   ├── docs.rs          # OpenAPI 文档定义（utoipa 从 Handler 注解收集）
│   ├── state.rs         # AppState（Arc<RwLock<HashMap<Uuid, User>>> 内存数据库）
│   ├── errors.rs        # AppError 统一错误类型，实现 IntoResponse
│   ├── handler/         # 各资源的 Handler（hello、user、404 fallback）
│   └── router/          # 各资源的路由（hello_router、user_router），api_router() 汇总并挂载 SwaggerUi
└── tests/
    └── e2e_test.rs      # 端到端测试：随机端口起真实服务，用 reqwest 发请求
```

## API 路由

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/hello` | 闭包 Handler，返回 HTML |
| GET | `/hello1` | Query 参数（`?name=Tom`，缺省为 World） |
| GET | `/hello2/{name}` | Path 路径参数 |
| GET | `/hello-q?name=Tom` | Query + JSON 结构化响应 |
| GET | `/users` | 列出所有用户 |
| POST | `/users` | 创建用户（JSON body：`name`、`email`） |
| GET | `/users/{id}` | 查询单个用户，不存在返回 404 |
| PUT | `/users/{id}` | 部分更新（`name` / `email` 均为可选字段） |
| DELETE | `/users/{id}` | 删除用户，成功返回 204 |

未匹配的路径走 `fallback`，返回 404 JSON：`{"error":"route not found"}`。

## Swagger / OpenAPI 文档

项目使用 [utoipa](https://github.com/juhaku/utoipa) 生态自动生成并托管 API 文档。utoipa 官方提供了四种 UI 渲染器，本项目全部接入，共用同一份 `ApiDoc`，按喜好任选：

| 地址 | UI | 特点 |
| --- | --- | --- |
| `http://127.0.0.1:8080/swagger-ui` | Swagger UI | 最经典，调试功能全（Try it out、Schema 折叠） |
| `http://127.0.0.1:8080/redoc` | Redoc | 三栏阅读式排版，适合对外发布静态文档 |
| `http://127.0.0.1:8080/scalar` | Scalar | 现代美观，交互体验好 |
| `http://127.0.0.1:8080/rapidoc` | RapiDoc | 轻量快速，支持 Try it out |
| `http://127.0.0.1:8080/api-docs/openapi.json` | OpenAPI JSON | 原始规范文件，可导入 Postman / Apifox 等 |

对应 crate（注意版本需与 utoipa 主版本配对，当前 utoipa 5 对应下述版本）：

| crate | 版本 | 挂载方式 |
| --- | --- | --- |
| `utoipa-swagger-ui` | 9 | `SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", spec)` |
| `utoipa-redoc` | 6 | `Redoc::with_url("/redoc", spec)` |
| `utoipa-scalar` | 0.3 | `Scalar::with_url("/scalar", spec)` |
| `utoipa-rapidoc` | 6 | `RapiDoc::new("/api-docs/openapi.json").path("/rapidoc")` |

### 实现方式

1. **Handler 注解**：每个 Handler 上方用 `#[utoipa::path(...)]` 声明方法、路径、参数、请求体与响应：

```rust
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(("id" = Uuid, Path, description = "用户 ID")),
    responses(
        (status = 200, description = "用户详情", body = User),
        (status = 404, description = "用户不存在")
    )
)]
pub async fn get_user(...) -> ... { ... }
```

2. **Schema 注解**：请求/响应结构体派生 `ToSchema`（查询参数用 `IntoParams`）：

```rust
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
```

3. **汇总生成**（`src/docs.rs`）：`ApiDoc` 通过 `#[derive(OpenApi)]` 收集所有 path 与 schema，在 `router/mod.rs` 挂载：

```rust
Router::<AppState>::new()
    .merge(hello_router::router())
    .nest("/users", user_router::routes())
    // 四种 UI 共用同一份 ApiDoc
    .merge(SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
    .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
```

> 注意：RapiDoc 要用 `new`（只引用 spec 地址）而不是 `with_openapi`（会自己注册 `/api-docs/openapi.json` 路由），否则与 SwaggerUi 的注册重叠，启动时 panic："Overlapping method route"。

### 新增接口时的操作

1. 新 Handler 加上 `#[utoipa::path(...)]` 注解（记得写 `tag`）；
2. 新增/变更的结构体派生 `ToSchema`；
3. 在 `src/docs.rs` 的 `paths(...)` / `components(schemas(...))` 中登记。

> 注意：忘记登记 `docs.rs` 时编译不会报错，但 Swagger 页面上看不到该接口。

## 快速开始

```bash
# 运行服务（默认监听 127.0.0.1:8080）
cargo run

# 访问接口
curl http://127.0.0.1:8080/hello
curl http://127.0.0.1:8080/hello2
```

启动后可在浏览器打开 API 文档页面（四种 UI 共用同一份文档，任选其一）：

- Swagger UI：<http://127.0.0.1:8080/swagger-ui>
- Redoc：<http://127.0.0.1:8080/redoc>
- Scalar：<http://127.0.0.1:8080/scalar>
- RapiDoc：<http://127.0.0.1:8080/rapidoc>
- OpenAPI JSON（可导入 Postman / Apifox）：<http://127.0.0.1:8080/api-docs/openapi.json>

## cargo watch 热重载开发

[cargo-watch](https://github.com/watchexec/cargo-watch) 可以监听文件变动并自动重新编译运行，非常适合本地开发调试。

### 安装

```bash
cargo install cargo-watch
```

### 常用命令

```bash
# 修改代码后自动重新运行（最基本的用法）
cargo watch -x run

# 自动运行测试（保存即跑测试）
cargo watch -x test

# 自动执行 clippy 检查
cargo watch -x "clippy --all-targets"

# 清屏后重新运行，日志更清爽
cargo watch -c -x run

# 同时跑多个命令：先编译检查再运行
cargo watch -x check -x run

# 监听指定的包/测试
cargo watch -x "test --test e2e_test"
```

### 常用参数说明

| 参数 | 说明 |
| --- | --- |
| `-x <cmd>` | 文件变动时执行的 cargo 命令，可多次指定 |
| `-c` | 每次执行前清空终端 |
| `-q` | 静默模式，不输出 watch 自身日志 |
| `-w <path>` | 只监听指定目录，如 `-w src`，减少无关触发 |
| `-i <pattern>` | 忽略匹配的文件/目录，如 `-i "*.log"` |
| `-s <cmd>` | 执行任意 shell 命令（而非 cargo 子命令） |

### 本项目推荐用法

```bash
cargo watch -c -q -w src -x run
```

只监听 `src/` 目录，清屏 + 静默 + 自动重启服务，改完代码保存即可在浏览器/curl 里直接看到效果。

> 注意：重启服务时会重新 bind 端口，若偶发 "Address already in use"，等待旧进程退出或换端口即可。

## 测试

### 测试策略总览

项目采用两种互补的测试方式，均不占用固定端口，可放心并行运行：

| | 进程内测试（单元测试） | e2e 测试（端到端测试） |
| --- | --- | --- |
| 位置 | `src/main.rs` 的 `#[cfg(test)] mod tests` | `tests/e2e_test.rs` |
| 原理 | `Router` 本身是 `tower::Service`，用 `oneshot` 在内存中直接塞入 `Request`，不 bind socket | `TcpListener::bind("127.0.0.1:0")` 随机端口启动真实服务，用 reqwest 发起真实 HTTP 请求 |
| 速度 | 毫秒级 | 较慢（走完整网络栈） |
| 覆盖 | Handler 逻辑、路由匹配、状态码、响应体 | 序列化、真实的请求/响应管线 |
| 依赖 | `tower`（`ServiceExt::oneshot`） | `reqwest`、`anyhow` |

### 测试用例清单

| 测试 | 文件 | 验证内容 |
| --- | --- | --- |
| `hello_world` | `src/main.rs` | GET `/hello` 返回 200，body 为 `<h1>Hello, World!</h1>`；演示 `into_parts` + `to_bytes` 读取流式 body |
| `not_found` | `src/main.rs` | 未知路径 `/nope` 返回 404 |
| `real_request` | `tests/e2e_test.rs` | 随机端口起服务，GET `/hello` 返回 200 且 body 正确 |

### 运行命令

```bash
# 运行全部测试（单元 + e2e）
cargo test

# 按测试名过滤（匹配测试函数名的任意子串）
cargo test hello_world
cargo test not_found

# 只运行某个测试目标
cargo test --lib              # 仅 src 内的单元测试
cargo test --test e2e_test     # 仅 tests/e2e_test.rs
cargo test --bin axum_tutorial # 仅 main.rs 内的单元测试

# 显示测试中 println! 的输出（默认成功时不显示）
cargo test -- --nocapture

# 修改后强制全量重跑（Rust 测试二进制默认不重新编译未变更目标）
cargo test --no-run --quiet && cargo test
```

### 实现要点说明

**进程内测试（oneshot）**：

```rust
use tower::ServiceExt; // 引入 oneshot 扩展方法

let response = app()
    .oneshot(
        Request::builder()
            .uri("/hello")
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();
```

- `Router` 实现了 `tower::Service`，`oneshot` 让 `Service` 只处理一次请求，全程零 socket。
- 读取响应体需要三步：`into_parts()` 拿到 body 所有权 → `to_bytes(body, usize::MAX)` 把流式 body 全部读出 → 转 `String` 断言。

**e2e 测试（随机端口 + 后台 task）**：

```rust
let listener = TcpListener::bind("127.0.0.1:0").await.unwrap(); // 端口写 0：系统随机分配
let addr = listener.local_addr().unwrap();
tokio::spawn(async move {
    axum::serve(listener, app()).await.unwrap();
});
```

- 端口 `0` 表示由系统随机分配，测试并行跑不会 "Address already in use"。
- 服务通过 `tokio::spawn` 在后台 task 中运行，因此测试需标注 `#[tokio::test(flavor = "multi_thread")]`，多线程 runtime 下更稳。
- 测试函数返回 `anyhow::Result<()>`，断言链路中可直接用 `?`。

### 如何新增测试

- **Handler 逻辑测试**：优先写进程内测试，在 `src/main.rs`（或专门的 `#[cfg(test)]` 模块）中用 `oneshot` 构造 `Request`，断言状态码与 body。
- **接口级测试**：在 `tests/e2e_test.rs` 中复用 `spawn_server()`，用 reqwest 调用真实端口。例如为 user CRUD 新增 e2e 测试时，依次 POST → GET → PUT → DELETE 并断言各状态码（201 / 200 / 204 / 404）。
- **注意**：`AppState` 目前每次 `app()` 都新建（内存数据库为空），如需带初始数据的测试，可改造 `app()` 接收自定义 state。
