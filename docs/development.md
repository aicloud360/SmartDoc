# SmartDoc 开发实施文档
*本开发文档基于《requirements.md》与《architecture.md》，用于指导 Codex/开发者逐步交付。*

## 1. 环境搭建
1. **依赖**：Rust 1.75+、Node 18+/pnpm、Tauri CLI、Leptos CLI、Docker（知识库服务）、ONLYOFFICE DocumentServer（已部署），以及三平台打包依赖（macOS Xcode CLT、Windows MSVC、Linux GTK/CSR）。未来阶段需额外安装 `tauri-mobile`、Android/iOS/鸿蒙 SDK、Web 构建链路，并评估混合框架（Capacitor/Flutter/React Native/uni-app 等）以便落地多端复用。
2. **本地配置**：
   - `.env`：`DOCUMENT_SERVER_URL`, `DOCUMENT_SERVER_JWT`, `MCP_SOCKET`, `STORAGE_ROOT` 等。
   - `pnpm install` / `cargo leptos build` 初始化前端。
   - `tauri info` 校验系统依赖。
3. **外部服务**：
   - 启动 Open WebUI / Cherry Studio（可 Docker），并暴露向量检索 API；若未就绪，以 Mock Server 占位。
   - MCP 工具以二进制或 Python 虚拟环境的形式放入 `tools/`，允许先提供 stub 响应。
   - DocumentServer Docker（`http://10.18.65.129:8085/example/`）保持在线，配置 JWT 密钥与回调。
4. **后端服务**：
   - 在 `services/api` 目录中选择 Python（FastAPI/Django）或 Node.js（NestJS/Express）实现鉴权与业务 API。
   - 可参考若依框架的权限/菜单/审计模块，但需根据 SmartDoc 需求裁剪，避免过度引入。
5. **版本控制**：
   - Git 仓库托管于 GitHub，默认分支 `main`（稳定）、`develop`（每日合流），功能特性以 `feature/<module>` 命名。
   - 使用 GitHub Actions 管道：`lint → test → build-demo → build-release → publish artifacts`，并在未来阶段扩展 `build-web`、`build-mobile`、`build-mini` 工作流。
   - 多智能体协作规范详见 `docs/agents-plan.md`，Coordinator Agent 负责审批调研、分配任务与跟踪 CI 状态。

6. **NAS 连通性**：
   - 在 NAS 上预安装 Docker/Compose、GPU/CPU 推理组件，开通 SSH/HTTP 访问。
   - `deploy/nas-compose.yml` 负责部署 DocumentServer、Auth、知识库、LLM 服务；开发者需验证 NAS 与本地/CI 的网络打通。
   - 若在 Web 服务器先行部署鉴权服务，可使用 `docker-compose.auth.yml`（FastAPI + PostgreSQL，暴露 9100/55432 端口，避免与本机 5432 冲突）。

## 2. 迭代拆分
| 阶段 | 关键任务 | 验收 |
| --- | --- | --- |
| Sprint 0 | 技术调研（云盘/知识库/AI/鉴权）+ 初始化 Tauri+Leptos 工程、配置文档服务地址、实现基础导航 | 调研记录沉淀在 `docs/investigation/`；App 能启动并打开 DocumentServer example 页；Demo stub 可展示“占位文档” |
| Sprint 1 | 云盘存储适配（本地+WebDAV）、文件上传/下载/预览 | 文件可双向同步，UI 提示清晰；同时跑通端到端 Demo 构建 |
| Sprint 2 | 知识库 ingest 流程、向量检索 API、AI 助手基础聊天 | AI 可引用知识片段，展示出处；若向量库未完成，需提供 Mock 并记测试项 |
| Sprint 3 | MCP 工具包装、智能体定义、Agent 运行面板 | Agent 可批量操作文档（例如转换），并支持多 Agent 并发演示 |
| Sprint 4 | 影视站集成、收藏/笔记与知识库联动、设置面板 | seerhut.uk 嵌入 + 笔记写入成功；Demo 流程包含影视收藏 |
| Sprint 5 | 日志、自动更新、三平台打包、端到端测试 | 交付 dmg/pkg/AppImage/msi；保留 Demo/正式双版本，并将成果推送至 GitHub Releases |
| Sprint 6（规划） | Web 端适配（Leptos SSR 与 API Proxy）+ 登录入口 + NAS 部署验证 | 产出可访问的 Web Demo + 架构文档 + NAS 部署手册 |
| Sprint 7（规划） | 移动端（Android/iOS/鸿蒙）桥接、Mini Program 设计 & 混合框架评估 | 输出 tauri-mobile/uni-app 占位工程与需求参数，桌面端保持可用 |

