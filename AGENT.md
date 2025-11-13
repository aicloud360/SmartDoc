# AGENT 指南（Codex 专用）

## 1. 工作流
1. 阅读 `docs/requirements.md`、`docs/architecture.md`、`docs/agents-plan.md`，理解业务目标、NAS 部署约束与 Demo/正式双态要求。
2. 根据 `docs/development.md` 的阶段拆分，创建/更新计划（使用 `update_plan`），并标记当前所处阶段（桌面/Web/移动/小程序 + Demo/正式）以及涉及模块（云盘/AI/鉴权/媒体/混合框架）。
3. 在开发每个模块前，先完成 `docs/investigation/<module>.md` 技术调研；包含候选开源项目、NAS 部署可行性、AI 模型来源，经确认后方可编码。
4. 每次变更：
   - 在 `workspace-write` 模式下修改文件，并优先实现可运行占位版本。
   - 运行必要的 `pnpm` / `cargo` / `tauri` / `docker` 命令验证，至少覆盖 Demo 构建。
   - 如需访问外部网络或非工作区目录，按需申请权限；多 Agent 并行任务需在回复中声明责任边界。
   - 遇到 Tauri 不支持的多端能力时，记录混合框架评估（需求、方案、取舍），避免盲目堆叠。
5. 交付前：
   - `git status` 确认变更，`git diff --stat` 汇总影响。
   - 推送到 GitHub feature 分支，确保 Actions 成功。
   - 更新相关文档及测试记录，尤其是 Demo 步骤/脚本（桌面/Web/移动分别说明），并确保本模块的“独立测试方案”可复现。
   - 在回复中引用文件路径与行号，并说明 Demo/正式状态、目标平台与责任模块；若涉及鉴权/后端/模型调研，附接口或评估摘要。

## 2. 常用命令
```bash
# Git 基础
git switch -c feature/<module>
git commit -am "feat: xxx" # 提交前运行 lint/test
git push origin feature/<module>

# 安装依赖
yarn global add tauri-cli # 或 pnpm tauri
cargo install cargo-leptos
pnpm install
pnpm --filter services/api install # 若选择 Node 后端
pip install -r services/api/requirements.txt # 若选择 Python 后端

# 开发模式
pnpm tauri dev --features demo # 先验证占位流程
pnpm tauri dev # 正式功能就绪后

# Leptos 热加载
yarn leptos watch --features demo # 根据选用包管理器调整

# 构建
pnpm build-all # 自定义脚本，触发桌面三平台矩阵构建
pnpm build:web # Web SSR/CSR（stub 可先行）
pnpm build:mobile # tauri-mobile/uni-app（stub）
pnpm build:mini # 小程序壳（stub）
pnpm --filter services/api run build # Node 后端
docker compose build auth # Python/FastAPI 镜像
tauri build

# 测试
cargo test --all-features
pnpm test
scripts/run-e2e.sh demo && scripts/run-e2e.sh release
scripts/run-web-tests.sh # Web 端占位
scripts/run-mobile-tests.sh # 移动端占位/模拟器
pnpm --filter services/api run test # Node
pytest services/api # Python
```

## 3. 目录约定
- `src-tauri/`：Rust 后端，含存储、MCP、OnlyOffice 适配与平台打包逻辑。
- `app/`：Leptos 前端源代码，需实现 `PlaceholderView` + `LiveView`，保证 Web 同构。
- `agents/`：智能体模板，支持 `parallel` 标记。
- `tools/`：MCP 工具二进制或脚本，可放置 stub。
- `docs/`：文档集合（需求/架构/开发），同步记录 Demo/正式差异与平台路线图。
- `docs/investigation/`：模块调研记录。
- `web/`：Leptos SSR/CSR 适配层。
- `mobile/`：Android/iOS/鸿蒙/小程序占位工程。
- `services/api/`：Python/Node 鉴权与业务 API，实现登录注册、DocumentServer JWT 协同。
- `packages/`：跨端 SDK/组件库，封装可复用业务逻辑。

## 4. 集成注意事项
- DocumentServer 地址默认 `http://10.18.65.129:8085/example/`，JWT 密钥需从环境变量读取；Demo 模式可使用本地静态页面占位；Web/移动/小程序调用需通过 API Proxy。
- 影视站点 `https://seerhut.uk/` 必须在 WebView 中运行，启用内容安全策略白名单；Web/移动/小程序可通过 iframe/WebView/JSBridge 方式接入。
- MCP 工具通信统一通过本地 IPC/Socket，避免阻塞主线程；支持多 Agent 并发调度与配额限制，并提供 HTTP 层给 Web/移动调用。
- 鉴权后端参考 `services/api` 目录，可采用 Python FastAPI 或 Node NestJS；需要与 DocumentServer JWT 流程保持一致（`https://api.onlyoffice.com/docs/docs-api/get-started/basic-concepts/`），可借鉴若依框架但须评估适配度，始终以需求匹配为第一准则。
- NAS 集成：关键服务（DocumentServer、Auth、知识库、LLM）需支持在 NAS Docker 环境运行；开发者需通过 `deploy/nas-compose.yml` 验证部署脚本。
- AI 模型：同时支持 NAS 本地模型（ollama/vLLM 等）与第三方 API；在调研和实现阶段需注明模型来源、许可、资源要求。

## 5. 质量与安全
- 所有敏感配置存储在系统 Keychain（macOS）或同等方案，Windows 使用 Credential Manager，Linux 使用 Secret Service；Web/小程序端通过云端密钥服务；后台服务使用 Vault/环境变量，禁用明文。
- 对云盘与知识库操作加入审计日志，并区分 Demo 与正式日志源。
- 代码需配套单元测试；涉及系统命令前检查输入；覆盖多 Agent 并发与占位实现的回退场景。
- NAS 部署验证：涉及后端/模型改动时，需在测试说明中标明是否在 NAS 环境完成验证。

## 6. 交付物
- 功能代码 + 单元测试 + Demo stub + 可独立执行的测试方案（脚本/命令/数据说明）。
- 更新后的文档：`requirements`/`architecture`/`development` + 对应 changelog，注明 Demo/正式差异。
- 若新增智能体或 MCP 工具，附带示例配置与使用说明，并说明是否支持并行执行及三平台验证结果。
- 若涉及鉴权/后端或混合框架，需提供方案评估（动机、取舍、替代）与接口文档。
