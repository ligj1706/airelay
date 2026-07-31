use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use tracing::info;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub display_name: String,
    pub api_key: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub models: Vec<String>,
}

impl ProviderConfig {
    pub fn base_url(&self) -> String {
        self.base_url.clone().unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub default: DefaultConfig,
    pub providers: BTreeMap<String, ProviderConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let mut providers = BTreeMap::new();

        providers.insert(
            "deepseek".into(),
            ProviderConfig {
                display_name: "DeepSeek".into(),
                api_key: String::new(),
                base_url: Some("https://api.deepseek.com/v1".into()),
                models: vec!["deepseek-v4-pro".into(), "deepseek-v4-flash".into()],
            },
        );

        providers.insert(
            "kimi".into(),
            ProviderConfig {
                display_name: "Kimi".into(),
                api_key: String::new(),
                base_url: Some("https://api.moonshot.cn/v1".into()),
                models: vec!["kimi-k3".into(), "kimi-k2.6".into(), "kimi-k2.7-code".into()],
            },
        );

        providers.insert(
            "glm".into(),
            ProviderConfig {
                display_name: "智谱 GLM".into(),
                api_key: String::new(),
                base_url: Some("https://open.bigmodel.cn/api/paas/v4".into()),
                models: vec!["glm-5.2".into(), "glm-5.1".into(), "glm-4.7-flash".into()],
            },
        );

        providers.insert(
            "minimax".into(),
            ProviderConfig {
                display_name: "MiniMax".into(),
                api_key: String::new(),
                base_url: Some("https://api.minimax.chat/v1".into()),
                models: vec!["MiniMax-M3".into(), "MiniMax-M2.7".into()],
            },
        );

        providers.insert(
            "qwen".into(),
            ProviderConfig {
                display_name: "阿里百炼 Qwen".into(),
                api_key: String::new(),
                base_url: Some("https://dashscope.aliyuncs.com/compatible-mode/v1".into()),
                models: vec!["qwen3-coder-next".into(), "qwen3-coder-plus".into(), "qwen3.7-max".into()],
            },
        );

        providers.insert(
            "openai".into(),
            ProviderConfig {
                display_name: "OpenAI".into(),
                api_key: String::new(),
                base_url: Some("https://api.openai.com/v1".into()),
                models: vec!["gpt-5.4".into(), "gpt-5.4-mini".into()],
            },
        );

        providers.insert(
            "ollama".into(),
            ProviderConfig {
                display_name: "Ollama (本地)".into(),
                api_key: "ollama".into(),
                base_url: Some("http://localhost:11434/v1".into()),
                models: vec!["qwen3-coder:latest".into(), "deepseek-r1:latest".into()],
            },
        );

        providers.insert(
            "lmstudio".into(),
            ProviderConfig {
                display_name: "LM Studio (本地)".into(),
                api_key: "lmstudio".into(),
                base_url: Some("http://localhost:1234/v1".into()),
                models: vec!["auto".into()],
            },
        );

        providers.insert(
            "custom".into(),
            ProviderConfig {
                display_name: "自定义 OpenAI 兼容".into(),
                api_key: String::new(),
                base_url: Some("https://your-api.example.com/v1".into()),
                models: vec!["your-model-name".into()],
            },
        );

        Self {
            server: ServerConfig {
                host: "127.0.0.1".into(),
                port: 8082,
            },
            default: DefaultConfig {
                provider: "deepseek".into(),
                model: "deepseek-v4-flash".into(),
            },
            providers,
        }
    }
}

pub fn load_or_create_config(path: &Path) -> RwLock<Config> {
    match fs::read_to_string(path) {
        Ok(content) => {
            let cfg: Config = toml::from_str(&content).unwrap_or_default();
            info!("加载配置: {}", path.display());
            RwLock::new(cfg)
        }
        Err(_) => {
            let cfg = Config::default();
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(toml_str) = toml::to_string_pretty(&cfg) {
                let _ = fs::write(path, &toml_str);
                info!("创建默认配置: {}", path.display());
            }
            RwLock::new(cfg)
        }
    }
}

pub fn save_config(path: &Path, config: &Config) -> Result<(), String> {
    let parent = path.parent().ok_or("无法获取配置目录")?;
    fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {e}"))?;
    let toml_str = toml::to_string_pretty(config).map_err(|e| format!("序列化失败: {e}"))?;

    let tmp = PathBuf::from(format!("{}.tmp", path.display()));
    fs::write(&tmp, &toml_str).map_err(|e| format!("写入失败: {e}"))?;
    fs::rename(&tmp, path).map_err(|e| format!("保存失败: {e}"))?;

    info!("配置已保存: {}", path.display());
    Ok(())
}

pub fn resolve_model<'a>(config: &'a Config, model_name: &'a str) -> (&'a str, &'a str, &'a str) {
    if let Some((provider, model)) = model_name.split_once('/') {
        if config.providers.contains_key(provider) {
            return (provider, model, model_name);
        }
    }

    (
        &config.default.provider,
        &config.default.model,
        model_name,
    )
}
