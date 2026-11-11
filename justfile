# Runs a scratch file from tests/snippets/ (your own scratchpad, not a real test).
run file="scratch_pad.deor" *args:
    DEOR_LIB=lib ./output/out tests/snippets/{{file}} output/run.rs
    rustc -O output/run.rs -o output/run
    ./output/run {{args}}

# Runs a single file from tests/unit_tests/ outside the full suite.
run-test file="crash_test.deor":
    DEOR_LIB=lib ./output/out tests/unit_tests/{{file}} output/run.rs
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
    ./output/out tests/unit_tests/cargo_test/main.deor tests/unit_tests/cargo_test/output/main.rs
    cargo run --manifest-path tests/unit_tests/cargo_test/Cargo.toml

run-spec:
    cd specification && npm run start

run-splash:
    cd ./site && bun run dev

sync-setup:
    cp output/out.rs setup/out.rs
    rm -rf setup/deor_specification
    mkdir -p setup/deor_specification
    cp -r specification/. setup/deor_specification/
    rm -rf setup/deor_specification/node_modules setup/deor_specification/railway.json
    cd deor-vscode && npm install --silent && npm run compile
    cd deor-vscode && npx --yes vsce package --allow-missing-repository
    cp "$(ls -t deor-vscode/*.vsix | head -1)" setup/deor-lang.vsix

# Runs every *.test.deor file under the current directory (see test_framework_plan.md).
test path=".":
    DEOR_LIB=lib ./output/out test {{path}}

test-examples:
    DEOR_LIB=lib ./output/out tests/unit_tester.deor output/test_runner.rs
    rustc -O output/test_runner.rs -o output/test_runner
    ./output/test_runner

# Self-compiles the transpiler (Deor -> Rust -> binary), then runs the example
# suite through the freshly built binary.
build-and-test: build-transpiler rebuild-binary test-examples


# Samples where the transpiler spends its time (needs gdb; perf is not required).
# Report goes to output/profile.txt. Pass another .deor file to profile that instead.
profile file="transpiler-deor/main.deor":
    DEOR_LIB=lib PROFILE_OUT=output/profile.txt PROFILE_ARGS="{{file}} output/profile_run.rs" gdb -q -batch -ex "set pagination off" -ex "set confirm off" -ex "handle SIGINT stop nopass" -x tools/profile_sampler.py ./output/out < /dev/null > /dev/null 2>&1
    @cat output/profile.txt

# Runs the code quality checker, skipping the generated out.rs files.
check:
    codequality . --full --exclude '*out.rs*'
