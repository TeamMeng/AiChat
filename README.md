# AiChat

AiChat 是一个包含 Web、桌面端和 Rust 后端服务的实时聊天项目。仓库当前由 Vue 3/Tauri 客户端和 `new-chat` Rust workspace 组成，覆盖聊天、通知、分析和 AI Agent 能力。

## 主要功能

- 用户注册、登录和基于 JWT 的身份认证
- 工作区管理与成员邀请
- 多人聊天、私聊、消息历史查询
- 文件上传
- 基于 SSE 的实时通知
- Chat Agent 管理与消息处理
- Ollama 适配器接入
- 分析事件采集与写入 ClickHouse
- OpenAPI 文档与 Swagger UI / Redoc / Scalar 页面
- Web 端和 Tauri 桌面端双端运行

## 仓库结构

- [chatapp](/Users/meng/Desktop/code/AiChat/chatapp): Vue 3 + Vite 前端，以及 Tauri 桌面壳
- [new-chat](/Users/meng/Desktop/code/AiChat/new-chat): Rust workspace
- [new-chat/chat_server](/Users/meng/Desktop/code/AiChat/new-chat/chat_server): 主聊天 API 服务
- [new-chat/notify_server](/Users/meng/Desktop/code/AiChat/new-chat/notify_server): SSE 通知服务
- [new-chat/analytics-server](/Users/meng/Desktop/code/AiChat/new-chat/analytics-server): 分析事件服务
- [new-chat/bot-server](/Users/meng/Desktop/code/AiChat/new-chat/bot-server): Bot 与索引相关服务
- [new-chat/ai_sdk](/Users/meng/Desktop/code/AiChat/new-chat/ai_sdk): AI 适配层
- [swiftide-pgvector](/Users/meng/Desktop/code/AiChat/swiftide-pgvector): pgvector 相关扩展代码

## 技术栈

- 前端: Vue 3, Vue Router, Vuex, Vite, Axios, Tailwind CSS
- 桌面端: Tauri 2
- 后端: Rust, Axum, Tokio, SQLx
- 数据库: PostgreSQL
- 分析: ClickHouse, Protobuf
- API 文档: Utoipa, Swagger UI, Redoc, Scalar
- AI: Ollama, Swiftide

## 后端服务能力

### chat-server

- 用户注册、登录、修改密码
- 聊天列表、聊天详情、创建聊天、更新聊天、删除聊天
- 聊天成员管理
- 消息发送与消息列表查询
- 文件上传
- Chat Agent 创建、更新、查询

### notify-server

- 通过 `/events` 提供 SSE 连接
- 监听 PostgreSQL `NOTIFY` 事件
- 推送新聊天、新消息、工作区变更等通知

### analytics-server

- 接收分析事件
- 写入 ClickHouse
- 提供 OpenAPI 文档页面

### bot-server

- 集成 `swiftide`
- 支持 Ollama 和向量检索相关处理

## 客户端功能

- 登录、注册
- 工作区邀请页
- 聊天侧边栏与成员列表
- 消息发送、消息列表、文件上传
- 基础分析事件上报
- Tauri 原生命令、日志、窗口状态持久化

## 环境要求

- Node.js 18+
- Rust stable
- PostgreSQL
- `protoc`
- Linux 下构建 Tauri 时通常还需要 `javascriptcoregtk-4.1`、`libsoup-3.0`、`webkit2gtk-4.1`、`pkg-config`

## 快速开始

### 1. 克隆仓库

```bash
git clone https://github.com/TeamMeng/AiChat.git
cd AiChat
```

### 2. 启动 Rust 服务

聊天服务：

```bash
cd new-chat/chat_server
cargo run
```

通知服务：

```bash
cd new-chat/notify_server
cargo run
```

分析服务：

```bash
cd new-chat/analytics-server
cargo run
```

### 3. 启动 Web 客户端

```bash
cd chatapp
npm install
npm run dev
```

### 4. 启动桌面客户端

```bash
cd chatapp
npm install
npm run tauri dev
```

## 测试

Rust workspace 测试：

```bash
cd new-chat
cargo test
```

只编译测试：

```bash
cd new-chat
cargo test --no-run
```

更具体的 Rust workspace 说明见 [new-chat/README.md](/Users/meng/Desktop/code/AiChat/new-chat/README.md)。

## 注意事项

- `chat_test` 的 SSE 集成测试目前直接使用 `reqwest` 流式读取事件，不再依赖 `reqwest-eventsource`，用于避免 Linux CI 下额外的 OpenSSL 链接问题。
- `utoipa-swagger-ui` 在构建时会下载 Swagger UI 静态资源。如果构建环境无法访问 GitHub，相关 crate 可能因为下载失败而导致编译失败。
