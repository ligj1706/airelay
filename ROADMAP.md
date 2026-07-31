# Roadmap

## Done

- [x] Protocol translation: Anthropic Messages ↔ OpenAI Chat Completions
- [x] SSE streaming with thinking/reasoning, tool use, text content
- [x] Dual protocol endpoints: `/v1/messages` (Claude Code) + `/v1/responses` (Codex CLI)
- [x] 9 preset providers with custom provider support
- [x] `/v1/models` endpoint for Claude Code `/model` picker
- [x] Web Admin UI with dark/light theme, provider CRUD
- [x] Hot config reload (Admin UI / CLI / tray)
- [x] macOS system tray with model switching
- [x] CLI: `switch`, `list`, `status`
- [x] Streaming usage tracking (real output_tokens from upstream)
- [x] Connection pooling (shared reqwest client)
- [x] Graceful shutdown (tray quit drains in-flight SSE)
- [x] Release build: 3.0 MB, `opt-level="z"` + LTO + strip

## Planned

- [ ] Codex CLI end-to-end test — `/v1/responses` implemented but needs real Codex verification
- [ ] `reasoning_effort` mapping — refine `thinking.budget_tokens` → `reasoning_effort` conversion
- [ ] SSE stream recovery — auto-reconnect/retry on upstream stream interruption
- [ ] Multi-turn `tool_result` edge cases — Anthropic `tool_result` → OpenAI `tool` role boundary cases
- [ ] Image / multimodal — Anthropic image content block conversion needs testing

## Maybe

- [ ] Proxy authentication — currently passes through any `ANTHROPIC_AUTH_TOKEN`
- [ ] Local HTTPS / TLS
- [ ] Docker image
- [ ] Windows / Linux tray verification — code is cross-platform but only tested on macOS
- [ ] Homebrew formula
