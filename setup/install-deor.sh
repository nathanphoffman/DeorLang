#!/bin/sh
set -e

REPO="nathanphoffman/DeorLang"
BRANCH="main"

DEOR_HOME="$HOME/.deor"
BIN_DIR="$DEOR_HOME/bin"
ENV_FILE="$DEOR_HOME/env"

# Dual-mode: use local files if running from a clone, otherwise fetch from GitHub
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd 2>/dev/null)" || SCRIPT_DIR=""
if [ -f "$SCRIPT_DIR/out.rs" ]; then
    OUT_RS="$SCRIPT_DIR/out.rs"
    VSIX_SRC="$SCRIPT_DIR/deor-lang.vsix"
else
    echo "  Downloading from GitHub..."
    TMP="$(mktemp -d)"
    trap 'rm -rf "$TMP"' EXIT
    curl -sL "https://github.com/$REPO/archive/refs/heads/$BRANCH.tar.gz" \
        | tar xz -C "$TMP"
    OUT_RS="$TMP/DeorLang-$BRANCH/setup/out.rs"
    VSIX_SRC="$TMP/DeorLang-$BRANCH/setup/deor-lang.vsix"
fi

echo "Installing Deor..."

mkdir -p "$BIN_DIR"

echo "  Compiling transpiler..."
rustc -O "$OUT_RS" -o "$BIN_DIR/deor"

cat > "$ENV_FILE" << EOF
export PATH="$BIN_DIR:\$PATH"
EOF

add_source_line() {
    RC="$1"
    LINE=". \"$ENV_FILE\""
    if [ -f "$RC" ] && grep -qF "$ENV_FILE" "$RC"; then
        return
    fi
    if [ -f "$RC" ]; then
        printf '\n# Deor\n%s\n' "$LINE" >> "$RC"
        echo "  Patched $RC"
    fi
}

add_source_line "$HOME/.bashrc"
add_source_line "$HOME/.zshrc"

printf "Install the Deor VS Code extension? [y/N]: " > /dev/tty
read -r INSTALL_EXT < /dev/tty
case "$INSTALL_EXT" in
    [yY]*)
        if command -v code > /dev/null 2>&1; then
            echo "  Installing VS Code extension..."
            code --install-extension "$VSIX_SRC"
        else
            echo "  VS Code 'code' command not found in PATH; skipping extension install."
        fi
        ;;
esac

echo ""
echo "Done! Restart your shell or run:"
echo "  . \"$ENV_FILE\""
