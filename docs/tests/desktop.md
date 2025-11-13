# 桌面模块独立测试方案

## 1. 目标
验证 SmartDoc 桌面壳（Tauri + Leptos 占位）可独立运行，并暴露基本命令：
- `open_document_demo`
- `health_check`

## 2. 前置条件
- 已安装 Rust 1.75+、Node 18+。
- 本地可访问 DocumentServer Demo (`http://10.18.65.129:8085/example/`)，即便作为占位。

## 3. 测试步骤
1. **准备前端资源（Leptos CSR）**
   ```bash
   ./scripts/build-frontend.sh
   ```
   - 脚本逻辑：运行 `cargo leptos build --features csr`，然后使用 `wasm-bindgen` 输出 `app/dist/pkg/`，并复制 `app/static/index.html`。
   - 若未来改用 Leptos SSR 或 Vite 模板（自动生成 HTML），可替换脚本逻辑为“直接将生成目录指向 `frontendDist`”，但目前仍需该 static 模板。
2. **运行 Tauri Demo**
   ```bash
   cd ../src-tauri
   cargo tauri dev
   ```
   （请确保公共服务 `docker-compose.auth.yml` 或本地 FastAPI 也在运行，API 默认为 `http://localhost:9100`）。
3. **验证命令**
   - 在应用窗口输入用户名/密码（需先在公共服务注册），点击“登录”后再点击“获取示例链接”。
   - 在“影视资源”卡片中点击“打开影视站”验证 Tauri WebView 是否加载 https://seerhut.uk/；在文本框中输入示例收藏并点击“保存到本地列表”确认占位收藏功能可用。
   - 如需调试，可打开 DevTools 执行：
     ```js
     await window.__TAURI__.core.invoke("health_check")
     ```

## 4. 期望结果
- 应用启动并显示“SmartDoc 桌面占位”界面。
- `open_document_demo` 返回携带 DocumentServer URL 的 JSON。
- `health_check` 返回版本字符串。

## 5. 后续扩展
- 接入真实云盘/知识库/AI 数据后，需为每个页面补充 e2e 脚本，并将验证结果回填至 `docs/plan/tasks.md`。
