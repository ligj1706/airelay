use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use tao::event::Event;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};

use crate::config::{self, Config};

const ID_CURRENT: &str = "current";
const ID_ADMIN: &str = "admin";
const ID_QUIT: &str = "quit";

#[allow(dead_code)]
enum UserEvent {
    Menu(MenuEvent),
    Tray(TrayIconEvent),
}

fn make_icon() -> Icon {
    const GLYPH: &[&str] = &[
        "..#####..###....",
        ".##...#..##.#...",
        "##.....#.##..#..",
        "##.....#.##..#..",
        "##.....#.##..#..",
        "##.....#.##..#..",
        ".#####.#.####...",
        ".##...#.####....",
        ".##...#.##.#....",
        ".##...#.##..#...",
        ".##...#.##..#...",
        ".##...#.##...#..",
        ".#####..##...#..",
        "................",
        "................",
        "................",
    ];
    let mut rgba = Vec::with_capacity(16 * 16 * 4);
    for row in GLYPH {
        for ch in row.chars() {
            match ch {
                '#' => rgba.extend_from_slice(&[0, 0, 0, 255]),
                _ => rgba.extend_from_slice(&[0, 0, 0, 0]),
            }
        }
    }
    Icon::from_rgba(rgba, 16, 16).expect("创建图标失败")
}

pub fn run(
    cfg: Arc<RwLock<Config>>,
    config_path: PathBuf,
    addr: &str,
    notify: Arc<tokio::sync::Notify>,
) {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    {
        let p = proxy.clone();
        MenuEvent::set_event_handler(Some(move |e| {
            let _ = p.send_event(UserEvent::Menu(e));
        }));
    }
    {
        let p = proxy;
        TrayIconEvent::set_event_handler(Some(move |e| {
            let _ = p.send_event(UserEvent::Tray(e));
        }));
    }

    let tray = TrayIconBuilder::new()
        .with_tooltip("airelay")
        .with_icon(make_icon())
        .with_icon_as_template(cfg!(target_os = "macos"))
        .with_menu(Box::new(build_menu(&cfg.read().unwrap())))
        .build()
        .expect("托盘图标创建失败");

    let mut last_fp = fingerprint(&cfg.read().unwrap());
    let admin_url = format!("http://{addr}/admin");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(1));

        match event {
            Event::UserEvent(UserEvent::Menu(me)) => {
                let id_str = me.id.0.as_str();
                if let Some(rest) = id_str.strip_prefix("model:") {
                    if let Some((provider, model)) = rest.split_once(':') {
                        switch_model(&cfg, &config_path, provider, model);
                        tray.set_menu(Some(Box::new(build_menu(
                            &cfg.read().unwrap(),
                        ))));
                    }
                } else if id_str == ID_ADMIN {
                    open_browser(&admin_url);
                } else if id_str == ID_QUIT {
                    notify.notify_one();
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::UserEvent(UserEvent::Tray(_)) => {}
            _ => {
                let cur = fingerprint(&cfg.read().unwrap());
                if cur != last_fp {
                    last_fp = cur;
                    tray.set_menu(Some(Box::new(build_menu(
                        &cfg.read().unwrap(),
                    ))));
                }
            }
        }
    });
}

