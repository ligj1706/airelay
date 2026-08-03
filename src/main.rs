use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

mod admin_ui;
mod config;
mod convert;
mod server;
mod tray;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "airelay=info".into()),
        )
        .with_target(false)
        .init();

    let args = Args::parse();

    let config_path = args.config.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".airelay")
            .join("config.toml")
    });

    match args.command {
        Command::Switch(model_spec) => run_switch(&config_path, &model_spec),
        Command::List => run_list(&config_path),
        Command::Status => run_status(&config_path),
        Command::Serve if args.no_tray => run_headless(config_path),
        Command::Serve => run_gui(config_path),
    }
}

// ==================== GUI / Tray path ====================

fn run_gui(config_path: PathBuf) {
    let cfg: Arc<std::sync::RwLock<config::Config>> =
        Arc::new(config::load_or_create_config(&config_path));

    let addr = {
        let c = cfg.read().unwrap();
        format!("{}:{}", c.server.host, c.server.port)
    };

    let notify = Arc::new(tokio::sync::Notify::new());
    let (bind_tx, bind_rx) = mpsc::channel();
    let state = Arc::new(server::AppState::new(cfg.clone(), config_path.clone()));
    let n2 = notify.clone();

    // Clone addr for server thread and log
    let addr_server = addr.clone();
    let addr_tray = addr.clone();
    let addr_log = addr.clone();

    // Spawn server on a background thread with its own tokio runtime
    let server_thread = thread::Builder::new()
        .name("airelay-server".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let router = server::build_router(state);
                let listener = match tokio::net::TcpListener::bind(&addr_server).await {
                    Ok(l) => {
                        bind_tx.send(Ok(())).unwrap();
                        l
                    }
                    Err(e) => {
                        bind_tx.send(Err(e.to_string())).unwrap();
                        return;
                    }
                };
                axum::serve(listener, router)
                    .with_graceful_shutdown(async move {
                        n2.notified().await;
                    })
                    .await
                    .unwrap();
            });
        })
        .unwrap();

    // Wait for bind result — fail fast on port conflict
    match bind_rx.recv_timeout(std::time::Duration::from_secs(2)) {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            eprintln!("启动失败: {e}");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("服务器启动超时");
            std::process::exit(1);
        }
    }

    tracing::info!("airelay running at http://{addr_log}");
    tracing::info!("Admin UI: http://{addr_log}/admin");

    // Run tray on main thread — must be main thread on macOS (AppKit constraint)
    tray::run(cfg, config_path, &addr_tray, notify);
    let _ = server_thread.join();
}

// ==================== Headless path (no tray) ====================

fn run_headless(config_path: PathBuf) {
    let cfg: Arc<std::sync::RwLock<config::Config>> =
        Arc::new(config::load_or_create_config(&config_path));

    let addr = {
        let c = cfg.read().unwrap();
        format!("{}:{}", c.server.host, c.server.port)
    };

    let notify = Arc::new(tokio::sync::Notify::new());
    let state = Arc::new(server::AppState::new(cfg.clone(), config_path.clone()));
    let n2 = notify.clone();

    let addr_server = addr.clone();
    let addr_log = addr.clone();

    let server_thread = thread::Builder::new()
        .name("airelay-server".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let router = server::build_router(state);
                let listener = match tokio::net::TcpListener::bind(&addr_server).await {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("启动失败: 无法绑定 {}: {e}", addr_server);
                        std::process::exit(1);
                    }
                };
                eprintln!("airelay headless running at http://{addr_server}");
                axum::serve(listener, router)
                    .with_graceful_shutdown(async move {
                        n2.notified().await;
                    })
                    .await
                    .unwrap();
            });
        })
        .unwrap();

    tracing::info!("airelay (headless) running at http://{addr_log}");
    tracing::info!("Admin UI: http://{addr_log}/admin");

    eprintln!("airelay 已启动 (headless 模式): http://{addr_log}");
    eprintln!("按 Ctrl+C 退出");

    // Block main thread until Ctrl+C
    let _ = server_thread.join();
}

// ==================== CLI subcommands ====================

