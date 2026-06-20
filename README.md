# Deor

A compiled language that transpiles to Rust.

## Prerequisites

| Tool | Purpose |
|------|---------|
| [Rust](https://rustup.rs) | Compiling transpiled output |
| [just](https://github.com/casey/just) | Running project commands |
| [Node.js / npm](https://nodejs.org) | VS Code extension (optional) |

## Running a Deor file

A prebuilt transpiler binary is included at `output/out`.

```sh
just run                        # runs examples/hello.deor
just run file=path/to/file.deor # runs a specific file
```

## Rebuilding the transpiler

The transpiler is written in Deor itself. To recompile it:

```sh
just build-transpiler   # transpile main.deor -> out.rs
just rebuild-binary     # compile out.rs -> output/out
```

Or do both in one shot (runs the result on a file too):

```sh
just run-deor
```

## VS Code Extension

Provides syntax highlighting and diagnostics for `.deor` files.

**Option 1 — via npx** (no global install needed, just npm):
```sh
just install-ext
```

**Option 2 — via global `vsce`** (install once, faster after that):
```sh
npm install -g @vscode/vsce
just install-ext-vsce
```

After installing, reload VS Code (`Ctrl+Shift+P` → `Reload Window`).
