#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use get_if_addrs::get_if_addrs;
use local_ip_address::local_ip;
use serde::Serialize;
use std::net::IpAddr;
use tauri::{AppHandle, Emitter};

const DOCUMENT_SERVER_URL: &str = "http://10.18.65.129:8085/example/";

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_document_demo,
            health_check,
            check_lan_access
        ])
        .setup(|app| {
            // 预留：后续在此读取配置文件或初始化与 NAS 的连接。
            app.emit("smartdoc://boot", DOCUMENT_SERVER_URL)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running SmartDoc Tauri application");
}
