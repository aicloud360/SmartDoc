# SmartDoc 开发任务矩阵（Sprint 0 起始）

| 模块 | 智能体 | 当前阶段 | 任务概述 | 独立测试方案 | 状态 |
| --- | --- | --- | --- | --- | --- |
| 桌面（Desktop Shell + UI） | Desktop Agent | Sprint 0 | - 完成 `docs/investigation/desktop-framework.md` 调研<br>- 初始化 Tauri+Leptos 导航骨架，接入 DocumentServer 占位页 | `pnpm tauri dev --features demo` + `scripts/run-e2e.sh demo` | 开发进行中（骨架+测试方案就绪） |
| 公共服务 / 鉴权 | Public Services Agent | Sprint 0 | - 编写/更新 `docs/investigation/auth.md`（比较自研 vs 若依/Keycloak/Supabase 等开源后台）<br>- 初始化 Tauri 自有鉴权 API（登录/注册/管理后台），规划与 DocumentServer/NAS 的映射方案 | `curl http://localhost:9000/health` + 未来 `pytest` | 调研与骨架完成（需选择最终方案） |
| 影视资源模块 | Desktop Agent | Sprint 0 | - 在 Tauri 中嵌入 `seerhut.uk` WebView，占位收藏/笔记功能，与登录态联动 | `cargo tauri dev` 手动验证：打开影视站+保存笔记 | 完成占位功能 |
| 知识库 / AI | Knowledge/AI Agent | Sprint 0 | - `docs/investigation/ai-models.md` 调研 NAS 本地 LLM + embedding<br>- 规划 MCP 工具接口 | `scripts/run-ai-mock.sh`（占位） | 待启动 |
| 移动 & 小程序 | Mobile Agent | Sprint 0 | - `docs/investigation/mobile.md`（tauri-mobile vs uni-app）<br>- 创建 `mobile/` 占位工程 | `pnpm build:mobile`（stub） | 待启动 |
| Web / SSR | Web/SSR Agent | Sprint 0 | - `docs/investigation/web-ssr.md`<br>- 初始化 `web/` SSR + API proxy stub | `pnpm build:web` + `scripts/run-web-tests.sh` | 待启动 |
| NAS 部署 & 运维 | Public Services Agent + Coordinator | Sprint 0 | - 起草 `deploy/nas-compose.yml` 与 `scripts/nas-sync.sh` 骨架<br>- 验证 NAS 连通性 | `docker compose -f deploy/nas-compose.yml config` + 手动健康检查 | 待启动 |
| 统筹/协调 | Coordinator Agent | Sprint 0 | - 维护此任务矩阵与里程碑<br>- 审批调研文档、组织每日同步 | N/A | 进行中 |

> 注：每项任务在提交 PR 前必须将“独立测试方案”列中的命令/脚本补齐并验证，通过后方可进入下一阶段。
