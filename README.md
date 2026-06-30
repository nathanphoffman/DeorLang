# Deor

Deor is a small, highly-procedural, tabbed-block language that transpiles to Rust. It enforces near-book readability, explicit typing, and predictable control flow — with a `rust` block escape hatch for when you need the full language.

Its goal is a comfier entrance point to Rust: simple syntactical sugar, uniform composition rules, and so little room for debate that when there *is* debate, you just drop into `rust {}`.

Inspired by Python's opinionated readability, Go's lightweight base syntax, and Rust's type system.

---

## Install

**Prerequisites:** [Rust / rustc](https://rustup.rs) must be installed.

**Unix — one-liner** (no clone needed):
```sh
curl -sSf https://raw.githubusercontent.com/nathanphoffman/DeorLang/main/setup/install.sh | sh
```

**Unix — manual** (if you prefer to inspect before running):
```sh
git clone https://github.com/nathanphoffman/DeorLang
cd DeorLang
bash setup/install.sh
```

Both options install the `deor` binary and standard library to `~/.deor/` and patch your `.bashrc`/`.zshrc`. Restart your shell (or run `. ~/.deor/env`) when done.

**Windows (PowerShell)**
```powershell
git clone https://github.com/nathanphoffman/DeorLang
cd DeorLang
.\setup\install.ps1
```
Restart your terminal. The binary is added to your user PATH automatically.

---

## Working on an existing Deor project

If you cloned a project that already uses Deor and just need the toolchain (the Deor programming language installed).

```sh
just install-deor
```

Or directly:
```sh
curl -sSf https://raw.githubusercontent.com/nathanphoffman/DeorLang/main/setup/install-deor.sh | sh
```

**Windows (PowerShell)**
```powershell
Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/nathanphoffman/DeorLang/main/setup/install-deor.ps1" -UseBasicParsing).Content
```

This installs only the `deor` binary and sets up your PATH — it does not create or modify any project files.

---

## Update

Pull the latest transpiler and standard library into an existing project:

Run this from inside your project directory and it will recompile `~/.deor/bin/deor` and refresh the `lib/` files in place:

```sh
curl -sSf https://raw.githubusercontent.com/nathanphoffman/DeorLang/main/setup/update.sh | sh
```


To target a specific project path instead:

```sh
curl -sSf https://raw.githubusercontent.com/nathanphoffman/DeorLang/main/setup/update.sh | sh -s -- /path/to/your/project
```

---

## Quick start

During install you'll be prompted for a folder to create your starter project in:

```
Where would you like to create your starter project? (default: /your/current/dir/hello-deor):
```

Hit enter to accept the default or type a custom path. A `main.deor` file will be placed there. Once your shell is reloaded, run it:

```sh
# Unix / macOS
cd hello-deor
deor main.deor main.rs && rustc main.rs -o main && ./main

# Windows
cd hello-deor
deor main.deor main.rs; rustc main.rs -o main.exe; .\main.exe
```

You should see:
```
Hello, world!
```

---

## Working on the transpiler

The transpiler is written in Deor itself (self-hosted). Clone the repo, then install the prerequisites:

| Tool | Purpose |
|------|---------|
| [Rust](https://rustup.rs) | Compiling transpiled output |
| [just](https://github.com/casey/just) | Running project commands |
| [Node.js / npm](https://nodejs.org) | VS Code extension (optional) |

A prebuilt binary lives at `output/out` so you can run files immediately without rebuilding.

**Run a Deor file**
```sh
just run                          # runs examples/hello.deor
just run file=path/to/file.deor   # runs a specific file
```

**Rebuild the transpiler**
```sh
just build-transpiler   # transpile transpiler-deor/main.deor -> output/out.rs
just rebuild-binary     # compile output/out.rs -> output/out
```

Or self-compile and run in one shot:
```sh
just run-deor
```

**VS Code extension** — syntax highlighting and diagnostics for `.deor` files:
```sh
just install-ext
```
After installing, reload VS Code (`Ctrl+Shift+P` → `Reload Window`).
