# SmartDoc 智能文档平台需求说明书

## 1. 项目背景
- 用户已在本地（macOS）部署 ONLYOFFICE DocumentServer，访问地址 `http://10.18.65.129:8085/example/`。
- 计划构建 Tauri + Leptos 桌面应用“SmartDoc”，作为智能文档工作台，整合云盘、知识库、AI 助手、智能体搭建、ONLYOFFICE 文档操作以及影视资源站点。
- 项目目标是在 Codex 自动化开发流程下，提供可复用的需求/架构/开发文档与 AGENT 工作说明，实现快速交付。

## 2. 总体目标
1. 以 Tauri（前端框架选型 Leptos）构建跨平台桌面端，并在 macOS/Windows/Linux 均可运行与打包，同时保持与 Web 端共享代码；若某端能力 Tauri 暂不支持，需规划混合框架（如 Capacitor、React Native、Flutter、uni-app 等）以复用业务模块。
2. 默认连接 ONLYOFFICE DocumentServer，实现文档预览/编辑/协同。
3. 提供统一的云盘与知识库入口，方便导入、挂载及索引文件。
4. 集成 MCP（Model Context Protocol）工具链，为 AI 助手与智能体搭建提供扩展能力。
5. 内嵌影视资源站点 `https://seerhut.uk/`，支持授权场景下的内容浏览与收藏。
6. 以开源项目（如 ONLYOFFICE/web-apps、Open WebUI、Cherry Studio 等）作为二次开发基座，贯穿端到端构建链路（开发、CI、发布），并将代码托管在 GitHub 以便持续协作；框架选型需以需求适配度为先，避免盲目堆叠。
7. 优先保障 NAS（网络附加存储）环境的部署需求：核心数据与关键服务需能够安装/运行在 NAS 上，并与 NAS 系统提供的账户、存储策略联动。

## 3. 用户角色与场景
| 角色 | 需求场景 |
| --- | --- |
| 文档使用者 | 打开云盘文件；用 ONLYOFFICE 进行编辑协作；检索知识库；观看影视资源。 |
| AI 使用者 | 向 MCP 驱动的助手提问；调用知识库/文档上下文；配置个人智能体。 |
| 运维/管理员 | 配置文档服务地址；管理云盘挂载；设置权限与密钥；监控任务。 |
| 开发者/Agent | 扩展前后端插件；编写 MCP 工具；维护自动构建脚本。 |

## 4. 功能需求
### 4.1 云盘存储
- 支持挂载第三方网盘（WebDAV、SMB、阿里云盘等）或本地路径。
- 提供“上传/下载/移动/复制/版本记录”基本操作。
- 与 ONLYOFFICE 编辑器打通：可从云盘进入编辑页，也可在编辑器保存回云盘。

### 4.2 知识库
- 允许选择云盘目录或独立数据集作为知识库来源。
- 提供分词/Embeddings/向量检索；可复用 Open WebUI 或 Cherry Studio pipeline。
- 支持多租户或工作区概念，每个工作区可绑定多知识源。

### 4.3 AI 助手（MCP）
- 界面提供聊天、多轮对话、引用来源展示。
- MCP 工具列表最小集合：云盘检索、知识库检索、ONLYOFFICE 文档操作、影视资源索引。
- 可配置模型（本地 LLM、云端 API）以及限流策略。

### 4.4 智能体搭建
- 通过可视化流程或 YAML 定义 Agent：指令集、工具权限、触发条件。
- 支持保存/导入/导出 Agent 模板，便于共享。
- Agent 可调用 AI 助手同样的 MCP 工具集合。

### 4.5 ONLYOFFICE 文档操作
- 通过 DocumentServer API（参考 `https://github.com/ONLYOFFICE`）完成：
  - 在线预览与编辑（Docx、Xlsx、Pptx、Pdf 等）。
  - 协同编辑与评论。
  - 模板填充、表单生成（如 `ONLYOFFICE/document-builder`）。
  - 文档格式转换与批处理任务。
- 支持 Tauri 端调用 DocumentServer example 页面或自定义 UI（参考 `ONLYOFFICE/web-apps`）。

