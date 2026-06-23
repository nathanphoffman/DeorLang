#!/bin/sh
set -e

REPO="nathanphoffman/DeorLang"
BRANCH="main"

DEOR_HOME="$HOME/.deor"
BIN_DIR="$DEOR_HOME/bin"

# Dual-mode: use local files if running from a clone, otherwise fetch from GitHub
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd 2>/dev/null)" || SCRIPT_DIR=""
if [ -f "$SCRIPT_DIR/out.rs" ] && [ -d "$SCRIPT_DIR/../lib" ]; then
    OUT_RS="$SCRIPT_DIR/out.rs"
    LIB_SRC="$SCRIPT_DIR/../lib"
else
    echo "  Downloading latest from GitHub..."
    TMP="$(mktemp -d)"
    trap 'rm -rf "$TMP"' EXIT
    curl -sL "https://github.com/$REPO/archive/refs/heads/$BRANCH.tar.gz" \
        | tar xz -C "$TMP"
    OUT_RS="$TMP/DeorLang-$BRANCH/setup/out.rs"
    LIB_SRC="$TMP/DeorLang-$BRANCH/lib"
fi

echo "Updating Deor..."

echo "  Recompiling transpiler..."
mkdir -p "$BIN_DIR"
rustc -O "$OUT_RS" -o "$BIN_DIR/deor"
echo "  Transpiler updated at $BIN_DIR/deor"

# Determine project directory to update libs in
if [ -n "$1" ]; then
    PROJECT_DIR="$1"
    case "$PROJECT_DIR" in
        "~/"*) PROJECT_DIR="$HOME/${PROJECT_DIR#~/}" ;;
        "~")   PROJECT_DIR="$HOME" ;;
    esac
elif [ -d "$(pwd)/lib" ] && ls "$(pwd)/lib/"*.deor > /dev/null 2>&1; then
    PROJECT_DIR="$(pwd)"
else
    PROJECT_DIR=""
fi

if [ -n "$PROJECT_DIR" ]; then
    if [ ! -d "$PROJECT_DIR" ]; then
        echo "  Error: '$PROJECT_DIR' is not a directory." >&2
        exit 1
    fi
    echo "  Updating lib files in $PROJECT_DIR/lib/ ..."
    mkdir -p "$PROJECT_DIR/lib"
    cp -r "$LIB_SRC/." "$PROJECT_DIR/lib/"
    echo "  Libs updated."
else
    echo ""
    echo "  No project directory detected. To also update a project's lib files:"
    echo "    cd /path/to/your/project && curl -sSf https://raw.githubusercontent.com/$REPO/main/setup/update.sh | sh"
    echo "  or: curl -sSf https://raw.githubusercontent.com/$REPO/main/setup/update.sh | sh -s -- /path/to/your/project"
fi

echo ""
echo "Done! Run 'deor --version' or 'just run' in your project to verify."