## 3. 代码结构与约定
- `src-tauri/src/main.rs`：入口，注册命令与插件，并根据平台注入差异化配置；为未来移动端编译预留 feature flags。
- `src-tauri/src/storage/*.rs`：实现 `StorageProvider` trait；新增 provider 需在此注册并提供 Demo stub。
- `src-tauri/src/mcp/*.rs`：MCP 客户端、工具定义、执行队列；支持 stub 工具返回；导出 Web/移动端可调用的 REST/IPC 层。
- `app/src/pages/*`：Leptos 页面；按路由拆分（cloud/knowledge/assistant/agents/media/settings），每页含 `PlaceholderView` 与 `LiveView`，并保持 SSR 兼容。
- `app/src/components/onlyoffice_host.rs`：封装 DocumentServer WebView，可切换到 `MockOnlyOffice`；未来在 Web 端用 iframe，在移动端用 WebView/JSBridge。
- `agents/*.yaml`：智能体模板文件；需要 schema 校验，并支持 `parallel: true` 配置。
- `web/`：Leptos SSR/CSR 适配层；当前主要包含接口与构建配置。
- `mobile/`：tauri-mobile/uni-app 适配工程骨架；记录平台差异和占位代码，并记录混合框架评估报告。
- `services/api/`：Python/Node 鉴权与业务 API，目录下包含 `auth`, `documentserver`, `storage`, `media` 等模块。
- `packages/`：可选跨端 SDK（Rust crate、TypeScript lib），用于封装复用逻辑。
- `docs/investigation/`：各模块的选型/调研记录（模板：背景→候选方案→评估→结论）。
- 测试：Rust 使用 `cargo test`，前端使用 `pnpm test`，端到端使用 `tauri-driver` + Playwright，额外提供占位 Demo 场景与 Web/移动仿真脚本。

## 4. 关键任务清单
1. **Tauri 命令层**
   - `open_document(file_id, mode)`：生成 DocumentServer 链接，支持 JWT，可选择 `demo` 模式返回占位链接。
   - `list_storage(provider, path)`、`upload_file`、`download_file`，默认实现 stub 行为，待 provider 可用后切换。
   - `run_mcp_tool(tool_id, payload)`：统一调用通道，支持并行执行与超时控制。
   - `save_agent(definition)`、`execute_agent(agent_id, parallel)`，可触发多 Agent 并发。
2. **Leptos/UI 层（桌面/Web/混合壳）**
   - 状态管理：以 `create_rw_signal` 存储云盘状态、聊天记录、Agent 列表。
   - WebView 控件：使用 `tauri-plugin-webview`（若需）嵌入 DocumentServer 与 seerhut.uk，并在 Demo 模式载入静态占位页面。
   - 组件库：可引入 `leptonic`/`tailwind`。
3. **知识库管线**
   - 文件上传后触发 `ingest` 队列。
   - Embedding worker（可调用 OpenAI、Local LLM API），并提供 `mock-embedding` worker 用于演示。
   - 查询接口对接 AI 助手，附带来源 metadata。
   - 模型调研要求：至少比较 2 种本地向量引擎（如 Milvus/Qdrant）与 1 种云服务，评估 NAS 部署成本，再确定接入方案。
4. **MCP & 智能体**
   - MCP 工具声明 JSON，示例：`cloud_search`, `kb_query`, `doc_convert`, `media_bookmark`。
   - Agent DSL（YAML）：`goal`, `tools`, `prompts`, `triggers`, `parallel`。
   - 执行器需支持链路追踪、失败重试、并发调度。
5. **影视集成**
   - WebView 与 JS Bridge，监听收藏事件。
   - 收藏记录存储在 SQLite，并可推送到知识库。
6. **鉴权/后端服务**
   - `services/api/auth`：注册、登录、MFA、Token 刷新、权限表。
   - `services/api/documentserver`：封装 OnlyOffice JWT 生成与回调校验，遵循官方文档 `https://api.onlyoffice.com/docs/docs-api/get-started/basic-concepts/`。
   - `services/api/gateway`：统一向桌面/Web/移动暴露 REST/GraphQL API，负责统一鉴权和速率限制。
   - 支持可插拔身份源（本地数据库、LDAP/SSO），并提供单元/集成测试样例。

7. **NAS & 部署工具**
   - `deploy/nas-compose.yml`：描述 NAS 上部署的服务、环境变量、卷映射。
   - `scripts/nas-sync.sh`：同步配置/模型到 NAS。
   - 提供混合部署指南（哪些组件在 NAS，哪些在独立 Web 服务器）。

## 5. 测试策略
- **单元测试**：Storage 适配器、MCP 工具、Agent 执行器、Auth 服务（登录/注册/JWT）。每个模块必须提供可独立运行的单元测试条目，并在 PR 描述中列出命令。
- **集成测试**：模拟 DocumentServer 回调、知识库检索、AI 聊天链路；可在 CI 中运行 Mock 版。模块完成后需附加“独立集成测试说明”，描述依赖、脚本和期望输出。
- **端到端**：通过 Playwright 驱动 Tauri 应用执行典型场景（注册/登录 → 打开文档 → 提问 AI → 收藏影视），分别在 Demo 模式与正式模式执行；Web/移动端在各自浏览器/模拟器跑同样脚本；NAS 环境需执行相同脚本验证本地模型链路。每个模块的 e2e 场景需记录在 `docs/tests/<module>.md`（由责任智能体维护）。
- **性能测试**：测量大文件上传、批量转换耗时；评估 MCP 工具超时处理，并验证多 Agent 并行场景；压测鉴权服务（登录 QPS、JWT 签发）；比较 NAS 本地模型 vs 第三方 API 的响应延迟与成本。性能用例同样要求独立脚本与复现场景。

