# airelay

[English](README_EN.md)

让 Claude Code / Codex CLI 使用任意大模型 API — 本地协议转换，开箱即用。

**3.0 MB 单文件二进制，启动几十毫秒，零外部运行时依赖。macOS 菜单栏托盘常驻。**

## 支持平台

| 平台 | 状态 |
|------|------|
| macOS (Apple Silicon) | 完整支持 — 二进制 + 托盘 |
| macOS (Intel) | 完整支持 — 二进制 + 托盘 |
| Windows (x86_64) | 二进制 + CLI + 服务器（托盘暂不可用） |
| Linux (x86_64) | 二进制 + CLI + 服务器 |

## 解决什么问题

Claude Code 默认只能用 Anthropic 官方的 Claude 模型（需要付费订阅）。Codex CLI 默认只能用 OpenAI 的模型。

airelay 在本地启动一个 HTTP 代理服务器，实时翻译 API 协议 — Anthropic Messages ↔ OpenAI Chat Completions、OpenAI Responses ↔ Chat — 让你可以用 DeepSeek、Kimi、GLM、Qwen、Ollama 或任何兼容 OpenAI 的 API 来驱动你常用的 AI 编程工具。

```
Claude Code ── Anthropic Messages ──▶ airelay ── OpenAI Chat ──▶ DeepSeek / Kimi / ...
Codex CLI  ── OpenAI Responses ────▶                          ▶ Ollama / LM Studio / ...
```

## 快速开始

### 1. 安装

**macOS / Linux**

```bash
curl -fsSL https://raw.githubusercontent.com/ligj1706/airelay/main/install.sh | bash
```

**Windows (PowerShell)**

```powershell
irm https://raw.githubusercontent.com/ligj1706/airelay/main/install.ps1 | iex
```

安装完成后，**关闭终端窗口重新打开**（或执行 `exec $SHELL`），让命令生效。

> 如果网络有问题，也可以从源码安装（需要 Rust 环境）：
> ```bash
> git clone https://github.com/ligj1706/airelay.git
> cd airelay && cargo build --release
> cp target/release/airelay ~/.local/bin/
> ```

### 2. 配置（仅第一次）

在终端输入 `ar`，菜单栏会出现 airelay 图标。然后浏览器打开 `http://127.0.0.1:8082/admin`（或点击托盘图标 → 打开管理界面）：

1. 选择提供商，如 DeepSeek
2. 填入 API Key
3. 点「测试连接」确认连通
4. 点「保存」

### 3. 开始写代码

```bash
cc
```

就这一行。`cc` 会自动检测 airelay 是否在跑（没跑就后台拉起），然后启动 Claude Code。关机重启后也只需要输 `cc`。

切换模型：

```
/model deepseek/deepseek-v4-pro
/model kimi/kimi-k3
```

> 如果用 Codex CLI：`OPENAI_BASE_URL=http://127.0.0.1:8082/v1 codex`

### 4. 开机自启（可选）

```bash
airelay-autostart on      # 开机自动启动 airelay
airelay-autostart off     # 关闭
airelay-autostart status  # 查看状态
```

开启后，开机就有托盘图标，任何时候输 `cc` 直接开始编码。

## 功能特性

- **协议转换** — 完整 Anthropic Messages ↔ OpenAI Chat Completions，以及 OpenAI Responses（Codex）
- **SSE 流式** — 实时流转换，支持 thinking/reasoning、tool_use 和 token 用量追踪
- **10 个预设提供商** — Anthropic、DeepSeek、Kimi、GLM、MiniMax、Qwen、OpenAI、Ollama、LM Studio，以及自定义
- **Web 管理界面** — 下拉配置，API Key 申请链接一键跳转，暗色/亮色主题，开机自启开关
- **热加载** — Admin UI / CLI / 托盘任何方式修改配置，即时生效
- **macOS 托盘** — 菜单栏图标，一键切换模型、打开管理界面、退出
- **CLI 命令行** — `airelay switch <provider/model>`、`airelay list`、`airelay status`
- **提供商增删** — Admin UI 支持新增/删除第三方提供商
- **流式用量追踪** — 从上游 SSE 实时解析 token 用量，Claude Code 显示真实数字
- **连接池复用** — reqwest 客户端全局共享
- **优雅退出** — 托盘退出触发 graceful shutdown，排空进行中的 SSE 流

## 预置提供商

| Provider | 模型列表 |
|----------|----------|
| `anthropic` | claude-opus-4-7, claude-sonnet-4-6, claude-haiku-4-5 |
| `deepseek` | deepseek-v4-pro, deepseek-v4-flash |
| `kimi` | kimi-k3, kimi-k2.6, kimi-k2.7-code |
| `glm` | glm-5.2, glm-5.1, glm-4.7-flash |
| `minimax` | MiniMax-M3, MiniMax-M2.7 |
| `qwen` | qwen3-coder-next, qwen3-coder-plus, qwen3.7-max |
| `openai` | gpt-5.4, gpt-5.4-mini |
| `ollama` | qwen3-coder:latest, deepseek-r1:latest |
| `lmstudio` | auto |
| `custom` | your-model-name |

## CLI 命令

```bash
airelay list                              # 列出所有提供商及状态
airelay status                            # 查看运行状态
airelay switch deepseek/deepseek-v4-pro   # 热加载切换默认模型
```

## API 端点

| 端点 | 方法 | 用途 |
|------|------|------|
| `/v1/messages` | POST | Claude Code 接入（Anthropic Messages） |
| `/v1/responses` | POST | Codex CLI 接入（OpenAI Responses） |
| `/v1/messages/count_tokens` | POST | Token 计数 |
| `/v1/models` | GET | 模型列表 |
| `/health` | GET | 健康检查 |
| `/admin` | GET | Web 管理界面 |
| `/admin/api/config` | GET/POST | 读取/更新配置 |
| `/admin/api/provider` | POST | 新增提供商 |
| `/admin/api/provider/{id}` | DELETE | 删除提供商 |
| `/admin/api/test` | POST | 测试提供商连接 |
| `/admin/api/autostart` | GET/POST | 查看/设置开机自启 |

## 技术栈

Rust (edition 2021) — axum 0.8, tokio 1, reqwest 0.12, tray-icon 0.24, tao 0.35, serde + toml。

## 许可证

MIT
