use gloo_net::http::Request;
use js_sys::{Function, Promise, Reflect};
use leptos::callback::Callback;
use leptos::ev::Event;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{Redirect, Route, Router, Routes, A};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::{path, NavigateOptions};
use serde::Deserialize;
use serde_json::json;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};

const DOCUMENT_SERVICE_URL: &str = "http://10.18.65.129:8085/example/";
const MEDIA_PORTAL_URL: &str = "https://seerhut.uk/";
const CLOUD_DRIVE_URL: &str = "https://www.yunpan.com/";

#[derive(Clone, Copy)]
struct NavItem {
    label: &'static str,
    subtitle: &'static str,
    path: &'static str,
}

const NAV_ITEMS: &[NavItem] = &[
    NavItem {
        label: "总览",
        subtitle: "状态 · 模块",
        path: "/overview",
    },
    NavItem {
        label: "云盘",
        subtitle: "远程挂载",
        path: "/cloud",
    },
    NavItem {
        label: "知识库",
        subtitle: "数据集",
        path: "/knowledge",
    },
    NavItem {
        label: "AI 搜索",
        subtitle: "问答 / 对话",
        path: "/ai-search",
    },
];

const EMBED_ROUTES: &[&str] = &["/doc-service", "/media", "/cloud-drive"];

#[derive(Deserialize, Debug, Clone)]
struct DocumentLink {
    url: String,
    filename: String,
}

#[derive(Deserialize, Debug, Clone)]
struct AuthResponse {
    access_token: String,
}

#[derive(Deserialize, Debug, Clone)]
struct NetworkAccess {
    allowed: bool,
    ip: String,
}

type VoidCallback = Callback<(), ()>;

async fn invoke_tauri<T: for<'de> Deserialize<'de>>(
    cmd: &str,
    payload: serde_json::Value,
) -> Result<T, String> {
    let global = js_sys::global();
    let tauri = Reflect::get(&global, &JsValue::from_str("__TAURI__"))
        .map_err(|_| "未找到 __TAURI__".to_string())?;
    let core = Reflect::get(&tauri, &JsValue::from_str("core"))
        .map_err(|_| "未找到 __TAURI__.core".to_string())?;
    let invoke = Reflect::get(&core, &JsValue::from_str("invoke"))
        .map_err(|_| "未找到 invoke 函数".to_string())?
        .dyn_into::<Function>()
        .map_err(|_| "invoke 不是函数".to_string())?;
    let js_payload = to_value(&payload).map_err(|e| e.to_string())?;
    let promise = invoke
        .call2(&core, &JsValue::from_str(cmd), &js_payload)
        .map_err(|e| format!("调用失败: {e:?}"))?
        .dyn_into::<Promise>()
        .map_err(|_| "invoke 未返回 Promise".to_string())?;
    let result = JsFuture::from(promise)
        .await
        .map_err(|e| format!("Promise 拒绝: {e:?}"))?;
    from_value(result).map_err(|e| e.to_string())
}

