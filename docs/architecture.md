# SmartDoc 架构设计说明

## 1. 总体架构概览
SmartDoc 采用三层结构：
1. **桌面壳层（Tauri）**：封装系统能力（文件、网络、MCP、云盘挂载）、暴露 Rust 命令给前端，面向 macOS/Windows/Linux 三平台构建。
2. **前端表现层（Leptos）**：负责 UI 组件（云盘、知识库、AI 聊天、智能体构建器、影视 WebView），提供占位/正式双态组件以支持 Demo 先行。
3. **协同服务层**：复用 ONLYOFFICE DocumentServer、知识库索引服务、MCP 工具进程以及第三方云盘/影视站点，结合 CI/CD 管线实现端到端交付，并为 Web/移动/小程序端共享同一后端能力。

```
┌─────────────┐
│  Leptos UI  │  云盘/知识库/AI/影视/设置模块
└─────▲───────┘
      │Signals/Events
┌─────┴───────┐
│  Tauri Core │  文件系统、MCP Runtime、云盘适配器、DocumentServer SDK
└─────▲───────┘
      │IPC/HTTP/WebSocket
┌─────┴─────────────────────────────┐
│ External Services & Tools         │
│ - ONLYOFFICE DocumentServer       │
│ - Knowledge Base (Open WebUI etc.)│
│ - MCP Tools (Rust/Python)         │
│ - Cloud Storage Providers         │
│ - seerhut.uk Media Site           │
└───────────────────────────────────┘
```

## 2. 模块划分
| 模块 | 关键子组件 | 说明 |
| --- | --- | --- |
| Tauri Shell | `storage-adapter`, `onlyoffice-bridge`, `mcp-runtime`, `media-embed`, `settings-store`, `platform-packager` | 使用 Rust 实现，提供命令给前端；负责持久化设置、IPC 代理、系统通知及桌面/移动/小程序打包参数，支撑模块化调用。 |
| Leptos UI | `pages/cloud`, `pages/knowledge`, `pages/assistant`, `pages/agents`, `pages/media`, `pages/settings`, `components/editor` | 状态管理基于 Signals + store；调用 Tauri Commands；支持多窗口与 Web DOM 兼容渲染模式，实现组件级复用。 |
| Web Runtime | `leptos-ssr`, `wasm-bindings`, `api-proxy` | 复用 Leptos 组件，通过 SSR / CSR 构建 Web 端；若需，可切换到混合框架（如 VitePress、Next.js + wasm 包）。 |
| Mobile Bridge | `tauri-mobile`, `capacitor/uni-app adapters`, `mini-program facade` | 将核心业务逻辑导出为跨平台 API，供 Android/iOS/鸿蒙/小程序复用；若 Tauri 能力不足，可接入 Flutter/React Native/uni-app 混合壳。 |
| 鉴权/后端服务 | `auth-service`, `user-profile`, `sso-adapter`, `documentserver-jwt` | 独立部署在 Web 服务器（Docker），负责 Tauri 用户登录/注册、管理后台；通过适配器映射到 DocumentServer/NAS 的账户体系，可复用若依/Keycloak 等开源组件或自研（FastAPI/NestJS）。 |
| 云盘服务 | `fs-local`, `webdav-client`, `sdk-wrapper` | 通过 trait `StorageProvider` 抽象，实现多网盘挂载与上传下载。 |
| 知识库引擎 | `ingest-service`, `embedding-worker`, `vector-store` | 可部署为本地微服务，或调用 Open WebUI/Cherry Studio API；对话上下文通过 MCP 返回。 |
| AI Orchestrator | `mcp-hub`, `conversation-manager`, `agent-designer`, `agent-parallel-runner` | 负责注册工具、调用模型、维护对话/Agent 配置，并支持多 Agent 并行执行。 |
| ONLYOFFICE 集成 | `editor-host`, `callback-handler`, `conversion-service` | 通过 DocumentServer 公开的 `example` 页或 web-apps 自部署页面，内嵌在 WebView；回调至 Tauri/后端处理保存与鉴权。 |
| 影视集成 | `media-webview`, `bookmark-service`, `note-sync` | 在 Tauri/Web/Mobile 中通过 WebView/iframe/JSBridge 接入；支持把观影笔记写入知识库。 |