fn run_switch(config_path: &std::path::Path, model_spec: &str) {
    let (provider, model) = match model_spec.split_once('/') {
        Some((p, m)) if !p.is_empty() && !m.is_empty() => (p, m),
        _ => {
            eprintln!("用法: airelay switch <provider/model>");
            eprintln!("示例: airelay switch deepseek/deepseek-v4-pro");
            std::process::exit(1);
        }
    };

    let (server_addr, provider_exists) = {
        let config = config::load_or_create_config(config_path);
        let cfg = config.read().unwrap();
        (
            format!("http://{}:{}", cfg.server.host, cfg.server.port),
            cfg.providers.contains_key(provider),
        )
    };

    if !provider_exists {
        eprintln!("错误: provider '{}' 不存在", provider);
        std::process::exit(1);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let updated = rt.block_on(async {
        let client = match reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(500))
            .build()
        {
            Ok(c) => c,
            Err(_) => return false,
        };

        if client.get(format!("{server_addr}/health")).send().await.is_ok() {
            let body = serde_json::json!({
                "default": {"provider": provider, "model": model}
            });
            match client
                .post(format!("{server_addr}/admin/api/config"))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    println!("已切换模型到 {}/{} (热加载生效)", provider, model);
                    return true;
                }
                Ok(resp) => {
                    eprintln!("Admin API 返回错误: {}", resp.status());
                    return false;
                }
                Err(_) => false,
            }
        } else {
            false
        }
    });

    if !updated {
        let config = config::load_or_create_config(config_path);
        let mut cfg = config.write().unwrap();
        cfg.default.provider = provider.to_string();
        cfg.default.model = model.to_string();
        if let Err(e) = config::save_config(config_path, &cfg) {
            eprintln!("保存配置失败: {e}");
            std::process::exit(1);
        }
        println!("已切换模型到 {}/{} (配置已保存)", provider, model);
    }
}

fn run_list(config_path: &std::path::Path) {
    let config = config::load_or_create_config(config_path);
    let cfg = config.read().unwrap();

    println!(
        "{:<14} {:<16} {:<6} {:<6}  {}",
        "ID", "显示名称", "状态", "模型数", "Base URL"
    );
    println!("{}", "-".repeat(80));

    for (id, p) in &cfg.providers {
        let status = if p.base_url().contains("localhost") || p.base_url().contains("127.0.0.1") {
            "本地"
        } else if p.api_key.is_empty() {
            "未配置"
        } else {
            "已配置"
        };
        let marker = if cfg.default.provider == *id { "  <- 默认" } else { "" };
        println!(
            "{:<14} {:<16} {:<6} {:<6}  {}{}",
            id,
            truncate(&p.display_name, 16),
            status,
            p.models.len(),
            p.base_url(),
            marker
        );
    }
    println!();
    println!("默认模型: {}/{}", cfg.default.provider, cfg.default.model);
}

fn run_status(config_path: &std::path::Path) {
    let config = config::load_or_create_config(config_path);
    let cfg = config.read().unwrap();

    let server_addr = format!("http://{}:{}", cfg.server.host, cfg.server.port);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let server_running = rt.block_on(async {
        match reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(500))
            .build()
        {
            Ok(client) => match client.get(format!("{server_addr}/health")).send().await {
                Ok(r) => r.status().is_success(),
                Err(_) => false,
            },
            Err(_) => false,
        }
    });

    println!("airelay 状态");
    println!("  服务状态: {}", if server_running { "运行中" } else { "未运行" });
    println!("  服务地址: {server_addr}");
    println!("  配置文件: {}", config_path.display());
    println!("  默认模型: {}/{}", cfg.default.provider, cfg.default.model);
    println!();

    let configured = cfg.providers.values().filter(|p| !p.api_key.is_empty()).count();
    let local = cfg
        .providers
        .values()
        .filter(|p| p.base_url().contains("localhost") || p.base_url().contains("127.0.0.1"))
        .count();
    println!("  已配置: {configured} 个提供商, {local} 个本地服务");

    if !server_running {
        println!();
        println!("  提示: 运行 airelay 启动服务");
    }
}

// ==================== Args parsing ====================

enum Command {
    Switch(String),
    List,
    Status,
    Serve,
}

struct Args {
    config: Option<PathBuf>,
    command: Command,
    no_tray: bool,
}

impl Args {
    fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut config = None;
        let mut command = Command::Serve;
        let mut no_tray = false;
        let mut i = 1;

        while i < args.len() {
            match args[i].as_str() {
                "--config" | "-c" => {
                    i += 1;
                    if i < args.len() {
                        config = Some(PathBuf::from(&args[i]));
                    }
                }
                "--no-tray" | "--headless" => no_tray = true,
                "switch" => {
                    i += 1;
                    if i < args.len() {
                        command = Command::Switch(args[i].clone());
                    } else {
                        eprintln!("用法: airelay switch <provider/model>");
                        std::process::exit(1);
                    }
                }
                "list" => command = Command::List,
                "status" => command = Command::Status,
                other => {
                    eprintln!("未知命令: {other}");
                    eprintln!("用法: airelay [--config PATH] [--no-tray] [switch <p/m> | list | status]");
                    std::process::exit(1);
                }
            }
            i += 1;
        }

        Self { config, command, no_tray }
    }
}

fn truncate(s: &str, max: usize) -> String {
    let s: Vec<char> = s.chars().collect();
    if s.len() <= max {
        s.iter().collect()
    } else {
        format!("{}…", s[..max - 1].iter().collect::<String>())
    }
}
