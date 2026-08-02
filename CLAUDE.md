# CLAUDE.md

## 项目概述

airelay 是一个 Rust 编写的极简本地代理服务，让 Claude Code / Codex 等 AI 编程工具使用任意第三方大模型 API。

核心原理：本地启动 HTTP 服务器，接收 Anthropic Messages API 格式的请求，翻译成 OpenAI Chat Completions 格式发给上游，再把响应翻译回来。

## 产品定位

airelay 是一款 **AI 编程工具的模型交换机**。核心价值：让 Claude Code / Codex CLI 等工具无缝使用任意大模型，不需要改工具代码。

### 设计原则

- **零感知** — 用户只做一次配置（填 API Key），之后全部无感。托盘常驻、开机自启、模型切换静默生效
- **够用就好** — 不对标 API 管理平台或模型网关。不做聊天界面，不做 prompt 管理，不存历史记录
- **本地优先** — 代理跑在本机，API Key 存本地文件，敏感数据不出本机。团队版是远期的事
- **单文件交付** — 始终保持 3-5 MB 单二进制，零依赖安装。不下发运行时、不带 Python 环境
- **macOS 一等公民** — 设计先满足 macOS 用户（Claude Code 主力人群），Linux/Windows 兼容但不牺牲 Mac 体验

### 产品演进方向

```
当前 v0.1     →    近期 v0.2-0.3    →    中期 v0.4-0.6    →    远期 v1.0
个人工具           体验打磨              智能调度              团队基础设施

✅ 协议转换        📊 用量看板           🧠 自动路由           🏢 团队配置共享
✅ 10 个提供商      ⚡ 模型测速           🔄 故障转移           📈 用量报表
✅ 托盘 + CLI      🍺 Homebrew           🏠 本地优先           🔐 统一代理
✅ Web Admin      📝 请求日志                               
✅ 流式 usage     💰 费用估算                              
```

详细路线图见 ROADMAP.md。

## 技术栈

- Rust (edition 2021), axum 0.8, tokio 1, reqwest 0.12
- tray-icon 0.24, tao 0.35 (macOS 系统托盘，跨平台代码)
- 3.0 MB 单文件二进制，startup < 100ms
- 零外部运行时依赖

## 线程架构

```
main() — 同步入口
  ├── CLI 子命令: switch / list / status → 各自 tokio runtime → 退出
  └── Serve 路径: run_gui()
         ├── std::thread (airelay-server): 独立 tokio runtime + axum server
         │     共享 Arc<RwLock<Config>>（std RwLock，跨线程安全）
         │     优雅退出: tokio::sync::Notify → with_graceful_shutdown()
         └── main thread: tao 事件循环 + tray (macOS GUI 主线程约束)
                轮询 1s + fingerprint 比对 → 变化重建菜单
                菜单事件通过 EventLoopProxy send
```

## 项目结构

```
src/
├── main.rs       # 入口：Args 解析 + 子命令分派 + CLI 函数 + run_gui 线程架构
├── config.rs     # TOML 配置管理，10 个 Provider 预设，配置热保存
├── convert.rs    # 核心协议转换：Anthropic ↔ OpenAI Chat，Responses ↔ Chat，SSE 流转换
├── server.rs     # 所有 HTTP 路由：/v1/messages, /v1/responses, /v1/models, Admin API
├── tray.rs       # 系统托盘：macOS 菜单栏图标，模型切换子菜单，管理界面/退出
└── admin_ui.rs   # Web Admin UI（内嵌 HTML，Anthropic 暖色，亮/暗主题切换）
```

## 架构关键点

- **协议转换**在 convert.rs 中，`anthropic_to_openai()` 做请求转换，`SseConverter` 做流转换，`ResponsesSseConverter` 做 Codex 适配
- **模型路由**在 config.rs 的 `resolve_model()` 中，`provider/model` 格式直接路由，否则用默认
- **SSE 流**用 async-stream 宏生成，在 server.rs 的 handler 中读取上游 bytes_stream 并逐行解析 `data: {json}`
- **配置热重载**通过 `Arc<RwLock<Config>>`（std 同步锁），Admin API/CLI/托盘写内存 + 原子保存到 TOML
- **流式 usage**通过 `stream_options: {"include_usage": true}` 请求上游返回 usage，SseConverter 解析后写入 message_delta
- **上游请求**直接拼 `POST {base_url}/chat/completions`，必须用 OpenAI 兼容格式
- **托盘热切换**：托盘菜单 Click → switch_model() 直接写共享 Config → save_config 落盘 → 立即重建菜单

## 关键约定

- 不要引入蓝紫色 UI 元素，使用 Anthropic 暖色系 (#D97746, #F9F8F6)
- 管理页面保持紧凑：下拉选择器 + 表单，不要长列表卡片
- Base URL 必须包含 `/v1` 前缀（代码会追加 `/chat/completions`）
- 默认模型名称保持与上游 API 最新名称同步（2026 年 7 月基准）
- 不要引入复杂依赖或过度抽象，KISS 原则
- 运行期配置变更统一经内存（共享 Arc<RwLock<Config>>）→ 托盘和 Admin API 都是写同一份内存
- CLI switch 优先走 Admin API 热加载，服务未运行才直改文件
- 托盘图标使用代码生成 RGBA（`Icon::from_rgba`），不依赖外部 PNG 文件
- Admin UI 提示用内联 `showMsg()`，禁止浏览器 `alert()`/`confirm()` 弹窗
- 大规模重命名/重构前先确认，不定夺性操作等用户决策后再执行
- 用户说"继续""开始""先做这个"时直接执行，不追问确认

## 编译

```bash
cargo build --release    # 生成 target/release/airelay (3.0 MB)
```

## 配置

默认配置位置 `~/.airelay/config.toml`，首次运行自动创建。
Provider 在没有 API Key 时不会出现在 `/v1/models` 列表中。

## CLI 命令

```bash
airelay list          # 列出所有提供商及状态
airelay status        # 查看运行状态（探测 server 是否在线）
airelay switch <p/m>  # 热加载切换默认模型（在线→HTTP / 离线→文件）
```
