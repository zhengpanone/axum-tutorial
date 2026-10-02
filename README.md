# axum-tutorial

一个基于 [axum](https://github.com/tokio-rs/axum) 的 Rust Web 服务学习项目，演示了路由、Handler、进程内测试（oneshot）与端到端（e2e）测试等基础用法。

## 技术栈

| 依赖 | 版本 | 说明 |
| --- | --- | --- |
| axum | 0.8 | Web 框架 |
| tokio | 1.53（`full` features） | 异步运行时 |
| reqwest（dev） | 0.13（`json` features） | e2e 测试用 HTTP 客户端 |
| tower（dev） | 0.5（`util` features） | 提供 `oneshot` 进程内测试能力 |
| anyhow（dev） | 1.0 | 测试中的错误处理 |

Rust edition：`2024`

## 项目结构

```
axum-tutorial/
├── src/
│   ├── main.rs      # 程序入口：绑定 127.0.0.1:8080 并启动服务，含进程内单元测试
│   └── lib.rs       # app() 构建路由，定义 /hello 与 /hello2 两个 Handler
└── tests/
    └── e2e_test.rs  # 端到端测试：随机端口起真实服务，用 reqwest 发请求
```

## API 路由

| 方法 | 路径 | 响应 | 说明 |
| --- | --- | --- | --- |
| GET | `/hello` | `<h1>Hello, World!</h1>` | 闭包形式的内联 Handler |
| GET | `/hello2` | `Hello <strong>World!</strong>` | 独立 async 函数 Handler |

## 快速开始

```bash
# 运行服务（默认监听 127.0.0.1:8080）
cargo run

# 访问接口
curl http://127.0.0.1:8080/hello
curl http://127.0.0.1:8080/hello2
```

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

项目包含两种测试方式，均不占用固定端口：

```bash
# 运行全部测试（单元 + e2e）
cargo test

# 仅运行进程内单元测试（src/main.rs 中，通过 tower oneshot 直接调用 Router，不 bind socket）
cargo test hello_world

# 仅运行 e2e 测试（随机端口启动真实服务，用 reqwest 发起真实 HTTP 请求）
cargo test --test e2e_test
```

- **进程内测试**：`Router` 本身就是 `tower::Service`，可用 `oneshot` 直接在内存中塞入 `Request`，毫秒级完成、无端口冲突。
- **e2e 测试**：`TcpListener::bind("127.0.0.1:0")` 由系统随机分配端口，测试并行运行不会出现 "Address already in use"；使用 `#[tokio::test(flavor = "multi_thread")]` 以支持后台 server task。