/// 纯入口模式：不绑端口、不起服务，只提供菜单栏图标入口。
/// 由 headless 服务进程独立承载代理，退出本进程不影响服务。
/// 模型切换走 Admin API 热加载，确保 headless 服务内存配置同步生效。
pub fn run_tray_only(cfg: Arc<RwLock<Config>>, config_path: &Path, addr: &str) {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    {
        let p = proxy.clone();
        MenuEvent::set_event_handler(Some(move |e| {
            let _ = p.send_event(UserEvent::Menu(e));
        }));
    }
    {
        let p = proxy;
        TrayIconEvent::set_event_handler(Some(move |e| {
            let _ = p.send_event(UserEvent::Tray(e));
        }));
    }

    let tray = TrayIconBuilder::new()
        .with_tooltip("airelay")
        .with_icon(make_icon())
        .with_icon_as_template(cfg!(target_os = "macos"))
        .with_menu(Box::new(build_menu(&cfg.read().unwrap())))
        .build()
        .expect("托盘图标创建失败");

    let mut last_fp = fingerprint(&cfg.read().unwrap());
    let admin_url = format!("http://{addr}/admin");
    let api_base = format!("http://{addr}");
    let config_path = config_path.to_path_buf();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(1));

        match event {
            Event::UserEvent(UserEvent::Menu(me)) => {
                let id_str = me.id.0.as_str();
                if let Some(rest) = id_str.strip_prefix("model:") {
                    if let Some((provider, model)) = rest.split_once(':') {
                        if switch_model_via_api(&api_base, provider, model) {
                            {
                                let mut c = cfg.write().unwrap();
                                c.default.provider = provider.to_string();
                                c.default.model = model.to_string();
                            }
                            tray.set_menu(Some(Box::new(build_menu(
                                &cfg.read().unwrap(),
                            ))));
                        }
                    }
                } else if id_str == ID_ADMIN {
                    open_browser(&admin_url);
                } else if id_str == ID_QUIT {
                    // 仅退出入口进程，headless 服务在独立进程不受影响
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::UserEvent(UserEvent::Tray(_)) => {}
            _ => {
                // 以 config.toml 为准周期性刷新：外部（Admin 页面/CLI）改动能反映到入口菜单
                if let Ok(content) = std::fs::read_to_string(&config_path) {
                    if let Ok(new_cfg) = toml::from_str::<Config>(&content) {
                        let cur = fingerprint(&new_cfg);
                        if cur != last_fp {
                            last_fp = cur;
                            *cfg.write().unwrap() = new_cfg;
                            tray.set_menu(Some(Box::new(build_menu(
                                &cfg.read().unwrap(),
                            ))));
                        }
                    }
                }
            }
        }
    });
}

/// 通过 Admin API 热切换模型：POST /admin/api/config → 服务内存更新并落盘。
/// 入口进程不直接改 config.toml，因为 headless 服务启动时一次性加载、不监听文件变化。
fn switch_model_via_api(api_base: &str, provider: &str, model: &str) -> bool {
    let client = match reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(500))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };
    let body = serde_json::json!({
        "default": {"provider": provider, "model": model}
    });
    match client
        .post(format!("{api_base}/admin/api/config"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
    {
        Ok(resp) => {
            let ok = resp.status().is_success();
            eprintln!("[tray] switch {provider}/{model} via api -> ok={ok}");
            ok
        }
        Err(e) => {
            eprintln!("[tray] switch {provider}/{model} via api -> error: {e}");
            false
        }
    }
}

fn build_menu(cfg: &Config) -> Menu {
    let menu = Menu::new();

    menu.append(&MenuItem::with_id(
        ID_CURRENT,
        format!("{}/{}", cfg.default.provider, cfg.default.model),
        false,
        None,
    ))
    .unwrap();

    menu.append(&PredefinedMenuItem::separator()).unwrap();

    let top = Submenu::new("切换模型", true);
    for (pid, p) in &cfg.providers {
        let sub = Submenu::new(&p.display_name, true);
        for model in &p.models {
            let id = format!("model:{pid}:{model}");
            let checked = cfg.default.provider == *pid && cfg.default.model == *model;
            sub.append(&CheckMenuItem::with_id(
                id, model, true, checked, None,
            ))
            .unwrap();
        }
        top.append(&sub).unwrap();
    }
    menu.append(&top).unwrap();

    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&MenuItem::with_id(
        ID_ADMIN,
        "打开管理界面",
        true,
        None,
    ))
    .unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&MenuItem::with_id(ID_QUIT, "退出", true, None))
        .unwrap();

    menu
}

fn switch_model(cfg: &Arc<RwLock<Config>>, path: &Path, provider: &str, model: &str) {
    let mut c = cfg.write().unwrap();
    c.default.provider = provider.to_string();
    c.default.model = model.to_string();
    let _ = config::save_config(path, &c);
}

fn fingerprint(cfg: &Config) -> String {
    let mut s = format!("{}|{}|", cfg.default.provider, cfg.default.model);
    for (id, p) in &cfg.providers {
        s.push_str(&format!("{id}={}|{:?};", p.display_name, p.models));
    }
    s
}

fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(["/c", "start", "", url])
        .spawn();
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}