#[component]
fn OverviewPage(
    user_display: RwSignal<String>,
    media_notes: RwSignal<Vec<String>>,
    media_note_input: RwSignal<String>,
    on_doc: VoidCallback,
    on_media_save: VoidCallback,
    on_doc_embed: VoidCallback,
    on_media_embed: VoidCallback,
) -> impl IntoView {
    view! {
        <section class="dashboard-grid">
            <section class="card user-card">
                <h2>{"用户信息"}</h2>
                <p>{move || format!("当前用户：{}", user_display.get())}</p>
                <p>{"Token 已获取，可调用云端服务"}</p>
            </section>
            <section class="card doc-card">
                <h2>{"DocumentServer"}</h2>
                <p>{"打开示例文档或直接进入服务以做联调验证"}</p>
                <div class="inline-actions">
                    {
                        let doc_cb = on_doc;
                        view! { <button on:click=move |_| doc_cb.run(())>{"获取示例链接"}</button> }
                    }
                    {
                        let embed_cb = on_doc_embed;
                        view! { <button class="ghost" on:click=move |_| embed_cb.run(())>{"打开服务窗口"}</button> }
                    }
                </div>
            </section>
            <section class="card media-card">
                <div class="media-header">
                    <h2>{"影视资源"}</h2>
                    {
                        let media_embed = on_media_embed;
                        view! { <button class="ghost" on:click=move |_| media_embed.run(())>{"打开影视站"}</button> }
                    }
                </div>
                <div class="field">
                    <label>{"观影笔记 / 收藏占位"}</label>
                    <textarea
                        placeholder="记录观影想法或收藏链接"
                        prop:value=move || media_note_input.get()
                        on:input=move |ev: Event| media_note_input.set(event_target_value(&ev))>
                    </textarea>
                </div>
                {
                    let media_save_cb = on_media_save;
                    view! { <button on:click=move |_| media_save_cb.run(())>{"保存到本地列表"}</button> }
                }
                <ul class="notes">
                    <Show
                        when=move || !media_notes.get().is_empty()
                        fallback=|| view! { <li class="muted">{"暂无记录"}</li> }>
                        <For
                            each=move || media_notes.get()
                            key=|item| item.clone()
                            children=move |item| view! { <li>{item}</li> }
                        />
                    </Show>
                </ul>
            </section>
        </section>
    }
}

#[component]
fn CloudPage(user_display: RwSignal<String>, login_state: RwSignal<bool>) -> impl IntoView {
    let navigate = use_navigate();
    let open_drive = move |_| navigate("/cloud-drive", NavigateOptions::default());

    view! {
        <section class="card page-card">
            <div class="page-meta">
                <div>
                    <h2>{"云盘总览"}</h2>
                    <p>{move || format!("{} 的空间挂载状态", user_display.get())}</p>
                </div>
                <span class="stat-chip" class:offline=move || !login_state.get()>
                    {move || if login_state.get() { "已认证" } else { "未登录" }}
                </span>
                <button class="ghost" on:click=open_drive>{"进入云盘"}</button>
            </div>
            <div class="two-column">
                <div class="list-card">
                    <h3>{"挂载点"}</h3>
                    <ul class="page-list">
                        <li><strong>{"OSS-Bucket"}</strong><span>{"延迟 32ms · 只读"}</span></li>
                        <li><strong>{"OneDrive"}</strong><span>{"延迟 58ms · 可写"}</span></li>
                        <li><strong>{"NAS@LAB"}</strong><span>{"LAN · 高速"}</span></li>
                    </ul>
                </div>
                <div class="list-card">
                    <h3>{"同步计划"}</h3>
                    <ul class="page-list">
                        <li><strong>{"/Movies"}</strong><span>{"每日 02:00 增量"}</span></li>
                        <li><strong>{"/Docs/contracts"}</strong><span>{"实时 · 零信任"}</span></li>
                        <li><strong>{"/Datasets"}</strong><span>{"周末 · 全量校验"}</span></li>
                    </ul>
                </div>
            </div>
            <footer class="inline-actions">
                <button>{"新建挂载"}</button>
                <button class="ghost">{"校验同步"}</button>
            </footer>
        </section>
    }
}

#[component]
fn KnowledgePage(user_display: RwSignal<String>, login_state: RwSignal<bool>) -> impl IntoView {
    view! {
        <section class="card page-card">
            <div class="page-meta">
                <div>
                    <h2>{"知识库"}</h2>
                    <p>{move || format!("{} 的数据集与索引占位", user_display.get())}</p>
                </div>
                <span class="stat-chip" class:offline=move || !login_state.get()>
                    {move || if login_state.get() { "索引在线" } else { "索引关闭" }}
                </span>
            </div>
            <div class="two-column">
                <div class="list-card">
                    <h3>{"数据集"}</h3>
                    <ul class="page-list">
                        <li><strong>{"产品手册"}</strong><span>{"1.2k 文档 · 87% 覆盖"}</span></li>
                        <li><strong>{"研发日报"}</strong><span>{"432 条 · RAG"}</span></li>
                        <li><strong>{"合规政策"}</strong><span>{"143 条 · 多语"}</span></li>
                    </ul>
                </div>
                <div class="list-card">
                    <h3>{"索引状态"}</h3>
                    <ul class="page-list">
                        <li><strong>{"Milvus 集群"}</strong><span>{"QPS 120 · 正常"}</span></li>
                        <li><strong>{"嵌入任务"}</strong><span>{"队列 3 · 进行时"}</span></li>
                        <li><strong>{"回溯窗口"}</strong><span>{"7 天"}</span></li>
                    </ul>
                </div>
            </div>
            <footer class="inline-actions">
                <button>{"上传文件"}</button>
                <button class="ghost">{"重建索引"}</button>
            </footer>
        </section>
    }
}