## 3. 关键数据流
1. **文档编辑流**：Leptos 云盘列表 → 选择文件 → 调用 `tauri://onlyoffice-open` → Shell 生成 JWT/回调 URL → WebView 打开 DocumentServer → 编辑结束回调 → Shell 将新版本写回云盘；Demo 阶段可用 Mock 文档。
2. **多鉴权协同流**：用户在 Tauri/Web/移动端登录自有 Auth Service（部署在 Web 服务器）→ 获取 SmartDoc Token → Auth Service 通过适配器调用 DocumentServer/NAS 的鉴权接口生成二次 token（或映射用户 ID）→ 前端在 WebView/iframe 中携带对应 JWT 打开 `http://10.18.65.129:8085/example/`；NAS 接口同理。
3. **知识库检索流**：用户在 AI 助手输入问题 → 前端触发 `mcp-runtime.query` → Rust 侧调用嵌入式 MCP 工具（向量检索、云盘搜索）→ 返回引用片段 → Leptos 渲染消息卡片；如运行在 Web 端则通过 `api-proxy` 调用相同后端；若向量库未就绪，可返回占位回答并记录 backlog。
4. **智能体执行流**：Agent 设定 YAML → 存储在本地 `agents/` 目录 → 触发运行 → `agent-executor`/`agent-parallel-runner` 调度 MCP 工具与模型 → 结果回写到任务面板/通知；支持多 Agent 并行执行，彼此隔离。
5. **影视站收藏流**：内嵌 WebView 监听 JS Bridge 事件（收藏/笔记）→ Tauri 写入本地数据库（SQLite）→ 可同步到知识库或云盘；在 Web/移动/小程序端改为调用云端收藏 API；Demo 阶段可由静态 HTML 模拟。

## 4. 技术栈与协议
- **语言**：Rust (Tauri backend, MCP tools), TypeScript/Rust-Wasm (Leptos 前端), Python/Node（鉴权/业务 API），必要时 Go/Java。
- **框架**：Tauri v2、Leptos 0.7+、Tokio async runtime、Serde、reqwest。
- **通信协议**：Tauri IPC、HTTP/HTTPS、WebSocket（DocumentServer、MCP 工具）、gRPC（可选 vector DB）。
- **存储**：SQLite（设置与缓存）、云盘文件系统、向量数据库（Milvus/Qdrant/Weaviate）。
- **安全**：JWT（DocumentServer）、本地 Keyring、HTTPS/TLS、内容安全策略 (CSP)。

## 5. 部署与运行
- **桌面分发**：macOS dmg、Windows msi、Linux AppImage。利用 Tauri 打包，并在 CI 中通过矩阵任务（macOS, Windows, Linux）生成 Demo 与 Release 两档构建；自动更新通过 `tauri://updater`。
- **Web 交付**：Leptos SSR/CSR 构建 pipeline，在 CI 中以 `pnpm build:web` 输出静态资源或 Edge Function；当前阶段输出 Demo stub。
- **移动 & 小程序**：预留 `pnpm build:mobile` / `pnpm build:mini` 脚本对接 tauri-mobile（Android/iOS/鸿蒙）与小程序壳（如 Uni-app、Taro）；首期仅输出空壳应用及文档，必要时评估混合框架（Flutter/React Native）。
- **后端服务**：Auth/业务 API 通过 Docker Compose 部署（Python FastAPI 或 Node NestJS），对接 DocumentServer Docker 实例。
- **外部服务**：
  - DocumentServer：已有部署，可通过 `.env` 配置地址与 JWT 密钥。
  - 知识库：Docker Compose（Open WebUI + 向量库）。
  - MCP 工具：随应用安装，或按需下载。
- **开发模式**：`pnpm tauri dev`（若 Leptos + Vite）、或 `cargo leptos watch` + `tauri dev` 并行。

