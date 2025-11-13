# macOS 本机打包测试指引

本文描述在 macOS（Apple Silicon）下为 SmartDoc 运行 `cargo tauri build` 的操作流程，并记录注意事项。

## 前置依赖
1. **Rust 与 Tauri CLI**：
   ```bash
   rustup default stable
   cargo install tauri-cli --locked
   ```
2. **Homebrew 组件**：Tauri 在 macOS 上生成 `.dmg` 需要 [`create-dmg`](https://github.com/create-dmg/create-dmg)。若 `brew install create-dmg` 因权限失败，可按提示修复 `/opt/homebrew` 权限，或只先生成 `.app` 包（命令见下）。
3. **前端依赖**：仓库附带 `scripts/build-frontend.sh` 会自动编译 Leptos 前端，无需额外安装 Node。

## 构建流程
```bash
# 生成全量（含 .app 和 .dmg）
cargo tauri build

# 若缺少 create-dmg，可临时只出 .app 包
cargo tauri build --bundles app
```
构建完成后产物位于：
- `.app`：`src-tauri/target/release/bundle/macos/SmartDoc.app`
- `.dmg`：`src-tauri/target/release/bundle/dmg/SmartDoc_<version>_arch.dmg`

## 常见问题
| 问题 | 解决方案 |
| --- | --- |
| `bundle_dmg.sh` 报错 `create-dmg: command not found` | 通过 `brew install create-dmg` 安装，或按提示修复 `/opt/homebrew` 目录写权限后再安装。 |
| 构建耗时较长 | 首次构建会拉取大量 Rust 依赖，可以使用 `cargo tauri build --bundles app` 测试前端，CI 会负责生成其它平台包。 |
| 无法访问 DocumentServer（10.18.65.* 网段） | 本地验证需在对应网段，CLI 内置 `check_lan_access` 命令会阻止未授权入口。 |

## 提交前检查
1. `cargo fmt` / `cargo check`（包含 `src-tauri`）。
2. 至少一次成功的 `cargo tauri build --bundles app`（或完整 build）并将结果写入 PR 说明。
3. 若已解决权限问题，建议附 `.dmg` 生成日志以便他人复现。
