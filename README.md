# airelay

极简本地代理，让 Claude Code / Codex 使用任意大模型 API。

**3.0 MB 单文件二进制，启动几十毫秒，零外部运行时依赖。macOS 菜单栏托盘常驻。**

## 解决什么问题

Claude Code 默认只能用 Anthropic 官方的 Claude 模型（需要付费订阅）。Codex CLI 默认只能用 OpenAI 的模型。

airelay 在本地启动一个代理服务器，接收 Claude Code/Codex 的 API 请求，自动翻译协议格式后转发给你配置的任意大模型 API（DeepSeek、Kimi、GLM、OpenAI、本地 Ollama 等），让这些编程 AI 工具可以免费或低成本使用第三方模型。

## 工作原理

```
Claude Code ── Anthropic Messages ──▶ airelay ── OpenAI Chat ──▶ DeepSeek
                                              │
Codex CLI  ── OpenAI Responses ───▶           └──────────▶ Kimi / GLM / OpenAI / ...
                                              │
                                         协议转换引擎
                                       Anthropic ↔ OpenAI
                                       Responses ↔ Chat
```

- 收到 Anthropic 格式请求 → 翻译成 OpenAI Chat 格式 → 发给上游 API
- 收到上游 OpenAI SSE 流 → 翻译成 Anthropic SSE 格式 → 返回给 Claude Code
- 收到 OpenAI Responses 格式（Codex）→ 同样翻译成 Chat 格式 → 发上游 → 翻译回 Responses 格式

## 已开发

- [x] **协议转换引擎** — Anthropic Messages ↔ OpenAI Chat Completions 完整互转
- [x] **SSE 流转换** — 实时流式响应转换，支持 thinking/reasoning、tool_use、文本内容
- [x] **双协议入口** — `/v1/messages`（Claude Code）+ `/v1/responses`（Codex CLI）
- [x] **9 个预设提供商** — DeepSeek、Kimi、GLM、MiniMax、Qwen、OpenAI、Ollama、LM Studio、自定义
- [x] **模型列表端点** — `/v1/models` 返回所有已配置的模型，CC 原生 `/model` 选择器可用
- [x] **Web Admin UI** — 下拉选择 + 表单配置，暗色/亮色主题切换，Anthropic 暖色风格
- [x] **配置热保存** — Admin UI / CLI / 托盘 任何方式改配置，下一个请求自动生效
- [x] **系统托盘** — macOS 菜单栏图标常驻，一键切换模型，打开管理界面，退出
- [x] **CLI 完整命令集** — `switch`（热加载切换模型）、`list`（列出所有提供商）、`status`（查看运行状态）
- [x] **provider 增删** — Admin UI 支持新增/删除第三方提供商，不限于预设 9 个
- [x] **流式 usage 追踪** — 从上游 OpenAI SSE 实时解析 output_tokens，Claude Code 显示真实用量
- [x] **连接池复用** — reqwest::Client 全局共享，避免每次请求新建连接
- [x] **优雅退出** — 托盘 Quit 触发 graceful shutdown，SSE 流等当前请求完成再关闭
- [x] **Release 编译** — 3.0 MB 单文件二进制，`opt-level="z"` + LTO + strip

## 未开发 / 待改进

- [ ] **Codex 实际验证** — `/v1/responses` 端点已实现但未用 Codex CLI 实际测试
- [ ] **reasoning_effort 映射** — Anthropic thinking budget_tokens → OpenAI reasoning_effort 映射较粗糙
- [ ] **tool_use 流式中途恢复** — 上游流中断时无自动续写/重试机制
- [ ] **多轮对话 tool_result 处理** — Anthropic tool_result → OpenAI tool role 转换可能有边界 case
- [ ] **图片/多模态** — Anthropic image content block 转换已写但未充分测试
- [ ] **认证 token** — 代理本身无认证（ANTHROPIC_AUTH_TOKEN 任意值即可通过）
- [ ] **HTTPS / TLS** — 仅 HTTP，无本地 TLS 支持
- [ ] **Docker 镜像** — 无容器化部署方案
- [ ] **Windows/Linux 托盘验证** — 代码跨平台但仅在 macOS 实际测试

## 项目结构

```
airelay/
├── Cargo.toml              # Rust 依赖（axum, tokio, reqwest, tray-icon, tao...）
├── config.example.toml     # 配置文件示例
├── README.md               # 本文件
├── assets/
│   └── tray.png            # 托盘图标（编译时内嵌）
└── src/
    ├── main.rs             # 入口：CLI 子命令分派 + 线程架构（server/tray）
    ├── config.rs           # 配置管理：TOML 读写 + 9 个 Provider 预设
    ├── convert.rs          # 核心：Anthropic↔OpenAI 协议转换 + SSE 流转换
    ├── server.rs           # axum 路由：所有 API 端点 + Admin API
    ├── tray.rs             # 系统托盘：菜单栏图标 + 模型切换菜单
    └── admin_ui.rs         # Web Admin UI（内嵌 HTML，Anthropic 暖色风格）
```

## 安装（发给别人用）

### 方式 A：一行安装（推荐）

```bash
curl -fsSL https://raw.githubusercontent.com/ligj1706/airelay/main/install.sh | bash
```

脚本自动检测 macOS/Linux 和芯片架构，下载对应二进制到 `/usr/local/bin/`。

