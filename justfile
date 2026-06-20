run file="examples/hello.deor":
    ./output/out {{file}} output/run.rs
    rustc -O -A warnings output/run.rs -o output/run
    ./output/run

# Self-compiles the Deor transpiler, then runs it on `file`.
# -O: optimize the transpiler binary (cuts codegen time significantly)
# -A warnings: suppress Rust warnings from Deor-generated clone-heavy code
run-deor file="examples/hello.deor":
    ./output/out transpiler-deor/main.deor output/out.rs
    rustc -O -A warnings output/out.rs -o output/out
    ./output/out {{file}} output/run.rs
    rustc -O -A warnings output/run.rs -o output/run
    ./output/run

build-transpiler:
    ./output/out transpiler-deor/main.deor output/out.rs
    
rebuild-binary:
    rm -rf output/out
    rustc -O -A warnings output/out.rs -o output/out

install-ext:
    cd deor-vscode && npm install --silent && npm run compile
    cd deor-vscode && npx --yes vsce package --allow-missing-repository
    code --install-extension $(ls deor-vscode/*.vsix | tail -1)
    @echo "Done — reload VS Code window to apply."

install-ext-vsce:
    cd deor-vscode && npm install --silent && npm run compile
    cd deor-vscode && vsce package --allow-missing-repository
    code --install-extension $(ls deor-vscode/*.vsix | tail -1)
    @echo "Done — reload VS Code window to apply."

run-cargo-test:
    ./output/out examples/cargo_test/main.deor examples/cargo_test/output/main.rs
    cargo run --manifest-path examples/cargo_test/Cargo.toml

run-spec:
    cd specification && npm run start

run-splash:
    cd ./site && bun run dev