## 6. 发布流程
1. `pnpm build && cargo build --release --features demo`（验证占位流程）。
2. `pnpm build && cargo build --release`（正式功能）。
3. `pnpm --filter services/api run build` 或 `docker build services/api`（后端鉴权服务）。
4. `docker compose -f deploy/nas-compose.yml config && docker compose up -d --build`（NAS 部署演练，可在 CI 中做 dry-run）。
5. `tauri build` 在 GitHub Actions 中以矩阵方式生成 mac dmg / win msi / linux AppImage，并附 Demo 包。
6. `pnpm build:web`、`pnpm build:mobile`、`pnpm build:mini`（可在 P1 以 stub 方式运行），确保流水线在 Web/移动阶段就绪；如评估后需要混合框架，新增 `pnpm build:hybrid`。
7. 运行自动化测试；收集覆盖率；上传 Demo 录屏/报告至 GitHub Releases。
8. 通过 `tauri signer`/`apple notarization` 等完成签名。
9. 发布版本号，更新 `CHANGELOG` 与 `docs/requirements.md` 状态，声明 Demo/正式差异。

## 7. 风险与回滚
- 若 DocumentServer 连接失败，提供离线只读模式；记录重试策略；Demo 模式默认使用离线 stub。
- MCP 工具异常时隔离进程，防止崩溃拖垮主应用；并在多 Agent 并发时按配额调度。
- Agent YAML 校验失败时阻止运行，提示具体错误行；如 `parallel: true` 配置不满足系统约束，需要提示并回退到串行执行。

## 8. 文档与沟通
- 所有新增模块需更新 `docs/architecture.md` 与 `AGENT.md`。
- Pull Request 模板包含：需求编号、测试记录、风险评估。
- 每个 Sprint 结束更新 `docs/development.md` 里程碑完成情况。
- 若评估 Tauri 未覆盖的多端能力，需在 `docs/architecture.md` & `docs/development.md` 记录混合框架选型理由、权衡与替代方案，避免盲目引入。

## 9. 并行协作策略
- **Agent 分工**：按模块/平台拆分（如“云盘 Provider”、“知识库管线”、“AI/智能体”、“影视/设置”、“三平台打包”），各 Agent 独立维护分支并通过每日 Demo 合入。
- **依赖声明**：每位 Agent 在计划中标记所需前置（如“需 Storage stub 完成”），完成后在 `docs/development.md` 中勾选。
- **并行验证**：CI 需开启并行工作流，分别执行 Demo/正式测试与三平台打包，避免相互阻塞。
- **冲突解决**：若公共模块被多个 Agent 修改，需在 PR 模板中说明接口变更并附 Demo 录屏，确保后续模块可继续迭代。
- **Git 策略**：所有 Agent 在 PR 中关联 GitHub Issue，CI 需通过后方可合并；并在 PR 描述中标记影响平台（桌面/Web/移动/小程序）。
- **调研流程**：每个 feature branch 在编码前需提交 `docs/investigation/<module>.md`，描述候选方案、NAS 部署可行性、AI 模型来源；未获批准不得进入开发阶段。

## 10. 技术调研模板（建议）
```
=== 基本信息 ===
模块：
日期/负责人：

=== 需求概述 ===
业务场景、性能/安全约束、NAS 部署要求。

=== 候选方案 ===
| 名称 | 类型 | 许可证 | 部署要求(NAS/云) | 优势 | 风险 |

=== 实验记录 ===
PoC 步骤、资源占用、与 SmartDoc 集成方式。

=== 推荐结论 ===
优先选择方案/备用方案、后续 TODO。
```
- **前端构建脚本**：桌面/Web 共享的 Leptos 前端通过 `./scripts/build-frontend.sh` 生成；该脚本依赖 `wasm-bindgen-cli`（可通过 `cargo install wasm-bindgen-cli --version 0.2.105` 安装）。
  - 当前模式使用纯 CSR + `app/static/index.html`，脚本会复制该文件到 `app/dist/index.html`。若未来改用 Leptos SSR/Vite 模板自动生成 HTML，可移除此复制步骤。
  - 可选升级路径：
    1. 采用 `cargo leptos` 完整配置（含 server/bin targets），使用 `site-root/site-pkg-dir` 自动产出静态站点。
    2. 改用 Vite/Tauri 官方模板（Node 生态），再以 `leptos` 或其他框架作为前端组件层。