### 4.6 用户与鉴权服务
- **Tauri 应用自有鉴权**：需提供登录、注册、找回密码、MFA、用户管理后台，可部署在独立 Web 服务器（Docker）并暴露 API；优先调研能否复用开源后台（若依、Supabase、PocketBase、Keycloak/Ory 等），若不合适则自研（FastAPI/NestJS）。
- **文档服务/NAS 鉴权并行存在**：ONLYOFFICE DocumentServer 与 NAS 均保留各自的认证体系；Tauri 鉴权成功后，需要通过服务层映射或 SSO 方式向文档服务、NAS 传递身份（如二次 JWT、API Key、SSO 回调）。
- **三方互通策略**：短期内仅打通 Tauri ↔ 文档服务（JWT）与 Tauri ↔ NAS（API Token）；长期目标是建立统一身份映射或集中式 IdP，同时允许每个系统独立运行。
- **部署要求**：鉴权服务需提供 Docker Compose/Helm 脚本，支持 Web 服务器或 NAS 上运行；配置文件区分 Tauri/文档/NAS 的 client id/secret。
- **前端集成**：桌面/Web/移动需共用同一 OAuth/Token 流程，支持刷新、注销、游客模式。

### 4.7 影视资源站点集成
- 在 Tauri 内嵌 `https://seerhut.uk/`（WebView），提供收藏/历史记录等本地功能。
- 可与知识库互通：例如将观影笔记写入知识库。

### 4.8 系统设置
- 文档服务地址、MCP 配置、模型密钥、云盘凭证均可在设置面板统一管理。
- 提供日志查看与导出功能，便于排错。
- 设置面板需标注哪些服务运行在 NAS、哪些运行在 Web/云端，支持对 NAS 服务的远程健康检查。

## 5. 非功能需求
- **安全**：本地加密敏感配置（Keychain/Keyring）；云盘权限隔离；AI 对话脱敏。
- **性能**：常用操作（打开文档、检索）< 2 秒响应；支持离线缓存。
- **可扩展性**：模块化插件架构，可替换存储/知识库/AI Provider。
- **可运维性**：提供日志、健康检查、版本信息；支持自动更新。
- **工程治理**：项目需使用 Git 仓库管理（默认 GitHub），约定分支策略（如 `main` + `develop` + feature branches），并配置 CI/CD（GitHub Actions 或同类）执行测试、构建与发布。
- **可复用性/模块化**：核心业务（存储、知识库、AI、鉴权、媒体、OnlyOffice 适配）需封装成独立模块/SDK，面向桌面/Web/移动统一调用；若 Tauri 无法原生支持某平台功能，需提前规划混合框架桥接层，确保代码复用度。
- **部署适配**：需区分 NAS 内部部署与独立 Web/云端部署的组件（例如 DocumentServer、鉴权服务、知识库索引、AI 模型推理）；提供一键安装/配置脚本。
- **可测试性**：每个模块在完成后必须具备可单独执行的测试方案（单元/集成/e2e 脚本、模拟数据及环境说明），并在文档中登记测试入口与依赖。

## 6. 外部系统与接口
| 名称 | 方式 | 说明 |
| --- | --- | --- |
| ONLYOFFICE DocumentServer | HTTPS/REST + WebSocket | 复用已部署服务；支持 JWT 认证与回调。 |
| MCP 工具 | 本地进程/IPC | 由 Tauri 后端调用，暴露文件、知识库检索等能力。 |
| 鉴权服务 | HTTP/REST | 自研 Python/Node 服务或复用若依等框架；与 DocumentServer、客户端协同 JWT/SSO。 |
| NAS 集成 | 文件协议/API | 对接 NAS 提供的账户、文件、监控接口，托管数据服务与模型。 |
| 云盘服务 | WebDAV/SDK/本地 FS | 根据挂载类型选择协议。 |
| 影视资源站点 | HTTPS | WebView 内嵌。 |

## 7. 开源选型建议
- 前端容器：Tauri + Leptos（Rust + Wasm）。
- 文档 UI：ONLYOFFICE/web-apps 二次封装。
- 知识库/向量检索：Open WebUI、Cherry Studio 或 Milvus + LlamaIndex 方案。
- AI Orchestration：MCP 参考实现、LangChain Rust 生态。
- 鉴权：Keycloak/Ory 轻量方案，或本地账户体系。
- AI 模型：支持 NAS 本地部署（如 ollama、vLLM、RWKV 等）与第三方 API（OpenAI、Azure、通义千问等）并行配置。