### 方式 B：从 GitHub Release 手动下载

在 [Releases](https://github.com/ligj1706/airelay/releases) 页面下载对应平台的 `tar.gz`，解压后放到 PATH 目录。

### 方式 C：从源码编译

需要 Rust 环境。

```bash
git clone https://github.com/ligj1706/airelay.git
cd airelay
cargo build --release
# 二进制: ./target/release/airelay (3.0 MB)
```

## 发布流程（给仓库 owner）

只需三步：

```bash
# 1. 创建 GitHub 仓库，推送代码
git remote add origin https://github.com/ligj1706/airelay.git
git push -u origin main

# 2. 在 install.sh 和 README.md 中把 "ligj1706/airelay" 替换成实际仓库名

# 3. 打 tag 发布（Actions 自动编译 macOS/Linux 二进制 + 创建 Release）
git tag v0.2.0
git push origin v0.2.0
```

之后每次发新版就打一个新 tag：`v0.3.0`、`v0.4.0`...

用户侧安装：`curl -fsSL https://raw.githubusercontent.com/ligj1706/airelay/main/install.sh | bash`

## 快速开始（本地开发）

### 2. 启动（带托盘）

```bash
./target/release/airelay
# 托盘启动: 菜单栏出现 airelay 图标
# 同时启动: http://127.0.0.1:8082 (server)
# Admin UI: http://127.0.0.1:8082/admin
```

首次启动自动创建 `~/.airelay/config.toml`。

### 3. 配置 API Key

浏览器打开 `http://127.0.0.1:8082/admin`（或点击托盘 → 打开管理界面）：

1. 下拉选择要配置的提供商（如 DeepSeek）
2. 填入 API Key
3. 点击「测试连接」
4. 点击「保存此提供商」
5. 顶部下拉选择当前使用的模型

### 4. 启动 Claude Code

```bash
ANTHROPIC_BASE_URL=http://127.0.0.1:8082 ANTHROPIC_AUTH_TOKEN=any claude
```

或者使用别名（已在 `~/.zshrc` 中配置）：
```bash
ar    # 启动 airelay（后台运行）
cc     # 通过代理启动 Claude Code
```

在 CC 中切换模型：
```
/model deepseek/deepseek-v4-pro
/model kimi/kimi-k3
/model openai/gpt-5.4
```

### 5. CLI 命令

```bash
airelay list                          # 列出所有提供商及状态
airelay status                        # 查看运行状态
airelay switch deepseek/deepseek-v4-pro   # 热加载切换默认模型
airelay switch kimi/kimi-k3              # 若服务未运行则直接改配置文件
```

CLI `switch` 自动探测服务是否运行：在线则通过 Admin API 热加载（0 秒生效），离线则直接改配置文件（重启后生效）。

### 6. 托盘操作

- 点击菜单栏图标 → 查看当前模型
- 切换模型 → 展开子菜单，点击目标模型即生效
- 打开管理界面 → 跳转浏览器
- 退出 → 优雅关闭代理服务

## 预置提供商（2026 年 7 月最新）

| Provider ID | 显示名称 | 默认模型 |
|-------------|----------|----------|
| `deepseek` | DeepSeek | `deepseek-v4-pro`, `deepseek-v4-flash` |
| `kimi` | Kimi | `kimi-k3`, `kimi-k2.6`, `kimi-k2.7-code` |
| `glm` | 智谱 GLM | `glm-5.2`, `glm-5.1`, `glm-4.7-flash` |
| `minimax` | MiniMax | `MiniMax-M3`, `MiniMax-M2.7` |
| `qwen` | 阿里百炼 Qwen | `qwen3-coder-next`, `qwen3-coder-plus`, `qwen3.7-max` |
| `openai` | OpenAI | `gpt-5.4`, `gpt-5.4-mini` |
| `ollama` | Ollama (本地) | `qwen3-coder:latest`, `deepseek-r1:latest` |
| `lmstudio` | LM Studio (本地) | `auto` |
| `custom` | 自定义 OpenAI 兼容 | `your-model-name` |

## API 端点

| 端点 | 方法 | 用途 |
|------|------|------|
| `POST /v1/messages` | POST | Claude Code 接入（Anthropic Messages API） |
| `POST /v1/responses` | POST | Codex CLI 接入（OpenAI Responses API） |
| `POST /v1/messages/count_tokens` | POST | Token 计数（简化实现，返回 0） |
| `GET /v1/models` | GET | 模型列表 |
| `GET /health` | GET | 健康检查 |
| `GET /admin` | GET | Web 管理界面 |
| `GET /admin/api/config` | GET | 获取当前配置 JSON |
| `POST /admin/api/config` | POST | 更新配置（provider 也可在此更新） |
| `POST /admin/api/provider` | POST | 新增提供商 |
| `DELETE /admin/api/provider/{id}` | DELETE | 删除提供商 |
| `POST /admin/api/test` | POST | 测试提供商连接 |

## 技术栈

- **语言**: Rust (edition 2021)
- **HTTP 框架**: axum 0.8
- **异步运行时**: tokio 1
- **HTTP 客户端**: reqwest 0.12 (rustls-tls)
- **系统托盘**: tray-icon 0.24 + tao 0.35
- **序列化**: serde + serde_json + toml
- **流处理**: futures + tokio-stream + async-stream
