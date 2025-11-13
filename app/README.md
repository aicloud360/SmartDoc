# SmartDoc Leptos 前端（占位）

此目录用于存放桌面/Web 端共享的 Leptos 组件。当前仅提供 CSR 占位，后续会根据 `cargo-leptos` / `pnpm` 脚本进行集成。

## 快速开始
1. 安装依赖
   ```bash
   cargo install cargo-leptos
   ```
2. 开发模式（CSR Demo）
   ```bash
   cargo leptos serve --features csr
   ```
3. 构建输出（供 Tauri/Web 使用）
   ```bash
   cargo leptos build --release --features csr
   ```

> 在 Tauri 集成阶段，会通过 `pnpm tauri dev --features demo` 调用此目录生成的 Wasm/JS 资源。