## 8. 验收标准
1. 能够上传文件到云盘并在 ONLYOFFICE 中打开、编辑、保存，三平台一致。
2. AI 助手可引用知识库内容回答问题，并显示引用。
3. 智能体可配置并执行至少一种自动化流程（如批量转换），每次新增 Agent 前需通过 Demo 验证。
4. 影视站内嵌页面可正常浏览，且可与本地收藏同步。
5. 所有核心配置可在设置界面修改并持久化，并通过端到端构建产出的安装包验证。
6. Demo 流程：每个模块上线前需提供可运行占位实现（Stub），并在测试通过后升级为正式功能。
7. GitHub Actions（或等效工具）中成功运行“lint → test → build → 发布”流水线，自动生成桌面端安装包，并上传构建工件。
8. 鉴权流程在三端可用，能成功对接 DocumentServer，并完成登录→打开文档→权限校验的端到端链路。
9. AI 助手可在 NAS 本地模型与外部 API 间切换，并记录模型来源以保障审计。

## 9. 平台路线与交付策略
- **阶段目标**：
  - P1（当前迭代）：完成桌面端（macOS/Windows/Linux）Tauri 应用，作为最小可交付。
  - P2：在同一代码基座上启用 Web 端（Leptos SSR/SPA），通过浏览器访问核心能力。
  - P3：拓展移动端（Android/iOS/鸿蒙）及小程序（微信/鸿蒙/支付宝等），复用核心业务逻辑与 UI 组件；此阶段聚焦架构设计与接口定义，先行完成需求/参数文档。
- **端到端构建**：提供 `scripts/build-all.{sh,ps1}`，在 CI 中跑通“拉取代码→安装依赖→占位 Demo → 正式模块”完整流程，确保三平台桌面端均可复现；为 Web/Mobile 预留构建脚本占位。
- **迭代节奏**：每个模块先以占位组件或 Mock 数据跑通 UI/交互，随后替换为真实实现；每阶段需保留 Demo 录屏或脚本，并在 GitHub Releases 中归档。
- **多 Agent 协作**：支持将 Sprint 内任务拆分为“平台兼容、MCP 工具、AI 体验、影视模块、Web/移动预研”等子任务，由不同 Agent 并行执行，统一在每日构建中合并。
- **质量门禁**：模块完成后必须通过端到端 Demo、单元/集成测试，以及三平台打包校验后才能进入下一模块开发；未来 Web/Mobile 模块需在模拟器/浏览器端通过相同门禁。
- **技术调研门禁**：每个模块（云盘、知识库、AI 助手、鉴权等）在开发前需提交调研记录，明确候选开源项目/模型、评估标准与推荐方案，经确认后方可开始开发。

## 11. 技术调研与选型要求
- **调研流程**：
  1. 提出需求 → 收集 2-3 种开源或商用方案（含许可证、社区活跃度、兼容性、可部署性）。
  2. 评估在 NAS 部署、资源占用、可扩展性、后续维护成本，并考虑与现有模块的集成复杂度。
  3. 形成《调研记录》（可放入 `docs/investigation/<module>.md`），经负责人确认后再进入开发。
- **重点模块**：
  - 知识库/向量模型（Open WebUI、Milvus、Qdrant、LlamaIndex、Ollama 等）。
  - 云盘/存储（SeaFile、Nextcloud、Rclone、NAS 自带协议等）。
  - AI 助手模型（NAS 本地 LLM、开源 embedding、第三方 API）。
  - 鉴权/若依类框架、NAS 系统接口封装。
- **评估维度**：性能指标、二次开发成本、社区维护度、许可证、NAS 兼容性、后续商业化风险。

## 10. 里程碑（建议）
| 阶段 | 目标 | 预估周期 |
| --- | --- | --- |
| M1 原型 | 完成 Tauri + Leptos 骨架、ONLYOFFICE 连接、云盘浏览 | 2 周 |
| M2 智能增强 | 接入知识库、AI 助手、MCP 工具 | 2 周 |
| M3 智能体与影视 | 智能体搭建器、影视站内嵌、设置面板 | 1.5 周 |
| M4 打磨 | 自动更新、日志、测试与上线准备 | 1 周 |
