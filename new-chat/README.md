# RUST 代码模板

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

```bash
cargo generate TeamMeng/template
```

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install` 即可。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```

### 安装 cargo audit
cargo audit 是一个 Rust 检查已知的漏洞安全和依赖安全的工具

```bash
cargo install cargo-audit
```

## 功能特性

### Redis 连接池

使用 `deadpool_redis` 实现 Redis 连接池，提高 Redis 访问性能。

配置项（`chat.yaml`）：

```yaml
redis:
  url: redis://localhost:6379
  pool_size: 16
```

### 登录限流

基于滑动窗口算法的登录限流中间件，支持三层防护：

| 限流维度 | 默认值 | 说明 |
|---------|--------|------|
| IP + Email | 3 次/分钟 | 最严格，针对同一 IP 尝试不同邮箱 |
| Email | 5 次/分钟 | 防止邮箱被暴力破解 |
| IP | 10 次/分钟 | 防止 IP 被封禁前的大量尝试 |

配置项（`chat.yaml`）：

```yaml
rate_limit:
  signin:
    max_attempts_ip: 10
    max_attempts_email: 5
    max_attempts_ip_email: 3
    window_secs: 60
```

限流中间件自动从请求头提取客户端 IP（优先 X-Forwarded-For → X-Real-IP → Host）。

## 测试与 CI

### 运行测试

在 `new-chat` workspace 根目录执行：

```bash
cargo test
```

如果只想编译测试而不执行，可以使用：

```bash
cargo test --no-run
```

### 关于 SSE 测试依赖

`chat_test` 中的 SSE 集成测试现在直接使用 `reqwest` 流式读取事件，不再依赖 `reqwest-eventsource`。这样可以避免在 Linux CI 上额外引入 `native-tls/OpenSSL` 链接问题。

### 关于无外网环境

当前项目依赖 `utoipa-swagger-ui`。它的构建脚本会在编译时下载 Swagger UI 静态资源。

如果构建环境无法访问 GitHub，编译可能失败，并出现类似下面的错误：

```text
failed to download Swagger UI
curl: (6) Could not resolve host: github.com
```

这类失败通常不是 Rust 代码本身的问题，而是构建环境缺少外网访问能力。
