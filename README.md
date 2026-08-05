# airelay

[中文](README_CN.md)

Use any LLM with Claude Code or Codex CLI — local protocol translation, zero setup.

**3.0 MB single binary. Starts in milliseconds. Zero runtime dependencies. macOS menu bar app.**

## Supported Platforms

| Platform | Status |
|----------|--------|
| macOS (Apple Silicon) | Full support — binary + tray |
| macOS (Intel) | Full support — binary + tray |
| Windows (x86_64) | Binary + CLI + server (tray not yet available) |
| Linux (x86_64) | Binary + CLI + server |

## What it does

Claude Code requires Anthropic's Claude models (paid subscription). Codex CLI requires OpenAI models.

airelay runs a local HTTP server that translates API protocols in real time — Anthropic Messages ↔ OpenAI Chat Completions, OpenAI Responses ↔ Chat — so you can use DeepSeek, Kimi, GLM, Qwen, Ollama, or any OpenAI-compatible API with your favorite AI coding tools.

```
Claude Code ── Anthropic Messages ──▶ airelay ── OpenAI Chat ──▶ DeepSeek / Kimi / ...
Codex CLI  ── OpenAI Responses ────▶                          ▶ Ollama / LM Studio / ...
```

## Quick Start

### 1. Install

**macOS / Linux**

```bash
curl -fsSL https://raw.githubusercontent.com/ligj1706/airelay/main/install.sh | bash
```

**Windows (PowerShell)**

```powershell
irm https://raw.githubusercontent.com/ligj1706/airelay/main/install.ps1 | iex
```

After installation, **close and reopen your terminal** (or run `exec $SHELL` on macOS/Linux) to activate the commands.

The install script handles three things: puts `airelay` in `~/.local/bin/`, appends to PATH, and writes the `ar` alias.

> If you prefer building from source (requires Rust):
> ```bash
> git clone https://github.com/ligj1706/airelay.git
> cd airelay && cargo build --release
> cp target/release/airelay ~/.local/bin/
> ```

### 2. Start

```bash
ar
```

The install script sets up the `ar` alias: it checks whether airelay is running and starts it in the background if not. Then open `http://127.0.0.1:8082/admin` to configure.

### 3. Configure

Open the admin UI in your browser, pick a provider (e.g. DeepSeek), enter your API key, click Test Connection, then Save.

### 4. Use

```bash
claude    # Claude Code
codex     # Codex CLI
```

The install script sets up required environment variables automatically. If you haven't installed Claude Code yet, see the [official docs](https://docs.anthropic.com/en/docs/claude-code/overview).

Switch models (inside Claude Code):

```
/model deepseek/deepseek-v4-pro
/model kimi/kimi-k3
```

### 5. Auto-start (optional)

Open `http://127.0.0.1:8082/admin` and toggle the auto-start switch. Or use the command line:

```bash
airelay-autostart on      # Enable
airelay-autostart off     # Disable
```

## Features

- **Protocol translation** — Full Anthropic Messages ↔ OpenAI Chat Completions, plus OpenAI Responses (Codex)
- **SSE streaming** — Real-time stream conversion with thinking/reasoning, tool use, and token usage tracking
- **10 built-in providers** — Anthropic, DeepSeek, Kimi, GLM, MiniMax, Qwen, OpenAI, Ollama, LM Studio, plus custom
- **Web Admin UI** — Dropdown config, one-click API key links, dark/light theme, auto-start toggle
- **Hot reload** — Config changes via Admin UI, CLI, or tray take effect immediately
- **macOS tray** — Menu bar icon with model switching, config access, and graceful quit
- **CLI** — `airelay switch <provider/model>`, `airelay list`, `airelay status`
- **Provider CRUD** — Add/remove third-party providers via Admin UI
- **Streaming usage** — Real output token counts from upstream SSE, shown in Claude Code
- **Connection pooling** — Shared reqwest client across requests
- **Graceful shutdown** — Tray quit triggers graceful shutdown, drains in-flight SSE streams

## Preset Providers

`anthropic` · `deepseek` · `kimi` · `glm` · `minimax` · `qwen` · `openai` · `ollama` · `lmstudio` · `custom`

Model lists are managed through the Admin UI. Add or remove models as needed.

## CLI

```bash
airelay list                              # List all providers and status
airelay status                            # Show running state and config
airelay switch deepseek/deepseek-v4-pro   # Hot-switch default model
```

## API Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/v1/messages` | POST | Claude Code (Anthropic Messages) |
| `/v1/responses` | POST | Codex CLI (OpenAI Responses) |
| `/v1/messages/count_tokens` | POST | Token counting |
| `/v1/models` | GET | Model list |
| `/health` | GET | Health check |
| `/admin` | GET | Web Admin UI |
| `/admin/api/config` | GET/POST | Read/update config |
| `/admin/api/provider` | POST | Add provider |
| `/admin/api/provider/{id}` | DELETE | Remove provider |
| `/admin/api/test` | POST | Test provider connection |
| `/admin/api/autostart` | GET/POST | Get/set auto-start on login |

## Tech Stack

Rust (edition 2021) — axum 0.8, tokio 1, reqwest 0.12, tray-icon 0.24, tao 0.35, serde + toml.

## License

MIT
