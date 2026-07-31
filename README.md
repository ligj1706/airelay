# airelay

[中文](README_CN.md)

Protocol relay for AI coding tools. Use any LLM with Claude Code or Codex CLI.

**3.0 MB single binary. Starts in milliseconds. Zero runtime dependencies. macOS menu bar app.**

## Supported Platforms

| Platform | Status |
|----------|--------|
| macOS (Apple Silicon) | Full support — binary + tray |
| macOS (Intel) | Full support — binary + tray |
| Linux (x86_64) | CLI + server (no tray) |
| Windows | CLI + server (tray untested) |

## What it does

Claude Code requires Anthropic's Claude models (paid subscription). Codex CLI requires OpenAI models.

airelay runs a local HTTP proxy that translates API protocols on the fly — Anthropic Messages ↔ OpenAI Chat Completions, OpenAI Responses ↔ Chat — so you can use DeepSeek, Kimi, GLM, Qwen, Ollama, or any OpenAI-compatible API with your favorite AI coding tools.

```
Claude Code ── Anthropic Messages ──▶ airelay ── OpenAI Chat ──▶ DeepSeek / Kimi / ...
Codex CLI  ── OpenAI Responses ────▶                          ▶ Ollama / LM Studio / ...
```

## Quick Start

### 1. Install

```bash
# One-liner (macOS / Linux)
curl -fsSL https://raw.githubusercontent.com/ligj1706/airelay/main/install.sh | bash

# Or from source (requires Rust)
git clone https://github.com/ligj1706/airelay.git
cd airelay && cargo build --release
```

### 2. Run

```bash
airelay
# Starts proxy on http://127.0.0.1:8082
# Admin UI at http://127.0.0.1:8082/admin
# Menu bar tray icon on macOS
```

### 3. Configure

Open `http://127.0.0.1:8082/admin` (or click tray icon → Open Admin):

1. Select a provider (e.g. DeepSeek)
2. Enter your API key
3. Click Test Connection
4. Save

### 4. Launch Claude Code

```bash
ANTHROPIC_BASE_URL=http://127.0.0.1:8082 ANTHROPIC_AUTH_TOKEN=any claude
```

Or with shell aliases:

```bash
alias ar="airelay &"
alias cc="ANTHROPIC_BASE_URL=http://127.0.0.1:8082 ANTHROPIC_AUTH_TOKEN=any claude"
```

Switch models inside Claude Code:

```
/model deepseek/deepseek-v4-pro
/model kimi/kimi-k3
```

## Features

- **Protocol translation** — Full Anthropic Messages ↔ OpenAI Chat Completions, plus OpenAI Responses (Codex)
- **SSE streaming** — Real-time stream conversion with thinking/reasoning, tool use, and token usage tracking
- **9 built-in providers** — DeepSeek, Kimi, GLM, MiniMax, Qwen, OpenAI, Ollama, LM Studio, plus custom
- **Web Admin UI** — Dropdown config, dark/light theme, Anthropic-inspired warm palette
- **Hot reload** — Config changes via Admin UI, CLI, or tray take effect immediately
- **macOS tray** — Menu bar icon with model switching, config access, and graceful quit
- **CLI** — `airelay switch <provider/model>`, `airelay list`, `airelay status`
- **Provider CRUD** — Add/remove third-party providers via Admin UI
- **Streaming usage** — Real output token counts from upstream SSE, shown in Claude Code
- **Connection pooling** — Shared reqwest client across requests
- **Graceful shutdown** — Tray quit triggers graceful shutdown, drains in-flight SSE streams

## Preset Providers

| Provider | Models |
|----------|--------|
| `deepseek` | deepseek-v4-pro, deepseek-v4-flash |
| `kimi` | kimi-k3, kimi-k2.6, kimi-k2.7-code |
| `glm` | glm-5.2, glm-5.1, glm-4.7-flash |
| `minimax` | MiniMax-M3, MiniMax-M2.7 |
| `qwen` | qwen3-coder-next, qwen3-coder-plus, qwen3.7-max |
| `openai` | gpt-5.4, gpt-5.4-mini |
| `ollama` | qwen3-coder:latest, deepseek-r1:latest |
| `lmstudio` | auto |
| `custom` | your-model-name |

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

## Tech Stack

Rust (edition 2021) — axum 0.8, tokio 1, reqwest 0.12, tray-icon 0.24, tao 0.35, serde + toml.

## License

MIT
