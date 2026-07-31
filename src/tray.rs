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
    let mut rgba = Vec::with_capacity(16 * 16 * 4);
    for y in 0..16u32 {
        for x in 0..16u32 {
            let cx: f64 = 7.5;
            let cy: f64 = 7.5;
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if (dist < 7.0 && dist > 4.5) || dist < 3.5 {
                rgba.extend_from_slice(&[0, 0, 0, 255]);
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
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
