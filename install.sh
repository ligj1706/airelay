#!/usr/bin/env bash
set -e

REPO="ligj1706/airelay"
VERSION="${1:-latest}"

# Auto-detect platform
case "$(uname -s)" in
    Darwin)
        ARCH=$(uname -m)
        case "$ARCH" in
            arm64)  NAME="macos-arm64" ;;
            x86_64) NAME="macos-x86_64" ;;
            *)      echo "不支持 Mac 架构: $ARCH"; exit 1 ;;
        esac
        INSTALL_DIR="/usr/local/bin"
        ;;
    Linux)
        ARCH=$(uname -m)
        case "$ARCH" in
            x86_64) NAME="linux-x86_64" ;;
            *)      echo "不支持 Linux 架构: $ARCH"; exit 1 ;;
        esac
        INSTALL_DIR="$HOME/.local/bin"
        ;;
    *)
        echo "不支持的操作系统: $(uname -s)"; exit 1
        ;;
esac

echo "→ 检测到: $NAME"
echo "→ 安装到: $INSTALL_DIR"

if [ "$VERSION" = "latest" ]; then
    URL="https://github.com/$REPO/releases/latest/download/airelay-$NAME.tar.gz"
else
    URL="https://github.com/$REPO/releases/download/$VERSION/airelay-$NAME.tar.gz"
fi

echo "→ 下载: $URL"

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

curl -fsSL "$URL" -o "$TMPDIR/airelay.tar.gz" || {
    echo "下载失败。请确认:"
    echo "  1. 网络连接正常"
    echo "  2. GitHub Release 已发布 (tag 格式: v0.2.0)"
    echo "  3. REPO 名称正确 (当前: $REPO)"
    exit 1
}

tar -xzf "$TMPDIR/airelay.tar.gz" -C "$TMPDIR"

# Install
mkdir -p "$INSTALL_DIR"
cp "$TMPDIR/airelay" "$INSTALL_DIR/airelay"
chmod +x "$INSTALL_DIR/airelay"

# macOS: strip quarantine
if [ "$(uname -s)" = "Darwin" ]; then
    xattr -cr "$INSTALL_DIR/airelay" 2>/dev/null || true
fi

echo ""
echo "✓ airelay 安装完成: $INSTALL_DIR/airelay"
echo ""
echo "使用方法:"
echo "  airelay              # 启动代理 + 托盘"
echo "  airelay list         # 列出所有提供商"
echo "  airelay switch p/m   # 切换模型"
echo "  airelay status       # 查看状态"
echo ""
echo "配置页面: http://localhost:8082/admin"
echo ""
echo "快速别名 (添加到 ~/.bashrc 或 ~/.zshrc):"
echo '  alias lx="ANTHROPIC_BASE_URL=http://127.0.0.1:8082 ANTHROPIC_AUTH_TOKEN=any claude"'
