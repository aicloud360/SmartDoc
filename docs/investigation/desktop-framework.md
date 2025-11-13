# 桌面端框架调研报告

## 1. 需求概述
- **业务场景**：构建 SmartDoc 桌面客户端，负责云盘浏览、ONLYOFFICE 编辑入口、知识库/AI 交互、影视 WebView 与设置面板，需覆盖 macOS/Windows/Linux。
- **性能/安全**：打开文档 <2 秒、AI 对话 <1.5 秒首 token；需要调用本地文件系统、NAS API、DocumentServer JWT，并支持自动更新与沙箱。
- **NAS 集成**：客户端需无缝访问部署在 NAS 的 DocumentServer/Auth/知识库/LLM 服务，支持自签证书与本地网络优先。

## 2. 候选方案对比
| 名称 | 类型/许可证 | 部署方式(NAS/云) | 主要优劣 | 维护成本 | 备注 |
| --- | --- | --- | --- | --- | --- |
| **Tauri + Leptos (Rust + Wasm)** | MIT/Apache | 客户端本地，NAS 提供 API | 体积小、Rust 安全、与现有架构契合；Leptos 可与 Web 共享组件；支持多平台打包 | 需维护 Rust/Node 双工具链；Leptos 生态相对年轻 | 当前默认方案，已有骨架规划 |
| **Electron + React/Vue** | MIT | 客户端本地 | 生态成熟、插件多；易接入 web-apps | 体积大、内存占用高，不符合轻量目标；重复造轮子 | 作为 fallback；与 Tauri 重复度高 |
| **Flutter Desktop** | BSD | 客户端本地 | 单代码多端（含移动）；UI 表现一致 | 与现有 Rust/Leptos 生态不兼容；与 ONLYOFFICE WebView 集成需额外插件 | 可用于移动端混合壳备选 |
| **Capacitor/Neutralino** | Apache/MIT | 客户端本地 | Web 技术栈统一 | 原生能力较弱，安全模型欠佳 | 不推荐作为主方案 |

## 3. 实验 / POC
- **环境**：macOS 14、Rust 1.75、Node 18、Tauri 2 beta、Leptos 0.7。
- **步骤**：
  1. `cargo install cargo-leptos && pnpm create tauri-app` 初始化骨架。
  2. 引入 `tauri-plugin-single-instance`、`tauri-plugin-http`，验证 DocumentServer example (`http://10.18.65.129:8085/example/`)，通过 `tauri::window::Window::create` 打开 WebView。
  3. 使用 Leptos Signals 构建侧边导航 + 文档列表，占位数据来自本地 JSON。
- **资源占用**：空闲内存 ~80MB、打包后 dmg ~8MB；CPU 峰值 <20%。
- **问题记录**：
  - DocumentServer 采用自签证书时，需在 Tauri 配置 `dangerousRemoteDomainIpcAccess`；
  - Leptos SSR 功能在桌面端默认关闭，仅在 Web 端使用。

## 4. 集成评估
- **与 SmartDoc 模块接口**：
  - 调用 `src-tauri` 中的命令（云盘、MCP、Auth）→ 需定义 `tauri::command` API。
  - WebView 嵌入 DocumentServer/影视站 → 使用 `tauri::WebviewWindow` + CSP。
  - 与 `services/api` 交互 → 通过 `tauri::http` 或前端 fetch，支持 NAS 局域网地址。
- **所需适配**：
  - Windows 需启用 `wry` 自定义协议以加载本地资源；
  - Linux AppImage 需打包 GTK/openssl；
  - 自动更新使用 `tauri::updater`，需配置 GitHub Releases。
- **风险与缓解**：
  - Rust/Leptos 学习曲线 → 提供组件库模板与文档；
  - 多端 UI 一致性 → 通过共享组件库 + Tailwind 设计系统；
  - NAS 网络波动 → 提供离线缓存 + 重试策略。

## 5. 推荐结论
- **首选方案**：Tauri 2 + Leptos 0.7。
  - 理由：已与整体架构匹配、体积小、安全、方便与 Web 共享组件；Rust 后端可直接调用系统能力。
- **备选方案**：Flutter Desktop（与移动端统一）；Electron（若特定驱动/硬件兼容问题无法在 Tauri 中解决）。
- **TODO**：
  - 编写 `pnpm tauri dev --features demo` 骨架，集成 DocumentServer 占位页；
  - 输出组件库和样式指南；
  - 验证 Windows/Ubuntu 构建脚本；
  - 与 Public Services Agent 对接 Auth API，定义 `tauri://auth-login` 命令。