#[component]
fn AISearchPage(user_display: RwSignal<String>, login_state: RwSignal<bool>) -> impl IntoView {
    let (prompt, set_prompt) = signal(String::new());

    view! {
        <section class="card page-card">
            <div class="page-meta">
                <div>
                    <h2>{"AI 搜索 / 助手"}</h2>
                    <p>{move || format!("{} 的多模态问答占位", user_display.get())}</p>
                </div>
                <span class="stat-chip" class:offline=move || !login_state.get()>
                    {move || if login_state.get() { "在线" } else { "离线" }}
                </span>
            </div>
            <div class="field">
                <label>{"提示词 / 问题"}</label>
                <textarea
                    placeholder="示例：总结云盘同步失败的原因，引用日志"
                    prop:value=move || prompt.get()
                    on:input=move |ev: Event| set_prompt.set(event_target_value(&ev))>
                </textarea>
            </div>
            <div class="inline-actions">
                <button>{"生成回答"}</button>
                <button class="ghost">{"清除上下文"}</button>
            </div>
            <div class="list-card">
                <h3>{"最近会话"}</h3>
                <ul class="page-list conversation-list">
                    <li>
                        <strong>{"联网检索"}</strong>
                        <span>{"追问 SmartDoc 架构 · 2 分钟前"}</span>
                    </li>
                    <li>
                        <strong>{"知识库匹配"}</strong>
                        <span>{"引用《产品手册》 · 5 分钟前"}</span>
                    </li>
                    <li>
                        <strong>{"草稿摘要"}</strong>
                        <span>{"生成 800 字草稿 · 12 分钟前"}</span>
                    </li>
                </ul>
            </div>
        </section>
    }
}

#[component]
fn EmbeddedPage(
    title: &'static str,
    url: &'static str,
    back_target: RwSignal<String>,
) -> impl IntoView {
    let navigate = use_navigate();
    let on_back = Callback::new(move |()| {
        let target = back_target.get();
        navigate(&target, NavigateOptions::default());
    });

    view! {
        <section class="card embed-card">
            <div class="page-meta">
                <div>
                    <h2>{title}</h2>
                    <p>{format!("在 SmartDoc 内嵌查看 {title}")}</p>
                </div>
                <button class="ghost" on:click=move |_| on_back.run(())>{"退出"}</button>
            </div>
            <iframe
                class="embedded-frame"
                title=title
                src=url
                allow="clipboard-read; clipboard-write; fullscreen"
            ></iframe>
        </section>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="card page-card">
            <h2>{"页面不存在"}</h2>
            <p>{"请选择左侧的导航条目，或返回总览。"}</p>
        </section>
    }
}

