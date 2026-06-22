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

## Quick start

After installing, create a file `hello.deor`:

```
fn void main()
    print("Hello, world!")
```

Then run it:

```sh
# Unix
bash setup/run.sh hello.deor

# Windows
.\setup\run.ps1 hello.deor
```

Or directly once you have `deor` on your PATH:

```sh
deor hello.deor out.rs
rustc -O -A warnings out.rs -o hello
./hello
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
