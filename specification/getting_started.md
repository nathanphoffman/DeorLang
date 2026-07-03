<!-- title: Deor Specification -->
<!-- [Deor Specification](main.md) -->
<!-- themes: dusk -->

# Getting Started
To get started with Deor you will need to install cargo/rust, then pull the latest version from the git repo.

**Prerequisites:** [Rust / Cargo](https://rustup.rs) must be installed first

---

# Install Deor



### Unix Install — One Line:
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

### Windows (PowerShell)
```powershell
git clone https://github.com/nathanphoffman/DeorLang
cd DeorLang
.\setup\install.ps1
```
Restart your terminal. The binary is added to your user PATH automatically.



## Important things to note
- Most importantly **deor is in extreme early revision** do not build production apps with it.
- The /lib folder contains deor-language wrappers of useful rust functions
- The rust compiler output suppresses warnings (as deor does overly-safe cloning to keep it pure) if you run it manually without the flag provided by just, you will see warnings, this is normal.
- You can find a vscode extension in the folder that can be installed to give better language highlighting but no syntax-checking and support is early with it.
- As a general rule of thumb: for optimal performance use `rust` blocks; otherwise write standard Deor

