# 鉴权与公共服务调研报告

## 1. 需求概述
- **Tauri 自有鉴权**：为 SmartDoc 桌面/Web/移动端提供登录、注册、MFA、权限管理与后台 UI，该服务部署在独立 Web 服务器（可 Docker）而非 NAS 上运行，确保可扩展性。
- **文档服务 / NAS**：ONLYOFFICE DocumentServer 与 NAS 继续维持各自鉴权；短期通过服务层映射用户 ID、签发二次 JWT；长期规划统一 IdP/SSO。
- **调研目标**：比较“自研（FastAPI/NestJS）”与“引入开源后台（若依、Keycloak、Supabase、PocketBase 等）”的成本、功能覆盖度，与 DocumentServer/NAS 的对接复杂度。
- **部署要求**：提供 Docker Compose/Helm 脚本，可横向扩展；同时提供与 DocumentServer/NAS 的适配配置。

## 2. 候选方案对比
| 名称 | 类型/许可证 | 部署方式 | 主要优劣 | 维护成本 | 适配评估 |
| --- | --- | --- | --- | --- | --- |
| **FastAPI + Pydantic** | Python / MIT | 轻量 Docker，NAS 友好 | 语法简单、生态丰富、与 Python 脚本/AI 工具兼容；性能对大并发一般 | 依赖少，易写 PoC；需注意 GIL，重并发需 gunicorn/uvicorn workers | ✅ 推荐：与 NAS/Python 工具链一致，便于调用 DocumentServer SDK |
| **Node.js (NestJS)** | Node / MIT | Docker | 结构化、TypeScript、依赖多 | 中等：需维护 TS 构建；优势是前端/后端语言统一 | 备选：若团队更熟悉 TS，可迁移 |
| **若依 (Java/Spring)** | Java / MIT | 需 JDK + MySQL | 开箱完整 RBAC/审计/管理后台 | 重量级、定制成本高 | 可作为统一用户后台候选；需开发 DocumentServer/NAS 适配插件 |
| **Keycloak / Ory / Supabase / PocketBase** | Java/Go/TS | Docker / 托管 | 标准 OIDC/SAML、托管方案 | 学习曲线较高，需要额外 UI/二次开发 | 适合未来接入企业 SSO；需评估许可、运维成本 |

## 3. 实验 / POC
- 选择 FastAPI + Uvicorn：在 macOS 本地创建 `services/api/main.py`，实现 `/health`, `/auth/login`, `/document/token` 等占位接口，使用 `python-dotenv` 读取 `DOCUMENT_SERVER_SECRET`。
- 使用 `httpx` 向 DocumentServer example 发出请求（模拟 `payload`），暂未签名；后续接入 JWT 签名库 `pyjwt`。
- Docker 镜像基于 `python:3.11-slim`，运行 `uvicorn app.main:app --host 0.0.0.0 --port 9000`，CPU < 2%，内存 < 60MB。

## 4. 集成评估
- **接口**：桌面/Web/移动通过 HTTPS 调用 `/api/v1/auth/login` 获取 `access_token`；Tauri 端通过 `tauri::http` 或 `fetch` 调用。
- **DocumentServer JWT**：`/api/v1/document/token` 接收 `file_id`、`permissions`，返回 `token` + `url`；后续可支持 `callbackUrl`。
- **NAS 配置**：使用 `.env` + `config.toml` 存储 DocumentServer 地址、NAS 路径；未来可挂载 NAS Keyring。
- **扩展**：可添加 `async` 任务处理 DocumentServer 回调、审计日志写入 SQLite/PostgreSQL。

## 5. 推荐结论
- **阶段策略**：
  1. Sprint 0：以 FastAPI + Uvicorn 快速交付 Tauri 登录/注册 API 与 DocumentServer JWT PoC（已经开始）。
  2. Sprint 1：并行调研若依/Keycloak/Supabase/PocketBase，确定是否需要引入完整后台；若选用，设计用户映射和同步 Flow。
  3. Sprint 2+：实现 Tauri ↔ DocumentServer ↔ NAS 的用户映射与 SSO 网关。
- **TODO**：
  - 完成 FastAPI PoC（登录/注册/管理 API + Docker Compose）。
  - 提交开源后台对比结果（功能、部署、定制成本），决定是否接入。
  - 设计用户映射模型与 token 翻译服务。
  - 在 `docs/tests/public-services.md` 基础上编写 pytest 集成测试与 NAS 部署脚本。
