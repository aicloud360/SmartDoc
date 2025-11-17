#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use get_if_addrs::get_if_addrs;
use local_ip_address::local_ip;
use serde::Serialize;
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
// thread module is used elsewhere; keep import if future background tasks are added.
use tauri::menu::MenuBuilder;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Listener, Manager, RunEvent, WindowEvent};
use tauri_plugin_single_instance::init as single_instance;

const DOCUMENT_SERVER_URL: &str = "http://10.18.65.129:8085/example/";
static FRONTEND_READY: AtomicBool = AtomicBool::new(false);
static USER_HIDDEN: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Debug)]
struct DocumentLink {
    url: String,
    filename: String,
}

#[derive(Serialize, Debug)]
struct NetworkGate {
    allowed: bool,
    ip: String,
}

/// 返回一个示例 DocumentServer 链接，后续将根据真实文件/JWT 动态生成。
#[tauri::command]
fn open_document_demo(filename: String) -> DocumentLink {
    let url = format!("{}?demo_file={}", DOCUMENT_SERVER_URL, filename);
    DocumentLink { url, filename }
}

/// 简单健康检查命令，便于 NAS/CI 调用。
#[tauri::command]
fn health_check(app: AppHandle) -> String {
    let version = app
        .config()
        .version
        .clone()
        .unwrap_or_else(|| "0.0.0".into());
    format!("SmartDoc Desktop Stub running, version {}", version)
}

/// 校验本机是否处于 10.18.65.* 网段。
#[tauri::command]
fn check_lan_access() -> NetworkGate {
    let mut fallback_ip: Option<String> = None;
    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            if let IpAddr::V4(addr) = iface.ip() {
                let ip_str = addr.to_string();
                if ip_str.starts_with("10.18.65.") {
                    return NetworkGate {
                        allowed: true,
                        ip: ip_str,
                    };
                }
                if fallback_ip.is_none() && !ip_str.starts_with("127.") {
                    fallback_ip = Some(ip_str);
                }
            }
        }
    }

    let detected = fallback_ip
        .or_else(|| local_ip().map(|ip| ip.to_string()).ok())
        .unwrap_or_else(|| "unknown".into());

    NetworkGate {
        allowed: false,
        ip: detected,
    }
}

/// 前端渲染完成后由 Web 侧主动调用，避免 Windows 启动阶段的白屏。
#[tauri::command]
fn frontend_ready(app: AppHandle) -> Result<(), String> {
    FRONTEND_READY.store(true, Ordering::SeqCst);
    if !USER_HIDDEN.load(Ordering::SeqCst) {
        reveal_main_window(&app);
    }
    Ok(())
}

fn reveal_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        USER_HIDDEN.store(false, Ordering::SeqCst);
    }
}

fn init_tray(app: &tauri::App) -> tauri::Result<()> {
    let handle = app.handle();
    let tray_menu = MenuBuilder::<_, AppHandle>::new(handle)
        .text("show_main", "显示 SmartDoc")
        .separator()
        .text("quit_app", "退出 SmartDoc")
        .build()?;

    let mut builder = TrayIconBuilder::new()
        .menu(&tray_menu)
        .tooltip("SmartDoc 已在后台运行");
    if let Some(icon) = app.default_window_icon().cloned() {
        builder = builder.icon(icon);
    }

    let tray_icon = builder
        .on_menu_event(|app_handle, event| match event.id().as_ref() {
            "show_main" => reveal_main_window(app_handle),
            "quit_app" => app_handle.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button,
                button_state,
                ..
            } => {
                if button == MouseButton::Left && button_state == MouseButtonState::Up {
                    reveal_main_window(tray.app_handle());
                }
            }
            TrayIconEvent::DoubleClick { .. } => reveal_main_window(tray.app_handle()),
            _ => {}
        })
        .build(app)?;

    app.manage(tray_icon);
    Ok(())
}

fn prevent_close_to_tray(window: &tauri::Window, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        let _ = window.hide();
        USER_HIDDEN.store(true, Ordering::SeqCst);
    }
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(single_instance(|app, _, _| {
            // 聚焦已有窗口，防止重复实例。
            reveal_main_window(app);
        }))
        .invoke_handler(tauri::generate_handler![
            open_document_demo,
            health_check,
            check_lan_access,
            frontend_ready
        ])
        .setup(|app| {
            // 预留：后续在此读取配置文件或初始化与 NAS 的连接。
            app.emit("smartdoc://boot", DOCUMENT_SERVER_URL)?;
            if let Some(window) = app.get_webview_window("main") {
                window.unmaximize()?;
                window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                    width: 1280.0,
                    height: 800.0,
                }))?;
                window.center()?;
                // Windows：启动阶段先隐藏，待前端 ready 再显示，避免白屏闪现。
                #[cfg(target_os = "windows")]
                {
                    window.hide()?;
                }
                #[cfg(not(target_os = "windows"))]
                {
                    window.show()?;
                }
            }
            let app_handle = app.handle();
            let listener_handle = app_handle.clone();
            listener_handle.clone().listen("frontend_ready", move |_| {
                FRONTEND_READY.store(true, Ordering::SeqCst);
                reveal_main_window(&listener_handle);
            });
            init_tray(app)?;
            Ok(())
        })
        .on_window_event(prevent_close_to_tray)
        .build(context)
        .expect("error while building SmartDoc Tauri application")
        .run(|app_handle, event| match event {
            #[cfg(target_os = "macos")]
            RunEvent::Reopen { .. } => {
                USER_HIDDEN.store(false, Ordering::SeqCst);
                reveal_main_window(app_handle);
            }
            _ => {}
        });
}
