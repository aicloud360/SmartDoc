# SmartDoc 多智能体协同方案

## 1. 总体目标
- 通过多编程智能体并行开发，加快桌面/Web/移动/NAS/公共服务等模块交付。
- 引入“协调智能体”统一管理需求、调研、接口与发布时间，确保架构一致性。
- 将“技术调研→开发→测试→NAS 部署验证”形成标准流水线，防止重复劳动。

## 2. 智能体角色与职责
| 智能体 | 主要职责 | 关键交付物 | 依赖 |
| --- | --- | --- | --- |
| **Coordinator Agent** | 需求澄清、调研审批、里程碑跟踪、冲突协调、NAS 部署验收、CI 监控 | - 每日/每周进度简报<br>- `docs/plan/roadmap.md` 更新<br>- 合并窗口与发布节奏 | 需要获取所有模块状态；可调用 CI、NAS 监控接口 |
| **Desktop Agent** | 负责 `src-tauri/` + `app/` 桌面端（macOS/Windows/Linux），实现云盘、知识库、AI、影视等 UI/命令交互 | - 功能代码 + `pnpm tauri dev/build` 验证<br>- 桌面端端到端测试报告<br>- Tauri 插件/命令文档 | 依赖公共服务 API、知识库/AI 工具、调研结论 |
| **Mobile Agent** | 构建 `mobile/`（tauri-mobile/uni-app/混合框架评估）、小程序壳、移动端 UI/网络适配 | - 移动端占位/正式代码、模拟器脚本<br>- 混合框架调研报告<br>- 小程序打包流程 | 依赖公共服务 API、Coordinator 评估通过的方案 |
| **Web/SSR Agent** | 维护 `web/` Leptos SSR/CSR 与 API Proxy，确保与桌面共享组件 | - Web 构建脚本与部署指南<br>- SSR 性能测试与安全策略 | 依赖公共服务 API、组件库 |
| **Public Services Agent** | 负责 `services/api/`（Python/Node）、DocumentServer JWT、NAS Compose、AI 模型接入、`packages/` SDK | - Auth/业务 API 代码 + 测试<br>- `deploy/nas-compose.yml` 与 `scripts/nas-sync.sh`<br>- AI 模型切换策略（NAS vs 第三方） | 需获取 DocumentServer 参数、NAS 访问权限 |
| **Knowledge/AI Agent** | 负责知识库 ingest、向量检索、MCP 工具、AI 助手模型调度 | - `docs/investigation/ai-*.md` 调研<br>- 向量引擎/模型部署脚本<br>- MCP 工具实现与测试 | 依赖 NAS GPU/CPU 资源、公共服务 Agent 的 API 网关 |

> 注：若团队规模有限，可由单个智能体兼任多个角色，但仍需按照此职责划分提交调研与交付物。

## 3. 工作流程
1. **需求/任务创建**：Coordinator Agent 在 `docs/plan/tasks.md` 或 Issue 中创建任务，指派具体智能体，并附上验收标准。
2. **技术调研**：被指派智能体在 `docs/investigation/<module>.md` 填写模板（方案对比、NAS 兼容性、AI 模型来源等），提交给 Coordinator 审批。
3. **开发实施**：调研通过后，智能体创建 `feature/<module>` 分支，按 AGENT.md 流程实现功能，提交代码+单元测试。
4. **联调与测试**：
   - 桌面/Web/移动智能体运行各自 e2e 脚本；
   - 公共服务智能体运行 API/性能测试并更新 NAS 部署；
   - Coordinator 汇总测试结果，触发 CI 矩阵构建。
5. **NAS 部署与验收**：公共服务智能体在 NAS 上更新 Compose 服务，Desktop/Web/Mobile 智能体执行连接验证；Coordinator 签署验收。
6. **发布与文档**：所有智能体更新相关文档（需求/架构/开发/AGENT/Changelog）并记录影响范围。

## 4. 通信与同步
- **每日同步**：Coordinator 提供 10 分钟会议或异步记录，跟踪阻塞项（如 NAS 资源、模型许可）。
- **PR 注释规范**：PR 模板包含“智能体名称、影响模块、调研链接、NAS 验证情况”。
- **Issue 标签**：`agent:desktop`, `agent:mobile`, `agent:public-services`, `agent:ai`, `agent:coordinator` 等，便于过滤工作量。

## 5. 并行开发原则
1. **接口先行**：公共服务 Agent 在实现前先定义 gRPC/HTTP schema，供桌面/移动/Web 预先 Mock；
2. **Mock 驱动**：在真实服务尚未完成时，各智能体使用 stub/mock，并在调研文档中注明替换计划；
3. **冲突解决**：当多个智能体修改公共模块（如 `packages/` 或 `docs/architecture.md`）时，需提前在调研阶段协调；
4. **NAS 优先**：任何涉及数据/模型的改动，需确认 NAS 部署是否受影响，并在 PR 中附 NAS 验证截图或日志；
5. **安全与合规**：AI 模型调用需注明来源（NAS/第三方 API）、许可证与数据出境策略。

## 6. 首批任务建议
| 模块 | 智能体 | 初始任务 |
| --- | --- | --- |
| 桌面 | Desktop Agent | 搭建基础导航、ONLYOFFICE 打开占位文件、输出调研 `desktop-framework.md` |
| 公共服务 | Public Services Agent | 初始化 `services/api`，完成 DocumentServer JWT PoC，编写 `docs/investigation/auth.md` |
| 知识库/AI | Knowledge/AI Agent | 调研 NAS 本地 LLM + embedding 方案（Ollama、Milvus/Qdrant），提交调研文档 |
| 移动 | Mobile Agent | 评估 tauri-mobile vs uni-app，输出 `docs/investigation/mobile.md` 并创建占位工程 |
| Web | Web/SSR Agent | 建立 SSR 项目骨架 + API proxy stub |
| Coordinator | Coordinator Agent | 建立任务矩阵、同步脚本、维护文档和 CI 状态 |
