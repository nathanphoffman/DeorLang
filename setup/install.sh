#!/bin/sh
set -e

DEOR_HOME="$HOME/.deor"
BIN_DIR="$DEOR_HOME/bin"
LIB_DIR="$DEOR_HOME/lib"
ENV_FILE="$DEOR_HOME/env"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Installing Deor..."

mkdir -p "$BIN_DIR"
mkdir -p "$LIB_DIR"

echo "  Compiling transpiler..."
rustc -O -A warnings "$SCRIPT_DIR/out.rs" -o "$BIN_DIR/deor"

echo "  Installing lib/..."
cp -r "$REPO_ROOT/lib/." "$LIB_DIR/"

cat > "$ENV_FILE" << EOF
export PATH="$BIN_DIR:\$PATH"
export DEOR_LIB="$LIB_DIR"
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

echo ""
echo "Done! Restart your shell or run:"
echo "  . \"$ENV_FILE\""
