run file="examples/args_example.deor":
    DEOR_LIB=lib ./output/out {{file}} output/run.rs
    rustc -O output/run.rs -o output/run
    ./output/run

# Self-compiles the Deor transpiler, then runs it on `file`.
# -O: optimize the transpiler binary (cuts codegen time significantly)
run-deor file="examples/args_example.deor":
    ./output/out transpiler-deor/main.deor output/out.rs
    rustc -O output/out.rs -o output/out
    DEOR_LIB=lib ./output/out {{file}} output/run.rs
    rustc -O output/run.rs -o output/run
    ./output/run

build-transpiler:
    ./output/out transpiler-deor/main.deor output/out.rs
    
rebuild-binary:
    rm -rf output/out
    rustc -O output/out.rs -o output/out

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

sync-setup:
    cp output/out.rs setup/out.rs

test-examples:
    DEOR_LIB=lib ./output/out tests/run_examples.deor output/test_runner.rs
    rustc -O output/test_runner.rs -o output/test_runner
    ./output/test_runner

# Self-compiles the transpiler (Deor -> Rust -> binary), then runs the example
# suite through the freshly built binary.
build-and-test: build-transpiler rebuild-binary test-examples

