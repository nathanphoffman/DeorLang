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
    GITIGNORE_SRC="$SCRIPT_DIR/.gitignore"
    CARGO_SRC="$SCRIPT_DIR/Cargo.toml"
    JUSTFILE_SRC="$SCRIPT_DIR/justfile"
else
    echo "  Downloading from GitHub..."
    TMP="$(mktemp -d)"
    trap 'rm -rf "$TMP"' EXIT
    curl -sL "https://github.com/$REPO/archive/refs/heads/$BRANCH.tar.gz" \
        | tar xz -C "$TMP"
    OUT_RS="$TMP/DeorLang-$BRANCH/setup/out.rs"
    LIB_SRC="$TMP/DeorLang-$BRANCH/lib"
    HELLO_SRC="$TMP/DeorLang-$BRANCH/setup/hello.deor"
    GITIGNORE_SRC="$TMP/DeorLang-$BRANCH/setup/.gitignore"
    CARGO_SRC="$TMP/DeorLang-$BRANCH/setup/Cargo.toml"
    JUSTFILE_SRC="$TMP/DeorLang-$BRANCH/setup/justfile"
fi

printf "Project name (default: hello): " > /dev/tty
read -r PROJECT_NAME < /dev/tty
[ -z "$PROJECT_NAME" ] && PROJECT_NAME="hello"

DEFAULT_PROJECT="$(pwd)/$PROJECT_NAME"
while true; do
    printf "Where would you like to create your project? (default: %s): " "$DEFAULT_PROJECT" > /dev/tty
    read -r PROJECT_DIR < /dev/tty
    [ -z "$PROJECT_DIR" ] && PROJECT_DIR="$DEFAULT_PROJECT"
    case "$PROJECT_DIR" in
        "~/"*) PROJECT_DIR="$HOME/${PROJECT_DIR#~/}" ;;
        "~")   PROJECT_DIR="$HOME" ;;
    esac

    if [ -f "$PROJECT_DIR" ]; then
        echo "  Error: '$PROJECT_DIR' is a file, not a directory. Please choose a different path." > /dev/tty
        continue
    fi

    if [ -d "$PROJECT_DIR" ]; then
        printf "  '%s' already exists. Install project there anyway? [Y/n]: " "$PROJECT_DIR" > /dev/tty
        read -r CONFIRM < /dev/tty
        case "$CONFIRM" in
            [nN]*) echo "  Aborted." > /dev/tty; exit 0 ;;
        esac
    fi

    break
done

echo "Installing Deor..."

mkdir -p "$BIN_DIR"
mkdir -p "$LIB_DIR"
mkdir -p "$PROJECT_DIR"

if ! command -v just > /dev/null 2>&1; then
    echo "  Installing just..."
    cargo install just
fi

echo "  Compiling transpiler..."
rustc -O -A warnings "$OUT_RS" -o "$BIN_DIR/deor"

echo "  Installing lib/..."
cp -r "$LIB_SRC/." "$LIB_DIR/"

echo "  Creating starter project..."
cp "$HELLO_SRC" "$PROJECT_DIR/hello.deor"
cp "$GITIGNORE_SRC" "$PROJECT_DIR/.gitignore"
cp "$JUSTFILE_SRC" "$PROJECT_DIR/justfile"
sed "s/{{PROJECT_NAME}}/$PROJECT_NAME/g" "$CARGO_SRC" > "$PROJECT_DIR/Cargo.toml"

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
echo "  just run"
echo ""
echo "  (Without just: deor hello.deor build/main.rs && cargo run)"