#[component]
fn AppRoot() -> impl IntoView {
    let (status, set_status) = signal("尚未登录".to_string());
    let (username, set_username) = signal(String::from("demo"));
    let (password, set_password) = signal(String::from("demo"));
    let auth_token = RwSignal::new(None::<String>);
    let login_state = RwSignal::new(false);
    let user_display = RwSignal::new(String::new());
    let media_notes = RwSignal::new(Vec::<String>::new());
    let media_note_input = RwSignal::new(String::new());
    let navigate = use_navigate();
    let location = use_location();
    let nav_pathname = location.pathname.clone();
    let last_route = RwSignal::new(String::from("/overview"));

    Effect::new(move |_| {
        let current = location.pathname.get();
        if !EMBED_ROUTES.iter().any(|route| *route == current.as_str()) {
            last_route.set(current);
        }
    });

    let on_login: Callback<(), ()> = {
        let set_status = set_status.clone();
        let username = username.clone();
        let password = password.clone();
        let user_display = user_display.clone();
        let auth_token = auth_token.clone();
        let login_state = login_state.clone();
        let password_reset = set_password.clone();
        let navigate = navigate.clone();
        Callback::new(move |()| {
            let user = username.get();
            let pass = password.get();
            if user.is_empty() || pass.is_empty() {
                set_status.set("请输入用户名与密码".into());
                return;
            }
            set_status.set("登录中...".into());
            login_state.set(false);
            let status_setter = set_status.clone();
            let token_signal = auth_token.clone();
            let user_signal = user_display.clone();
            let login_state_signal = login_state.clone();
            let entered_user = user.clone();
            let entered_pass = pass.clone();
            let password_reset = password_reset.clone();
            let navigate_fn = navigate.clone();
            spawn_local(async move {
                let builder = Request::post("http://localhost:9100/auth/login")
                    .header("Content-Type", "application/json");
                let builder = match builder.body(
                    json!({"username": entered_user.clone(), "password": entered_pass}).to_string(),
                ) {
                    Ok(b) => b,
                    Err(err) => {
                        status_setter.set(format!("构建请求失败: {err}"));
                        login_state_signal.set(false);
                        token_signal.set(None);
                        return;
                    }
                };
                let response = builder.send().await;
                match response {
                    Ok(resp) => match resp.json::<AuthResponse>().await {
                        Ok(auth) => {
                            token_signal.set(Some(auth.access_token));
                            user_signal.set(entered_user.clone());
                            login_state_signal.set(true);
                            password_reset.set(String::new());
                            status_setter.set("登录成功，请使用各模块".into());
                            navigate_fn("/overview", NavigateOptions::default());
                        }
                        Err(err) => {
                            token_signal.set(None);
                            login_state_signal.set(false);
                            status_setter.set(format!("解析登录响应失败: {err}"));
                        }
                    },
                    Err(err) => {
                        token_signal.set(None);
                        login_state_signal.set(false);
                        status_setter.set(format!("登录失败: {err}"));
                    }
                }
            });
        })
    };

    let on_doc: VoidCallback = {
        let set_status = set_status.clone();
        let auth_token = auth_token.clone();
        Callback::new(move |()| {
            if auth_token.get_untracked().is_none() {
                set_status.set("请先登录".into());
                return;
            }
            set_status.set("调用 DocumentServer...".into());
            let status_setter = set_status.clone();
            spawn_local(async move {
                match invoke_tauri::<DocumentLink>(
                    "open_document_demo",
                    json!({"filename": "demo.docx"}),
                )
                .await
                {
                    Ok(link) => {
                        status_setter.set(format!("成功: {} -> {}", link.filename, link.url))
                    }
                    Err(err) => status_setter.set(format!("调用失败: {err}")),
                }
            });
        })
    };

    let on_media_save: VoidCallback = {
        let set_status = set_status.clone();
        let media_notes = media_notes.clone();
        let media_note_input = media_note_input.clone();
        Callback::new(move |()| {
            let note = media_note_input.get_untracked();
            if note.trim().is_empty() {
                set_status.set("请输入观影笔记".into());
                return;
            }
            media_notes.update(|list| list.push(note.clone()));
            media_note_input.set(String::new());
            set_status.set("已保存观影笔记（占位）".into());
        })
    };

    let open_doc_embed: VoidCallback = {
        let set_status = set_status.clone();
        let navigate = navigate.clone();
        Callback::new(move |()| {
            let status_setter = set_status.clone();
            let navigate_fn = navigate.clone();
            spawn_local(async move {
                match invoke_tauri::<NetworkAccess>("check_lan_access", json!({})).await {
                    Ok(info) => {
                        if info.allowed {
                            navigate_fn("/doc-service", NavigateOptions::default());
                        } else {
                            status_setter.set(format!(
                                "当前 IP ({}) 不在 10.18.65.* 网段，仅内网可访问 DocumentServer",
                                info.ip
                            ));
                        }
                    }
                    Err(err) => status_setter.set(format!("无法检测网段: {err}")),
                }
            });
        })
    };

    let open_media_embed: VoidCallback = {
        let navigate = navigate.clone();
        Callback::new(move |()| {
            navigate("/media", NavigateOptions::default());
        })
    };

    view! {
        <main class="smartdoc-shell">
            <header class="app-header">
                <div>
                    <h1>{"SmartDoc 桌面"}</h1>
                    <p>{move || if login_state.get() { format!("欢迎回来，{}", user_display.get()) } else { "请登录以使用各模块".into() }}</p>
                </div>
                <span class="status-text">{status}</span>
            </header>

            <Show
                when=move || login_state.get()
                fallback=move || view! {
                    <section class="card login-card">
                        <h2>{"登录到 SmartDoc"}</h2>
                        <div class="field">
                            <label>{"用户名"}</label>
                            <input
                                type="text"
                                placeholder="请输入用户名"
                                prop:value=move || username.get()
                                on:input=move |ev| set_username.set(event_target_value(&ev)) />
                        </div>
                        <div class="field">
                            <label>{"密码"}</label>
                            <input
                                type="password"
                                placeholder="请输入密码"
                                prop:value=move || password.get()
                                on:input=move |ev| set_password.set(event_target_value(&ev)) />
                        </div>
                        {
                            let login_cb = on_login;
                            view! { <button on:click=move |_| login_cb.run(())>{"登录"}</button> }
                        }
                    </section>
                }>
                <section class="dashboard-shell">
                    <section class="card dashboard-nav">
                        <For
                            each=move || NAV_ITEMS.iter().copied()
                            key=|item| item.path
                            children=move |item| {
                                let current_path = nav_pathname.clone();
                                let path = item.path;
                                let label = item.label;
                                let subtitle = item.subtitle;
                                view! {
                                    <A href=path>
                                        <div
                                            class="nav-item"
                                            role="button"
                                            class:active=move || current_path.get() == path
                                        >
                                            <span>{label}</span>
                                            <small>{subtitle}</small>
                                        </div>
                                    </A>
                                }
                            }
                        />
                    </section>
                    <section class="content-stack">
                        <Routes fallback=move || view! { <NotFoundPage/> }>
                            <Route
                                path=path!("/")
                                view=|| view! { <Redirect path="/overview"/> }
                            />
                            <Route
                                path=path!("/overview")
                                view=move || {
                                    view! {
                                        <OverviewPage
                                            user_display=user_display
                                            media_notes=media_notes
                                            media_note_input=media_note_input
                                            on_doc=on_doc.clone()
                                            on_media_save=on_media_save.clone()
                                            on_doc_embed=open_doc_embed.clone()
                                            on_media_embed=open_media_embed.clone()
                                        />
                                    }
                                }
                            />
                            <Route
                                path=path!("/cloud")
                                view=move || view! { <CloudPage user_display=user_display login_state=login_state /> }
                            />
                            <Route
                                path=path!("/knowledge")
                                view=move || view! { <KnowledgePage user_display=user_display login_state=login_state /> }
                            />
                            <Route
                                path=path!("/ai-search")
                                view=move || view! { <AISearchPage user_display=user_display login_state=login_state /> }
                            />
                            <Route
                                path=path!("/doc-service")
                                view=move || {
                                    view! { <EmbeddedPage title="DocumentServer" url=DOCUMENT_SERVICE_URL back_target=last_route /> }
                                }
                            />
                            <Route
                                path=path!("/media")
                                view=move || {
                                    view! { <EmbeddedPage title="影视资源" url=MEDIA_PORTAL_URL back_target=last_route /> }
                                }
                            />
                            <Route
                                path=path!("/cloud-drive")
                                view=move || {
                                    view! { <EmbeddedPage title="云盘" url=CLOUD_DRIVE_URL back_target=last_route /> }
                                }
                            />
                            <Route
                                path=path!("/*any")
                                view=|| view! { <NotFoundPage/> }
                            />
                        </Routes>
                    </section>
                </section>
            </Show>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! { <Router><AppRoot /></Router> }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