## 6. NAS 集成与部署拓扑
| 部署位置 | 服务/模块 | 说明 |
| --- | --- | --- |
| NAS（核心数据面） | DocumentServer Docker、Auth 服务（Python/Node）、知识库向量库（Milvus/Qdrant）、本地 LLM/Embedding（如 Ollama、vLLM）、云盘存储适配器（WebDAV/SMB 挂载） | NAS 负责高可靠数据、文档和模型托管；通过局域网提供 API；需与 NAS 账户体系对接。 |
| NAS（可选） | MCP 工具（文件检索、批处理脚本）、媒体收藏数据库（SQLite/PostgreSQL） | 根据资源情况部署，便于离线使用。 |
| Web/独立服务器 | Web SSR 应用、反向代理、外部 API 代理、第三方 AI API key 管理 | 面向公网访问；与 NAS 通过 VPN 或专线通信，保护数据。 |
| 桌面/移动客户端 | Tauri/混合应用、前端 UI、轻量缓存 | 运行在终端设备，调用 NAS/Web 服务；可离线缓存数据。 |

- **部署策略**：
  - 提供 `deploy/nas-compose.yml` 用于在 NAS 上一键部署 DocumentServer、Auth、知识库、LLM 服务。
  - Web/移动端通过 `.env` 指定 NAS API 网关地址，支持内外网切换。
  - 区分本地模型 vs 第三方 API：NAS 模式调用本地推理服务，云模式调用 OpenAI/Azure/通义等。
  - 必要时允许部分服务（如高算力 LLM）部署到独立 GPU 服务器，但保留与 NAS 的数据同步机制。

## 6. 可扩展性设计
- 插件化 `StorageProvider` 与 `McpTool` trait，新增类型无需改动核心逻辑，可由不同 Agent 独立维护，同时在 Web/移动端通过同一 API 暴露。
- 将智能体定义存为 `agents/<name>.yaml`，支持社区共享与版本化，每次扩展后提供 Demo，并为 Web/移动端同步策略。
- WebView 承载第三方页面时，通过 `postMessage` + 自研协议保证安全，并允许在 Demo 阶段替换成本地静态资源；移动/小程序端通过 WebView/JSBridge 等价实现。

## 7. 风险与缓解
| 风险 | 影响 | 缓解措施 |
| --- | --- | --- |
| DocumentServer 跨域/证书问题 | 无法加载编辑器 | 在 Tauri 设置自签证书信任，使用代理或本地域名映射。 |
| MCP 工具资源消耗 | 桌面端性能下降 | 对工具进程做超时/限流，UI 显示状态。 |
| 影视站合法性与内容安全 | 法规风险 | 在设置中增加免责声明与访问控制，必要时允许禁用模块。 |
| 多云盘权限差异 | 数据泄露 | 每个挂载单独授权；提供最小权限模式。 |
| 多端框架差异 | 功能不一致 | 通过模块化 SDK、抽象适配层、必要时引入混合框架，保持业务逻辑共享。 |
| 外部鉴权服务复杂度 | 登录失败/安全风险 | 复用成熟框架（若依等）并裁剪，建立自动化测试覆盖登录/注册/SSO 流程。 |
| NAS 资源瓶颈 | 本地模型或服务不可用 | 预估 NAS CPU/GPU/存储，必要时支持弹性扩容或切换到云端 API；提供限流与降级策略。 |

## 8. 关键目录建议
```
SmartDoc/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── storage/
│   │   ├── onlyoffice/
│   │   ├── mcp/
│   │   └── media/
│   └── agents/
    ├── app/
    │   ├── src/
    │   │   ├── pages/
    │   │   ├── components/
    │   │   └── stores/
    │   └── package.json
    ├── web/
    │   ├── ssr/
    │   └── adapters/
    ├── mobile/
    │   ├── android/
    │   ├── ios/
    │   ├── harmony/
    │   └── mini-program/
├── docs/
│   ├── requirements.md
│   ├── architecture.md
│   └── development.md
└── AGENT.md
```

## 9. 指标与可观测性
- 关键事件埋点：文档打开、AI 会话、Agent 运行、影视收藏。
- 日志分级：`tauri.log`（INFO/ERROR）、`mcp.log`、`knowledge.log`。
- 健康检查：自检 DocumentServer 连通、云盘挂载状态、知识库索引状态。
- NAS 监控：采集 NAS CPU/内存/磁盘/网络指标，监控本地点推理服务延迟；若切换到第三方 API，需记录调用耗时与费用。
