#!/bin/sh
set -e

REPO="nathanphoffman/DeorLang"
BRANCH="main"

DEOR_HOME="$HOME/.deor"
BIN_DIR="$DEOR_HOME/bin"
LIB_DIR="$DEOR_HOME/lib"
ENV_FILE="$DEOR_HOME/env"

# Dual-mode: use local files if running from a clone, otherwise fetch from GitHub
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd 2>/dev/null)" || SCRIPT_DIR=""
if [ -f "$SCRIPT_DIR/out.rs" ] && [ -d "$SCRIPT_DIR/../lib" ]; then
    OUT_RS="$SCRIPT_DIR/out.rs"
    LIB_SRC="$SCRIPT_DIR/../lib"
    HELLO_SRC="$SCRIPT_DIR/hello.deor"
else
    echo "  Downloading from GitHub..."
    TMP="$(mktemp -d)"
    trap 'rm -rf "$TMP"' EXIT
    curl -sL "https://github.com/$REPO/archive/refs/heads/$BRANCH.tar.gz" \
        | tar xz -C "$TMP"
    OUT_RS="$TMP/DeorLang-$BRANCH/setup/out.rs"
    LIB_SRC="$TMP/DeorLang-$BRANCH/lib"
    HELLO_SRC="$TMP/DeorLang-$BRANCH/setup/hello.deor"
fi

DEFAULT_PROJECT="$(pwd)/hello-deor"
printf "Where would you like to create your starter project? (default: %s): " "$DEFAULT_PROJECT" >&2
read -r PROJECT_DIR < /dev/tty
if [ -z "$PROJECT_DIR" ]; then
    PROJECT_DIR="$DEFAULT_PROJECT"
fi

if [ -d "$PROJECT_DIR" ]; then
    printf "  '%s' already exists. Continue anyway? [y/N]: " "$PROJECT_DIR" >&2
    read -r CONFIRM < /dev/tty
    case "$CONFIRM" in
        [yY]*) ;;
        *) echo "Aborted."; exit 0 ;;
    esac
fi

echo "Installing Deor..."

mkdir -p "$BIN_DIR"
mkdir -p "$LIB_DIR"
mkdir -p "$PROJECT_DIR"

echo "  Compiling transpiler..."
rustc -O -A warnings "$OUT_RS" -o "$BIN_DIR/deor"

echo "  Installing lib/..."
cp -r "$LIB_SRC/." "$LIB_DIR/"

echo "  Creating starter project..."
cp "$HELLO_SRC" "$PROJECT_DIR/hello.deor"

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
echo ""
echo "To run your hello world:"
echo "  cd \"$PROJECT_DIR\""
echo "  deor hello.deor hello.rs && rustc hello.rs -o hello && ./hello"
