#!/usr/bin/env bash
set -e

REPO="ligj1706/airelay"
VERSION="${1:-latest}"

case "$(uname -s)" in
    Darwin)
        ARCH=$(uname -m)
        case "$ARCH" in
            arm64)  NAME="macos-arm64" ;;
            x86_64) NAME="macos-x86_64" ;;
            *)      echo "不支持 Mac 架构: $ARCH"; exit 1 ;;
        esac
        IS_MACOS=1
        INSTALL_DIR="$HOME/.local/bin"
        LAUNCHD_PLIST="$HOME/Library/LaunchAgents/com.airelay.proxy.plist"
        ;;
    Linux)
        ARCH=$(uname -m)
        case "$ARCH" in
            x86_64) NAME="linux-x86_64" ;;
            *)      echo "不支持 Linux 架构: $ARCH"; exit 1 ;;
        esac
        IS_MACOS=0
        INSTALL_DIR="$HOME/.local/bin"
        ;;
    *)
        echo "不支持的操作系统: $(uname -s)"; exit 1
        ;;
esac

mkdir -p "$INSTALL_DIR"

echo "→ 检测到: $NAME"
echo "→ 安装到: $INSTALL_DIR"
echo ""

if [ "$VERSION" = "latest" ]; then
    URL="https://github.com/$REPO/releases/latest/download/airelay-$NAME.tar.gz"
else
    URL="https://github.com/$REPO/releases/download/$VERSION/airelay-$NAME.tar.gz"
fi

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

echo "→ 下载: $URL"
curl -fsSL "$URL" -o "$TMPDIR/airelay.tar.gz" || {
    echo "下载失败。请确认:"
    echo "  1. 网络连接正常"
    echo "  2. GitHub Release 已发布"
    exit 1
}

tar -xzf "$TMPDIR/airelay.tar.gz" -C "$TMPDIR"
cp "$TMPDIR/airelay" "$INSTALL_DIR/airelay"
chmod +x "$INSTALL_DIR/airelay"

# macOS: strip quarantine
if [ "$IS_MACOS" = "1" ]; then
    xattr -cr "$INSTALL_DIR/airelay" 2>/dev/null || true
fi

echo "✓ 二进制安装完成"
echo ""

# === 检测 shell 配置文件 ===
SHELL_RC=""
case "$SHELL" in
    */zsh)  SHELL_RC="$HOME/.zshrc" ;;
    */bash) SHELL_RC="$HOME/.bashrc" ;;
    */fish) SHELL_RC="$HOME/.config/fish/config.fish" ;;
esac

[ -z "$SHELL_RC" ] && SHELL_RC="$HOME/.zshrc"
touch "$SHELL_RC"

# === 添加到 PATH ===
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
    echo "✓ PATH 已添加: $SHELL_RC"
fi

# === 添加别名 (v4) ===
ALIASES_ADDED=0
MARKER_V4="# === airelay v4 ==="

if grep -qF "$MARKER_V4" "$SHELL_RC" 2>/dev/null; then
    echo "✓ 别名已是最新版本 (v4)"
    ALIASES_ADDED=1
else
    # 清理旧版冲突项
    if grep -q "airelay" "$SHELL_RC" 2>/dev/null; then
        echo "→ 检测到旧版 airelay 配置，正在升级..."
        # 删除所有旧版 airelay 标记行
        sed -i '' '/^# === airelay/d' "$SHELL_RC" 2>/dev/null || sed -i '/^# === airelay/d' "$SHELL_RC"
        # 删除旧的 alias ar=... 行
        sed -i '' '/^alias ar=.*airelay/d' "$SHELL_RC" 2>/dev/null || sed -i '/^alias ar=.*airelay/d' "$SHELL_RC"
    fi

    cat >> "$SHELL_RC" << 'ALIASEOF'

# === airelay v4 ===
alias ar='curl -s -o /dev/null http://127.0.0.1:8082/health 2>/dev/null || (airelay --no-tray >/dev/null 2>&1 &); pgrep -f "airelay tray" >/dev/null 2>&1 || (airelay tray >/dev/null 2>&1 &); echo "airelay 就绪: 服务 + 入口"'
export ANTHROPIC_BASE_URL=http://127.0.0.1:8082
export ANTHROPIC_AUTH_TOKEN=any
export OPENAI_BASE_URL=http://127.0.0.1:8082/v1

# 开机自启管理 (仅 macOS)
airelay-autostart() {
    local PLIST="$HOME/Library/LaunchAgents/com.airelay.proxy.plist"
    case "${1:-status}" in
        on)
            mkdir -p "$(dirname "$PLIST")"
            cat > "$PLIST" << PLISTEOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
    <key>Label</key><string>com.airelay.proxy</string>
    <key>ProgramArguments</key><array><string>$HOME/.local/bin/airelay</string></array>
    <key>RunAtLoad</key><true/>
    <key>KeepAlive</key><true/>
    <key>StandardOutPath</key><string>/tmp/airelay.stdout.log</string>
    <key>StandardErrorPath</key><string>/tmp/airelay.stderr.log</string>
</dict></plist>
PLISTEOF
            launchctl load "$PLIST" 2>/dev/null
            echo "✓ airelay 开机自启已启用"
            ;;
        off)
            launchctl unload "$PLIST" 2>/dev/null
            rm -f "$PLIST"
            echo "✓ airelay 开机自启已关闭"
            ;;
        status)
            if [ -f "$PLIST" ]; then echo "状态: 已启用"; else echo "状态: 已关闭"; fi
            curl -s http://127.0.0.1:8082/health >/dev/null 2>&1 && echo "运行: ✅" || echo "运行: ❌"
            ;;
    esac
}
ALIASEOF
    ALIASES_ADDED=1
    echo "✓ 别名已添加: ar / airelay-autostart (v4)"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  airelay 安装完成!"
echo ""
echo "  用法:"
echo "    ar       启动代理"
echo "    claude   启动 Claude Code（已自动配置环境变量）"
echo "    codex    启动 Codex CLI（已自动配置环境变量）"
echo ""
echo "  开机自启: Admin UI → 开机自启开关，或:"
echo "    airelay-autostart on/off/status"
echo ""
echo "  配置页面:  http://127.0.0.1:8082/admin"
echo ""
echo "  新终端生效: exec \$SHELL  或重新打开窗口"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
