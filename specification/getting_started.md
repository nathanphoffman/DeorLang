# Getting Started
To get started with Deor you will need to install cargo/rust, then pull the latest version from the git repo.

## Install Cargo (if you don't have it)
It is worth [visiting this link](https://rust-lang.org/tools/install/) for the latest cargo instructions, below is the latest linux/macOS install for cargo (as of June 18th, 2026).  If you are on windows try one of the [standalone installers](https://forge.rust-lang.org/infra/other-installation-methods.html#standalone).

### Linux Cargo Install
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Pull Latest Deor From GitHub
```
git clone https://github.com/nathanphoffman/DeorLang
```

For manual instructions:

1. Pull down or open up ```https://github.com/nathanphoffman/DeorLang```
2. Copy out the setup folder (the rest you don't need) into a new project folder of your choosing
3. Run ```cargo build``` in this folder (using a command line)
4. Run ```just run```
5. You should see hello world, you are ready to begin!

## Important things to note
- Most importantly **deor is in extreme early revision** do not build production apps with it.
- The /lib folder contains deor-language wrappers of useful rust functions
- The rust compiler output suppresses warnings (as deor does overly-safe cloning to keep it pure) if you run it manually without the flag provided by just, you will see warnings, this is normal.
- You can find a vscode extension in the folder that can be installed to give better language highlighting but no syntax-checking and support is early with it.
- As a general rule of thumb: for optimal performance use `rust` blocks; otherwise write standard Deor

